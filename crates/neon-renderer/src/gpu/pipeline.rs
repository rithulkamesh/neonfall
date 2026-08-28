use tracing::{debug, instrument};
use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, Device, Face, FragmentState,
    FrontFace, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexState,
};

use crate::gpu::{NFInstanceRaw, NFVertex};

pub struct NFPipeline {
    pub(crate) raw: RenderPipeline,
}

const SHADER: &str = r#"
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) model_0: vec4<f32>,
    @location(3) model_1: vec4<f32>,
    @location(4) model_2: vec4<f32>,
    @location(5) model_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let instance_model = mat4x4<f32>(
        model.model_0,
        model.model_1,
        model.model_2,
        model.model_3
    );

    out.color = model.color;
    out.clip_position =
        camera.view_proj * instance_model * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

impl NFPipeline {
    #[instrument(name = "pipeline.new", skip(device, camera_bind_group_layout), fields(format = ?format
    ))]
    pub fn new(
        device: &Device,
        format: TextureFormat,
        camera_bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(SHADER.into()),
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
                buffers: &[Some(NFVertex::desc()), Some(NFInstanceRaw::desc())],
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
