mod mesh;

use glam::Vec2;
use neon_engine::{self, Mesh};

use mesh::{INDICES, VERTICES};

fn main() {
    neon_engine::init(
        ("Neonfall", Vec2::new(1280.0, 720.0), true),
        Mesh::new(VERTICES, INDICES),
    );
}
