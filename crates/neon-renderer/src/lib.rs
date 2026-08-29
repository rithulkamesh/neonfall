//! wgpu renderer for Neonfall: mesh loading, instanced draws, textures, depth.
//!
//! The main entry point is [`NFState`], created from an [`NFMesh`] and driven
//! each frame with [`NFState::set_view_proj`] and [`NFState::render`].
//!
//! # Example
//!
//! ```no_run
//! use neon_renderer::{NFDepthConfig, NFMesh, NFState};
//! use std::sync::Arc;
//! use wgpu::Color;
//!
//! # async fn example(window: Arc<winit::window::Window>) -> anyhow::Result<()> {
//! let mesh = NFMesh::from("./models/cube.glb");
//! let mut state = NFState::new(
//!     window,
//!     &mesh,
//!     NFDepthConfig::default(),
//!     Color::BLACK,
//! ).await?;
//! state.render()?;
//! # Ok(())
//! # }
//! ```

pub mod gpu;
pub mod state;

pub use gpu::{
    NFDepthConfig, NFDepthTexture, NFInstance, NFInstanceRaw, NFMesh, NFPipeline, NFTextureImage,
    NFTextures, NFVertex,
};
pub use state::NFState;
