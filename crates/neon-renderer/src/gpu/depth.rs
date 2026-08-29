use tracing::{debug, instrument};
use wgpu::{
    CompareFunction, DepthBiasState, DepthStencilState, Device, Extent3d, StencilState, Texture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

/// Renderer-side depth settings passed in from the engine.
#[derive(Debug, Clone, Copy)]
pub struct NFDepthConfig {
    pub enabled: bool,
    pub clear: f32,
}

impl Default for NFDepthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            clear: 1.0,
        }
    }
}

#[cfg(test)]
mod depth_config_tests {
    use super::*;

    #[test]
    fn default_is_enabled_with_far_clear() {
        let cfg = NFDepthConfig::default();
        assert!(cfg.enabled);
        assert_eq!(cfg.clear, 1.0);
    }

    #[test]
    fn depth_stencil_state_uses_depth32_float() {
        let state = NFDepthConfig::depth_stencil_state();
        assert_eq!(state.format, NFDepthTexture::FORMAT);
        assert_eq!(state.depth_compare, Some(CompareFunction::Less));
    }
}

impl NFDepthConfig {
    pub fn depth_stencil_state() -> DepthStencilState {
        DepthStencilState {
            format: NFDepthTexture::FORMAT,
            depth_write_enabled: Some(true),
            depth_compare: Some(CompareFunction::Less),
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        }
    }
}

pub struct NFDepthTexture {
    _texture: Texture,
    view: TextureView,
}

impl NFDepthTexture {
    pub const FORMAT: TextureFormat = TextureFormat::Depth32Float;

    #[instrument(name = "depth.new", skip(device), fields(width, height))]
    pub fn new(device: &Device, width: u32, height: u32, label: &str) -> Self {
        let width = width.max(1);
        let height = height.max(1);

        debug!(label, width, height, "creating depth texture");

        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: Self::FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        Self {
            _texture: texture,
            view,
        }
    }

    pub fn view(&self) -> &TextureView {
        &self.view
    }
}
