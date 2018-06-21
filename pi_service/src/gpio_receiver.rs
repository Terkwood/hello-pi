//MIT License

//Copyright (c) 2018 Terkwood

//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

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
