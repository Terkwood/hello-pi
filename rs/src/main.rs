extern crate sysfs_gpio;

pub mod lights;

use std::env;
use std::time::Duration;

const TIMER_WAIT: Duration = Duration::from_secs(5);

fn main() {
    match env::args().nth(1).as_ref().map(|s| s.as_str()) {
        Some("blink3") => lights::blink3(),
        Some("flashy") => lights::flashy(),
        Some("timer") => lights::timer(TIMER_WAIT, &lights::flashy),
        _ => lights::blink(),
    }
}
