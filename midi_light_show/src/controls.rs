use crate::song::*;
pub trait Controls {
    fn play(&mut self) -> Result<SongStatus, ControlsErr>;
    fn stop(&mut self) -> Result<SongStatus, ControlsErr>;
    fn next(&mut self) -> Result<SongStatus, ControlsErr>;
    fn prev(&mut self) -> Result<SongStatus, ControlsErr>;
}

#[derive(Debug)]
pub enum ControlsErr {}
