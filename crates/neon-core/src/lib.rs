//! Shared configuration for Neonfall crates.
//!
//! This crate holds engine-agnostic settings (window title, size, clear color)
//! so games and the engine can share one type without pulling in wgpu or winit.
//!
//! # Example
//!
//! ```
//! use glam::Vec2;
//! use neon_core::config::NFConfig;
//!
//! let cfg = NFConfig::from(("My Game", Vec2::new(800.0, 600.0), true));
//! assert_eq!(cfg.window_title, "My Game");
//! ```

pub mod config;
