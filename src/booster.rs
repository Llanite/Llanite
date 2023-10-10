/// Booster. The file where everything kicks off in the engine loop.
use crate::config::Config;
use crate::state::State;

use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::dpi::PhysicalSize;
use winit::event::*;

use tracing_subscriber::fmt::time::LocalTime;
use time::macros::format_description;

use tracing::{warn, error, Level};

use std::sync::{Arc, Mutex};

use anyhow::Result;

pub struct Booster {
    pub(crate) state: Option<Arc<Mutex<State<'static>>>>,
}

impl Booster {
    pub fn new() -> Result<Booster> {
        let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));

        tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .with_thread_names(true)
            .with_timer(timer)
            .init();

        Ok(Self { state: None })
    }

    pub async fn launch(&mut self, config: Config) -> Result<()> {
        let (width, height) = (config.width, config.height);

        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width, height))
            .with_title(config.window_name)
            .build(&event_loop)?;

        let state = Arc::new(Mutex::new(State::new(window).await?));

        self.state = Some(state.clone());

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.lock().unwrap().window().id() => {
                if !state.lock().unwrap().event(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
    
                        WindowEvent::Resized(physical_size) => {
                            let _ = state.lock().unwrap().resize(*physical_size);
                        }
    
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            let _ = state.lock().unwrap().resize(**new_inner_size);
                        }
    
                        _ => {}
                    }
                }
            }
    
            Event::RedrawRequested(window_id) if window_id == state.lock().unwrap().window().id() => {
                state.lock().unwrap().update();
    
                match state.lock().unwrap().render() {
                    Ok(_) => {}
    
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.lock().unwrap().resize(state.lock().unwrap().size).unwrap();
                    }
    
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        error!("Ran out of memory");
                        *control_flow = ControlFlow::Exit
                    }
    
                    Err(wgpu::SurfaceError::Timeout) => warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                state.lock().unwrap().window().request_redraw();
            }
    
            _ => {}
        });
    }
}