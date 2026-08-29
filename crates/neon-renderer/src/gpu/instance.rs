use glam::{Quat, Vec3};

#[derive(Clone, Debug)]
pub struct NFInstance {
    pub position: Vec3,
    pub rotation: Quat,
    pub texture_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NFInstanceRaw {
    model: [[f32; 4]; 4],
    texture_index: f32,
    _pad: [f32; 3],
}

impl NFInstance {
    pub fn new(position: Vec3, rotation: Quat) -> Self {
        Self {
            position,
            rotation,
            texture_index: 0,
        }
    }

    pub fn with_texture_index(mut self, texture_index: u32) -> Self {
        self.texture_index = texture_index;
        self
    }

    pub fn to_raw(&self) -> NFInstanceRaw {
        NFInstanceRaw {
            model: (glam::Mat4::from_translation(self.position)
                * glam::Mat4::from_quat(self.rotation))
            .to_cols_array_2d(),
            texture_index: self.texture_index as f32,
            _pad: [0.0; 3],
        }
    }
}

impl NFInstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem::size_of;

        wgpu::VertexBufferLayout {
            array_stride: size_of::<NFInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 4]>() as u64,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 8]>() as u64,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 12]>() as u64,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 16]>() as u64,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Mat4, Quat, Vec3};

    #[test]
    fn identity_instance_produces_identity_model_matrix() {
        let inst = NFInstance::new(Vec3::ZERO, Quat::IDENTITY);
        let raw = inst.to_raw();
        let m = Mat4::from_cols_array_2d(&raw.model);
        assert_eq!(m, Mat4::IDENTITY);
    }

    #[test]
    fn translation_encoded_in_model_matrix() {
        let pos = Vec3::new(1.0, 2.0, 3.0);
        let inst = NFInstance::new(pos, Quat::IDENTITY);
        let raw = inst.to_raw();
        let m = Mat4::from_cols_array_2d(&raw.model);
        assert_eq!(m.w_axis.truncate(), pos);
    }

    #[test]
    fn texture_index_preserved_in_raw() {
        let inst = NFInstance::new(Vec3::ZERO, Quat::IDENTITY).with_texture_index(7);
        assert_eq!(inst.to_raw().texture_index, 7.0);
    }

    #[test]
    fn instance_raw_stride_matches_size() {
        let desc = NFInstanceRaw::desc();
        assert_eq!(desc.array_stride as usize, std::mem::size_of::<NFInstanceRaw>());
        assert_eq!(desc.step_mode, wgpu::VertexStepMode::Instance);
    }

    #[test]
    fn instance_raw_has_five_attributes() {
        let desc = NFInstanceRaw::desc();
        assert_eq!(desc.attributes.len(), 5);
    }
}
