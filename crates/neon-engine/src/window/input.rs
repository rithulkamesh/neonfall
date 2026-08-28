use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

use crate::engine::Camera;

pub struct Input;

impl Input {
    pub fn handle_key(
        event_loop: &ActiveEventLoop,
        code: KeyCode,
        is_pressed: bool,
        camera: &mut Camera,
        camera_speed: f32,
    ) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();
        let right = forward_norm.cross(camera.up);

        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            (KeyCode::KeyW, true) => {
                if forward_mag > camera_speed {
                    camera.eye += forward_norm * camera_speed;
                }
            }
            (KeyCode::KeyS, true) => {
                if forward_mag > camera_speed {
                    camera.eye -= forward_norm * camera_speed;
                }
            }

            (KeyCode::KeyA, true) => {
                camera.eye =
                    camera.target - (forward + right * camera_speed).normalize() * forward_mag;
            }

            (KeyCode::KeyD, true) => {
                camera.eye =
                    camera.target - (forward - right * camera_speed).normalize() * forward_mag;
            }

            _ => {}
        }
    }
}
