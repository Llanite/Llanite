use llanite::prelude::*;
use tracing::Level;

fn main() {
    Llanite::enable_logger(LogConfig {
        level: Level::INFO,
        ..Default::default()
    });

    let mut controller = Controller::default();
    let mut llanite = Llanite::default();

    controller.add_stage(|state| {
        state
            .pipeline_composer
            .new_pipeline(Some("./shaders/custom.wgsl"))
            .unwrap();
    });

    llanite.start(Config::default(), controller);
}
