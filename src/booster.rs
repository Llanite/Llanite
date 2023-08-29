/// Booster. The file where everything kicks off in the engine loop.

use crate::errors::BoosterError;
use crate::config::Config;

use winit::dpi::PhysicalSize;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

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

    todo!("Launch!")
}
