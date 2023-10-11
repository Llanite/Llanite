use tracing_subscriber::fmt::time::LocalTime;
use time::macros::format_description;
use tracing::Level;

mod config;
mod booster;

/// The main struct for the engine.
#[derive(Default)]
pub struct Llanite();

impl Llanite {
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
