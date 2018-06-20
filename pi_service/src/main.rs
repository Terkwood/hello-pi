extern crate wiringpi;

use std::thread;
use std::time::Duration;
use wiringpi::pin::Value::High;

fn main() {
    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let pi = wiringpi::setup_gpio();

    let mut alice = pi.soft_pwm_pin(12);

    loop {
        // Duty cycle ranges from 0 to 100
        for i in 0..101 {
            alice.pwm_write(i);
            thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(10));

        for i in 0..101 {
            alice.pwm_write(100 - i);
            thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(10));
    }
}
