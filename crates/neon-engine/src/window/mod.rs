use std::sync::Arc;
use std::time::Instant;

mod input;

use glam::Vec2;
use neon_renderer::{NFDepthConfig, NFMesh, NFState};
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
    depth: NFDepthConfig,
    camera_speed: f32,
    input: Input,
    last_frame: Instant,
}

impl NFWindow {
    #[instrument(
        name = "window.new",
        skip(mesh, depth),
        fields(
            title = %title,
            width = size.x,
            height = size.y,
            depth_enabled = depth.enabled,
            depth_clear = depth.clear
        )
    )]
    pub fn new(
        title: String,
        size: Vec2,
        mesh: NFMesh,
        camera: Camera,
        depth: NFDepthConfig,
    ) -> Self {
        Self {
            window: None,
            title,
            size,
            mesh,
            state: None,
            camera,
            depth,
            camera_speed: 5.0,
            input: Input::default(),
            last_frame: Instant::now(),
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

        let mut state = pollster::block_on(NFState::new(window.clone(), &self.mesh, self.depth))
            .expect("failed to create state");
        let size = state.window_size();
        state.resize(size.width, size.height);
        state.request_redraw();

        self.window = Some(window);
        self.state = Some(state);
        self.last_frame = Instant::now();
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
            } => self.input.handle_key(event_loop, code, key_state.is_pressed()),
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = now.duration_since(self.last_frame).as_secs_f32();
                self.last_frame = now;

                self.input
                    .update_camera(&mut self.camera, self.camera_speed, dt);
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
