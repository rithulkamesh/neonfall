use glam::{Quat, Vec3};

pub struct NFInstance {
    position: Vec3,
    rotation: Quat,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NFInstanceRaw {
    model: [[f32; 4]; 4],
}


impl NFInstance {
    pub fn to_raw(&self) -> NFInstanceRaw {
        NFInstanceRaw {
            model: (glam::Mat4::from_translation(self.position) * glam::Mat4::from_quat(self.rotation)).to_cols_array_2d()

        }
    }
}
