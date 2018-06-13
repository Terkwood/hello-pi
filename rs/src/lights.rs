extern crate sysfs_gpio;

use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::Pin;

pub const BLUE_PIN: u64 = 17;
pub const YELLOW_PIN: u64 = 5;
pub const RED_PIN: u64 = 26;

pub fn blink() {
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

pub fn blink3() {
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

pub fn flashy() {
    let blue_pin = Pin::new(BLUE_PIN);
    let yellow_pin = Pin::new(YELLOW_PIN);
    let red_pin = Pin::new(RED_PIN);

    let blue_flag = 0b0_001u32;
    let yellow_flag = 0b0_010u32;
    let red_flag = 0b0_100u32;

    let duration = 31;
    let mut count = 0;

    if let Err(_e) = blue_pin.with_exported(|| {
        yellow_pin.with_exported(|| {
            red_pin.with_exported(|| loop {
                count = (count + 1) % 8;
                if let Err(_e) = blue_pin.set_value((count & blue_flag > 0) as u8) {
                    println!("Couldn't set blue");
                };
                yellow_pin
                    .set_value((count & yellow_flag > 0) as u8)
                    .unwrap();
                red_pin.set_value((count & red_flag > 0) as u8).unwrap();
                sleep(Duration::from_millis(duration));
            })
        })
    }) {
        println!("Failed - unexporting pins");
        blue_pin.unexport().unwrap();
        yellow_pin.unexport().unwrap();
        red_pin.unexport().unwrap();
    }
}
