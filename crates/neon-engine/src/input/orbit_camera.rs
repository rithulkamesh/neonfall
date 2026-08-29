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
