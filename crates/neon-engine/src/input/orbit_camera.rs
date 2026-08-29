use winit::keyboard::KeyCode;

use crate::engine::Camera;

#[derive(Default)]
pub struct OrbitCameraInput {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
}

impl OrbitCameraInput {
    pub fn handle_key(&mut self, key: KeyCode, pressed: bool) {
        match key {
            KeyCode::KeyW => self.forward = pressed,
            KeyCode::KeyS => self.backward = pressed,
            KeyCode::KeyA => self.left = pressed,
            KeyCode::KeyD => self.right = pressed,
            _ => {}
        }
    }

    pub fn update_camera(&self, camera: &mut Camera, speed: f32, dt: f32) {
        if dt <= 0.0 {
            return;
        }

        let step = speed * dt;
        let mut to_target = camera.target - camera.eye;
        let mut distance = to_target.length();

        if distance <= f32::EPSILON {
            return;
        }

        let forward = to_target / distance;
        let right = forward.cross(camera.up).normalize();

        if self.forward {
            camera.eye += forward * step;
        }
        if self.backward {
            camera.eye -= forward * step;
        }

        if self.left || self.right {
            to_target = camera.target - camera.eye;
            distance = to_target.length();
            if distance <= f32::EPSILON {
                return;
            }
        }

        if self.left {
            camera.eye -= right * step;
            let offset = camera.eye - camera.target;
            camera.eye = camera.target + offset.normalize() * distance;
        }
        if self.right {
            camera.eye += right * step;
            let offset = camera.eye - camera.target;
            camera.eye = camera.target + offset.normalize() * distance;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;
    use winit::keyboard::KeyCode;

    fn orbit_camera() -> Camera {
        Camera::new(
            Vec3::new(0.0, 0.0, 10.0),
            Vec3::ZERO,
            Vec3::Y,
            1.0,
            45.0,
            0.1,
            100.0,
        )
    }

    #[test]
    fn forward_moves_eye_toward_target() {
        let mut input = OrbitCameraInput::default();
        input.handle_key(KeyCode::KeyW, true);
        let mut cam = orbit_camera();
        let dist_before = cam.eye.distance(cam.target);
        input.update_camera(&mut cam, 10.0, 0.1);
        let dist_after = cam.eye.distance(cam.target);
        assert!(dist_after < dist_before);
    }

    #[test]
    fn backward_moves_eye_away_from_target() {
        let mut input = OrbitCameraInput::default();
        input.handle_key(KeyCode::KeyS, true);
        let mut cam = orbit_camera();
        let dist_before = cam.eye.distance(cam.target);
        input.update_camera(&mut cam, 10.0, 0.1);
        let dist_after = cam.eye.distance(cam.target);
        assert!(dist_after > dist_before);
    }

    #[test]
    fn zero_dt_is_noop() {
        let mut input = OrbitCameraInput::default();
        input.handle_key(KeyCode::KeyW, true);
        let mut cam = orbit_camera();
        let before = cam.eye;
        input.update_camera(&mut cam, 10.0, 0.0);
        assert_eq!(cam.eye, before);
    }

    #[test]
    fn negative_dt_is_noop() {
        let mut input = OrbitCameraInput::default();
        input.handle_key(KeyCode::KeyW, true);
        let mut cam = orbit_camera();
        let before = cam.eye;
        input.update_camera(&mut cam, 10.0, -1.0);
        assert_eq!(cam.eye, before);
    }

    #[test]
    fn left_strafe_preserves_distance_to_target() {
        let mut input = OrbitCameraInput::default();
        input.handle_key(KeyCode::KeyA, true);
        let mut cam = orbit_camera();
        let dist_before = cam.eye.distance(cam.target);
        input.update_camera(&mut cam, 5.0, 0.2);
        let dist_after = cam.eye.distance(cam.target);
        assert!((dist_after - dist_before).abs() < 1e-4);
    }

    #[test]
    fn unhandled_key_is_ignored() {
        let mut input = OrbitCameraInput::default();
        input.handle_key(KeyCode::Escape, true);
        let mut cam = orbit_camera();
        let before = cam.eye;
        input.update_camera(&mut cam, 5.0, 0.1);
        assert_eq!(cam.eye, before);
    }

    #[test]
    fn key_release_stops_movement() {
        let mut input = OrbitCameraInput::default();
        input.handle_key(KeyCode::KeyW, true);
        input.handle_key(KeyCode::KeyW, false);
        let mut cam = orbit_camera();
        let before = cam.eye;
        input.update_camera(&mut cam, 10.0, 0.1);
        assert_eq!(cam.eye, before);
    }
}
