extern crate crossbeam_channel as channel;
extern crate redis;
extern crate wiringpi;

pub mod model;
mod pi_receiver;
pub mod pins;
mod redis_subscribe;

use model::WritePwm;
use pins::*;
use std::thread;
use std::time::Duration;

fn main() {
    let (output_s, output_r) = channel::bounded(1);

    thread::spawn(move || pi_receiver::run(output_r));

    // clear
    output_s.send(WritePwm {
        pin: RED_GPIO,
        value: 0,
    });
    output_s.send(WritePwm {
        pin: GREEN_GPIO,
        value: 0,
    });
    output_s.send(WritePwm {
        pin: BLUE_GPIO,
        value: 0,
    });

    // show a pleasant green
    output_s.send(WritePwm {
        pin: RED_GPIO,
        value: from_color(47),
    });
    output_s.send(WritePwm {
        pin: GREEN_GPIO,
        value: from_color(181),
    });
    output_s.send(WritePwm {
        pin: BLUE_GPIO,
        value: from_color(47),
    });

    thread::sleep(Duration::from_secs(5));

    // clear green
    output_s.send(WritePwm {
        pin: GREEN_GPIO,
        value: 0,
    });

    loop {
        for i in 0..256 {
            let v = from_color(i);
            output_s.send(WritePwm {
                pin: RED_GPIO,
                value: v,
            });
            output_s.send(WritePwm {
                pin: BLUE_GPIO,
                value: v,
            });
            thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(10));

        for i in 0..256 {
            let v = 100 - from_color(i);
            output_s.send(WritePwm {
                pin: RED_GPIO,
                value: v,
            });
            output_s.send(WritePwm {
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
