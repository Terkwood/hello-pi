extern crate crossbeam_channel as channel;
extern crate redis;
extern crate wiringpi;

mod gpio_receiver;
mod model;
mod pins;
mod redis_subscribe;

use model::WritePwm;
use pins::*;
use std::thread;
use std::time::Duration;

fn main() {
    let (gpio_s, gpio_r) = channel::bounded(5);

    thread::spawn(move || gpio_receiver::run(gpio_r));
    thread::spawn(move || redis_subscribe::run(gpio_s));

    // Duty cycle ranges from 0 to 100
    fn from_color(color: i32) -> i32 {
        (color as f32 / 255.0 * 100.0) as i32
    }
}
