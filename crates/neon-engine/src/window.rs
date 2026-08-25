use std::sync::Arc;

use glam::Vec2;
use neon_renderer::state::NFState;
use tracing::info;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

pub struct NFWindow {
    window: Option<Arc<Window>>,
    title: String,
    size: Vec2,
    state: Option<NFState>,
}

impl NFWindow {
    pub fn new(title: String, size: Vec2) -> Self {
        Self {
            window: None,
            title,
            size,
            state: None,
        }
    }
}

impl ApplicationHandler for NFWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Some(Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&self.title)
                        .with_inner_size(LogicalSize::new(self.size.x, self.size.y)),
                )
                .unwrap(),
        ));

        let state = pollster::block_on(neon_renderer::state::NFState::new(
            window.clone().expect("failed to create window"),
        ));
        self.window = window;
        self.state = Some(state.expect("failed to create state"));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                info!("a close was requested. exiting.");
                event_loop.exit();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => self
                .state
                .as_ref()
                .expect("window_state isn't initialized yet")
                .handle_key(event_loop, code, key_state.is_pressed()),
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
