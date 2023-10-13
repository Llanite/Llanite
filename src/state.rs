use shipyard::World;
use wgpu::{
    util::DeviceExt, Buffer, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Instance,
    InstanceDescriptor, Limits, LoadOp, Operations, PowerPreference, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RequestAdapterOptionsBase,
    Surface, SurfaceConfiguration, SurfaceError, TextureUsages, TextureViewDescriptor,
};

use std::sync::Arc;
use winit::{dpi::PhysicalSize, event::*, window::Window};

use anyhow::Result;

use crate::errors::StateError;
use crate::pipeline_composer::PipelineComposer;
use crate::vertex::VERTICES;

pub struct State {
    backup_pipeline: RenderPipeline,
    config: SurfaceConfiguration,
    vertex_buffer: Buffer,
    device: Arc<Device>,
    num_vertices: u32,
    surface: Surface,
    window: Window,
    queue: Queue,

    pub pipeline_composer: PipelineComposer,

    pub(crate) size: PhysicalSize<u32>,
    pub(crate) world: World,
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

        let mut pipeline_composer = PipelineComposer::new(device.clone(), config.clone());

        let backup_pipeline = pipeline_composer.create_pipeline("shaders/triangle.wgsl".into())?;

        let world = World::default();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let num_vertices = VERTICES.len();

        let state = State {
            num_vertices: num_vertices as u32,
            backup_pipeline,
            device: device.clone(),
            pipeline_composer,
            vertex_buffer,
            config,
            size,
            surface,
            window,
            queue,
            world,
        };

        Ok(state)
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new: PhysicalSize<u32>) -> Result<()> {
        if new.width <= 0 && new.height <= 0 {
            return Err(StateError::ResizeTooSmall.into());
        }

        self.size = new;

        self.config.width = new.width;
        self.config.height = new.height;

        self.surface.configure(&self.device, &self.config);

        Ok(())
    }

    pub fn event(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        // TODO: Update world.
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let out = self.surface.get_current_texture()?;

        let view = out.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Use a new scope to drop mutable reference.
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),

                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
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

            // TODO: Alternative error handling method
            if let Some(pipeline) = &self.pipeline_composer.pipeline {
                render_pass.set_pipeline(pipeline);
            } else {
                render_pass.set_pipeline(&self.backup_pipeline);
                tracing::warn!("Using backup pipeline");
            }

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        out.present();

        Ok(())
    }
}
