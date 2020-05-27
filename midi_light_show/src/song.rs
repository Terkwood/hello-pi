/// Represents the name of an audio file being controlled by
/// a player, whether the audio is currently playing,
/// and the time when the
#[derive(Debug)]
pub struct SongStatus {
    pub name: String,
    pub play_time: PlayTime,
    pub is_playing: bool,
}

/// The position in the track's playback, measured in milliseconds.
#[derive(Copy, Clone, Debug)]
pub struct PlayTime {
    pub millis: u32,
}
