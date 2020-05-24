extern crate midir;
extern crate rimd;

use std::env::args;
use std::thread;
use std::time::Duration;

pub fn main() {
    let notes: Vec<u8> = args().skip(1).map(|n| str::parse(&n).unwrap()).collect();

    let client = midir::MidiOutput::new("test example test").expect("c");
    // Get an output port (read from console if multiple are available)
    let output_port = &client.ports()[0];

    let mut conn_out = client.connect(&output_port, "tone_test").unwrap();

    const NOTE_ON_CHANNEL: u8 = 144;
    const VELOCITY: u8 = 80;

    for n in notes {
        conn_out.send(&[NOTE_ON_CHANNEL, n, VELOCITY]).unwrap();

        thread::sleep(Duration::from_secs(1));
    }
}
