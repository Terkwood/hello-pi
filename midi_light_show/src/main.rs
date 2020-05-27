// SPDX-License-Identifier: MIT
extern crate midi_light_show;

use midi_light_show::controls::*;

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
    let output_device: usize = match args.next() {
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

    let mut controls = NaiveMidiList {
        file_index: 0,
        filenames: vec![pathstr.to_string()],
        output_device,
    };

    let status = controls.play().expect("play");
    info!("Playing {}", status.name)
}
