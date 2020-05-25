extern crate env_logger;
extern crate log;
extern crate rodio;
extern crate wiringpi;

mod aubionotes;
mod jukebox;
mod music_pin;

use log::info;
use std::thread;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const FREQ_FILE: &str = "/tmp/bread.txt";
const MP3_FILE: &str = "/tmp/bread.mp3";

fn main() {
    env_logger::init();
    info!("{}", VERSION);

    let note_times = aubionotes::process_audio_file(MP3_FILE).expect("parsed file");

    info!(
        "Examination of the audio file revealed {} notes",
        note_times.len()
    );

    thread::spawn(move || jukebox::blink_lights(note_times));

    // start playing the mp3
    jukebox::play_music(MP3_FILE);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TimeFreq {
    pub time: f32,
    pub freq: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NoteTime {
    pub note: f32,
    pub start_secs: f32,
    pub stop_secs: f32,
}
