use std::path::PathBuf;

use config::LogConfig;
use time::macros::format_description;

use tracing::{Level, error, info};
use tracing_subscriber::fmt::time::LocalTime;

mod pipeline_composer;

mod controller;
mod booster;
mod config;
mod errors;
mod state;

pub mod prelude {
    pub use crate::controller::Controller;
    pub use crate::Llanite;

    pub use crate::config::{LogConfig, Config};
    pub use tracing;
}

use booster::Booster;
use config::Config;

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

    pub fn start(&mut self, config: Config) {
        let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));

        // Make sure there is some sort of logging for errors.
        let _ = tracing_subscriber::fmt()
            .with_line_number(true)
            .with_max_level(Level::WARN)
            .with_thread_names(true)
            .with_timer(timer)
            .try_init();

        if let Err(e) = self.0.launch(config) {
            error!("Error: {e}")
        }
    }

    pub fn set_pipeline(&mut self, shader_path: PathBuf) {
        if let Some(state) = &self.0.state {
            state.lock().unwrap().pipeline_composer.new_pipeline(shader_path).unwrap();
        }
    }
}
