extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};

pub fn run() {
    let led_pin = Pin::new(18);
    let button_pin = Pin::new(25);
    led_pin
        .with_exported(|| {
            button_pin.with_exported(|| {
                led_pin.set_direction(Direction::Out).unwrap();
                button_pin.set_direction(Direction::In).unwrap();
                loop {
                    match button_pin.get_value() {
                        Ok(1) => led_pin.set_value(0).unwrap(),
                        _ => led_pin.set_value(1).unwrap(),
                    }
                }
            })
        })
        .unwrap();
}
