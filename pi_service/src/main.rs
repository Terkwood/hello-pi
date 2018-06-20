extern crate wiringpi;

use wiringpi::pin::Value::High;
use std::time::Duration;
use std::thread;

fn main() {
    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let pi = wiringpi::setup_gpio();

    let mut alice = pi.soft_pwm_pin(12);

    // Use a duty cycle of 0.5 on both pins
    alice.pwm_write(50);

    thread::sleep(Duration::from_millis(2000));

    let alice_out = alice.into_output();
    alice_out.digital_write(High);

    thread::sleep(Duration::from_millis(2000));

    // Switch `alice_out` (pin 12) back to software PWM mode
    alice = alice_out.into_soft_pwm();
    alice.pwm_write(1024);

    thread::sleep(Duration::from_millis(2000));

    alice.pwm_write(0);

}
