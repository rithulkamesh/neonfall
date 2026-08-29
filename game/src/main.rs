use glam::{EulerRot, Quat, Vec2, Vec3};
use neon_engine::{
    Camera, Game, GameContext, NFDepth, NFInstance, NFMesh, OrbitCameraInput, KeyCode,
};
use tracing::{info, instrument};

struct Neonfall {
    camera_input: OrbitCameraInput,
    camera_speed: f32,
}

impl Game for Neonfall {
    fn update(&mut self, ctx: &mut GameContext, dt: f32) {
        self.camera_input.update_camera(ctx.camera, self.camera_speed, dt);
    }

    fn on_key(&mut self, ctx: &mut GameContext, key: KeyCode, pressed: bool) {
        if key == KeyCode::Escape && pressed {
            ctx.exit();
        }
        self.camera_input.handle_key(key, pressed);
    }
}

fn main() {
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

    neon_engine::run(
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
        NFDepth::enabled(),
        Neonfall {
            camera_input: OrbitCameraInput::default(),
            camera_speed: 5.0,
        },
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

    NFMesh::from(cube_model_path())
        .with_color_atlas(&colors)
        .with_instances(instances)
}

fn cube_model_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../models/cube.glb")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsv_red_hue() {
        let rgb = hsv_to_rgb(0.0, 1.0, 1.0);
        assert_eq!(rgb[0], 255);
        assert!(rgb[1] < 5);
        assert!(rgb[2] < 5);
    }

    #[test]
    fn hsv_green_hue() {
        let rgb = hsv_to_rgb(120.0, 1.0, 1.0);
        assert!(rgb[0] < 5);
        assert_eq!(rgb[1], 255);
        assert!(rgb[2] < 5);
    }

    #[test]
    fn hsv_blue_hue() {
        let rgb = hsv_to_rgb(240.0, 1.0, 1.0);
        assert!(rgb[0] < 5);
        assert!(rgb[1] < 5);
        assert_eq!(rgb[2], 255);
    }

    #[test]
    fn hsv_zero_saturation_is_gray() {
        let rgb = hsv_to_rgb(200.0, 0.0, 0.5);
        assert_eq!(rgb[0], 127);
        assert_eq!(rgb[1], 127);
        assert_eq!(rgb[2], 127);
    }

    #[test]
    fn build_scene_instance_count_matches_grid() {
        let grid_size = 4;
        let mesh = build_scene(grid_size);
        assert_eq!(mesh.instances.len(), grid_size as usize * grid_size as usize);
        assert!(mesh.diffuse.is_some());
        assert_eq!(mesh.atlas_grid, 4);
    }

    #[test]
    fn build_scene_assigns_unique_texture_indices() {
        let grid_size = 2;
        let mesh = build_scene(grid_size);
        let mut indices: Vec<u32> = mesh.instances.iter().map(|i| i.texture_index).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![0, 1, 2, 3]);
    }
}
