use glam::Mat4;
use std::sync::Arc;
use tracing::{debug, info, instrument, warn};
use wgpu::{
    Buffer, Color, CommandEncoderDescriptor, CurrentSurfaceTexture, LoadOp, Operations,
    RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp,
    TextureViewDescriptor, util::DeviceExt,
};

use winit::{dpi::PhysicalSize, window::Window};

use crate::gpu::{
    NFGpu, NFDepthConfig, NFDepthTexture, NFInstance, NFInstanceRaw, NFMesh, NFPipeline, NFTextures,
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    fn update_view_proj(&mut self, view_proj: Mat4) {
        self.view_proj = view_proj.to_cols_array_2d();
    }
}

pub struct NFState {
    gpu: NFGpu,
    is_surface_configured: bool,
    window: Arc<Window>,
    render_pipeline: NFPipeline,
    textures: NFTextures,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,

    camera_buffer: Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_uniform: CameraUniform,
    instances: Vec<NFInstance>,
    instance_buffer: Buffer,
    depth_config: NFDepthConfig,
    depth_texture: Option<NFDepthTexture>,
    clear_color: Color,
}

impl NFState {
    #[instrument(
        name = "state.new",
        skip(window, mesh, depth),
        fields(
            instance_count = mesh.instances.len(),
            depth_enabled = depth.enabled,
            depth_clear = depth.clear
        ),
        err
    )]
    pub async fn new(
        window: Arc<Window>,
        mesh: &NFMesh,
        depth: NFDepthConfig,
        clear_color: Color,
    ) -> anyhow::Result<Self> {
        let instances = &mesh.instances;
        anyhow::ensure!(
            !instances.is_empty(),
            "at least one instance is required to render"
        );

        let gpu = NFGpu::new(window.clone()).await?;

        let textures_bind_group_layout = NFTextures::bind_group_layout(&gpu.device);

        let camera_bind_group_layout =
            gpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("camera_bind_group_layout"),
                });

        let render_pipeline = NFPipeline::new(
            &gpu.device,
            gpu.config.format,
            &textures_bind_group_layout,
            &camera_bind_group_layout,
            &depth,
        );

        debug!(
            has_diffuse = mesh.diffuse.is_some(),
            atlas_grid = mesh.atlas_grid,
            texture_width = mesh.diffuse.as_ref().map(|image| image.width),
            texture_height = mesh.diffuse.as_ref().map(|image| image.height),
            "uploading mesh textures"
        );

        let textures = match mesh.diffuse.as_ref() {
            Some(image) => NFTextures::from_image(
                &gpu.device,
                &gpu.queue,
                &textures_bind_group_layout,
                image,
                mesh.atlas_grid,
            ),
            None => {
                debug!("mesh has no diffuse; falling back to white placeholder");
                NFTextures::white(&gpu.device, &gpu.queue, &textures_bind_group_layout)
            }
        };

        let vertex_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&mesh.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        let instance_data: Vec<NFInstanceRaw> = instances.iter().map(NFInstance::to_raw).collect();
        let instance_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        let camera_uniform = CameraUniform::new();

        let camera_buffer = gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let camera_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let depth_texture = depth.enabled.then(|| {
            NFDepthTexture::new(
                &gpu.device,
                gpu.config.width,
                gpu.config.height,
                "depth_texture",
            )
        });

        info!(
            vertices = mesh.vertices.len(),
            indices = mesh.indices.len(),
            instances = instances.len(),
            has_diffuse = mesh.diffuse.is_some(),
            atlas_grid = mesh.atlas_grid,
            texture_width = mesh.diffuse.as_ref().map(|image| image.width),
            texture_height = mesh.diffuse.as_ref().map(|image| image.height),
            depth_enabled = depth.enabled,
            depth_clear = depth.clear,
            depth_width = depth_texture.as_ref().map(|_| gpu.config.width),
            depth_height = depth_texture.as_ref().map(|_| gpu.config.height),
            "renderer state ready"
        );

        Ok(Self {
            window,
            gpu,
            is_surface_configured: false,
            render_pipeline,
            textures,
            vertex_buffer,
            index_buffer,
            num_indices: mesh.indices.len() as u32,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            instances: instances.to_vec(),
            instance_buffer,
            depth_config: depth,
            depth_texture,
            clear_color,
        })
    }

    pub fn set_clear_color(&mut self, clear_color: Color) {
        self.clear_color = clear_color;
    }

    pub fn window_size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    #[instrument(name = "state.resize", skip(self), fields(width, height, depth_enabled = self.depth_config.enabled))]
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.gpu.config.width = width;
            self.gpu.config.height = height;
            self.gpu
                .surface
                .configure(&self.gpu.device, &self.gpu.config);
            self.is_surface_configured = true;

            if self.depth_config.enabled {
                self.depth_texture = Some(NFDepthTexture::new(
                    &self.gpu.device,
                    width,
                    height,
                    "depth_texture",
                ));
                debug!(width, height, "depth texture resized");
            }

            debug!(width, height, "surface resized");
        }
    }

    pub fn set_view_proj(&mut self, view_proj: Mat4) {
        self.camera_uniform.update_view_proj(view_proj);
        self.gpu.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    #[instrument(name = "state.set_instances", skip(self, instances), fields(instance_count = instances.len()), err)]
    pub fn set_instances(&mut self, instances: &[NFInstance]) -> anyhow::Result<()> {
        anyhow::ensure!(
            instances.len() == self.instances.len(),
            "instance count cannot change after renderer initialization"
        );

        let instance_data: Vec<NFInstanceRaw> = instances.iter().map(NFInstance::to_raw).collect();
        self.gpu.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&instance_data),
        );
        self.instances.clone_from_slice(instances);
        debug!(instance_count = instances.len(), "instance buffer updated");
        Ok(())
    }

    #[instrument(name = "state.render", level = "trace", skip(self), err)]
    pub fn render(&mut self) -> anyhow::Result<()> {
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.gpu.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            CurrentSurfaceTexture::Timeout
            | CurrentSurfaceTexture::Occluded
            | CurrentSurfaceTexture::Validation => {
                return Ok(());
            }
            CurrentSurfaceTexture::Outdated => {
                warn!("surface outdated; reconfiguring");
                self.gpu
                    .surface
                    .configure(&self.gpu.device, &self.gpu.config);
                return Ok(());
            }
            CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost device");
            }
        };

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let depth_stencil_attachment =
                self.depth_texture.as_ref().map(|depth_texture| {
                    RenderPassDepthStencilAttachment {
                        view: depth_texture.view(),
                        depth_ops: Some(Operations {
                            load: LoadOp::Clear(self.depth_config.clear),
                            store: StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }
                });

            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: Operations {
                        load: LoadOp::Clear(self.clear_color),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.render_pipeline.raw);
            render_pass.set_bind_group(0, self.textures.bind_group(), &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as u32);
        }

        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        self.gpu.queue.present(output);

        Ok(())
    }
}
