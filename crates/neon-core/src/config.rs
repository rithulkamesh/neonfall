use glam::Vec2;

/// Window and framebuffer settings for `neon_engine::run`.
///
/// Use [`Default`] for stock values or the `(&str, Vec2, bool)` tuple via [`From`]
/// for title, size, and vsync. `clear_color` stays at the default unless you
/// build the struct manually.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let cfg = NFConfig::default();
        assert_eq!(cfg.window_title, "Neon");
        assert_eq!(cfg.window_size, Vec2::new(1280.0, 720.0));
        assert!(cfg.vsync_enabled);
        assert_eq!(cfg.clear_color, [0.1, 0.2, 0.3, 1.0]);
    }

    #[test]
    fn from_tuple_sets_window_fields() {
        let cfg = NFConfig::from(("Neonfall", Vec2::new(800.0, 600.0), false));
        assert_eq!(cfg.window_title, "Neonfall");
        assert_eq!(cfg.window_size, Vec2::new(800.0, 600.0));
        assert!(!cfg.vsync_enabled);
        assert_eq!(cfg.clear_color, [0.1, 0.2, 0.3, 1.0]);
    }

    #[test]
    fn into_nf_config_via_tuple() {
        let cfg: NFConfig = ("Test", Vec2::ONE, true).into();
        assert_eq!(cfg.window_title, "Test");
        assert_eq!(cfg.window_size, Vec2::ONE);
        assert!(cfg.vsync_enabled);
    }

    #[test]
    fn clear_color_can_be_set_manually() {
        let cfg = NFConfig {
            window_title: "x".into(),
            window_size: Vec2::ZERO,
            vsync_enabled: false,
            clear_color: [0.0, 0.0, 0.0, 0.0],
        };
        assert_eq!(cfg.clear_color, [0.0, 0.0, 0.0, 0.0]);
    }
}
