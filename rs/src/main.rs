extern crate sysfs_gpio;

pub mod lights;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.first() {
        Some(program) => unimplemented!(),
        None => lights::blink(),
    }
}
