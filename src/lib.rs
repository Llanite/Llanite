pub use config::Config;
use time::macros::format_description;

use tracing::{Level, error};
use tracing_subscriber::fmt::time::LocalTime;

mod pipeline_composer;

mod booster;
mod config;
mod errors;
mod state;

use booster::Booster;

/// The main struct for the engine.
#[derive(Default)]
pub struct Llanite(Booster);

impl Llanite {
    pub fn enable_logger(level: Level) {
        let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));

        tracing_subscriber::fmt()
            .with_max_level(level)
            .with_thread_names(true)
            .with_timer(timer)
            .init();
    }

    pub fn start(&mut self, config: Config) {
        let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));

        // Make sure there is some sort of logging for errors.
        let _ = tracing_subscriber::fmt()
            .with_max_level(Level::ERROR)
            .with_thread_names(true)
            .with_timer(timer)
            .try_init();

        if let Err(e) = self.0.launch(config) {
            error!("Error: {e}")
        }
    }
}
