extern crate sysfs_gpio;

pub mod lights;

use std::env;

fn main() {
    match env::args().nth(1).as_ref().map(|s| s.as_str()) {
        Some("blink3") => lights::blink3(),
        Some("flashy") => lights::flashy(),
        _ => lights::blink(),
    }
}
