mod config;

use config::Config;

#[derive(Default)]
pub struct Llanite {}

impl Llanite {
    pub fn start(&self, config: Config) {
        todo!("Start engine!")
    }
}
