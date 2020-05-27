// SPDX-License-Identifier: MIT
extern crate midi_light_show;

use midi_light_show::*;

use crossbeam_channel as channel;
use log::info;
use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
fn main() {
    env_logger::init();
    info!("{}", VERSION);
    let mut args: env::Args = env::args();
    args.next();
    let pathstr = &match args.next() {
        Some(s) => s,
        None => panic!("Please pass a path to an SMF to test"),
    }[..];

    const DEFAULT_OUTPUT_DEVICE: usize = 1;
    let output_device: &usize = &match args.next() {
        Some(n) => {
            println!("User requested output device {}", n);
            str::parse(&n)
        }
        .unwrap_or(0),
        None => {
            println!(
                "No output device specified, defaulting to {}",
                DEFAULT_OUTPUT_DEVICE
            );
            DEFAULT_OUTPUT_DEVICE
        }
    };

    let (track_events, time_info) = load_midi_file(pathstr);

    let events = transform_events(track_events);

    // Create a channel for emitting midi events,
    // spawn a thread to handle the LED lights

    let (midi_s, midi_r) = channel::bounded(5);
    std::thread::spawn(move || light::run(midi_r));

    match play_from_beginning(*output_device, events, time_info, midi_s) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.to_string()),
    }
}
