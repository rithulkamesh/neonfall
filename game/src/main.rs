use glam::{EulerRot, Quat, Vec2, Vec3};
use neon_engine::{self, Camera, NFInstance, NFMesh};
use tracing::{info, instrument};

fn main() {
    neon_engine::install_tracing();

    let size = Vec2::new(1280.0, 720.0);
    let grid_size = 4;
    let mesh = build_scene(grid_size);

    info!(
        grid_size,
        instance_count = grid_size * grid_size,
        width = size.x,
        height = size.y,
        "starting neonfall"
    );

    neon_engine::init(
        ("Neonfall", size, true),
        mesh,
        Camera::new(
            Vec3::new(0.0, 10.0, 10.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Y,
            size.x / size.y,
            45.0,
            0.1,
            100.0,
        ),
    );
}

#[instrument(name = "game.scene", fields(grid_size, instance_count = grid_size * grid_size))]
fn build_scene(grid_size: u32) -> NFMesh {
    let instance_count = (grid_size * grid_size) as usize;

    let mut colors = Vec::with_capacity(instance_count);
    for i in 0..instance_count {
        let hue = (i as f32 / instance_count as f32) * 360.0;
        colors.push(hsv_to_rgb(hue, 0.75, 0.9));
    }

    info!(
        color_count = colors.len(),
        atlas_cells = instance_count,
        "generated runtime color swatches"
    );

    let mut instances = Vec::with_capacity(instance_count);
    let spacing = 2.2;
    for x in 0..grid_size {
        for y in 0..grid_size {
            let index = (y * grid_size + x) as u32;
            let position = Vec3::new(
                (x as f32 - (grid_size as f32) / 2.0 + 0.5) * spacing,
                (y as f32 - (grid_size as f32) / 2.0 + 0.5) * spacing,
                0.0,
            );
            let angle_x = (x as f32) * 0.3;
            let angle_y = (y as f32) * 0.2;
            let rotation = Quat::from_euler(EulerRot::YXZ, angle_y, angle_x, 0.0);
            instances.push(NFInstance::new(position, rotation).with_texture_index(index));
        }
    }

    info!(
        instance_count = instances.len(),
        model = "./models/cube.glb",
        "built instanced cube grid"
    );

    NFMesh::from("./models/cube.glb")
        .with_color_atlas(&colors)
        .with_instances(instances)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
    let c = v * s;
    let h = h / 60.0;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let (r, g, b) = match h as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ]
}
