use llanite::Llanite;
use llanite::Config;

fn main() {
    let llanite = Llanite::default();

    let config = Config {
        window_name: String::from("Start example"),
        height: 480,
        width: 640,
    };

    llanite.start(config);
}
