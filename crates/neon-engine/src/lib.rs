mod engine;
mod tracing;
mod window;

use neon_core::config::NFConfig;
use neon_renderer::NFDepthConfig;
use window::NFWindow;
use winit::event_loop::{ControlFlow, EventLoop};

pub use engine::{Camera, NFDepth};
pub use neon_renderer::{NFInstance, NFMesh, NFState, NFTextureImage, NFVertex};

pub fn install_tracing() {
    crate::tracing::install();
}

pub fn init(config: impl Into<NFConfig>, mesh: NFMesh, camera: Camera, depth: NFDepth) {
    let config = config.into();
    let depth = NFDepthConfig::from(depth);

    install_tracing();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut win = NFWindow::new(config.window_title, config.window_size, mesh, camera, depth);
    ::tracing::info!(
        depth_enabled = depth.enabled,
        depth_clear = depth.clear,
        "initialized neon."
    );

    event_loop.run_app(&mut win).expect("failed to run window");
}
