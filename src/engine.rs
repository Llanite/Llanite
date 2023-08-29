use crate::config::Config;
use crate::booster::booster;

use std::sync::{Arc, Mutex};
use ecs_rust::world::World;

use tracing::error;

pub struct Llanite {
    world: Arc<Mutex<World>>,
}

impl Default for Llanite {
    fn default() -> Self {
        let world = Arc::new(Mutex::new(World::new()));

        Self { world }
    }
}

impl Llanite {
    pub fn start(&self, config: Config) {
        if let Err(e) = booster::initiate(config) {
            error!("Error starting window: {e}");
        }
    }
}
