mod booster;
mod config;
mod controller;
mod errors;
mod pipeline_composer;
mod state;

pub mod prelude;

use booster::Booster;
use config::Config;
use config::LogConfig;
use prelude::Controller;
use time::macros::format_description;
use tracing::{error, info, Level};
use tracing_subscriber::fmt::time::LocalTime;

/// The main struct for the engine.
#[derive(Default)]
pub struct Llanite(Booster);

impl Llanite {
    pub fn enable_logger(log_config: LogConfig) {
        let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));

        tracing_subscriber::fmt()
            .with_thread_names(log_config.thread_names)
            .with_line_number(log_config.line_numbers)
            .with_max_level(log_config.level)
            .with_timer(timer)
            .init();

        info!("Started logging with level {:?}", log_config.level);
    }

    pub fn start(&mut self, config: Config, controller: Controller) {
        let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));

        // Make sure there is some sort of logging for errors.
        let _ = tracing_subscriber::fmt()
            .with_line_number(true)
            .with_max_level(Level::WARN)
            .with_thread_names(true)
            .with_timer(timer)
            .try_init();

        if let Err(e) = self.0.launch(config, controller) {
            error!("Error: {e}")
        }
    }
}
