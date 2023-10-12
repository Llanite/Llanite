use crate::state::State;

/// Contains all of the startup code.
/// Each function contained here will be ran before the event loop.
#[derive(Default)]
pub struct Controller {
    pub stages: Vec<Box<dyn Fn(&'static mut State)>>,
}

impl Controller {
    pub fn add_stage<F>(&mut self, f: F)
    where
        F: Fn(&'static mut State) + 'static,
    {
        self.stages.push(Box::new(f));
    }
}
