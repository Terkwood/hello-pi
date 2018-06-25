extern crate midir;
extern crate rimd;

use midir::MidiOutput;
use rimd::{Event, SMFError, TrackEvent, SMF};
use std::env;
use std::error::Error;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

mod channels;

const DEFAULT_VEC_CAPACITY: usize = 133000;

#[derive(Debug)]
pub enum ChannelNote {
    On(u8),
    Off(u8),
}

impl ChannelNote {
    pub fn new(channel: u8) -> Option<ChannelNote> {
        if channel >= channels::CHANNEL_OFF_FIRST && channel <= channels::CHANNEL_OFF_LAST {
            Some(ChannelNote::Off(channel))
        } else if channel >= channels::CHANNEL_ON_FIRST && channel <= channels::CHANNEL_ON_LAST {
            Some(ChannelNote::On(channel))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct MidiNote {
    channel_note: ChannelNote,
    time: u64,
    vtime: u64,
    note: u8,
    velocity: u8,
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
        }.unwrap_or(0),
        None => {
            println!(
                "No output device specified, defaulting to {}",
                DEFAULT_OUTPUT_DEVICE
            );
            DEFAULT_OUTPUT_DEVICE
        }
    };

    let (track_events, time_info) = load_midi_file(pathstr);

    let timed_midi_messages = midi_messages_from(track_events);

    let notes = notes_in_channel(timed_midi_messages);

    match run(*output_device, notes, time_info.micros_per_clock()) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description()),
    }
}

/// An important question to ask:
/// How many ticks are in a Quarter Note?
///
/// The Short answer:  Usually 24
/// The Long answer:   It varies based on num_32nd_notes_per_24_ticks    
pub struct MidiTimeInfo {
    pub micros_per_qnote: u64,
    pub num_32nd_notes_per_24_ticks: u8, // usually 8
    pub clocks_per_tick: u8,             // usually 24
}

impl MidiTimeInfo {
    /// See documentation in rimd.rs:
    /// > The parameter `clocks_per_tick` is the number of MIDI Clocks per metronome tick.
    /// > Normally, there are 24 MIDI Clocks per quarter note.
    /// > However, some software allows this to be set by the user.
    /// > The parameter `num_32nd_notes_per_24_clocks` defines this in terms of the
    /// > number of 1/32 notes which make up the usual 24 MIDI Clocks
    /// > (the 'standard' quarter note).  8 is standard

    pub fn micros_per_clock(self: &Self) -> u64 {
        // SO, THIS IS A ROUGH ESTIMATE
        // ...and if `num_32nd_notes_per_24_ticks` is set in your MIDI file,
        // ...you should do more arithmetic.
        (self.micros_per_qnote as f32 / 32 as f32 / 12 as f32) as u64
    }
}

fn load_midi_file(pathstr: &str) -> (Vec<TrackEvent>, MidiTimeInfo) {
    let mut events: Vec<TrackEvent> = Vec::with_capacity(DEFAULT_VEC_CAPACITY);

    let mut micros_per_qnote: Option<u64> = None;
    let mut num_32nd_notes_per_24_clocks: u8 = 8;
    let mut clocks_per_tick: u8 = 24;

    match SMF::from_file(&Path::new(&pathstr[..])) {
        Ok(smf) => {
            for track in smf.tracks.iter() {
                for event in track.events.iter() {
                    if let rimd::Event::Meta(rimd::MetaEvent {
                        command: rimd::MetaCommand::TempoSetting,
                        length: _,
                        data: ref micros_per_qnote_vec,
                    }) = event.event
                    {
                        // so meta
                        micros_per_qnote = Some(data_as_u64(micros_per_qnote_vec))
                    }

                    if let rimd::Event::Meta(rimd::MetaEvent {
                        command: rimd::MetaCommand::TimeSignature,
                        length: _,
                        ref data,
                    }) = event.event
                    {
                        clocks_per_tick = data[2];
                        num_32nd_notes_per_24_clocks = data[3];
                    }

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

    const DEFAULT_MICROS_PER_QNOTE: u64 = 681817;
    (
        events,
        MidiTimeInfo {
            micros_per_qnote: micros_per_qnote.unwrap_or(DEFAULT_MICROS_PER_QNOTE),
            clocks_per_tick: clocks_per_tick,
            num_32nd_notes_per_24_ticks: num_32nd_notes_per_24_clocks,
        },
    )
}

#[derive(Debug)]
pub struct TimedMidiMessage {
    pub vtime: u64,
    pub data: Vec<u8>,
}

fn midi_messages_from(track_events: Vec<TrackEvent>) -> Vec<TimedMidiMessage> {
    let mut midi_messages: Vec<TimedMidiMessage> = Vec::with_capacity(DEFAULT_VEC_CAPACITY);

    for te in track_events {
        match te {
            TrackEvent { vtime, event } => match event {
                Event::Midi(m) => midi_messages.push(TimedMidiMessage {
                    vtime,
                    data: m.data,
                }),
                Event::Meta(_m) => {}
            },
        }
    }

    midi_messages
}

fn notes_in_channel(midi_messages: Vec<TimedMidiMessage>) -> Vec<MidiNote> {
    let mut time: u64 = 0;
    let mut notes: Vec<MidiNote> = Vec::with_capacity(DEFAULT_VEC_CAPACITY);
    for msg in midi_messages {
        time += msg.vtime;
        if let Some(cn) = ChannelNote::new(msg.data[0]) {
            notes.push(MidiNote {
                channel_note: cn,
                time: time,
                vtime: msg.vtime,
                note: msg.data[1],
                velocity: msg.data[2],
            })
        }
    }

    notes
}

fn run(output_device: usize, notes: Vec<MidiNote>, micros_per_tick: u64) -> Result<(), Box<Error>> {
    let midi_out = MidiOutput::new("MIDI Magic Machine")?;

    let mut conn_out = midi_out.connect(output_device, "led_midi_show")?;
    println!("[ [ Show Starts Now ] ]");
    {
        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = |midi: MidiNote| {
            sleep(Duration::from_micros(midi.vtime * micros_per_tick));

            let _ = match midi.channel_note {
                ChannelNote::On(c) => conn_out.send(&[c, midi.note, midi.velocity]),
                ChannelNote::Off(c) => conn_out.send(&[c, midi.note, midi.velocity]),
            };
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
