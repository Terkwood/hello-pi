#[derive(Debug)]
pub struct SongStatus {
    pub name: String,
    pub play_time: PlayTime,
    pub is_playing: bool,
}
#[derive(Copy, Clone, Debug)]
pub struct PlayTime {
    pub secs: u32,
}
