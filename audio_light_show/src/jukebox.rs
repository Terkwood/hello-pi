use crate::music_pin::*;
use crate::NoteTime;

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
pub fn blink_lights(time_freqs: Vec<NoteTime>) {
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
    for NoteTime {
        note,
        start_secs,
        stop_secs,
    } in time_freqs
    {
        thread::sleep(Duration::from_secs_f32(start_secs - secs));
        secs = start_secs;

        let pin = note_to_led(note as i8, PINS.len());

        &led_pin_outs[pin as usize].write(LED_ON);
        thread::sleep(Duration::from_secs_f32(stop_secs - secs));
        secs = stop_secs;
        &led_pin_outs[pin as usize].write(0);
    }
}
