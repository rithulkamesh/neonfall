use tracing::{debug, instrument};
use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, Device, Face, FragmentState,
    FrontFace, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexState,
};

use crate::gpu::NFVertex;

pub struct NFPipeline {
    pub(crate) raw: RenderPipeline,
}

impl NFPipeline {
    #[instrument(name = "pipeline.new", skip(device, camera_bind_group_layout), fields(format = ?format))]
    pub fn new(
        device: &Device,
        format: TextureFormat,
        camera_bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/test.wgsl").into()),
        });

        debug!(?format, "building render pipeline");

        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render NFPipeline Layout"),
            bind_group_layouts: &[Some(camera_bind_group_layout)],
            immediate_size: 0,
        });

        let raw = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render NFPipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(NFVertex::desc())],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Self { raw }
    }
}
