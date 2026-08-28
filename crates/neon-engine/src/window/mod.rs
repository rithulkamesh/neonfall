use std::sync::Arc;

mod input;

use glam::Vec2;
use neon_renderer::{NFMesh, NFState};
use tracing::{error, info, instrument};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use input::Input;

use crate::engine::Camera;

pub struct NFWindow {
    window: Option<Arc<Window>>,
    title: String,
    size: Vec2,
    mesh: NFMesh,
    state: Option<NFState>,
    camera: Camera,
    camera_speed: f32,
}

impl NFWindow {
    #[instrument(name = "window.new", skip(mesh), fields(title = %title, width = size.x, height = size.y))]
    pub fn new(title: String, size: Vec2, mesh: NFMesh, camera: Camera) -> Self {
        Self {
            window: None,
            title,
            size,
            mesh,
            state: None,
            camera,
            camera_speed: 0.2 as f32,
        }
    }
}

impl ApplicationHandler for NFWindow {
    #[instrument(name = "window.resumed", skip(self, event_loop), fields(title = %self.title))]
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

        let mut state = pollster::block_on(NFState::new(window.clone(), &self.mesh))
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
            } => Input::handle_key(
                event_loop,
                code,
                key_state.is_pressed(),
                &mut self.camera,
                self.camera_speed,
            ),
            WindowEvent::RedrawRequested => {
                state.set_view_proj(self.camera.build_view_projection_matrix());
                match state.render() {
                    Ok(()) => state.request_redraw(),
                    Err(e) => {
                        error!(error = %e, "render failed");
                        event_loop.exit();
                    }
                }
            }
            _ => (),
        }
    }
}
