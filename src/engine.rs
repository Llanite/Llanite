use crate::config::Config;

use std::sync::{Arc, Mutex, MutexGuard};
use ecs_rust::world::World;

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
        todo!("Start engine!")
    }
}
