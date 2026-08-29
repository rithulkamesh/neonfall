use bytemuck::{Pod, Zeroable};
use tracing::{debug, info, instrument, trace};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Device, Extent3d, Queue, Sampler,
    SamplerBindingType, SamplerDescriptor, ShaderStages, Texture, TextureDescriptor,
    TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView,
    TextureViewDescriptor, util::DeviceExt,
};

/// CPU-side pixel data uploaded to the GPU when the renderer starts.
#[derive(Debug, Clone)]
pub struct NFTextureImage {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

impl NFTextureImage {
    pub fn white() -> Self {
        Self::solid([255, 255, 255])
    }

    #[instrument(name = "texture.solid", level = "debug", fields(r = rgb[0], g = rgb[1], b = rgb[2]))]
    pub fn solid(rgb: [u8; 3]) -> Self {
        trace!("creating 1x1 solid texture swatch");
        Self {
            width: 1,
            height: 1,
            rgba: vec![rgb[0], rgb[1], rgb[2], 255],
        }
    }

    /// Pack solid-color swatches into a square atlas (one cell per color).
    #[instrument(name = "texture.color_atlas", skip(colors), fields(color_count = colors.len()))]
    pub fn color_atlas(colors: &[[u8; 3]]) -> (Self, u32) {
        let count = colors.len().max(1) as u32;
        let grid = ((count as f32).sqrt().ceil() as u32).next_power_of_two().max(1);
        let cell = 8;
        let width = grid * cell;
        let height = grid * cell;
        let mut rgba = vec![255u8; (width * height * 4) as usize];

        for (index, rgb) in colors.iter().enumerate() {
            let col = index as u32 % grid;
            let row = index as u32 / grid;
            for y in 0..cell {
                for x in 0..cell {
                    let px = (col * cell + x) as usize;
                    let py = (row * cell + y) as usize;
                    let offset = (py * width as usize + px) * 4;
                    rgba[offset..offset + 3].copy_from_slice(rgb);
                    rgba[offset + 3] = 255;
                }
            }
        }

        debug!(
            grid,
            width,
            height,
            bytes = rgba.len(),
            "packed runtime color atlas"
        );

        (Self { width, height, rgba }, grid)
    }

    #[instrument(
        name = "texture.from_gltf",
        skip(image),
        fields(width = image.width, height = image.height, bytes = image.pixels.len())
    )]
    pub fn from_gltf(image: &gltf::image::Data) -> Self {
        debug!("copying glTF image into cpu texture buffer");
        Self {
            width: image.width,
            height: image.height,
            rgba: image.pixels.clone(),
        }
    }
}

/// Atlas layout for indexed instance sampling (`@group(0) @binding(2)`).
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct NFAtlasUniform {
    pub grid: f32,
    pub _pad: [f32; 3],
}

impl NFAtlasUniform {
    pub fn new(grid: u32) -> Self {
        Self {
            grid: grid as f32,
            _pad: [0.0; 3],
        }
    }
}

/// A single GPU texture with its view and sampler.
struct NFTexture {
    _texture: Texture,
    view: TextureView,
    sampler: Sampler,
}

impl NFTexture {
    #[instrument(
        name = "texture.from_rgba",
        skip(device, queue, rgba),
        fields(label, width, height, bytes = rgba.len())
    )]
    fn from_rgba(
        device: &Device,
        queue: &Queue,
        label: &str,
        width: u32,
        height: u32,
        rgba: &[u8],
    ) -> Self {
        debug!(label, "uploading texture to gpu");

        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            texture.as_image_copy(),
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        Self {
            _texture: texture,
            view,
            sampler,
        }
    }
}

/// Material textures and their bind group (`@group(0)` in the shader).
pub struct NFTextures {
    _diffuse: NFTexture,
    _atlas_buffer: wgpu::Buffer,
    bind_group: BindGroup,
}

impl NFTextures {
    #[instrument(name = "textures.layout", skip(device))]
    pub fn bind_group_layout(device: &Device) -> BindGroupLayout {
        trace!("creating textures bind group layout");
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("textures_bind_group_layout"),
        })
    }

    #[instrument(name = "textures.white", skip(device, queue, layout))]
    pub fn white(device: &Device, queue: &Queue, layout: &BindGroupLayout) -> Self {
        debug!("using default white diffuse texture");
        Self::from_image(device, queue, layout, &NFTextureImage::white(), 1)
    }

    #[instrument(
        name = "textures.from_image",
        skip(device, queue, layout, image),
        fields(
            width = image.width,
            height = image.height,
            bytes = image.rgba.len(),
            atlas_grid
        )
    )]
    pub fn from_image(
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
        image: &NFTextureImage,
        atlas_grid: u32,
    ) -> Self {
        info!(
            width = image.width,
            height = image.height,
            atlas_grid,
            "material textures ready"
        );

        let diffuse = NFTexture::from_rgba(
            device,
            queue,
            "diffuse_texture",
            image.width,
            image.height,
            &image.rgba,
        );

        let atlas_uniform = NFAtlasUniform::new(atlas_grid);
        let atlas_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Atlas Uniform Buffer"),
            contents: bytemuck::bytes_of(&atlas_uniform),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&diffuse.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&diffuse.sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: atlas_buffer.as_entire_binding(),
                },
            ],
            label: Some("textures_bind_group"),
        });

        Self {
            _diffuse: diffuse,
            _atlas_buffer: atlas_buffer,
            bind_group,
        }
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}
