extern crate env_logger;
extern crate log;
extern crate wiringpi;

mod freq_file;
mod music_pin;

use log::info;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const FILENAME: &str = "/tmp/bread.txt";

fn main() {
    env_logger::init();
    info!("{}", VERSION);

    let time_freqs = freq_file::load(FILENAME).expect("load");
    for tf in time_freqs {
        log::info!("{:?}", tf);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TimeFreq {
    pub time: f32,
    pub freq: f32,
}
