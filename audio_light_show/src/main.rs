extern crate env_logger;
extern crate log;
extern crate wiringpi;

mod music_pin;

use log::info;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
fn main() {
    env_logger::init();
    info!("{}", VERSION);
}
