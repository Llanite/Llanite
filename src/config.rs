use std::fmt::Display;
use tracing::Level;

pub struct Config {
    pub title: &'static str,
    pub height: i32,
    pub width: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "Hello Llanite",
            width: 640,
            height: 480,
        }
    }
}

#[derive(Debug)]
pub struct LogConfig {
    pub thread_names: bool,
    pub line_numbers: bool,
    pub level: Level,
}

impl Display for LogConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;

        writeln!(f, "    ├─ Display thread names : {}", self.thread_names)?;
        writeln!(f, "    ├─ Display line numbers : {}", self.line_numbers)?;
        writeln!(f, "    └─ Log filter level     : {}", self.level)?;

        Ok(())
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            thread_names: true,
            line_numbers: true,
            level: Level::WARN,
        }
    }
}
