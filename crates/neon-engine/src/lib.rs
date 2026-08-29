//! Application shell for Neonfall games.
//!
//! Owns the winit event loop, window lifecycle, camera math, orbit input,
//! tracing setup, and the [`Game`] trait. Rendering is delegated to
//! `neon-renderer`; window settings come from `neon-core::NFConfig`.
//!
//! # Example
//!
//! ```no_run
//! use glam::{Vec3, Vec2};
//! use neon_engine::{Camera, Game, GameContext, NFDepth, NFMesh, run, KeyCode};
//!
//! struct MyGame;
//!
//! impl Game for MyGame {
//!     fn update(&mut self, _ctx: &mut GameContext, _dt: f32) {}
//! }
//!
//! fn main() {
//!     let size = Vec2::new(1280.0, 720.0);
//!     run(
//!         ("My Game", size, true),
//!         NFMesh::from("./models/cube.glb"),
//!         Camera::new(Vec3::new(0.0, 5.0, 10.0), Vec3::ZERO, Vec3::Y, size.x / size.y, 45.0, 0.1, 100.0),
//!         NFDepth::enabled(),
//!         MyGame,
//!     );
//! }
//! ```

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

/// Start the engine: create a window, initialize the renderer, and run the
/// event loop until the game calls [`GameContext::exit`] or the window closes.
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
