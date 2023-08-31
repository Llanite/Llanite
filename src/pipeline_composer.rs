use wgpu::{Device, RenderPipeline, SurfaceConfiguration};
use crate::errors::PipelineError;
use std::{fs, sync::Arc};
use tracing::info;

pub struct PipelineComposer {
    pub(crate) pipeline: Result<RenderPipeline, PipelineError>,
}
