use llanite::{Llanite, Config};

fn main() {
    let mut llanite = Llanite::default();

    llanite.start(Config::default());
}
