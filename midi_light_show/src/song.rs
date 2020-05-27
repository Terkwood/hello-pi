// SPDX-License-Identifier: MIT
use serde_derive::{Deserialize, Serialize};
/// Represents the name of an audio file being controlled by
/// a player, whether the audio is currently playing,
/// and the time when the
#[derive(Debug, Serialize, Deserialize)]
pub struct SongStatus {
    pub name: String,
    pub play_time: PlayTime,
    pub is_playing: bool,
}

/// The position in the track's playback, measured in milliseconds.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlayTime {
    pub millis: u32,
}
impl Default for PlayTime {
    fn default() -> Self {
        PlayTime { millis: 0 }
    }
}
