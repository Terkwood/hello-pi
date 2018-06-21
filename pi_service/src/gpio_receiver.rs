extern crate crossbeam_channel as channel;
extern crate wiringpi;

use model::WritePwm;
use pins::*;
use std::collections::HashMap;

pub fn run(output_r: channel::Receiver<WritePwm>) {
    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let pi = wiringpi::setup_gpio();
    let pins = {
        let mut p = HashMap::new();

        // track some pins
        p.insert(RED_GPIO, pi.soft_pwm_pin(RED_GPIO)); // red
        p.insert(GREEN_GPIO, pi.soft_pwm_pin(GREEN_GPIO)); // green
        p.insert(BLUE_GPIO, pi.soft_pwm_pin(BLUE_GPIO)); // blue
        p
    };
    loop {
        match output_r.recv() {
            Some(WritePwm { pin, value }) => if let Some(p) = pins.get(&pin) {
                p.pwm_write(value);
            },
            None => {}
        }
    }
}
