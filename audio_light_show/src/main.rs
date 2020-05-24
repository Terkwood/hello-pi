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

const A4_TUNING_HZ: f32 = 440.0;
const A4_KEY_POSITION: u8 = 49;
/// See https://en.m.wikipedia.org/wiki/Piano_key_frequencies
fn freq_to_note(freq: f32) -> u8 {
    (39.86 * (freq / A4_TUNING_HZ).log10()) as u8 + A4_KEY_POSITION
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_freq_to_note() {
        assert_eq!(A4_KEY_POSITION, freq_to_note(A4_TUNING_HZ));
    }
}
