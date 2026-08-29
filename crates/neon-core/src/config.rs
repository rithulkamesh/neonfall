use glam::Vec2;

pub struct NFConfig {
    pub window_title: String,
    pub window_size: Vec2,
    pub vsync_enabled: bool,
    pub clear_color: [f32; 4],
}

impl Default for NFConfig {
    fn default() -> Self {
        Self {
            window_title: "Neon".to_string(),
            window_size: Vec2::new(1280.0, 720.0),
            vsync_enabled: true,
            clear_color: [0.1, 0.2, 0.3, 1.0],
        }
    }
}

impl From<(&str, Vec2, bool)> for NFConfig {
    fn from((window_title, window_size, vsync_enabled): (&str, Vec2, bool)) -> Self {
        Self {
            window_title: window_title.to_string(),
            window_size,
            vsync_enabled,
            clear_color: [0.1, 0.2, 0.3, 1.0],
        }
    }
}
