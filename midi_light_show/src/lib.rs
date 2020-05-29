// SPDX-License-Identifier: MIT
extern crate crossbeam_channel as channel;
extern crate env_logger;
extern crate log;
extern crate midir;
extern crate rimd;

pub mod controls;
pub mod light;
mod song;

use log::{error, info, warn};
use rimd::{SMFError, TrackEvent, SMF};
use std::path::Path;

pub const DEFAULT_OUTPUT_DEVICE: usize = 1;
pub const DEFAULT_VEC_CAPACITY: usize = 133000;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum ChannelEvent {
    ChannelOn(u8),
    ChannelOff(u8),
}

// midi channel addresses
// from http://www.onicos.com/staff/iz/formats/midi-event.html
pub const CHANNEL_OFF_FIRST: u8 = 0x80;
const CHANNEL_OFF_LAST: u8 = 0x8F;
pub const CHANNEL_ON_FIRST: u8 = 0x90;
const CHANNEL_ON_LAST: u8 = 0x9F;

impl ChannelEvent {
    pub fn new(channel: u8) -> Option<ChannelEvent> {
        if channel >= CHANNEL_OFF_FIRST && channel <= CHANNEL_OFF_LAST {
            Some(ChannelEvent::ChannelOff(channel))
        } else if channel >= CHANNEL_ON_FIRST && channel <= CHANNEL_ON_LAST {
            Some(ChannelEvent::ChannelOn(channel))
        } else {
            None
        }
    }
}

/// These are read from the MIDI file and
/// will be used to produce audio.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct NoteEvent {
    pub channel_event: ChannelEvent,
    pub time: u64,
    pub vtime: u64,
    pub note: u8,
    pub velocity: u8,
}

#[derive(Clone, Copy)]
pub struct TempoEvent {
    pub _time: u64,
    pub _vtime: u64,
    pub micros_per_qnote: u64,
}

#[derive(Clone, Copy)]
pub enum MidiEvent {
    Note(NoteEvent),
    Tempo(TempoEvent),
    SustainPedal(SustainPedalEvent),
}

/// https://cecm.indiana.edu/etext/MIDI/chapter3_MIDI5.shtml
///
/// > In general, controller #'s 0-63 are reserved for continous-type
/// > data, such as volume, mod wheel, etc., controllers 64-121 have
/// > been reserved for switch-type controllers (i.e. on-off, up-down),
/// > such as the sustain pedal. Older conventions of switch values,
/// > ssuch as any data value over 0 = 'ON,' or
/// > recognizing only 0 = 'OFF' and 127 = 'ON' and ignoring the rest,
/// > have been replaced by the convention 0-63 = 'ON' and
/// > 64-127 = 'OFF.'
#[derive(Clone, Copy, Debug)]
pub struct SustainPedalEvent(pub PedalState);
const BREAD_CHANNEL: u8 = 0xb0;
const PEDAL_CONTROLLER: u8 = 0x40;

impl SustainPedalEvent {
    pub fn new(data: &[u8]) -> Option<Self> {
        match data {
            &[BREAD_CHANNEL, PEDAL_CONTROLLER, v] => {
                Some(SustainPedalEvent(if v < PEDAL_CONTROLLER {
                    PedalState::Off
                } else {
                    PedalState::On
                }))
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PedalState {
    On,
    Off,
}

/// An important question to ask:
/// How many ticks are in a Quarter Note?
///
/// The Short answer:  Usually 24
/// The Long answer:   It varies based on num_32nd_notes_per_24_ticks    
/// Also:
/// - http://www.recordingblogs.com/wiki/time-division-of-a-midi-file
/// - https://stackoverflow.com/questions/5288593/how-to-convert-midi-timeline-into-the-actual-timeline-that-should-be-played/5297236#5297236
/// - http://www.deluge.co/?q=midi-tempo-bpm
pub struct MidiTimeInfo {
    pub division: i16,
}

impl MidiTimeInfo {
    pub fn micros_per_clock(self: &Self, micros_per_qnote: u64) -> u64 {
        (micros_per_qnote as f32 / self.division as f32) as u64
    }
}

// The unit of time for delta timing. If the value is positive,
// then it represents the units per beat. For example, +96 would
// mean 96 ticks per beat. If the value is negative, delta times
// are in SMPTE compatible units.
#[derive(Copy, Clone)]
pub struct DeltaTiming(pub i16);

fn load_midi_file(pathstr: &str) -> (Vec<TrackEvent>, DeltaTiming) {
    let mut events: Vec<TrackEvent> = Vec::with_capacity(DEFAULT_VEC_CAPACITY);

    let mut division: i16 = 0;

    match SMF::from_file(&Path::new(&pathstr[..])) {
        Ok(smf) => {
            info!("Division Header: {}", smf.division);
            division = smf.division;
            if division < 0 {
                panic!("We don't know how to deal with negative Division Header values!  Failing.")
            }
            for track in smf.tracks.iter() {
                for event in track.events.iter() {
                    events.push(event.clone());
                }
            }
        }
        Err(e) => match e {
            SMFError::InvalidSMFFile(s) => {
                error!("{}", s);
            }
            SMFError::Error(e) => {
                error!("io: {}", e);
            }
            SMFError::MidiError(e) => {
                error!("Midi Error: {}", e);
            }
            SMFError::MetaError(_) => {
                error!("Meta Error");
            }
        },
    };

    (events, DeltaTiming(division))
}

pub fn transform_events(track_events: Vec<TrackEvent>) -> Vec<MidiEvent> {
    let mut time: u64 = 0;
    let mut events: Vec<MidiEvent> = Vec::with_capacity(DEFAULT_VEC_CAPACITY);
    for te in track_events {
        time += te.vtime;

        match &te {
            TrackEvent {
                vtime,
                event: rimd::Event::Midi(msg),
            } => {
                if let Some(cn) = ChannelEvent::new(msg.data[0]) {
                    let e = NoteEvent {
                        channel_event: cn,
                        time: time,
                        vtime: *vtime,
                        note: msg.data[1],
                        velocity: msg.data[2],
                    };
                    events.push(MidiEvent::Note(e));
                } else if let Some(pedal_event) = SustainPedalEvent::new(&msg.data) {
                    events.push(MidiEvent::SustainPedal(pedal_event));
                } else {
                    // You can find fun and interesting things like Damper Pedal (sustain)
                    // Being turned on and off
                    // See http://www.onicos.com/staff/iz/formats/midi-cntl.html
                    warn!("How about this unknown track event: {:?}", te);
                }
            }
            TrackEvent {
                vtime,
                event:
                    rimd::Event::Meta(rimd::MetaEvent {
                        command: rimd::MetaCommand::TempoSetting,
                        length: _,
                        data,
                    }),
            } => events.push(MidiEvent::Tempo(TempoEvent {
                _time: time,
                _vtime: *vtime,
                micros_per_qnote: data_as_u64(&data),
            })),
            _ => (),
        }
    }

    events
}

// figure out how many microsec are in a quarter note
// available in header TempoSetting = 0x51
// it's a three element array which you want to combine into
// a single u64
/// Turn `bytes` bytes of the data of this event into a u64
fn data_as_u64(data: &Vec<u8>) -> u64 {
    let mut res = 0;
    for i in 0..3 {
        res <<= 8;
        res |= data[i] as u64;
    }
    res
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
