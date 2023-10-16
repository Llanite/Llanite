mod booster;
mod config;
mod controller;
mod errors;
mod logging;
mod pipeline_composer;
mod state;
mod vertex;

pub mod prelude;

use booster::Booster;
use config::Config;
use config::LogConfig;
use prelude::Controller;

use tracing::error;

/// The main struct for the engine.
#[derive(Default)]
pub struct Llanite(Booster);

impl Llanite {
    pub fn start(&mut self, config: Config, controller: Controller) {
        // Make sure there is some sort of logging for errors.
        logging::enable_logging(None);

        if let Err(e) = self.0.launch(config, controller) {
            error!("Error: {e}")
        }
    }
}
