use crate::state::State;
use std::sync::{Arc, Mutex};

type Stage = Box<dyn Fn(&mut State)>;

/// Contains all of the startup code.
/// Each function contained here will be ran before the event loop.
#[derive(Default)]
pub struct Controller {
    pub stages: Vec<Stage>,
}

impl Controller {
    pub fn add_stage<F>(&mut self, f: F)
    where
        F: Fn(&mut State) + 'static,
    {
        self.stages.push(Box::new(f));
    }

    pub fn run(&self, state: Arc<Mutex<State>>) {
        let mut state = state.lock().unwrap();

        for stage in &self.stages {
            stage(&mut state);
        }
    }
}
