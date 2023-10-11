use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::dpi::PhysicalSize;
use winit::event::*;

use crate::config::Config;

use anyhow::Result;

#[derive(Default)]
pub struct Booster {}

impl Booster {
    pub fn launch(config: Config) -> Result<()> {
        let (width, height) = (config.width, config.height);

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width, height))
            .with_title(config.title)
            .build(&event_loop);

        Ok(())
    }
}
