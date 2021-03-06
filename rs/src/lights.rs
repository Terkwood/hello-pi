extern crate sysfs_gpio;

use on_export;
use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

pub const BLUE_PIN: u64 = 17;
pub const YELLOW_PIN: u64 = 5;
pub const RED_PIN: u64 = 26;

pub fn blink() {
    let blue_pin = Pin::new(BLUE_PIN);
    blue_pin
        .with_exported(|| {
            // give the system a little time
            // to catch up to export
            on_export::wait();
            blue_pin.set_direction(Direction::Out).unwrap();
            loop {
                blue_pin.set_value(0).unwrap();
                sleep(Duration::from_millis(200));
                blue_pin.set_value(1).unwrap();
                sleep(Duration::from_millis(200));
            }
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
                red_pin.with_exported(|| {
                    on_export::wait();
                    blue_pin.set_direction(Direction::Out).unwrap();
                    yellow_pin.set_direction(Direction::Out).unwrap();
                    red_pin.set_direction(Direction::Out).unwrap();

                    loop {
                        blue_pin.set_value(1).unwrap();
                        yellow_pin.set_value(1).unwrap();
                        red_pin.set_value(1).unwrap();
                        sleep(Duration::from_millis(duration));
                        blue_pin.set_value(0).unwrap();
                        yellow_pin.set_value(0).unwrap();
                        red_pin.set_value(0).unwrap();
                        sleep(Duration::from_millis(duration));
                    }
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

    blue_pin
        .with_exported(|| {
            yellow_pin.with_exported(|| {
                red_pin.with_exported(|| {
                    on_export::wait();
                    blue_pin.set_direction(Direction::Out).unwrap();
                    yellow_pin.set_direction(Direction::Out).unwrap();
                    red_pin.set_direction(Direction::Out).unwrap();

                    loop {
                        count = (count + 1) % 8;
                        blue_pin.set_value((count & blue_flag > 0) as u8).unwrap();
                        yellow_pin
                            .set_value((count & yellow_flag > 0) as u8)
                            .unwrap();
                        red_pin.set_value((count & red_flag > 0) as u8).unwrap();
                        sleep(Duration::from_millis(duration));
                    }
                })
            })
        })
        .unwrap();
}

/// Flash some lights after a given amount of time
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// let duration = Duration::from_seconds(5);
/// let blink_fn = () -> blink()
///
/// timer(duration, blink_fn)
/// ```
pub fn timer(duration: Duration, blink_fn: &Fn() -> ()) {
    println!("See you in {} seconds\n... zzz ...\n. . . z z z . . . \n.   .   .     z  z z     .     .    .\n", duration.as_secs());
    sleep(duration);
    blink_fn()
}
