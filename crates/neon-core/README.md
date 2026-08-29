# neon-core

Shared configuration types for the Neonfall engine. This crate has no GPU or window dependencies — only `glam` for vector math.

## `NFConfig`

Window and framebuffer settings passed into `neon_engine::run`.

| field | default | description |
| --- | --- | --- |
| `window_title` | `"Neon"` | OS window title |
| `window_size` | `1280 × 720` | logical size in pixels (`glam::Vec2`) |
| `vsync_enabled` | `true` | reserved for future present-mode control |
| `clear_color` | `[0.1, 0.2, 0.3, 1.0]` | RGBA clear color for the swapchain |

## usage

```rust
use glam::Vec2;
use neon_core::config::NFConfig;

// defaults
let cfg = NFConfig::default();

// tuple shorthand: (title, size, vsync)
let cfg = NFConfig::from(("Neonfall", Vec2::new(1280.0, 720.0), true));
```

`neon_engine::run` accepts `impl Into<NFConfig>`, so you can pass an `NFConfig` directly or the tuple form above.

## layout

```
src/
  lib.rs      — crate root, re-exports `config`
  config.rs   — `NFConfig`
```
