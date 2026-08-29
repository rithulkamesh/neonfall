# neon-engine

Application shell for Neonfall games: winit event loop, window lifecycle, camera math, orbit input, tracing setup, and the `Game` trait that drives per-frame logic.

Rendering is delegated to [`neon-renderer`](../../crates/neon-renderer/README.md); configuration comes from [`neon-core`](../../crates/neon-core/README.md).

## quick start

```rust
use glam::{Vec3, Vec2};
use neon_engine::{Camera, Game, GameContext, NFDepth, NFMesh, run, KeyCode};

struct MyGame;

impl Game for MyGame {
    fn update(&mut self, _ctx: &mut GameContext, _dt: f32) {}

    fn on_key(&mut self, ctx: &mut GameContext, key: KeyCode, pressed: bool) {
        if key == KeyCode::Escape && pressed {
            ctx.exit();
        }
    }
}

fn main() {
    let size = Vec2::new(1280.0, 720.0);
    run(
        ("My Game", size, true),
        NFMesh::from("./models/cube.glb"),
        Camera::new(
            Vec3::new(0.0, 5.0, 10.0),
            Vec3::ZERO,
            Vec3::Y,
            size.x / size.y,
            45.0,
            0.1,
            100.0,
        ),
        NFDepth::enabled(),
        MyGame,
    );
}
```

## `Game` trait

| method | when | notes |
| --- | --- | --- |
| `update` | every redraw | receives `dt` in seconds |
| `on_key` | keyboard event | `pressed` is `true` on key down |

`GameContext` exposes `camera`, `state` (`NFState`), and `exit()` to close the app.

## camera

`Camera` holds eye, target, up, aspect, vertical FOV (degrees), and clip planes. Call `build_view_projection_matrix()` each frame (the engine does this automatically after `update`).

Uses DirectX-style perspective projection via `glam`.

## depth

`NFDepth` is the engine-side depth-buffer policy:

| constructor | `enabled` | `clear` |
| --- | --- | --- |
| `NFDepth::enabled()` / `Default` | `true` | `1.0` |
| `NFDepth::disabled()` | `false` | `1.0` |
| `NFDepth::with_clear(v)` | `true` | `v` |

Converted to `NFDepthConfig` before the renderer sees it.

## orbit camera input

`OrbitCameraInput` maps WASD to camera movement:

- **W / S** — move eye toward / away from target along the view axis
- **A / D** — strafe and preserve distance to target (orbit-style)

Wire it in `on_key` with `handle_key` and in `update` with `update_camera(camera, speed, dt)`.

## tracing

`install_tracing()` sets up the global subscriber. `run` calls it automatically; call it earlier if you need logs during setup.

See [docs/tracing.md](../../docs/tracing.md) for `RUST_LOG` and `NEON_LOG_FORMAT`.

## layout

```
src/
  lib.rs              — `run`, re-exports
  engine/
    camera.rs         — `Camera`
    depth.rs          — `NFDepth`
  game/mod.rs         — `Game`, `GameContext`
  input/
    orbit_camera.rs   — `OrbitCameraInput`
  window/mod.rs       — `NFWindow` (winit `ApplicationHandler`)
  tracing.rs          — subscriber install
```

## tests

Pure math and input logic are covered by unit tests (no GPU required):

```bash
cargo test -p neon-engine
```
