# midi_light_show

This project implements a simple LED light and music show which can be executed on a Raspberry Pi.  You can provide a MIDI file and it will display a pleasing series of blinkenlights.

[![Youtube: Rustberry Pi Plays Erik Satie](http://img.youtube.com/vi/bh_AfZ8rU/0.jpg)](https://www.youtube.com/watch?v=M-bh_AfZ8rU "YouTube: Rustberry Pi Plays Erik Satie")

## Raspberry Pi LED Wiring

This project supports multiple LED layouts.  Shown below is the "basic" layout with eight LEDs.  Be careful to check the GPIO pins of your specific model of Pi, as they may not match what is shown below.

![Fritzing Diagram](doc/midi_light_show.jpg)

You can also choose an "extended" layout which uses additional LEDs, or specify additional layouts, by modifying the `[pins]` section of [Settings.toml](Settings.toml):

```toml
layout = "basic"   ## select which pin layout you want to use

[pins]
extended = [13, 6, 5, 7, 23, 18, 15, 14, 21, 26, 20, 16, 19, 11, 9, 10, 22, 27, 8, 25, 17, 4, 3, 2]
basic = [13, 6, 5, 7, 23, 18, 15, 14]
```

The diagrams for the "extended" layout are available in the `doc` directory:

* [Extended layout Fritzing file](doc/midi_light_show_extended.fzz)
* [Extended layout image](doc/midi_light_show_extended.jpg)

## Usage

We recommend downloading the JS Bach Goldberg Variations from https://www.opengoldbergvariations.org/.

You need to start `fluidsynth` as a server, allow MIDI input, connect it to your ALSA sound device on the Pi, and then correctly select the device number when starting this application:

```sh
sudo apt-get install fluidsynth fluid-soundfont-gm libasound2-dev

fluidsynth -a alsa -i /usr/share/sounds/sf2/FluidR3_GM.sf2 --server

aconnect -lio   #  and COUNT the number of MIDI related devices

cargo run SOME_MIDI_FILE 1  # if fluidsynth is the only virtual midi device on your pi, it should have ID 1.  see above
```

Most of the time, on Raspberry Pi, after enabling fluidsynth, you can omit the audio device ID:

```sh
cargo run SOME_MIDI_FILE
```

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

### Virtual MIDI

Required reading: http://sandsoftwaresound.net/qsynth-fluidsynth-raspberry-pi/

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

### Output to HDMI monitor

The easiest way to have your Raspberry Pi output sound in a timely fashion is to hook up an HDMI monitor and select it as the output source.

### Transforming MP3s to MIDI

Although it often results in MIDI files with strange sound artifacts, one can use the following pipeline to transform MP3s to MIDI was as follows:

* Use [Audacity](https://www.audacityteam.org/) to convert the MP3 to a WAV.
* (optional) Use Audacity to perform Noise Reduction on the WAV file (helpful if there's a hiss or a hum in the recording): https://www.podfeet.com/blog/recording/how-to-remove-noise-with-audacity/
* Use [AmazingMIDI](http://www.pluto.dti.ne.jp/~araki/amazingmidi/) to convert the WAV to a MIDI.

For instance, after converting the MP3 for [Rachmaninov's Prelude in C Sharp Minor, Op 3](https://www.gutenberg.org/ebooks/10175) to a WAV, we used the following settings in `AmazingMIDI` in order to generate a [noisy but somewhat accurate MIDI output file](resources/rach3.mid):

![Rach Prelude to MIDI](doc/rach_to_midi.png)

## Further Reading

* [MIDI Event Table](http://www.onicos.com/staff/iz/formats/midi-event.html)
* [The MIDI File Format](https://www.csie.ntu.edu.tw/~r92092/ref/midi/)
* [Ted's Linux MIDI Guide](http://tedfelix.com/linux/linux-midi.html)
* [The Linux MIDI-HOWTO](http://tldp.org/HOWTO/MIDI-HOWTO-10.html)

## Acknowledgements

Bach's Goldberg Variations are [available under Creative Commons License here](https://www.opengoldbergvariations.org/).

The availability of the following audio libraries is greatly appreciated:

* Big thanks to [midir library](https://github.com/Boddlnagg/midir).
* Big thanks to [rimd library](https://github.com/RustAudio/rimd).
* Big thanks to [rtmidi library](https://github.com/thestk/rtmidi).

Thank you to [fluidsynth](http://www.fluidsynth.org/), which allowed us to send MIDI output to an audio device on Raspberry Pi.

### Audio and MIDI Resources

[Wikipedia provides a delicious selection of classical MIDI music](https://commons.m.wikimedia.org/wiki/Category:MIDI_files_of_classical_music), most of which are in the public domain/licensed in a Creative Commons fashion, etc.

We drew from MP3 audio found [at Project Gutenberg Music Category](https://www.gutenberg.org/browse/categories/3).

We recommend listening to selections from [Bernd Kreuger's Classical Piano MIDI Page](http://www.piano-midi.de/midi_files.htm).  There are a number of different composers represented here, and it was fun to try some of these out on the Pi!

We also recommend listening to [Fredrik Johansson's MIDI repository](https://github.com/fredrik-johansson/midi), an excellent, extensive body of work!  Bravo!  Their work is available for download.
