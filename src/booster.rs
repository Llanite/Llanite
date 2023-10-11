use winit::dpi::PhysicalSize;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::config::Config;
use crate::state::State;

use std::sync::{Arc, Mutex};

use anyhow::Result;

pub struct Booster {
    state: Arc<Mutex<State>>,
}

impl Booster {
    pub fn new(config: Config) -> Result<Booster> {
        let (width, height) = (config.width, config.height);

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width, height))
            .with_title(config.title)
            .build(&event_loop)?;

        let state = Arc::new(Mutex::new(pollster::block_on(State::new(window))?));

        Ok(Self { state })
    }

    pub fn launch(&self) {
        
    }
}
