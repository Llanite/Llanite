use time::macros::format_description;
use tracing::info;
use tracing_subscriber::fmt::time::LocalTime;

use crate::LogConfig;

pub fn enable_logging(log_config: Option<LogConfig>) {
    let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));

    let log_config = match log_config {
        None => LogConfig::default(),
        Some(v) => v,
    };

    tracing_subscriber::fmt()
        .with_thread_names(log_config.thread_names)
        .with_line_number(log_config.line_numbers)
        .with_max_level(log_config.level)
        .with_timer(timer)
        .init();

    info!("Logging started with config: {log_config:#?}\n")
}
