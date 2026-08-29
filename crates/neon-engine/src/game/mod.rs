use neon_renderer::NFState;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

use crate::engine::Camera;

/// Per-frame context passed to [`Game::update`] and [`Game::on_key`].
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

/// Game logic hook. Implement this trait and pass your type to [`crate::run`].
pub trait Game {
    /// Called every redraw with elapsed time in seconds.
    fn update(&mut self, ctx: &mut GameContext, dt: f32);

    /// Called for keyboard events. `pressed` is true on key down.
    fn on_key(&mut self, ctx: &mut GameContext, key: KeyCode, pressed: bool) {
        let _ = (ctx, key, pressed);
    }
}
