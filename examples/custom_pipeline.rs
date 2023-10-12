use llanite::{Llanite, Config};
use tracing::Level;

fn main() {
    let mut llanite = Llanite::default();

    llanite.set_pipeline("./shaders/custom.wgsl".into());

    llanite.start(Config::default());
}
