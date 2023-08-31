use crate::errors::PipelineError;
use std::{fs, sync::Arc};
use tracing::info;
use wgpu::{Device, FragmentState, RenderPipeline, SurfaceConfiguration};

pub struct PipelineComposer {
    pub(crate) pipeline: Result<RenderPipeline, PipelineError>,

    config: SurfaceConfiguration,
    device: Arc<Device>,
}

impl PipelineComposer {
    pub fn new(device: Arc<Device>, config: SurfaceConfiguration) -> Self {
        Self {
            pipeline: Err(PipelineError::NotInitialised),

            device,
            config,
        }
    }

    pub fn new_pipeline(&mut self, shader_path: &str) {
        info!("New pipeline from {shader_path}");

        let shader_source = match fs::read_to_string(shader_path) {
            Ok(v) => v,
            Err(_) => {
                self.pipeline = Err(PipelineError::InvalidPath);

                return;
            }
        };

        info!("Read shader source file from: {shader_path}");

        // TODO: Need to change label for each shader?
        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        // TODO: Need to change label for each layout?
        let layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        // TODO: Give extra access to options for pipeline descriptor?
        // TODO: Need to change label for each render pipeline?
        // TODO: Need to create multiple render pipelines?
        self.pipeline = Ok(self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&layout),

                vertex: wgpu::VertexState {
                    module: &shader,

                    // NOTE: Change to `vertex_main`?
                    entry_point: "vs_main",
                    buffers: &[],
                },

                fragment: Some(FragmentState {
                    module: &shader,

                    // NOTE: Change to `fragment_main`?
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.config.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),

                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),

                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,

                    // Set which face is at the front
                    front_face: wgpu::FrontFace::Ccw,

                    // Backface culling is enabled
                    cull_mode: Some(wgpu::Face::Back),

                    // Using anything other than Fill requires Features::POLYGON_MODE_LINE
                    // or Features::POLYGON_MODE_POINT
                    polygon_mode: wgpu::PolygonMode::Fill,

                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,

                    // Requirest Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },

                depth_stencil: None,

                multisample: wgpu::MultisampleState {
                    alpha_to_coverage_enabled: false,

                    mask: !0,
                    count: 1,
                },

                multiview: None,
            }));
    }
}
