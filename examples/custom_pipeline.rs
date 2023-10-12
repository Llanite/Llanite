use llanite::prelude::*;

fn main() {
    let mut controller = Controller::default();
    let mut llanite = Llanite::default();

    controller.add_stage(|state| {
        state
            .pipeline_composer
            .new_pipeline("./shaders/custom.wgsl".into())
            .unwrap();
    });

    llanite.start(Config::default(), controller);
}
