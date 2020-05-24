extern crate env_logger;
extern crate log;
extern crate rodio;
extern crate wiringpi;

mod aubiopitch;
mod jukebox;
mod music_pin;

use log::info;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const FREQ_FILE: &str = "/tmp/bread.txt";
const MP3_FILE: &str = "/tmp/bread.mp3";

fn main() {
    env_logger::init();
    info!("{}", VERSION);

    let time_freqs = aubiopitch::parse_file(FREQ_FILE).expect("parsed file");

    // start playing the mp3
    jukebox::play(MP3_FILE);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TimeFreq {
    pub time: f32,
    pub freq: f32,
}
