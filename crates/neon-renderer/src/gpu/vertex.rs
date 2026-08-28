use bytemuck::{Pod, Zeroable};
use gltf::import;

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

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
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

    pub fn load_gltf(path: impl AsRef<std::path::Path>) -> Result<(Vec<NFVertex>, Vec<u16>), gltf::Error> {
        let (document, buffers, _images) = import(path)?;

        let (vertices, indices) = document
            .meshes()
            .flat_map(|mesh| mesh.primitives())
            .fold(
                (Vec::new(), Vec::new()), // Returns 2 Vectors
                |(mut vertices, mut indices), primitive| {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                    let base_vertex = vertices.len() as u16;

                    let positions: Vec<[f32; 3]> = reader
                        .read_positions()
                        .map(|iter| iter.collect())
                        .unwrap_or_default();

                    let colors: Vec<[f32; 3]> = reader
                        .read_colors(0)
                        .map(|colors| colors.into_rgb_f32().collect())
                        .unwrap_or_else(|| {
                            let base_color = primitive
                                .material()
                                .pbr_metallic_roughness()
                                .base_color_factor();
                            vec![[base_color[0], base_color[1], base_color[2]]; positions.len()]
                        });

                    vertices.extend(
                        positions
                            .into_iter()
                            .zip(colors.into_iter().chain(std::iter::repeat([1.0; 3])))
                            .map(|(pos, col)| NFVertex::new(pos, col)),
                    );

                    let vertex_count = vertices.len() as u16 - base_vertex;
                    indices.extend(
                        reader
                            .read_indices()
                            .map(|read_indices| {
                                read_indices
                                    .into_u32()
                                    .map(|idx| base_vertex + idx as u16)
                                    .collect::<Vec<u16>>()
                            })
                            .unwrap_or_else(|| {
                                (0..vertex_count)
                                    .map(|offset| base_vertex + offset)
                                    .collect::<Vec<u16>>()
                            }),
                    );

                    (vertices, indices)
                },
            );

        Ok((vertices, indices))
    }
}

impl From<&str> for NFMesh {
    fn from(path: &str) -> Self {
        let (loaded_vertices, loaded_indices) =
            NFMesh::load_gltf(path).expect("failed to load model");
        Self {
            vertices: loaded_vertices,
            indices: loaded_indices,
        }
    }
}

impl From<String> for NFMesh {
    fn from(path: String) -> Self {
        Self::from(path.as_str())
    }
}

impl From<&std::path::Path> for NFMesh {
    fn from(path: &std::path::Path) -> Self {
        let (loaded_vertices, loaded_indices) =
            NFMesh::load_gltf(path).expect("failed to load model");
        Self {
            vertices: loaded_vertices,
            indices: loaded_indices,
        }
    }
}

impl From<std::path::PathBuf> for NFMesh {
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