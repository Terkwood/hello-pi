extern crate wiringpi;

use std::thread;
use std::time::Duration;

fn main() {
    // Setup wiringPi in GPIO mode (with original BCM numbering order)
    let pi = wiringpi::setup_gpio();

    let red_led = pi.soft_pwm_pin(12);
    let green_led = pi.soft_pwm_pin(16);
    let blue_led = pi.soft_pwm_pin(20);

    red_led.pwm_write(0);
    green_led.pwm_write(0);
    blue_led.pwm_write(0);

    // show a pleasant green
    red_led.pwm_write(pwm_value(47));
    green_led.pwm_write(pwm_value(181));
    blue_led.pwm_write(pwm_value(47));

    thread::sleep(Duration::from_secs(5));

    green_led.pwm_write(0);

    loop {
        // Duty cycle ranges from 0 to 100
        for i in 0..256 {
            let v = pwm_value(i);
            red_led.pwm_write(v);
            blue_led.pwm_write(v);
            thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(10));

        for i in 0..256 {
            let v = 100 - pwm_value(i);
            red_led.pwm_write(v);
            blue_led.pwm_write(v);
            thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(10));
    }

    fn pwm_value(color_value: i32) -> i32 {
        (color_value as f32 / 255.0 * 100.0) as i32
    }
}
