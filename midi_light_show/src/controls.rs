use crate::song::*;

/// Simple model for the controls used in a music player
pub trait Controls {
    fn play(&mut self) -> Result<SongStatus, ControlsErr>;
    fn stop(&mut self) -> Result<SongStatus, ControlsErr>;
    fn next(&mut self) -> Result<SongStatus, ControlsErr>;
    fn prev(&mut self) -> Result<SongStatus, ControlsErr>;
}

/// Errors that can occur when controlling a music player
#[derive(Debug)]
pub enum ControlsErr {}
