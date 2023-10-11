use llanite::{Llanite, Config};
use tracing::Level;

fn main() {
    let mut llanite = Llanite::default();

    // Llanite::enable_logger(Level::WARN);
    Llanite::enable_logger(Level::ERROR);

    llanite.set_pipeline("./shaders/custom.wgsl".into());

    llanite.start(Config::default());
}
