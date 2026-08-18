use glam::Vec2;

pub struct NeonConfig {
    pub window_title: String,
    pub window_size: Vec2,
    pub vsync_enabled: bool,
}

impl Default for NeonConfig {
    fn default() -> Self {
        Self {
            window_title: "Neon".to_string(),
            window_size: Vec2::new(1280.0, 720.0),
            vsync_enabled: true,
        }
    }
}

impl From<(&str, Vec2, bool)> for NeonConfig {
    fn from((window_title, window_size, vsync_enabled): (&str, Vec2, bool)) -> Self {
        Self {
            window_title: window_title.to_string(),
            window_size,
            vsync_enabled,
        }
    }
}
