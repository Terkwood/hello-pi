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

extern crate config;
extern crate crossbeam_channel as channel;
extern crate wiringpi;

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use ChannelEvent;
use ChannelEvent::{ChannelOff, ChannelOn};
use NoteEvent;
use CHANNEL_OFF_FIRST;
use CHANNEL_ON_FIRST;

pub fn run(output_r: channel::Receiver<NoteEvent>) {
    let mut settings = config::Config::default();
    settings
            // Add in `./Settings.toml`
            .merge(config::File::with_name("Settings")).unwrap();

    let layout_selection = settings.get::<String>("layout").unwrap();
    let pins = &settings
        .get::<Vec<u16>>(&format!("pins.{}", &layout_selection[..])[..])
        .unwrap();
    let num_leds: usize = pins.len();

    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let gpio = wiringpi::setup_gpio();

    let led_pin_outs = {
        let mut lpos = Vec::new();
        // setup software-driven pulse width modulation pins
        for &p in pins {
            lpos.push(gpio.soft_pwm_pin(p));
        }

        lpos
    };

    // clear everything when you start up
    for po in &led_pin_outs {
        po.pwm_write(0);
    }

    // key: index from 0..8 corresponding to the physical order of the LEDs
    // value: MIDI channel
    let mut led_to_cn: HashMap<usize, ChannelNote> = HashMap::new();

    loop {
        let r = &output_r.recv();
        let rc = r.clone();
        let maybe_channel_note = rc.map(|mne| ChannelNote::new(mne.channel_event, mne.note));
        match r {
            &Some(NoteEvent {
                channel_event: ChannelOff(_c),
                time: _,
                vtime: _,
                note,
                velocity: _,
            })
            | &Some(NoteEvent {
                channel_event: ChannelOn(_c),
                time: _,
                vtime: _,
                note,
                velocity: 0,
            }) => {
                let midi_chan = maybe_channel_note.unwrap().channel;
                let mut unset: Vec<usize> = vec![];

                for (led, lcn) in &led_to_cn {
                    if note == lcn.note && midi_chan == lcn.channel {
                        // turn off the LED
                        led_pin_outs[*led].pwm_write(0);
                        unset.push(*led);
                    }
                }
                for u in unset {
                    led_to_cn.remove(&u);
                }
            }
            &Some(NoteEvent {
                channel_event: ChannelOn(c),
                time: _,
                vtime: _,
                note,
                velocity,
            }) => {
                let led = midi_note_to_led(note, num_leds);

                // only mess with this note if it's
                // not being used by another channel
                match led_to_cn.entry(led) {
                    Vacant(entry) => {
                        // PWM duty cycle ranges from 0 to 100
                        led_pin_outs[led].pwm_write(velocity as i32);
                        entry.insert(ChannelNote::new(ChannelOn(c), note));
                    }
                    Occupied(_entry) => (),
                }
            }
            None => {}
        }
    }
}

struct ChannelNote {
    channel: u8,
    note: u8,
}

impl ChannelNote {
    fn new(channel_event: ChannelEvent, note: u8) -> ChannelNote {
        match channel_event {
            ChannelOn(c) => ChannelNote {
                channel: c - CHANNEL_ON_FIRST,
                note,
            },
            ChannelOff(c) => ChannelNote {
                channel: c - CHANNEL_OFF_FIRST,
                note,
            },
        }
    }
}

fn midi_note_to_led(c: u8, num_leds: usize) -> usize {
    ((60 - c as i8).modulo(num_leds as i8)) as usize
}

///
/// Modulo that handles negative numbers, works the same as Python's `%`.
///
/// eg: `(a + b).modulo(c)`
/// from https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
pub trait ModuloSignedExt {
    fn modulo(&self, n: Self) -> Self;
}
macro_rules! modulo_signed_ext_impl {
    ($($t:ty)*) => ($(
        impl ModuloSignedExt for $t {
            #[inline]
            fn modulo(&self, n: Self) -> Self {
                (self % n + n) % n
            }
        }
    )*)
}
modulo_signed_ext_impl! { i8 i16 i32 i64 }
