use std::sync::Arc;

mod input;

use glam::Vec2;
use neon_renderer::{Mesh, NFState};
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
    mesh: Mesh,
    state: Option<NFState>,
}

impl NFWindow {
    pub fn new(title: String, size: Vec2, mesh: Mesh) -> Self {
        Self {
            window: None,
            title,
            size,
            mesh,
            state: None,
        }
    }
}

impl ApplicationHandler for NFWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&self.title)
                        .with_inner_size(LogicalSize::new(self.size.x, self.size.y)),
                )
                .unwrap(),
        );

        info!(title = %self.title, "window created");

        let mut state = pollster::block_on(NFState::new(window.clone(), self.mesh))
            .expect("failed to create state");
        let size = state.window_size();
        state.resize(size.width, size.height);
        state.request_redraw();

        self.window = Some(window);
        self.state = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                info!("close requested; exiting");
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                state.resize(size.width, size.height);
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
            WindowEvent::RedrawRequested => match state.render() {
                Ok(()) => state.request_redraw(),
                Err(e) => {
                    error!("{e}");
                    event_loop.exit();
                }
            },
            _ => (),
        }
    }
}
