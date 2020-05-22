// SPDX-License-Identifier: MIT
extern crate crossbeam_channel as channel;
extern crate midir;
extern crate rimd;

use midir::MidiOutput;
use rimd::{SMFError, TrackEvent, SMF};
use std::env;
use std::error::Error;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

mod light;

const DEFAULT_VEC_CAPACITY: usize = 133000;

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct NoteEvent {
    channel_event: ChannelEvent,
    time: u64,
    vtime: u64,
    note: u8,
    velocity: u8,
}

#[derive(Clone)]
pub struct TempoEvent {
    time: u64,
    vtime: u64,
    micros_per_qnote: u64,
}

#[derive(Clone)]
pub enum MidiEvent {
    Note(NoteEvent),
    Tempo(TempoEvent),
}

fn main() {
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

    match run(*output_device, events, time_info, midi_s) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description()),
    }
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

fn load_midi_file(pathstr: &str) -> (Vec<TrackEvent>, i16) {
    let mut events: Vec<TrackEvent> = Vec::with_capacity(DEFAULT_VEC_CAPACITY);

    let mut division: i16 = 0;

    match SMF::from_file(&Path::new(&pathstr[..])) {
        Ok(smf) => {
            /// The unit of time for delta timing. If the value is positive,
            /// then it represents the units per beat. For example, +96 would
            /// mean 96 ticks per beat. If the value is negative, delta times
            /// are in SMPTE compatible units.
            println!("Division Header: {}", smf.division);
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
                println!("{}", s);
            }
            SMFError::Error(e) => {
                println!("io: {}", e);
            }
            SMFError::MidiError(e) => {
                println!("Midi Error: {}", e);
            }
            SMFError::MetaError(_) => {
                println!("Meta Error");
            }
        },
    };

    (events, division)
}

fn transform_events(track_events: Vec<TrackEvent>) -> Vec<MidiEvent> {
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
                } else {
                    // You can find fun and interesting things like Damper Pedal (sustain)
                    // Being turned on and off
                    // See http://www.onicos.com/staff/iz/formats/midi-cntl.html
                    println!("How about this unknown track event: {:?}", te);
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
                time: time,
                vtime: *vtime,
                micros_per_qnote: data_as_u64(&data),
            })),
            _ => (),
        }
    }

    events
}

fn run(
    output_device: usize,
    notes: Vec<MidiEvent>,
    division: i16,
    midi_sender: channel::Sender<NoteEvent>,
) -> Result<(), Box<Error>> {
    let midi_out = MidiOutput::new("MIDI Magic Machine")?;

    let mut conn_out = midi_out.connect(output_device, "led_midi_show")?;

    const DEFAULT_MICROS_PER_QNOTE: u64 = 681817;
    let mut micros_per_tick = (DEFAULT_MICROS_PER_QNOTE as f32 / division as f32) as u64;

    println!("[ [   Show Starts Now   ] ]");
    {
        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = |midi: MidiEvent| match midi {
            MidiEvent::Tempo(tempo_change) => {
                let u = (tempo_change.micros_per_qnote as f32 / division as f32) as u64;
                println!("Update micros per tick: {}", u);
                micros_per_tick = u;
            }
            MidiEvent::Note(note) => {
                sleep(Duration::from_micros(note.vtime * micros_per_tick));

                midi_sender.send(note.clone());

                let _ = match note.channel_event {
                    ChannelEvent::ChannelOn(c) => conn_out.send(&[c, note.note, note.velocity]),
                    ChannelEvent::ChannelOff(c) => conn_out.send(&[c, note.note, note.velocity]),
                };
            }
        };

        for n in notes {
            play_note(n)
        }
    }

    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    Ok(())
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
