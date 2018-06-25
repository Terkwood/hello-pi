# hello_midi

Uses [rtmidi](https://github.com/thestk/rtmidi), [rimd](https://github.com/RustAudio/rimd) and [midir](https://github.com/Boddlnagg/midir) libs to read MIDI and play it. Tested on Mac OS X using [SimpleSynth](http://notahat.com/simplesynth/) and the public domain JS Bach Goldberg variations.

## Usage

We recommend downloading the JS Bach Goldberg Variations from https://www.opengoldbergvariations.org/.

```sh
cargo run ~/Documents/Goldberg_Variations.mid
```

We recommend listening to [Fredrik Johansson's MIDI repository](https://github.com/fredrik-johansson/midi), an excellent, extensive body of work!  Bravo!

## Building on Raspbian

You need to install `libsound2`.

```sh
sudo apt-get install libasound2-dev
```

At this point you can then build the `alsa` crate for rust on your Pi.

## Misc OS Configs

### Mac OS X

* Install SimpleSynth according to https://github.com/wbsoft/frescobaldi/wiki/MIDI-playback-on-Mac-OS-X

### Raspbian

#### Playing MIDI

Hook up a speaker to your Pi, then:

```sh
sudo apt-get install wildmidi
wildmidi Bach_Party.mid
```

### Virtual MIDI

Required reading: http://sandsoftwaresound.net/qsynth-fluidsynth-raspberry-pi/

Key insight:  you need to start `fluidsynth` as a server, allow MIDI input, connect it to your ALSA sound device on the Pi, and then correctly select the device number when starting this application:

```sh
fluidsynth -a alsa -i /usr/share/sounds/sf2/FluidR3_GM.sf2 --server
aconnect -lio   #  and COUNT the number of MIDI related devices
cargo run ~/Goldberg_Variations.mid 1  # if fluidsynth is the only virtual midi device on your pi, it should have ID 1.  see `aconnect -lio`
```

Helper command -- play a midi file to your speaker using `fluidsynth`:

```sh
fluidsynth -a alsa -n -i /usr/share/sounds/sf2/FluidR3_GM.sf ~/Goldberg_Variations.mid
```

See MIDI devices:

```sh
aconnect -lio
```

See sound cards:

```sh
aplay -l
```

### Raspbian MIDI to mp3 conversion

Install `mplayer` to get going quickly with playing an mp3 file to a bluetooth speaker.

You can use a combination of `timidity` and `lame` to convert MIDI files to mp3.

```sh
sudo apt-get install mplayer timidity lame

timidity MIDI_sample.mid -Ow -o - | lame - -b 64 sample.mp3

mplayer sample.mp3
```

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

## Using SoundFlower to record MIDI

Not strictly related to the build, but...

This is helpful for redirecting your successful
playback of MIDI to a sound file that you're recording.

https://github.com/mattingalls/Soundflower/releases/tag/2.0b2

## Acknowledgements

Bach's Goldberg Variations are [available under Creative Commons License here](https://www.opengoldbergvariations.org/).

Big thanks to [midir library](https://github.com/Boddlnagg/midir).

Big thanks to [rimd library](https://github.com/RustAudio/rimd).

Big thanks to [rtmidi library](https://github.com/thestk/rtmidi).

Thank you to [fluidsynth](http://www.fluidsynth.org/), which allowed us to send MIDI output to our audio device.
