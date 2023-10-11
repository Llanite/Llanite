use wgpu::{
    include_wgsl, Backends, Instance, InstanceDescriptor, PipelineLayoutDescriptor,
    PowerPreference, RenderPipelineDescriptor, RequestAdapterOptionsBase, TextureUsages,
    VertexState,
};
use wgpu::{
    Device, DeviceDescriptor, Limits, Queue, RenderPipeline, Surface, SurfaceConfiguration,
};

use std::sync::Arc;
use winit::{dpi::PhysicalSize, event::*, window::Window};

use anyhow::Result;

use crate::errors::StateError;
use crate::pipeline_composer::PipelineComposer;

pub struct State {
    pipeline_composer: PipelineComposer,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
    device: Arc<Device>,
    surface: Surface,
    window: Window,
    queue: Queue,
}

impl State {
    pub async fn new(window: Window) -> Result<State> {
        let size = window.inner_size();
        let (width, height) = (size.width, size.height);

        let instance = Instance::new(InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window) }?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptionsBase {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(StateError::NoAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: match cfg!(target_arch = "wasm32") {
                        true => Limits::downlevel_webgl2_defaults(),
                        false => Limits::default(),
                    },
                    label: None,
                },
                None,
            )
            .await?;

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            present_mode: swapchain_capabilities.present_modes[0],
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            format: swapchain_format,
            view_formats: vec![],
            width,
            height,
        };

        surface.configure(&device, &config);

        let device = Arc::new(device);

        let pipeline_composer = PipelineComposer::new(device.clone(), config.clone());

        let state = State {
            device: device.clone(),
            pipeline_composer,
            config,
            size,
            surface,
            window,
            queue,
        };

        Ok(state)
    }
}
