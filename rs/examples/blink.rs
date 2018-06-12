extern crate sysfs_gpio;

use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

fn main() {
    let blue_pin = Pin::new(BLUE_PIN);
    blue_pin
        .with_exported(|| loop {
            blue_pin.set_value(0).unwrap();
            sleep(Duration::from_millis(200));
            blue_pin.set_value(1).unwrap();
            sleep(Duration::from_millis(200));
        })
        .unwrap();
}