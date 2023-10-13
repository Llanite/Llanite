use llanite::prelude::*;

fn main() {
    Llanite::enable_logger(LogConfig {
        level: Level::INFO,
        ..Default::default()
    });

    let mut controller = Controller::default();
    let mut llanite = Llanite::default();

    controller.add_stage(|state| new_pipeline!(state, "./shaders/custom.wgsl"));

    llanite.start(Config::default(), controller);
}
