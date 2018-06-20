#[macro_use]
extern crate crossbeam_channel as channel;
extern crate wiringpi;

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

enum PiOutput {
    Led { pin: u16, value: i32 },
    Blink { pin: u16, millis: u32 },
}

const RED_GPIO: u16 = 12;
const GREEN_GPIO: u16 = 16;
const BLUE_GPIO: u16 = 20;

fn pi_consumer(output_r: channel::Receiver<PiOutput>) {
    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let pi = wiringpi::setup_gpio();
    let pins = {
        let mut p = HashMap::new();

        // track some pins
        p.insert(RED_GPIO, pi.soft_pwm_pin(RED_GPIO)); // red
        p.insert(GREEN_GPIO, pi.soft_pwm_pin(GREEN_GPIO)); // green
        p.insert(BLUE_GPIO, pi.soft_pwm_pin(BLUE_GPIO)); // blue
        p
    };
    loop {
        match output_r.recv() {
            Some(PiOutput::Led { pin, value }) => if let Some(p) = pins.get(&pin) {
                p.pwm_write(value);
            },
            Some(PiOutput::Blink { pin: _, millis: _ }) => {}
            None => {}
        }
    }
}

fn main() {
    let (output_s, output_r) = channel::bounded(1);

    thread::spawn(move || pi_consumer(output_r));

    // clear
    output_s.send(PiOutput::Led {
        pin: RED_GPIO,
        value: 0,
    });
    output_s.send(PiOutput::Led {
        pin: GREEN_GPIO,
        value: 0,
    });
    output_s.send(PiOutput::Led {
        pin: BLUE_GPIO,
        value: 0,
    });

    // show a pleasant green
    output_s.send(PiOutput::Led {
        pin: RED_GPIO,
        value: from_color(47),
    });
    output_s.send(PiOutput::Led {
        pin: GREEN_GPIO,
        value: from_color(181),
    });
    output_s.send(PiOutput::Led {
        pin: BLUE_GPIO,
        value: from_color(47),
    });

    thread::sleep(Duration::from_secs(5));

    // clear green
    output_s.send(PiOutput::Led {
        pin: GREEN_GPIO,
        value: 0,
    });

    loop {
        for i in 0..256 {
            let v = from_color(i);
            output_s.send(PiOutput::Led {
                pin: RED_GPIO,
                value: v,
            });
            output_s.send(PiOutput::Led {
                pin: BLUE_GPIO,
                value: v,
            });
            thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(10));

        for i in 0..256 {
            let v = 100 - from_color(i);
            output_s.send(PiOutput::Led {
                pin: RED_GPIO,
                value: v,
            });
            output_s.send(PiOutput::Led {
                pin: BLUE_GPIO,
                value: v,
            });
            thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(10));
    }

    // Duty cycle ranges from 0 to 100
    fn from_color(color: i32) -> i32 {
        (color as f32 / 255.0 * 100.0) as i32
    }
}
