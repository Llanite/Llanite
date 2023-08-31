use llanite::Config;
use llanite::Llanite;

fn main() {
    let llanite = Llanite::default();

    llanite.start(Config {
        window_name: String::from("Start example"),
        height: 480,
        width: 640,
    });
}
