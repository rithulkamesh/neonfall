# tracing

logging goes to stdout via [`tracing`](https://docs.rs/tracing): a global subscriber in `neon-engine`, spans on gpu/window setup, and events in the renderer.

installed from `neon_engine::init` via `tracing::install()`. call `neon_engine::install_tracing()` earlier if you want logs from game setup before `init` runs.

## quick start

```bash
# defaults (info, wgpu noise muted)
cargo run -p game

# see renderer debug events + span close timings
RUST_LOG=neon_renderer=debug,neon_engine=debug,game=debug cargo run -p game

# every frame (noisy)
RUST_LOG=neon_renderer::gpu::texture=trace,neon_renderer=debug,game=debug cargo run -p game

# structured json on stdout
NEON_LOG_FORMAT=json RUST_LOG=info cargo run -p game
```

## what is installed

| piece | role |
| --- | --- |
| `EnvFilter` | `RUST_LOG` (or a quiet default) |
| `fmt` layer | compact human logs, or json when `NEON_LOG_FORMAT=json` |
| `ErrorLayer` (`tracing-error`) | attach span context to errors |
| `LogTracer` (`tracing-log`) | bridge `log` from wgpu/winit into tracing |
| `FmtSpan::CLOSE` | print span duration when a span ends |
| spans (`#[instrument]`) | `game.scene`, `gpu.new`, `depth.new`, `texture.*`, `textures.*`, `mesh.*`, `pipeline.new`, `state.new` / `resize` / `render` / `set_instances`, `window.*` |

default filter when `RUST_LOG` is unset:

```text
info,neon_engine=info,neon_renderer=info,wgpu_core=warn,wgpu_hal=warn,naga=warn
```

## environment variables

| variable | values | effect |
| --- | --- | --- |
| `RUST_LOG` | envfilter directive | level / target filter (crate::module=level) |
| `NEON_LOG_FORMAT` | `json` (else compact text) | output format |

### useful `RUST_LOG` examples

```bash
RUST_LOG=info
RUST_LOG=neon_renderer=debug
RUST_LOG=neon_renderer::gpu=trace,neon_engine=debug
RUST_LOG=wgpu_core=error,wgpu_hal=error,neon_renderer=debug
```

`state.render` is `#[instrument(level = "trace")]` so it only shows when you ask for trace on that target.

## adding more spans

prefer `#[instrument]` on functions that do real work (init, load, resize). skip hot per-frame paths unless you set `level = "trace"`.

```rust
use tracing::instrument;

#[instrument(name = "assets.load", skip(path), err)]
fn load(path: &Path) -> anyhow::Result<Asset> {
    tracing::debug!(?path, "loading");
    // ...
}
```

structured fields beat string formatting:

```rust
info!(adapter = %name, backend = ?backend, "acquired gpu adapter");
error!(error = %e, "render failed");
```

## layout

- install: `crates/neon-engine/src/tracing.rs`
- call site: `crates/neon-engine/src/lib.rs` (`init`)
- spans / events: `neon-engine` window + `neon-renderer` gpu/state
