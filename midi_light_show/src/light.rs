// SPDX-License-Identifier: MIT
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
        .merge(config::File::with_name("Settings"))
        .expect("settings");

    let layout_selection = settings.get::<String>("layout").unwrap();
    let pins = &settings
        .get::<Vec<u16>>(&format!("pins.{}", &layout_selection[..])[..])
        .unwrap();
    let num_leds: usize = pins.len();

    let use_pwm = settings.get::<bool>("pwm").unwrap();
    let pwm_message = if use_pwm {
        "Pulse Width Modulation (PWM)"
    } else {
        "Simple On/Off"
    };
    println!("Using {} for LED brightness", pwm_message);

    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let gpio = &wiringpi::setup_gpio();

    let led_pin_outs: Vec<Box<MusicPin>> = {
        let mut lpos: Vec<Box<MusicPin>> = Vec::new();
        for &p in pins {
            lpos.push(MusicPin::new(use_pwm, gpio, p))
        }

        lpos
    };

    // clear everything when you start up
    for po in &led_pin_outs {
        po.write(0);
    }

    // key: index from 0..8 corresponding to the physical order of the LEDs
    // value: MIDI channel
    let mut led_to_cn: HashMap<usize, ChannelNote> = HashMap::new();

    loop {
        let r = &output_r.recv();
        let rc = r.clone();
        let maybe_channel_note = rc.map(|mne| ChannelNote::new(mne.channel_event, mne.note));
        match r {
            &Ok(NoteEvent {
                channel_event: ChannelOff(_c),
                time: _,
                vtime: _,
                note,
                velocity: _,
            })
            | &Ok(NoteEvent {
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
                        led_pin_outs[*led].write(0);
                        unset.push(*led);
                    }
                }
                for u in unset {
                    led_to_cn.remove(&u);
                }
            }
            &Ok(NoteEvent {
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
                        led_pin_outs[led].write(velocity as i32);
                        entry.insert(ChannelNote::new(ChannelOn(c), note));
                    }
                    Occupied(_entry) => (),
                }
            }
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

trait MusicPin {
    fn write(&self, velocity: i32);
}

/// This constructor will generate the appropriate type
/// of pin based on whether you want PWM or not
impl MusicPin {
    fn new(
        pwm: bool,
        gpio: &wiringpi::WiringPi<wiringpi::pin::Gpio>,
        pin_num: u16,
    ) -> Box<MusicPin> {
        if pwm {
            Box::new(SoftPwmMusicPin::new(gpio, pin_num))
        } else {
            Box::new(DigitalMusicPin::new(gpio, pin_num))
        }
    }
}

struct DigitalMusicPin {
    pin: wiringpi::pin::OutputPin<wiringpi::pin::Gpio>,
}

impl DigitalMusicPin {
    fn new(gpio: &wiringpi::WiringPi<wiringpi::pin::Gpio>, pin_num: u16) -> DigitalMusicPin {
        DigitalMusicPin {
            pin: gpio.output_pin(pin_num),
        }
    }
}

impl MusicPin for DigitalMusicPin {
    fn write(&self, velocity: i32) {
        let v = if velocity > 0 {
            wiringpi::pin::Value::High
        } else {
            wiringpi::pin::Value::Low
        };
        self.pin.digital_write(v)
    }
}

struct SoftPwmMusicPin {
    pin: wiringpi::pin::SoftPwmPin<wiringpi::pin::Gpio>,
}

impl SoftPwmMusicPin {
    fn new(gpio: &wiringpi::WiringPi<wiringpi::pin::Gpio>, pin_num: u16) -> SoftPwmMusicPin {
        SoftPwmMusicPin {
            pin: gpio.soft_pwm_pin(pin_num),
        }
    }
}

/// By using the wiringpi interface to software PWM,
/// we can vary the brightness of the LED based on
/// the note velocity.
/// See https://en.wikipedia.org/wiki/Pulse-width_modulation
impl MusicPin for SoftPwmMusicPin {
    fn write(&self, velocity: i32) {
        self.pin.pwm_write(velocity)
    }
}
