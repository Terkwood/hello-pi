// SPDX-License-Identifier: MIT
extern crate midi_light_show;

use midi_light_show::controls::*;
use midi_light_show::DEFAULT_OUTPUT_DEVICE;

use log::info;
use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
fn main() {
    env_logger::init();
    info!("{}", VERSION);
    let mut args: env::Args = env::args();
    args.next();
    let mut pathstrs = vec![];
    while let Some(p) = args.next() {
        pathstrs.push(p);
    }

    let (mut controls, handles) = MsgPassingControls::create(pathstrs, DEFAULT_OUTPUT_DEVICE);

    let status = controls.play().expect("play");
    info!("Playing {}", status.name);
    for h in handles {
        h.join().expect("join")
    }
}
