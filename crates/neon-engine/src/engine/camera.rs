use glam::{
    Mat4, Vec3,
    camera::rh::{proj::directx::perspective, view::look_at_mat4},
};

#[derive(Debug)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = look_at_mat4(self.eye, self.target, self.up);
        let proj = perspective(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);
        proj * view
    }

    pub fn new(
        eye: Vec3,
        target: Vec3,
        up: Vec3,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Camera {
            eye,
            target,
            up,
            aspect,
            fovy,
            znear,
            zfar,
        }
    }
}
