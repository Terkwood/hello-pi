// SPDX-License-Identifier: MIT
extern crate config;
extern crate crossbeam_channel as channel;
extern crate wiringpi;

use crate::controls::ControlCommands;
use crate::ChannelEvent;
use crate::ChannelEvent::{ChannelOff, ChannelOn};
use crate::ModuloSignedExt;
use crate::NoteEvent;
use crate::CHANNEL_OFF_FIRST;
use crate::CHANNEL_ON_FIRST;
use log::{error, info, warn};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

pub fn start(
    notes_out: channel::Receiver<NoteEvent>,
    commands_out: channel::Receiver<crate::controls::ControlCommands>,
) {
    let mut is_playing = false;
    let (abort_in, abort_out) = channel::bounded(1);
    loop {
        channel::select! {
            recv(commands_out) -> msg => match (msg, is_playing) {
                (Ok(ControlCommands::Play(_)), false) => {
                    let nn = notes_out.clone();
                    let aa = abort_out.clone();
                    std::thread::spawn(move || {
                        begin_show(nn,aa)
                    });
                    is_playing = true;
                },
                (Ok(ControlCommands::Play(_)), _) => warn!("Already playing"),
                (Ok(ControlCommands::Stop), _) =>
                    if let Err(_) = abort_in.send(Abort) {
                        error!("failed to send abort")
                    } else {
                        is_playing = false;
                    },
                (Ok(ControlCommands::Next(_)), _) |
                 (Ok(ControlCommands::Prev(_)), _) => {
                    if let Err(_) = abort_in.send(Abort) {
                        error!("failed to send abort")
                    } else {
                        is_playing = true;
                        let nn = notes_out.clone();
                        let aa = abort_out.clone();
                        std::thread::spawn(move || {
                            begin_show(nn,aa)
                        });
                    }
                },
                (Err(_), _) => panic!("LIGHT")
            }
        }
    }
}

struct Abort;
fn begin_show(notes_out: channel::Receiver<NoteEvent>, abort_out: channel::Receiver<Abort>) {
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

    let mut led_pin_outs: Vec<Box<dyn MusicPin>> = {
        let mut lpos: Vec<Box<dyn MusicPin>> = Vec::new();
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
        channel::select! {
            recv(notes_out) -> r =>
                process_note(r, &mut led_to_cn,
                    &mut led_pin_outs, num_leds),
            recv(abort_out) -> c =>
                match c {
                    Ok(Abort) => {
                        info!("Light show received STOP");
                        return;
                    },
                    Err(e) => error!("{:?}",e)
                }
        }
    }
}

fn process_note(
    r: Result<NoteEvent, channel::RecvError>,
    led_to_cn: &mut HashMap<usize, ChannelNote>,
    led_pin_outs: &mut Vec<Box<dyn MusicPin>>,
    num_leds: usize,
) {
    let rc = r.clone();
    let maybe_channel_note = rc.map(|mne| ChannelNote::new(mne.channel_event, mne.note));
    match r {
        Ok(NoteEvent {
            channel_event: ChannelOff(_c),
            time: _,
            vtime: _,
            note,
            velocity: _,
        })
        | Ok(NoteEvent {
            channel_event: ChannelOn(_c),
            time: _,
            vtime: _,
            note,
            velocity: 0,
        }) => {
            let midi_chan = maybe_channel_note.unwrap().channel;
            let mut unset: Vec<usize> = vec![];

            for (led, lcn) in led_to_cn.iter() {
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
        Ok(NoteEvent {
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
        Err(e) => error!("err {}", e),
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

trait MusicPin {
    fn write(&self, velocity: i32);
}

/// This constructor will generate the appropriate type
/// of pin based on whether you want PWM or not
impl dyn MusicPin {
    fn new(
        pwm: bool,
        gpio: &wiringpi::WiringPi<wiringpi::pin::Gpio>,
        pin_num: u16,
    ) -> Box<dyn MusicPin> {
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
