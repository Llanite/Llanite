use crate::booster;
use crate::config::Config;

use shipyard::World;
use std::sync::{Arc, Mutex};

use booster::Booster;

use tracing::error;

pub struct Llanite {
    booster: Booster,
}

impl Default for Llanite {
    fn default() -> Self {
        let booster = Booster::new().unwrap();

        Self { booster }
    }
}

impl Llanite {
    pub fn start(&self, config: Config) {
        let mut booster = Booster::new().unwrap();

        if let Err(e) = pollster::block_on(booster.launch(config)) {
            error!("Launch error: {e}");
        }
    }

    pub fn update_pipeline(&mut self) {
        if let Some(state) = &self.booster.state {
            let mut state = state.lock().unwrap();

            state.update_pipeline();
        }
    }
}
