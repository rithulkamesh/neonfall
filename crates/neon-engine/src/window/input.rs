use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

pub struct Input;

impl Input {
    pub fn handle_key(event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }
}
