use bytemuck::{Pod, Zeroable};
use gltf::import;
use tracing::{debug, info, instrument, trace, warn};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct NFVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl NFVertex {
    pub const fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { position, color }
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
            ],
        }
    }
}

#[derive(Debug)]
pub struct NFMesh {
    pub vertices: Vec<NFVertex>,
    pub indices: Vec<u16>,
}

impl NFMesh {
    pub const fn new(vertices: Vec<NFVertex>, indices: Vec<u16>) -> Self {
        Self { vertices, indices }
    }

    #[instrument(name = "mesh.load_gltf", skip(path), err)]
    pub fn load_gltf(
        path: impl AsRef<std::path::Path>,
    ) -> Result<(Vec<NFVertex>, Vec<u16>), gltf::Error> {
        let path = path.as_ref();
        info!(path = ?path, "loading glTF mesh");
        let (document, buffers, _images) = import(path)?;
        debug!(
            path = ?path,
            meshes = document.meshes().count(),
            buffers = buffers.len(),
            "glTF imported"
        );

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

                    vertices.extend(
                        positions
                            .into_iter()
                            .zip(colors.into_iter().chain(std::iter::repeat([1.0; 3])))
                            .map(|(pos, col)| NFVertex::new(pos, col)),
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
            "glTF mesh loaded"
        );
        Ok((vertices, indices))
    }
}

impl From<&str> for NFMesh {
    #[instrument(name = "mesh.from_str", skip(path))]
    fn from(path: &str) -> Self {
        debug!(path, "creating mesh from glTF path");
        let (loaded_vertices, loaded_indices) =
            NFMesh::load_gltf(path).expect("failed to load model");
        Self {
            vertices: loaded_vertices,
            indices: loaded_indices,
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
        let (loaded_vertices, loaded_indices) =
            NFMesh::load_gltf(path).expect("failed to load model");
        Self {
            vertices: loaded_vertices,
            indices: loaded_indices,
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

    #[test]
    fn test_load_gltf_cube() {
        let result = NFMesh::load_gltf("../../models/cube.glb");
        assert!(result.is_ok());
        let (vertices, indices) = result.unwrap();
        assert!(!vertices.is_empty());
        assert!(!indices.is_empty());

        let mesh = NFMesh::from("../../models/cube.glb");
        assert_eq!(mesh.vertices.len(), vertices.len());
        assert_eq!(mesh.indices.len(), indices.len());
    }
}
