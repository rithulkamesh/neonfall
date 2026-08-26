use std::sync::Arc;

mod input;

use glam::Vec2;
use neon_renderer::state::NFState;
use tracing::{error, info};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use input::Input;

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

        let mut state = pollster::block_on(neon_renderer::state::NFState::new(
            window.clone().expect("failed to create window"),
        ))
        .expect("failed to create state");
        let size = state.window.inner_size();
        state.resize(size.width, size.height);

        self.window = window;
        self.state = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

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
            } => Input::handle_key(event_loop, code, key_state.is_pressed()),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(e) => {
                        // Log the error and exit gracefully
                        error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            _ => (),
        }
    }
}
