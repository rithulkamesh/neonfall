use glam::Vec2;
use neon_engine::{self, NFMesh};

fn main() {
    neon_engine::init(
        ("Neonfall", Vec2::new(1280.0, 720.0), true),
        NFMesh::from("./models/cube.glb"),
    );
}
