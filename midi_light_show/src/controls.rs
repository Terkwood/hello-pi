// SPDX-License-Identifier: MIT
use crate::light;
use crate::song::*;
use crate::*;
use crossbeam_channel::{bounded, select, Receiver, Sender};
use log::{error, info};
use midir::MidiOutput;
use std::collections::HashSet;
use std::error::Error;
use std::thread::{sleep, spawn};
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
    MsgPassing,
}

#[derive(Debug, Clone)]
pub enum ControlCommands {
    Play(String),
    Stop,
    Next(String),
    Prev(String),
}
pub struct MsgPassingControls {
    pub filenames: Vec<String>,
    pub file_index: usize,
    pub output_device: usize,
    pub commands_in: Sender<ControlCommands>,
    pub is_playing: bool,
}

pub struct ControlStartup(pub MsgPassingControls, pub Vec<std::thread::JoinHandle<()>>);
const BOUNDED_SIZE: usize = 5;
impl MsgPassingControls {
    /// START ALL THREADS NEEDED TO CONTROL THE MAGIC!
    pub fn create(
        filenames: Vec<String>,
        output_device: usize,
    ) -> (Self, Vec<std::thread::JoinHandle<()>>) {
        let (commands_in, commands_out) = crossbeam_channel::unbounded();

        // Create a channel for emitting midi events,
        // spawn a thread to handle the LED lights
        let (midi_s, midi_r) = bounded(BOUNDED_SIZE);

        let (light_commands_in, light_commands_out) = bounded(BOUNDED_SIZE);
        let (midi_commands_in, midi_commands_out) = bounded(BOUNDED_SIZE);

        let spammer =
            spawn(move || broadcast(commands_out, vec![midi_commands_in, light_commands_in]));
        let blinken = spawn(move || light::start(midi_r, light_commands_out));
        let music = spawn(move || midi_play_loop(output_device, midi_s, midi_commands_out));

        let handles = vec![spammer, blinken, music];
        (
            MsgPassingControls {
                filenames,
                file_index: 0,
                output_device,
                commands_in,
                is_playing: false,
            },
            handles,
        )
    }

    fn current_filename(&self) -> Option<String> {
        self.filenames.get(self.file_index).map(|s| s.to_string())
    }

    fn prev_file_index(&self) -> usize {
        (self.file_index + self.filenames.len() - 1) % self.filenames.len()
    }
}

impl Controls for MsgPassingControls {
    fn play(&mut self) -> Result<SongStatus, ControlsErr> {
        if let Some(pathstr) = self.current_filename() {
            if let Err(_) = self
                .commands_in
                .send(ControlCommands::Play(pathstr.clone()))
            {
                Err(ControlsErr::MsgPassing)
            } else {
                self.is_playing = true;
                Ok(SongStatus {
                    is_playing: true,
                    name: pathstr.to_string(),
                    play_time: PlayTime { millis: 0 },
                })
            }
        } else {
            Err(ControlsErr::NoFile)
        }
    }
    fn stop(&mut self) -> Result<SongStatus, ControlsErr> {
        if let Err(_) = self.commands_in.send(ControlCommands::Stop) {
            Err(ControlsErr::MsgPassing)
        } else {
            self.is_playing = false;
            Ok(SongStatus {
                is_playing: false,
                name: self.current_filename().unwrap_or_default(),
                play_time: PlayTime { millis: 0 },
            })
        }
    }
    fn next(&mut self) -> Result<SongStatus, ControlsErr> {
        self.file_index = (self.file_index + 1) % self.filenames.len();

        if let Some(pathstr) = self.current_filename() {
            if let Err(_) = self.commands_in.send(ControlCommands::Next(pathstr)) {
                Err(ControlsErr::MsgPassing)
            } else {
                self.is_playing = true;
                Ok(SongStatus {
                    is_playing: true,
                    name: self.current_filename().unwrap_or_default(),
                    play_time: PlayTime { millis: 0 },
                })
            }
        } else {
            Err(ControlsErr::NoFile)
        }
    }
    fn prev(&mut self) -> Result<SongStatus, ControlsErr> {
        self.file_index = self.prev_file_index();
        if let Some(pathstr) = self.current_filename() {
            if let Err(_) = self.commands_in.send(ControlCommands::Prev(pathstr)) {
                Err(ControlsErr::MsgPassing)
            } else {
                self.is_playing = true;
                Ok(SongStatus {
                    is_playing: true,
                    name: self.current_filename().unwrap_or_default(),
                    play_time: PlayTime { millis: 0 },
                })
            }
        } else {
            Err(ControlsErr::NoFile)
        }
    }
}

