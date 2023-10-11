use config::Config;
use time::macros::format_description;

use tracing_subscriber::fmt::time::LocalTime;
use tracing::Level;

use anyhow::Result;

mod pipeline_composer;

mod booster;
mod config;
mod errors;
mod state;

use booster::Booster;

/// The main struct for the engine.
pub struct Llanite(Booster);


impl Llanite {
    fn new(config: Config) -> Result<Self> {
        let booster = Booster::new(config)?;

        Ok(Self(booster))
    }

    pub fn enable_logger() {
        let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));

        tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .with_thread_names(true)
            .with_timer(timer)
            .init();
    }

    pub fn start(&self) {
        todo!("Start the engine")
    }
}
