/// Booster. The file where everything kicks off in the engine loop.

use crate::errors::BoosterError;
use crate::config::Config;
use crate::state::State;

use winit::dpi::PhysicalSize;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use tracing::{error, warn};

/// Call the asynchronous launch function.
pub fn initiate(config: Config) -> Result<(), BoosterError> {
    pollster::block_on(self::launch(config))?;

    Ok(())
}

/// Create the event loop and start up.
pub async fn launch(config: Config) -> Result<(), BoosterError> {
    let (width, height) = (config.width, config.height);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(width, height))
        .with_title(config.window_name)
        .build(&event_loop)
        .map_err(|_| BoosterError::WindowFailure)?;

    let mut state = State::new(window).await?;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.event(event) {
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
                            let _ = state.resize(*physical_size);
                        }

                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            let _ = state.resize(**new_inner_size);
                        }

                        _ => {}
                    }
                }
            }

            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();

                match state.render() {
                    Ok(_) => {}

                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size).unwrap();
                    }

                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        error!("Ran out of memory");
                        *control_flow = ControlFlow::Exit
                    },

                    Err(wgpu::SurfaceError::Timeout) => warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                state.window().request_redraw();
            }

            _ => {}
        }
    });
}
