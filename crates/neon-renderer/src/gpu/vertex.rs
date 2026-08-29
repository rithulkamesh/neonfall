use bytemuck::{Pod, Zeroable};
use gltf::import;
use tracing::{debug, info, instrument, trace, warn};

use super::NFInstance;
use super::NFTextureImage;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct NFVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl NFVertex {
    pub const fn new(position: [f32; 3], color: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            color,
            tex_coords,
        }
    }

    #[instrument(name = "vertex.desc", level = "trace")]
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        trace!(
            stride = std::mem::size_of::<NFVertex>(),
            "building vertex buffer layout"
        );
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<NFVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct NFMesh {
    pub vertices: Vec<NFVertex>,
    pub indices: Vec<u16>,
    pub instances: Vec<NFInstance>,
    pub diffuse: Option<NFTextureImage>,
    pub atlas_grid: u32,
}

impl NFMesh {
    pub fn new(vertices: Vec<NFVertex>, indices: Vec<u16>) -> Self {
        Self {
            vertices,
            indices,
            instances: vec![NFInstance::new(glam::Vec3::ZERO, glam::Quat::IDENTITY)],
            diffuse: None,
            atlas_grid: 1,
        }
    }

    #[instrument(name = "mesh.with_instances", skip(self, instances), fields(instance_count = instances.len()))]
    pub fn with_instances(mut self, instances: Vec<NFInstance>) -> Self {
        debug!(instance_count = instances.len(), "attached mesh instances");
        self.instances = instances;
        self
    }

    #[instrument(
        name = "mesh.with_diffuse",
        skip(self, image),
        fields(width = image.width, height = image.height, bytes = image.rgba.len())
    )]
    pub fn with_diffuse(mut self, image: NFTextureImage) -> Self {
        debug!("using single diffuse texture");
        self.diffuse = Some(image);
        self.atlas_grid = 1;
        self
    }

    #[instrument(name = "mesh.with_color_atlas", skip(self, colors), fields(color_count = colors.len()))]
    pub fn with_color_atlas(mut self, colors: &[[u8; 3]]) -> Self {
        let (image, grid) = NFTextureImage::color_atlas(colors);
        debug!(
            atlas_grid = grid,
            width = image.width,
            height = image.height,
            "attached runtime color atlas to mesh"
        );
        self.diffuse = Some(image);
        self.atlas_grid = grid;
        for vertex in &mut self.vertices {
            vertex.color = [1.0, 1.0, 1.0];
        }
        self
    }

    #[instrument(name = "mesh.load_gltf", skip(path), err)]
    pub fn load_gltf(
        path: impl AsRef<std::path::Path>,
    ) -> Result<(Vec<NFVertex>, Vec<u16>, Option<NFTextureImage>), gltf::Error> {
        let path = path.as_ref();
        info!(path = ?path, "loading glTF mesh");
        let (document, buffers, images) = import(path)?;
        debug!(
            path = ?path,
            meshes = document.meshes().count(),
            buffers = buffers.len(),
            images = images.len(),
            "glTF imported"
        );

        let diffuse = Self::diffuse_from_gltf(&document, &images);

        let (vertices, indices) = document
            .meshes()
            .flat_map(|mesh| mesh.primitives())
            .enumerate()
            .fold(
                (Vec::new(), Vec::new()), // Returns 2 Vectors
                |(mut vertices, mut indices), (primitive_index, primitive)| {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                    let base_vertex = vertices.len() as u16;

                    let positions: Vec<[f32; 3]> = reader
                        .read_positions()
                        .map(|iter| iter.collect())
                        .unwrap_or_else(|| {
                            warn!(
                                primitive = primitive_index,
                                "glTF primitive has no position attribute"
                            );
                            Vec::new()
                        });

                    let colors: Vec<[f32; 3]> = reader
                        .read_colors(0)
                        .map(|colors| colors.into_rgb_f32().collect())
                        .unwrap_or_else(|| {
                            let base_color = primitive
                                .material()
                                .pbr_metallic_roughness()
                                .base_color_factor();
                            debug!(
                                primitive = primitive_index,
                                color = ?&base_color[..3],
                                "using material base color for primitive"
                            );
                            vec![[base_color[0], base_color[1], base_color[2]]; positions.len()]
                        });

                    let tex_coords: Vec<[f32; 2]> = reader
                        .read_tex_coords(0)
                        .map(|coords| coords.into_f32().collect())
                        .unwrap_or_else(|| {
                            debug!(
                                primitive = primitive_index,
                                "glTF primitive has no tex coords; using zero UVs"
                            );
                            vec![[0.0, 0.0]; positions.len()]
                        });

                    vertices.extend(
                        positions
                            .into_iter()
                            .zip(
                                colors
                                    .into_iter()
                                    .chain(std::iter::repeat([1.0, 1.0, 1.0])),
                            )
                            .zip(
                                tex_coords
                                    .into_iter()
                                    .chain(std::iter::repeat([0.0, 0.0])),
                            )
                            .map(|((pos, col), uv)| NFVertex::new(pos, col, uv)),
                    );

                    let vertex_count = vertices.len() as u16 - base_vertex;
                    let primitive_indices = reader.read_indices().map(|read_indices| {
                        read_indices
                            .into_u32()
                            .map(|idx| base_vertex + idx as u16)
                            .collect::<Vec<u16>>()
                    });
                    let primitive_indices = primitive_indices.unwrap_or_else(|| {
                        debug!(
                            primitive = primitive_index,
                            "glTF primitive has no index accessor; generating sequential indices"
                        );
                        (0..vertex_count)
                            .map(|offset| base_vertex + offset)
                            .collect::<Vec<u16>>()
                    });
                    indices.extend(primitive_indices);

                    trace!(
                        primitive = primitive_index,
                        vertices = vertex_count,
                        total_vertices = vertices.len(),
                        total_indices = indices.len(),
                        "processed glTF primitive"
                    );

                    (vertices, indices)
                },
            );

        info!(
            path = ?path,
            vertices = vertices.len(),
            indices = indices.len(),
            has_diffuse = diffuse.is_some(),
            "glTF mesh loaded"
        );
        Ok((vertices, indices, diffuse))
    }

    fn diffuse_from_gltf(
        document: &gltf::Document,
        images: &[gltf::image::Data],
    ) -> Option<NFTextureImage> {
        document.meshes().find_map(|mesh| {
            mesh.primitives().find_map(|primitive| {
                primitive
                    .material()
                    .pbr_metallic_roughness()
                    .base_color_texture()
                    .map(|info| info.texture().source().index())
            })
        }).map(|image_index| {
            debug!(image_index, "using glTF base color texture");
            NFTextureImage::from_gltf(&images[image_index])
        })
    }
}

