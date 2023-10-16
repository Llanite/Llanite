use wgpu::{
    BlendComponent, BlendState, ColorTargetState, ColorWrites, Device, Face, FragmentState,
    FrontFace, MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState,
    PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    ShaderSource, SurfaceConfiguration, VertexState,
};

use crate::vertex::Vertex;
use anyhow::Result;
use std::sync::Arc;
use tracing::info;

const BACKUP_SOURCE: &str = include_str!("../shaders/triangle.wgsl");

pub struct PipelineComposer {
    pub(crate) pipeline: Option<RenderPipeline>,

    config: SurfaceConfiguration,
    device: Arc<Device>,
}

impl PipelineComposer {
    pub fn new(device: Arc<Device>, config: SurfaceConfiguration) -> PipelineComposer {
        Self {
            pipeline: None,
            config,
            device,
        }
    }

    pub fn new_pipeline(&mut self, path: Option<&str>) -> Result<()> {
        self.pipeline = Some(self.create_pipeline(path)?);

        let path = match path {
            Some(v) => v.to_string(),
            None => "backup.".to_string(),
        };

        info!("Read pipeline {path}");

        Ok(())
    }

    pub(crate) fn create_pipeline(&mut self, path: Option<&str>) -> Result<RenderPipeline> {
        let source = match path {
            Some(v) => std::fs::read_to_string(v)?,
            None => BACKUP_SOURCE.to_string(),
        };

        let path = match path {
            Some(v) => v.to_string(),
            None => "backup.".to_string(),
        };

        info!("Read shader source {path}");

        let shader = self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(source.into()),
        });

        let layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let pipeline = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&layout),

                vertex: VertexState {
                    module: &shader,

                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },

                fragment: Some(FragmentState {
                    module: &shader,

                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        format: self.config.format,
                        blend: Some(BlendState {
                            color: BlendComponent::REPLACE,
                            alpha: BlendComponent::REPLACE,
                        }),

                        write_mask: ColorWrites::ALL,
                    })],
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
                    alpha_to_coverage_enabled: false,

                    mask: !0,
                    count: 1,
                },

                multiview: None,
            });

        Ok(pipeline)
    }
}
