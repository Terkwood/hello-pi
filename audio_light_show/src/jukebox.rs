use crate::music_pin::*;
use crate::TimeFreq;
use log::info;

use std::io::BufReader;
use std::thread;
use std::time::Duration;
pub fn play_music(filename: &str) {
    let device = rodio::default_output_device().expect("device");
    let sink = rodio::Sink::new(&device);

    let file = std::fs::File::open(filename).expect("open");
    sink.append(rodio::Decoder::new(BufReader::new(file)).expect("decoder"));

    sink.sleep_until_end();
}

const PINS: &[u16] = &[
    2, 3, 4, 17, 25, 8, 27, 22, 10, 9, 11, 19, 16, 20, 26, 21, 14, 15, 18, 23, 7, 5, 6, 13,
];
const LED_ON: i32 = 255;
pub fn blink_lights(time_freqs: Vec<TimeFreq>) {
    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let gpio = &wiringpi::setup_gpio();

    let led_pin_outs: Vec<Box<dyn MusicPin>> = {
        let mut lpos: Vec<Box<dyn MusicPin>> = Vec::new();
        for &p in PINS {
            lpos.push(Box::new(DigitalMusicPin::new(gpio, p)))
        }

        lpos
    };
    // clear
    for p in &led_pin_outs {
        p.write(0);
    }

    let mut secs: f32 = 0.0;
    let mut lit_pin: Option<u16> = None;
    for TimeFreq { time, freq } in time_freqs {
        thread::sleep(Duration::from_secs_f32(time - secs));
        secs = time;

        let note = freq_to_note(freq);
        let pin_to_light = note_to_led(note as i8, PINS.len());

        info!("Freq {}, Note {}, Pin   {}", freq, note, pin_to_light);
        match lit_pin {
            Some(p) if p == pin_to_light => {}
            Some(old_pin) => {
                &led_pin_outs[old_pin as usize].write(0);
                &led_pin_outs[pin_to_light as usize].write(LED_ON);
                lit_pin = Some(pin_to_light);
            }
            None => {
                &led_pin_outs[pin_to_light as usize].write(LED_ON);
                lit_pin = Some(pin_to_light);
            }
        }
    }
}

const A4_TUNING_HZ: f32 = 440.0;
const A4_KEY_POSITION: u8 = 49;
/// See https://en.m.wikipedia.org/wiki/Piano_key_frequencies
fn freq_to_note(freq: f32) -> u8 {
    (39.86 * (freq / A4_TUNING_HZ).log10()) as u8 + A4_KEY_POSITION
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_freq_to_note() {
        assert_eq!(A4_KEY_POSITION, freq_to_note(A4_TUNING_HZ));
        assert_eq!(103, freq_to_note(5919.911)); // F#8
        assert_eq!(26, freq_to_note(116.5409)); // Bb2
    }
}
