extern crate env_logger;
extern crate log;
extern crate rodio;
extern crate wiringpi;

mod aubionotes;
mod jukebox;
mod music_pin;

use log::{error, info};
use std::{env, fs, thread};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();
    info!("{}", VERSION);

    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        error!("Please specify an MP3 or WAV file as an argument");
        std::process::exit(1);
    }

    let filename = &args[1];

    if fs::metadata(filename).is_err() {
        error!("Please specify a file that exists and is readable by your user");
        std::process::exit(2);
    }

    let note_times = aubionotes::process_audio_file(&filename).expect("parsed file");

    info!(
        "Examination of the audio file revealed {} notes",
        note_times.len()
    );

    thread::spawn(move || jukebox::blink_lights(note_times));

    // start playing the mp3
    jukebox::play_music(&filename);
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
