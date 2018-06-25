# Assorted notes

## MIDI sample data

We ran [rimd](https://github.com/RustAudio/rimd) test bin against the Goldberg Variations and found the expected header data, as well as plenty of Note On events.

```sh
% cargo run --bin test # rimd

Reading: /Documents/Goldberg_Variations.mid
format: multiple track
tracks: 2
division: 480

1: Track, copyright: [none], name: [none]
events:
  time: 0	Meta Event: Time Signature: 3/4, 24 ticks/metronome click, 8 32nd notes/quarter note
  time: 0	Meta Event: Key Signature, 1 sharps/flats, Major
  time: 0	Meta Event: Set Tempo, microseconds/quarter note: 833333
  ...
  time: 0	Note On: [79,80]	channel: Some(0)
  time: 479	Note On: [79,0]	channel: Some(0)
  time: 480	Note On: [79,80]	channel: Some(0)
  time: 959	Note On: [79,0]	channel: Some(0)
  ...
  time: 3715	Note On: [67,0]	channel: Some(0)
  time: 3720	Note On: [69,80]	channel: Some(0)
  time: 3778	Note On: [67,80]	channel: Some(0)
  time: 3784	Note On: [69,0]	channel: Some(0)
```

We can see by reading http://www.onicos.com/staff/iz/formats/midi-event.html that most of these are "Channel 1 Note On".

## Notes on Midi Time Management

MIDI Note On / Note Off events are ordered in time.  Each event contains a `delta_time` (vtime) field representing the time elapsed since the last event.

In most cases, you can use the headers/metadata at the beginning of the file to establish a simple relationship between microseconds and "ticks" of the MIDI score.

From `rimd` documentation in `meta.rs`:

```rust
    /// The parameter `clocks_per_tick` is the number of MIDI Clocks per metronome tick.

    /// Normally, there are 24 MIDI Clocks per quarter note.
    /// However, some software allows this to be set by the user.
    /// The parameter `num_32nd_notes_per_24_clocks` defines this in terms of the
    /// number of 1/32 notes which make up the usual 24 MIDI Clocks
    /// (the 'standard' quarter note).  8 is standard
    pub fn time_signature(numerator: u8, denominator: u8, clocks_per_tick: u8, num_32nd_notes_per_24_clocks: u8) -> MetaEvent {
        MetaEvent {
            command: MetaCommand::TimeSignature,
            length: 4,
            data: vec![numerator,denominator,clocks_per_tick,num_32nd_notes_per_24_clocks],
        }
    }
```

Open Goldberg Variations JS Bach midi appears to support the standard timing when loaded:

```text
vtime 0 event time: 0	Meta Event: Time Signature: 3/4, 24 ticks/metronome click, 8 32nd notes/quarter note
  ...snip...
  vtime 0 event time: 0	Meta Event: Set Tempo, microseconds/quarter note: 833333
```

A more detailed specification of the byte-level MIDI delta time (vtime) field can be found here:

https://www.csie.ntu.edu.tw/~r92092/ref/midi/
