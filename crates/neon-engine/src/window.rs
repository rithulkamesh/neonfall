use glam::Vec2;
use tracing::info;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct NeonWindow {
    window: Option<Window>,
    title: String,
    size: Vec2,
}

impl NeonWindow {
    pub fn new(title: String, size: Vec2) -> Self {
        Self {
            window: None,
            title,
            size,
        }
    }
}

impl ApplicationHandler for NeonWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&self.title)
                        .with_inner_size(LogicalSize::new(self.size.x, self.size.y)),
                )
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                info!("a close was requested. exiting.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {}
            _ => (),
        }
    }
}
