extern crate sysfs_gpio;

use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::Pin;

pub const BLUE_PIN: u64 = 17;
pub const YELLOW_PIN: u64 = 5;
pub const RED_PIN: u64 = 26;

fn main() {
    let blue_pin = Pin::new(BLUE_PIN);
    let yellow_pin = Pin::new(YELLOW_PIN);
    let red_pin = Pin::new(RED_PIN);

    let duration = 125;
    blue_pin
        .with_exported(|| {
            yellow_pin.with_exported(|| {
                red_pin.with_exported(|| loop {
                    blue_pin.set_value(1).unwrap();
                    yellow_pin.set_value(1).unwrap();
                    red_pin.set_value(1).unwrap();
                    sleep(Duration::from_millis(duration));
                    blue_pin.set_value(0).unwrap();
                    yellow_pin.set_value(0).unwrap();
                    red_pin.set_value(0).unwrap();
                    sleep(Duration::from_millis(duration));
                })
            })
        })
        .unwrap();
}
