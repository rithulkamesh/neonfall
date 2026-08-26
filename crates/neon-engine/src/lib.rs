mod window;

use neon_core::config::NFConfig;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use window::NFWindow;
use winit::event_loop::{ControlFlow, EventLoop};

pub use neon_renderer::{Mesh, Vertex, NFState};

pub fn init(config: impl Into<NFConfig>, mesh: Mesh) {
    let config = config.into();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to initialize tracing");

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut win = NFWindow::new(config.window_title, config.window_size, mesh);
    info!("initialized neon.");

    event_loop.run_app(&mut win).expect("failed to run window");
}
