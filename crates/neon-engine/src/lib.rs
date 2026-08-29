mod engine;
mod game;
mod input;
mod tracing;
mod window;

use neon_renderer::NFDepthConfig;
use window::NFWindow;
use wgpu::Color;
use winit::event_loop::{ControlFlow, EventLoop};

pub use engine::{Camera, NFDepth};
pub use game::{Game, GameContext};
pub use input::OrbitCameraInput;
pub use neon_core::config::NFConfig;
pub use neon_renderer::{NFInstance, NFMesh, NFState, NFTextureImage, NFVertex};
pub use winit::keyboard::KeyCode;

pub fn install_tracing() {
    crate::tracing::install();
}

pub fn run<G: Game>(
    config: impl Into<NFConfig>,
    mesh: NFMesh,
    camera: Camera,
    depth: NFDepth,
    game: G,
) {
    let config = config.into();
    let depth = NFDepthConfig::from(depth);
    let clear_color = Color {
        r: config.clear_color[0] as f64,
        g: config.clear_color[1] as f64,
        b: config.clear_color[2] as f64,
        a: config.clear_color[3] as f64,
    };

    install_tracing();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut win = NFWindow::new(
        config.window_title,
        config.window_size,
        mesh,
        camera,
        depth,
        clear_color,
        game,
    );
    ::tracing::info!(
        depth_enabled = depth.enabled,
        depth_clear = depth.clear,
        clear_r = config.clear_color[0],
        clear_g = config.clear_color[1],
        clear_b = config.clear_color[2],
        "initialized neon."
    );

    event_loop.run_app(&mut win).expect("failed to run window");
}
