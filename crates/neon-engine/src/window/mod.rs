use std::sync::Arc;

mod input;

use glam::Vec2;
use neon_renderer::{Mesh, NFState};
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
    mesh: Mesh,
    state: Option<NFState>,
    camera: Camera
}

impl NFWindow {
    #[instrument(name = "window.new", skip(mesh), fields(title = %title, width = size.x, height = size.y))]
    pub fn new(title: String, size: Vec2, mesh: Mesh) -> Self {

        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: glam::Vec3::Y,
            aspect: size.x / size.y,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };
     

        Self {
            window: None,
            title,
            size,
            mesh,
            state: None,
            camera
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
            WindowEvent::RedrawRequested => { 
                state.set_view_proj(self.camera.build_view_projection_matrix());
                match state.render() {
                Ok(()) => state.request_redraw(),
                Err(e) => {
                    error!(error = %e, "render failed");
                    event_loop.exit();
                }
            }},
            _ => (),
        }
    }
}
