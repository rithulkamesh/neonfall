mod engine;
mod tracing;
mod window;

use crate::tracing::install;
use neon_core::config::NFConfig;
use window::NFWindow;
use winit::event_loop::{ControlFlow, EventLoop};

pub use neon_renderer::{Mesh, NFState, Vertex};

pub fn init(config: impl Into<NFConfig>, mesh: Mesh) {
    let config = config.into();

    install();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut win = NFWindow::new(config.window_title, config.window_size, mesh);
    ::tracing::info!("initialized neon.");

    event_loop.run_app(&mut win).expect("failed to run window");
}
