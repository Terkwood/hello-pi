extern crate sysfs_gpio;

pub mod lights;

use std::env;
use std::str::FromStr;
use std::time::Duration;

const TIMER_DEFAULT_SECS: u64 = 5;

fn main() {
    match env::args().nth(1).as_ref().map(|s| s.as_str()) {
        Some("blink3") => lights::blink3(),
        Some("flashy") => lights::flashy(),
        Some("timer") => {
            let secs = Duration::from_secs(
                env::args()
                    .nth(2)
                    .as_ref()
                    .map(|s| u64::from_str(s).unwrap_or(TIMER_DEFAULT_SECS))
                    .unwrap_or(TIMER_DEFAULT_SECS),
            );
            lights::timer(secs, &lights::flashy)
        }
        _ => lights::blink(),
    }
}
