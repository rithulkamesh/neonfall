<p align="center">
  <img src="docs/neonfall.png" alt="Neonfall" width="420" />
</p>

Civilization collapsed.

Cities sit empty. Factories still run. Corporate ruins hum under reclaiming green. You're left to explore what's left, rebuild what matters, and figure out what the world became after people stopped being the point.

A game engine and a game, written by hand — no AI — so I can actually understand how game making works. Don't care if it takes months or years. It's mine. Solo. Not accepting PRs.

I have to add that I do use AI to learn how to write these pieces of code, where they slot in, why they exist, etc., and other purposes like formatting/formalizing my devlogs, creating READMEs and documentation, because as a sane human I want to be focused on relaxing through writing this engine. I do want to experience the joys of coding by hand and that's solely the reason I'm doing this, doesn't mean I'll not push mindless tasks to an agent :)

Daily notes live in [`docs/log/`](docs/log/) (`dd-mm-yyyy`).

## crates

| crate | docs |
| --- | --- |
| `neon-core` | [README](crates/neon-core/README.md) — window/config types |
| `neon-engine` | [README](crates/neon-engine/README.md) — event loop, `Game` trait, camera |
| `neon-renderer` | [README](crates/neon-renderer/README.md) — wgpu mesh, textures, draw |
| `game` | [README](game/README.md) — Neonfall demo binary |

Run tests (no GPU required for most):

```bash
cargo test -p neon-core -p neon-engine -p neon-renderer -p game
```

CI runs `cargo test --workspace` on every push/PR via [`.github/workflows/test.yml`](.github/workflows/test.yml).

logging / spans: [`docs/tracing.md`](docs/tracing.md) (`RUST_LOG`, json).