fn pppplay(
    output_device: usize,
    pathstr: &str,
    notes_in: channel::Sender<NoteEvent>,
    abort_out: channel::Receiver<Abort>,
) {
    let (track_events, time_info) = load_midi_file(pathstr);

    let events = transform_events(track_events);

    play_from_beginning(
        output_device,
        events,
        time_info,
        notes_in.clone(),
        abort_out,
    )
    .expect("played");
}

fn midi_play_loop(
    output_device: usize,
    notes_in: Sender<NoteEvent>,
    commands_out: Receiver<ControlCommands>,
) {
    let mut status: Option<SongStatus> = None;
    let (mut abort_in, _) = bounded(BOUNDED_SIZE);
    loop {
        select! {
            recv(commands_out) -> msg => {
                info!("STATUS {:?}", status);
                match msg {
                    Ok(ControlCommands::Play(s)) => {
                        let start_now =  match &status {
                            Some(SongStatus { name, is_playing, play_time: _ }) if name != &s || !is_playing => {
                            true
                            },
                            None => {
                            true
                            },
                            _ => false
                        };
                        if start_now {
                            info!("SOUND START NOW");
                            let nn = notes_in.clone();
                            let ss = s.clone();
                            let (ai, ao) = bounded(BOUNDED_SIZE);
                            abort_in = ai;
                            spawn(move|| pppplay(output_device, &ss, nn,  ao));
                            status = Some(SongStatus{name: s, is_playing: true , play_time: PlayTime::default()});
                        }
                    },
                    Ok(ControlCommands::Stop) => {
                        if let Err(e) = abort_in.send(Abort) {
                            error!("failed to send abort {:?}", e)
                        } else {
                            if let Some(s) = status {
                                status = Some(SongStatus{name: s.name, is_playing: false, play_time: PlayTime::default()})
                            }
                        }
                    }
                    Ok(ControlCommands::Next(song)) | Ok(ControlCommands::Prev(song))=> {
                        if let Err(e) = abort_in.send(Abort) {
                            error!("failed to send abort {:?}", e)
                        } else {
                            status = Some(SongStatus{name: song.to_string(), is_playing: true, play_time: PlayTime::default()});

                            info!("SKIP START NOW");
                            let nn = notes_in.clone();
                            let ss = song.clone();
                            let (ai, ao) = bounded(BOUNDED_SIZE);
                            abort_in = ai;
                            spawn(move|| pppplay(output_device, &ss, nn,  ao));
                        }
                    }
                    Err(e) => error!("error on recv {:?}",e)
                }
            }
        }
    }
}

struct Abort;
/// Process all of the MIDI events and send them to output_device
/// Each note calls `thread::sleep` based on its `vtime` attribute
/// and `delta_timing`
fn play_from_beginning(
    output_device: usize,
    midi_events: Vec<MidiEvent>,
    delta_timing: DeltaTiming,
    midi_sender: channel::Sender<NoteEvent>,
    abort_out: channel::Receiver<Abort>,
) -> Result<(), Box<dyn Error>> {
    info!("call");
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
            let maybe_abort = abort_out.try_recv().ok();
            if let Some(Abort) = maybe_abort {
                info!("ABORT!");
                return Ok(());
            }

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

fn broadcast(out: Receiver<ControlCommands>, ins: Vec<Sender<ControlCommands>>) {
    loop {
        select! {
            recv(out) -> msg => match msg {
                Ok(c) => {
                    for ii in &ins {
                        if let Err(e) = ii.send(c.clone()) {
                            error!("Error broadcasting: {:?}",e)
                        }
                    }
                }
                Err(e) => error!("recv {:?}",e )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam_channel::unbounded;
    #[test]
    fn test_prev_file_index() {
        let (commands_in, _) = unbounded();
        let mut it = MsgPassingControls {
            filenames: vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string(),
                "qux".to_string(),
            ],
            is_playing: false,
            commands_in,
            file_index: 0,
            output_device: 0,
        };

        assert_eq!(it.prev_file_index(), 3);
        it.file_index = 2;
        assert_eq!(it.prev_file_index(), 1);
    }
}
