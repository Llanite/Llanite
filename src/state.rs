use winit::{event::*, window::Window, dpi::PhysicalSize};
use wgpu::{Surface, Device, Queue, SurfaceConfiguration, RenderPipeline};
use crate::errors::BoosterError;

// TODO: Exterminate all `unwraps`.

// TODO: Only make public what needs to be public.
pub struct State {
    pub(crate) surface: Surface,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) config: SurfaceConfiguration,
    pub(crate) size: PhysicalSize<u32>,
    pub(crate) window: Window,
    pub(crate) render_pipeline: RenderPipeline,
}

impl State {
    /// Create a new state.
    pub async fn new(window: Window) -> Result<Self, BoosterError> {
        let size = window.inner_size();
        let (width, height) = (size.width, size.height);

        // All backends:
        // * Vulkan
        // * Metal
        // * DX12
        // * Browser WebGPU

        let instace = wgpu::Instance::new(wgpu::InstanceDescriptor {
            dx12_shader_compiler: Default::default(),
            backends: wgpu::Backends::all(),
        });

        let surface = unsafe { instance.create_surface(&window) }
            .map_err(|_| BoosterError::SurfaceFailure)?;

        let adapter = instace
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                comptabile_surface: Some(&surface),
                force_fallback_adapter: false,
            }).await.ok_or(BoosterError::NoGPU)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: match cfg!(target_arch = "wasm32") {
                        true => wgpu::Limits::downlevel_webgl2_defaults(),
                        false => wgpu::Limits::default(),
                    },
                    label: None,
                },
                None,
            ).await.unwrap();
        
        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|format| format.describe().srgb)
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            format: surface_format,
            view_formats: vec![],
            width,
            height,
        };

        surface.configure(&device, &config);
    }
}
