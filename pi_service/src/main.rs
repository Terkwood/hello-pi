#[macro_use]
extern crate crossbeam_channel as channel;
extern crate wiringpi;

use std::thread;
use std::time::Duration;

enum PiOutput {
    Led { pin: u32, value: u32 },
    Blink { pin: u32, millis: u32 },
}

fn main() {
    //let (output_s, output_r) = channel::bounded(1);

    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let pi = wiringpi::setup_gpio();

    let red_led = pi.soft_pwm_pin(12);
    let green_led = pi.soft_pwm_pin(16);
    let blue_led = pi.soft_pwm_pin(20);

    // clear
    red_led.pwm_write(0);
    green_led.pwm_write(0);
    blue_led.pwm_write(0);

    // show a pleasant green
    red_led.pwm_write(duty_value(47));
    green_led.pwm_write(duty_value(181));
    blue_led.pwm_write(duty_value(47));

    thread::sleep(Duration::from_secs(5));

    // clear green
    green_led.pwm_write(0);

    loop {
        for i in 0..256 {
            let v = duty_value(i);
            red_led.pwm_write(v);
            blue_led.pwm_write(v);
            thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(10));

        for i in 0..256 {
            let v = 100 - duty_value(i);
            red_led.pwm_write(v);
            blue_led.pwm_write(v);
            thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(10));
    }

    // Duty cycle ranges from 0 to 100
    fn duty_value(color: i32) -> i32 {
        (color as f32 / 255.0 * 100.0) as i32
    }
}
