extern crate wiringpi;

use std::thread;
use std::time::Duration;
use wiringpi::pin::Value::High;

fn main() {
    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let pi = wiringpi::setup_gpio();

    let red_led = pi.soft_pwm_pin(12);
    let green_led = pi.soft_pwm_pin(16);
    let blue_led = pi.soft_pwm_pin(20);

    loop {
        // Duty cycle ranges from 0 to 100
        for i in 0..256 {
            let v = i as f32 / 255.0;
            red_led.pwm_write(v as i32);
            thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(10));

        for i in 0..256 {
            let v = i as f32 / 255.0;
            red_led.pwm_write(100 - i);
            thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(10));
    }
}
