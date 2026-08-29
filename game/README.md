# game (neonfall)

The Neonfall demo binary: a grid of instanced cubes with per-instance colors from an HSV sweep, orbit camera on WASD, Escape to quit.

## run

```bash
cargo run -p game
```

From the repo root so `./models/cube.glb` resolves:

```bash
cd /path/to/neonfall && cargo run -p game
```

## what it does

1. Builds a `grid_size × grid_size` instance grid (default 4×4 = 16 cubes)
2. Assigns each instance a unique hue via `hsv_to_rgb`
3. Loads `models/cube.glb` and attaches a runtime color atlas
4. Runs the engine with an orbit camera (`OrbitCameraInput`)

## controls

| key | action |
| --- | --- |
| W / S | move camera toward / away from target |
| A / D | strafe (preserves distance to target) |
| Escape | quit |

## logging

```bash
RUST_LOG=game=debug cargo run -p game
```

See [docs/tracing.md](../docs/tracing.md).

## layout

```
src/main.rs
  Neonfall          — `Game` impl (camera input + escape)
  build_scene       — grid instances + color atlas
  hsv_to_rgb        — hue → RGB for swatches
```

## tests

```bash
cargo test -p game
```

Covers `hsv_to_rgb` and scene builder instance counts (uses `models/cube.glb`).
