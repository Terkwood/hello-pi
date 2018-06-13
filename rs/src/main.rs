extern crate sysfs_gpio;

pub mod lights;

use std::env;

fn main() {
    lights::blink()
}
