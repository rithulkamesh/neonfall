use glam::{Vec2, Vec3};
use neon_engine::{self, Camera, NFMesh};

fn main() {
    let size = Vec2::new(1280.0, 720.0);
    neon_engine::init(
        ("Neonfall", size, true),
        NFMesh::from("./models/cube.glb"),
        Camera::new(
            Vec3::new(0.0, 2.0, 4.0), // eye -- Distance from Target
            Vec3::new(0.0, 0.0, 0.0), // target
            Vec3::Y,                  // Up
            size.x / size.y,          // Aspect Ratio
            45.0,                     // yFOV
            0.1,                      // zNear
            100.0,                    // zFar
        ),
    );
}
