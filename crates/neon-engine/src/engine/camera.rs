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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_camera() -> Camera {
        Camera::new(
            Vec3::new(0.0, 5.0, 10.0),
            Vec3::ZERO,
            Vec3::Y,
            16.0 / 9.0,
            45.0,
            0.1,
            100.0,
        )
    }

    #[test]
    fn view_projection_is_finite() {
        let m = sample_camera().build_view_projection_matrix();
        assert!(m.to_cols_array().iter().all(|v| v.is_finite()));
    }

    #[test]
    fn origin_projects_near_center_when_on_view_axis() {
        let cam = Camera::new(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::ZERO,
            Vec3::Y,
            1.0,
            90.0,
            0.1,
            100.0,
        );
        let vp = cam.build_view_projection_matrix();
        let projected = vp * Vec3::ZERO.extend(1.0);
        let ndc = projected / projected.w;
        assert!(ndc.x.abs() < 0.05);
        assert!(ndc.y.abs() < 0.05);
    }

    #[test]
    fn stores_constructor_fields() {
        let cam = sample_camera();
        assert_eq!(cam.eye, Vec3::new(0.0, 5.0, 10.0));
        assert_eq!(cam.target, Vec3::ZERO);
        assert_eq!(cam.up, Vec3::Y);
        assert_eq!(cam.aspect, 16.0 / 9.0);
        assert_eq!(cam.fovy, 45.0);
        assert_eq!(cam.znear, 0.1);
        assert_eq!(cam.zfar, 100.0);
    }
}