impl From<&str> for NFMesh {
    #[instrument(name = "mesh.from_str", skip(path))]
    fn from(path: &str) -> Self {
        debug!(path, "creating mesh from glTF path");
        let (loaded_vertices, loaded_indices, diffuse) =
            NFMesh::load_gltf(path).expect("failed to load model");
        Self {
            vertices: loaded_vertices,
            indices: loaded_indices,
            instances: vec![NFInstance::new(glam::Vec3::ZERO, glam::Quat::IDENTITY)],
            diffuse,
            atlas_grid: 1,
        }
    }
}

impl From<String> for NFMesh {
    #[instrument(name = "mesh.from_string", skip(path))]
    fn from(path: String) -> Self {
        Self::from(path.as_str())
    }
}

impl From<&std::path::Path> for NFMesh {
    #[instrument(name = "mesh.from_path", skip(path))]
    fn from(path: &std::path::Path) -> Self {
        debug!(path = ?path, "creating mesh from glTF path");
        let (loaded_vertices, loaded_indices, diffuse) =
            NFMesh::load_gltf(path).expect("failed to load model");
        Self {
            vertices: loaded_vertices,
            indices: loaded_indices,
            instances: vec![NFInstance::new(glam::Vec3::ZERO, glam::Quat::IDENTITY)],
            diffuse,
            atlas_grid: 1,
        }
    }
}

impl From<std::path::PathBuf> for NFMesh {
    #[instrument(name = "mesh.from_path_buf", skip(path))]
    fn from(path: std::path::PathBuf) -> Self {
        Self::from(path.as_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Quat, Vec3};

    #[test]
    fn test_load_gltf_cube() {
        let result = NFMesh::load_gltf("../../models/cube.glb");
        assert!(result.is_ok());
        let (vertices, indices, _diffuse) = result.unwrap();
        assert!(!vertices.is_empty());
        assert!(!indices.is_empty());

        let mesh = NFMesh::from("../../models/cube.glb");
        assert_eq!(mesh.vertices.len(), vertices.len());
        assert_eq!(mesh.indices.len(), indices.len());
    }

    #[test]
    fn vertex_desc_stride_matches_size() {
        let desc = NFVertex::desc();
        assert_eq!(desc.array_stride as usize, std::mem::size_of::<NFVertex>());
        assert_eq!(desc.step_mode, wgpu::VertexStepMode::Vertex);
    }

    #[test]
    fn new_mesh_has_one_default_instance() {
        let mesh = NFMesh::new(vec![], vec![]);
        assert_eq!(mesh.instances.len(), 1);
        assert_eq!(mesh.atlas_grid, 1);
        assert!(mesh.diffuse.is_none());
    }

    #[test]
    fn with_instances_replaces_defaults() {
        let instances = vec![
            NFInstance::new(Vec3::ONE, Quat::IDENTITY),
            NFInstance::new(Vec3::splat(2.0), Quat::IDENTITY),
        ];
        let mesh = NFMesh::new(vec![], vec![]).with_instances(instances);
        assert_eq!(mesh.instances.len(), 2);
    }

    #[test]
    fn with_color_atlas_whitens_vertex_colors() {
        let vertex = NFVertex::new([0.0, 0.0, 0.0], [0.2, 0.3, 0.4], [0.0, 0.0]);
        let mesh = NFMesh::new(vec![vertex], vec![]).with_color_atlas(&[[255, 0, 0]]);
        assert_eq!(mesh.vertices[0].color, [1.0, 1.0, 1.0]);
        assert!(mesh.diffuse.is_some());
        assert_eq!(mesh.atlas_grid, 1);
    }

    #[test]
    fn with_diffuse_sets_single_cell_atlas() {
        let image = NFTextureImage::solid([128, 64, 32]);
        let mesh = NFMesh::new(vec![], vec![]).with_diffuse(image);
        assert!(mesh.diffuse.is_some());
        assert_eq!(mesh.atlas_grid, 1);
    }

    #[test]
    fn load_gltf_cube_indices_are_valid() {
        let (vertices, indices, _) =
            NFMesh::load_gltf("../../models/cube.glb").expect("cube glb should load");
        let max_index = indices.iter().copied().max().unwrap_or(0);
        assert!(max_index < vertices.len() as u16);
    }
}
