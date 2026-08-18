mod app;
mod window;

use neon_core::config::NeonConfig;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use window::NeonWindow;
use winit::event_loop::{ControlFlow, EventLoop};

pub fn init(config: impl Into<NeonConfig>) {
    let config = config.into();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to initialize tracing");

    let event_loop = EventLoop::new().unwrap();

    // event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut win = NeonWindow::new(config.window_title, config.window_size);
    info!("initialized neon.");

    event_loop.run_app(&mut win).expect("failed to run window");
}
