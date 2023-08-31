use wgpu::{Device, RenderPipeline, SurfaceConfiguration, FragmentState};
use crate::errors::PipelineError;
use std::{fs, sync::Arc};
use tracing::info;

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
}
