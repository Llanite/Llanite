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
