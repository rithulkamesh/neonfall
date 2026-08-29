use neon_renderer::NFDepthConfig;

/// Engine-level depth buffer configuration.
///
/// The renderer owns the GPU texture; the game chooses whether depth testing is on
/// and what value clears the buffer each frame.
#[derive(Debug, Clone, Copy)]
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
