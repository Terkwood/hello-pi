//MIT License

//Copyright (c) 2018 Terkwood

//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

extern crate crossbeam_channel as channel;
extern crate wiringpi;

use std::collections::HashMap;
use ChannelEvent::{ChannelOff, ChannelOn};
use MidiNoteEvent;

static PINS: &'static [u16; 8] = &[13, 6, 5, 7, 23, 18, 15, 14];

pub fn run(output_r: channel::Receiver<MidiNoteEvent>) {
    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let pi = wiringpi::setup_gpio();

    //let led_pin_outs: &Vec<wiringpi::pin::OutputPin<wiringpi::pin::Pin>> = &PINS.iter().map(|p| pi.output_pin(*p)).collect();
    let led_pin_outs = {
        let mut lpos = Vec::new();

        let mut c = 0;
        // track some pins
        for &p in PINS {
            lpos.push(pi.output_pin(p));
        }

        lpos
    };
    // key: index from 0..8 corresponding to the physical order of the LEDs
    // value: MIDI channel
    let mut led_to_midi_channel: HashMap<usize, u8> = HashMap::new();

    loop {
        match output_r.recv() {
            Some(MidiNoteEvent {
                channel_event: ChannelOff(c),
                time: _,
                vtime: _,
                note: _,
                velocity: _,
            })
            | Some(MidiNoteEvent {
                channel_event: ChannelOn(c),
                time: _,
                vtime: _,
                note: _,
                velocity: 0,
            }) => {
                let mut unset: Vec<usize> = vec![];
                for (led, lchan) in &led_to_midi_channel {
                    if c == *lchan {
                        // turn off the LED
                        led_pin_outs[*led].digital_write(wiringpi::pin::Value::Low);
                        unset.push(*led);
                    }
                }
                for u in unset {
                    led_to_midi_channel.remove(&u);
                }
            }
            Some(MidiNoteEvent {
                channel_event: ChannelOn(c),
                time: _,
                vtime: _,
                note,
                velocity: _,
            }) => {
                let led = midi_note_to_led(note);
                // only mess with this note if it's
                // not being used by another channel

                /*if !led_to_midi_channel.contains_key(&led) {
                    led_to_midi_channel[&led] = c;
                }*/
            }
            None => {}
        }
    }
}

fn midi_note_to_led(c: u8) -> usize {
    unimplemented!()
}
