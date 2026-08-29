# neon-renderer

wgpu-based renderer for Neonfall: GPU device/surface setup, mesh loading from glTF, instanced draws, diffuse textures and color atlases, depth buffer, and the per-frame `NFState` render loop.

## data flow

```
NFMesh (CPU)  →  NFState::new  →  GPU buffers + pipeline
Camera matrix →  set_view_proj →  uniform buffer
                →  render()     →  swapchain present
```

## `NFMesh`

Built from glTF or manually:

```rust
use neon_renderer::{NFInstance, NFMesh};
use glam::{Quat, Vec3};

// from file (panics on failure — use load_gltf for Result)
let mesh = NFMesh::from("./models/cube.glb");

// runtime color atlas (one swatch per instance index)
let colors: Vec<[u8; 3]> = vec![[255, 0, 0], [0, 255, 0]];
let mesh = NFMesh::from("./models/cube.glb")
    .with_color_atlas(&colors)
    .with_instances(vec![
        NFInstance::new(Vec3::ZERO, Quat::IDENTITY).with_texture_index(0),
        NFInstance::new(Vec3::new(2.0, 0.0, 0.0), Quat::IDENTITY).with_texture_index(1),
    ]);
```

| method | effect |
| --- | --- |
| `NFMesh::new(vertices, indices)` | empty mesh, one default instance |
| `with_instances` | replace instance list |
| `with_diffuse` | single diffuse `NFTextureImage` |
| `with_color_atlas` | pack RGB swatches into a square atlas; vertex colors set to white |
| `load_gltf(path)` | `Result` — vertices, indices, optional embedded diffuse |

## `NFInstance`

Per-draw instance data: position, rotation (`Quat`), and `texture_index` for atlas sampling.

`to_raw()` packs a model matrix + index for the instance vertex buffer.

## `NFTextureImage`

CPU-side RGBA pixels uploaded at init:

- `NFTextureImage::white()` — 1×1 white
- `NFTextureImage::solid([r, g, b])` — 1×1 solid color
- `NFTextureImage::color_atlas(colors)` — square power-of-two grid of 8×8 swatches

## `NFState`

Created async with a winit `Window`, mesh, depth config, and clear color:

```rust
let state = NFState::new(window, &mesh, NFDepthConfig::default(), clear_color).await?;
state.resize(width, height);
state.set_view_proj(camera_matrix);
state.render()?;
```

| method | purpose |
| --- | --- |
| `resize` | reconfigure surface + depth texture |
| `set_view_proj` | update camera uniform |
| `set_instances` | update instance buffer (count must match init) |
| `set_clear_color` | change framebuffer clear |
| `request_redraw` | ask winit for another frame |

## depth

`NFDepthConfig { enabled, clear }` controls whether a `Depth32Float` attachment is used and its clear value. When disabled, the pipeline has no depth stencil state.

## shader

Instanced vertex shader: per-vertex position/color/UV + per-instance model matrix and texture index. Fragment shader samples the diffuse atlas and multiplies vertex color.

Atlas UVs are computed in WGSL from `texture_index` and a uniform grid size.

## layout

```
src/
  lib.rs
  state/mod.rs          — `NFState`
  gpu/
    device.rs           — `NFGpu` (adapter, surface, queue)
    vertex.rs           — `NFVertex`, `NFMesh`, glTF load
    instance.rs         — `NFInstance`, `NFInstanceRaw`
    texture.rs          — `NFTextureImage`, `NFTextures`
    depth.rs            — `NFDepthConfig`, `NFDepthTexture`
    pipeline.rs         — `NFPipeline` + WGSL
```

## tests

Unit tests cover mesh loading, instance packing, texture atlases, and buffer layouts. glTF tests read `../../models/cube.glb` relative to the crate:

```bash
cargo test -p neon-renderer
```

GPU initialization is not tested in CI-style unit tests (requires a window and adapter).
