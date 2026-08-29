use neon_renderer::NFDepthConfig;

/// Engine-level depth buffer configuration.
///
/// The renderer owns the GPU texture; the game chooses whether depth testing is on
/// and what value clears the buffer each frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NFDepth {
    pub enabled: bool,
    pub clear: f32,
}

impl Default for NFDepth {
    fn default() -> Self {
        Self::enabled()
    }
}

impl NFDepth {
    pub const fn enabled() -> Self {
        Self {
            enabled: true,
            clear: 1.0,
        }
    }

    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            clear: 1.0,
        }
    }

    pub const fn with_clear(clear: f32) -> Self {
        Self {
            enabled: true,
            clear,
        }
    }
}

impl From<NFDepth> for NFDepthConfig {
    fn from(depth: NFDepth) -> Self {
        Self {
            enabled: depth.enabled,
            clear: depth.clear,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_enabled() {
        let depth = NFDepth::default();
        assert!(depth.enabled);
        assert_eq!(depth.clear, 1.0);
    }

    #[test]
    fn disabled_has_depth_off() {
        let depth = NFDepth::disabled();
        assert!(!depth.enabled);
        assert_eq!(depth.clear, 1.0);
    }

    #[test]
    fn with_clear_preserves_enabled() {
        let depth = NFDepth::with_clear(0.5);
        assert!(depth.enabled);
        assert_eq!(depth.clear, 0.5);
    }

    #[test]
    fn converts_to_renderer_config() {
        let depth = NFDepth::with_clear(0.25);
        let cfg: NFDepthConfig = depth.into();
        assert!(cfg.enabled);
        assert_eq!(cfg.clear, 0.25);
    }

    #[test]
    fn enabled_constructor_matches_default() {
        assert_eq!(NFDepth::enabled(), NFDepth::default());
    }
}
