use crate::light;
use crate::song::*;
use crate::*;
use log::{error, info};
use midir::MidiOutput;
use std::collections::HashSet;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
/// Simple model for the controls used in a music player
pub trait Controls {
    fn play(&mut self) -> Result<SongStatus, ControlsErr>;
    fn stop(&mut self) -> Result<SongStatus, ControlsErr>;
    fn next(&mut self) -> Result<SongStatus, ControlsErr>;
    fn prev(&mut self) -> Result<SongStatus, ControlsErr>;
}

/// Errors that can occur when controlling a music player
#[derive(Debug)]
pub enum ControlsErr {
    NoFile,
    Playback,
}

pub enum ControlCommands {
    Play,
    Stop,
    Next,
    Prev,
}
pub struct NaiveMidiList {
    pub filenames: Vec<String>,
    pub file_index: u16,
    pub output_device: usize,
}
impl NaiveMidiList {
    fn current_filename(&self) -> Option<String> {
        todo!()
    }
}

impl Controls for NaiveMidiList {
    fn play(&mut self) -> Result<SongStatus, ControlsErr> {
        if let Some(pathstr) = self.current_filename() {
            let (track_events, time_info) = load_midi_file(&pathstr);

            let events = transform_events(track_events);

            // Create a channel for emitting midi events,
            // spawn a thread to handle the LED lights

            let (midi_s, midi_r) = channel::bounded(5);
            std::thread::spawn(move || light::run(midi_r));

            match play_from_beginning(self.output_device, events, time_info, midi_s) {
                Ok(_) => Ok(SongStatus {
                    is_playing: true,
                    name: pathstr,
                    play_time: PlayTime { millis: 0 },
                }),
                Err(err) => {
                    error!("Error: {}", err.to_string());
                    Err(ControlsErr::Playback)
                }
            }
        } else {
            Err(ControlsErr::NoFile)
        }
    }
    fn stop(&mut self) -> Result<SongStatus, ControlsErr> {
        todo!()
    }
    fn next(&mut self) -> Result<SongStatus, ControlsErr> {
        todo!()
    }
    fn prev(&mut self) -> Result<SongStatus, ControlsErr> {
        todo!()
    }
}

/// Process all of the MIDI events and send them to output_device
/// Each note calls `thread::sleep` based on its `vtime` attribute
/// and `delta_timing`
fn play_from_beginning(
    output_device: usize,
    midi_events: Vec<MidiEvent>,
    delta_timing: DeltaTiming,
    midi_sender: channel::Sender<NoteEvent>,
) -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("MIDI Magic Machine")?;

    let port_number = &midi_out.ports()[output_device];
    let mut conn_out = midi_out.connect(port_number, "led_midi_show")?;

    const DEFAULT_MICROS_PER_QNOTE: u64 = 681817;
    let mut micros_per_tick = (DEFAULT_MICROS_PER_QNOTE as f32 / delta_timing.0 as f32) as u64;

    println!("[ [   Show Starts Now   ] ]");
    {
        // Define a new scope in which the closure `sleep_play` borrows conn_out, so it can be called easily
        let mut sleep_play = |midi: &MidiEvent| match midi {
            MidiEvent::Tempo(tempo_change) => {
                let u = (tempo_change.micros_per_qnote as f32 / delta_timing.0 as f32) as u64;
                info!("Update micros per tick: {}", u);
                micros_per_tick = u;
            }
            MidiEvent::Note(note) => {
                sleep(Duration::from_micros(note.vtime * micros_per_tick));

                if let Err(e) = midi_sender.send(note.clone()) {
                    error!("send err {:?}", e)
                }

                let _ = match note.channel_event {
                    ChannelEvent::ChannelOn(c) => conn_out.send(&[c, note.note, note.velocity]),
                    ChannelEvent::ChannelOff(c) => conn_out.send(&[c, note.note, note.velocity]),
                };
            }
            MidiEvent::SustainPedal(p) => info!("Sustain pedal: {:?}", p),
        };

        let mut pedal_state = PedalState::Off;
        let mut sustained = HashSet::new();

        for n in midi_events {
            match (&n, pedal_state) {
                (MidiEvent::SustainPedal(SustainPedalEvent(PedalState::Off)), PedalState::On) => {
                    for s in sustained.drain() {
                        sleep_play(&MidiEvent::Note(s));
                    }
                    pedal_state = PedalState::Off;
                }
                (MidiEvent::SustainPedal(SustainPedalEvent(PedalState::On)), PedalState::Off) => {
                    pedal_state = PedalState::On;
                }
                (
                    MidiEvent::Note(NoteEvent {
                        channel_event: ChannelEvent::ChannelOff(c),
                        time,
                        vtime,
                        note,
                        velocity,
                    }),
                    PedalState::On,
                ) => {
                    sustained.insert(NoteEvent {
                        channel_event: ChannelEvent::ChannelOff(*c),
                        time: *time,
                        vtime: *vtime,
                        note: *note,
                        velocity: *velocity,
                    });
                }
                (n, _p) => sleep_play(&n),
            }
        }
    }

    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    Ok(())
}
