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

pub struct LogConfig {
    pub thread_names: bool,
    pub line_numbers: bool,
    pub level: Level,
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
