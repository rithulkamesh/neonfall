use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

use crate::engine::Camera;

#[derive(Default)]
pub struct Input {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
}

impl Input {
    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match code {
            KeyCode::Escape if is_pressed => event_loop.exit(),
            KeyCode::KeyW => self.forward = is_pressed,
            KeyCode::KeyS => self.backward = is_pressed,
            KeyCode::KeyA => self.left = is_pressed,
            KeyCode::KeyD => self.right = is_pressed,
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
