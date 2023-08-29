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

        let shader = device
            .create_shader_module(wgpu::include_wgsl!("base_shaders/triangle.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),

            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
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
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },

            depth_stencil: None,

            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },

            multiview: None,
        });

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(
        &mut self,
        new: PhysicalSize<u32>
    ) -> Result<(), BoosterError> {
        if new.width > 0 && new.height > 0 {
            self.size = new;

            self.config.width = new.width;
            self.config.height = new.height;

            self.surface.configure(&self.device, &self.config);
        } else {
            Err(BoosterError::ResizeFailure)
        }

        Ok(())
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        // Allow the event loop to continue despite input.
        false
    }

    pub(crate) fn update(&mut self) {
        // Nothing to update yet
    }
}
