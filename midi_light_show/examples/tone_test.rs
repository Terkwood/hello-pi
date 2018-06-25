extern crate midir;
extern crate rimd;

use midir::MidiOutput;

use std::env::args;
use std::io::stdin;
use std::thread;
use std::time::Duration;

pub fn main() {
    let notes: Vec<u8> = args().skip(1).map(|n| str::parse(&n).unwrap()).collect();

    let midi_out = MidiOutput::new("Tone Test").unwrap();

    // Get an output port (read from console if multiple are available)
    let output_device = match midi_out.port_count() {
        0 => panic!("no output port found"),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(0).unwrap()
            );
            0
        }
        _ => {
            println!("\nAvailable output ports:");
            for i in 0..midi_out.port_count() {
                println!("{}: {}", i, midi_out.port_name(i).unwrap());
            }
            println!("Please select output port: ");

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input.trim().parse().unwrap()
        }
    };

    let mut conn_out = midi_out.connect(output_device, "tone_test").unwrap();

    const NOTE_ON_CHANNEL: u8 = 144;
    const VELOCITY: u8 = 80;

    for n in notes {
        conn_out.send(&[NOTE_ON_CHANNEL, n, VELOCITY]).unwrap();

        thread::sleep(Duration::from_secs(1));
    }
}
