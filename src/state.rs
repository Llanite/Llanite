use std::sync::Arc;

use crate::{errors::{BoosterError, PipelineError}, pipeline_composer::PipelineComposer};
use wgpu::{Device, Queue, RenderPipeline, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, event::*, window::Window};
use tracing::error;

// TODO: Exterminate all `unwraps`.

// TODO: Only make public what needs to be public.
pub struct State<'a> {
    pub(crate) pipeline_composer: PipelineComposer,
    pub(crate) render_pipeline: RenderPipeline,
    pub(crate) config: SurfaceConfiguration,
    pub(crate) size: PhysicalSize<u32>,
    pub(crate) device: Arc<Device>,
    pub(crate) surface: Surface,
    pub(crate) window: Window,
    pub(crate) queue: Queue,

    pipeline_ref: Option<&'a RenderPipeline>,
}

impl<'a> State<'a> {
    /// Create a new state.
    pub async fn new(window: Window) -> Result<State<'static>, BoosterError> {
        let size = window.inner_size();
        let (width, height) = (size.width, size.height);

        // All backends:
        // * Vulkan
        // * Metal
        // * DX12
        // * Browser WebGPU

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            dx12_shader_compiler: Default::default(),
            backends: wgpu::Backends::all(),
        });

        let surface = unsafe { instance.create_surface(&window) }
            .map_err(|_| BoosterError::SurfaceFailure)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(BoosterError::NoGPU)?;

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
            )
            .await
            .unwrap();

        let swapchain_capabilities = surface.get_capabilities(&adapter);

        // Only works on older version of wgpu
        // let swapchain_format = swapchain_capabilities
        // .formats
        // .iter()
        // .copied()
        // .find(|format| format.describe().srgb)
        // .unwrap_or(swapchain_capabilities.formats[0]);

        let swapchain_format = swapchain_capabilities.formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            present_mode: swapchain_capabilities.present_modes[0],
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            format: swapchain_format,
            view_formats: vec![],
            width,
            height,
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/triangle.wgsl"));

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

        let device = Arc::new(device);

        let pipeline_composer = PipelineComposer::new(device.clone(), config.clone());

        Ok(State {
            device: device.clone(),
            pipeline_ref: None,
            pipeline_composer,
            render_pipeline,
            config,
            size,
            surface,
            window,
            queue,
        })
    }

    pub fn update_pipeline(&'a mut self) -> Result<(), PipelineError> {
        match &self.pipeline_composer.pipeline {
            Ok(pipeline) => self.pipeline_ref = Some(pipeline),
            Err(e) => return Err(*e),
        }

        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new: PhysicalSize<u32>) -> Result<(), BoosterError> {
        if new.width > 0 && new.height > 0 {
            self.size = new;

            self.config.width = new.width;
            self.config.height = new.height;

            self.surface.configure(&self.device, &self.config);
        } else {
            return Err(BoosterError::ResizeFailure);
        }

        Ok(())
    }

    pub(crate) fn event(&mut self, _event: &WindowEvent) -> bool {
        // Allow the event loop to continue despite input.
        false
    }

    pub(crate) fn update(&mut self) {
        // TODO: Update world.
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Get the current texture
        let out = self.surface.get_current_texture()?;

        let view = out
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a render encoder
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Make a render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // Clear the screen
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.015,
                            g: 0.015,
                            b: 0.015,
                            a: 1.0,
                        }),

                        store: true,
                    },
                })],

                depth_stencil_attachment: None,
            });

            // This unwrap is okay as there should *always*, due to code before,
            // be a value in there.

            match self.pipeline_ref {
                Some(v) => render_pass.set_pipeline(v),
                None => render_pass.set_pipeline(&self.render_pipeline),
            }

            render_pass.draw(0..3, 0..1);
        }

        // Submit the command
        self.queue.submit(std::iter::once(encoder.finish()));

        // Present
        out.present();

        Ok(())
    }
}
