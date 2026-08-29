use neon_renderer::NFState;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

use crate::engine::Camera;

pub struct GameContext<'a> {
    pub camera: &'a mut Camera,
    pub state: &'a mut NFState,
    event_loop: &'a ActiveEventLoop,
}

impl<'a> GameContext<'a> {
    pub fn new(
        camera: &'a mut Camera,
        state: &'a mut NFState,
        event_loop: &'a ActiveEventLoop,
    ) -> Self {
        Self {
            camera,
            state,
            event_loop,
        }
    }

    pub fn exit(&self) {
        self.event_loop.exit();
    }
}

pub trait Game {
    fn update(&mut self, ctx: &mut GameContext, dt: f32);

    fn on_key(&mut self, ctx: &mut GameContext, key: KeyCode, pressed: bool) {
        let _ = (ctx, key, pressed);
    }
}
