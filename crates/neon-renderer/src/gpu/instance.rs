use glam::{Quat, Vec3};

#[derive(Clone, Debug)]
pub struct NFInstance {
    pub position: Vec3,
    pub rotation: Quat,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NFInstanceRaw {
    model: [[f32; 4]; 4],
}

impl NFInstance {
    pub fn new(position: Vec3, rotation: Quat) -> Self {
        Self { position, rotation }
    }
    pub fn to_raw(&self) -> NFInstanceRaw {
        NFInstanceRaw {
            model: (glam::Mat4::from_translation(self.position)
                * glam::Mat4::from_quat(self.rotation))
            .to_cols_array_2d(),
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
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 4]>() as u64,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 8]>() as u64,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 12]>() as u64,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
