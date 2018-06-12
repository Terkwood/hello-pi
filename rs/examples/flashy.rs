extern crate sysfs_gpio;

use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

fn main() {
    let blue_pin = Pin::new(BLUE_PIN);
    let yellow_pin = Pin::new(YELLOW_PIN);
    let red_pin = Pin::new(RED_PIN);

    let blue_flag = 0b0_001u32;
    let yellow_flag = 0b0_010u32;
    let red_flag = 0b0_100u32;

    let duration = 31;
    let mut count = 0;

    blue_pin
        .with_exported(|| {
            yellow_pin.with_exported(|| {
                red_pin.with_exported(|| loop {
                    count = (count + 1) % 8;
                    blue_pin.set_value((count & blue_flag > 0) as u8).unwrap();
                    yellow_pin
                        .set_value((count & yellow_flag > 0) as u8)
                        .unwrap();
                    red_pin.set_value((count & red_flag > 0) as u8).unwrap();
                    sleep(Duration::from_millis(duration));
                })
            })
        })
        .unwrap();
}
