extern crate wiringpi;

use wiringpi::pin::Value::{High, Low};
use std::{thread, time};

fn main() {
    //Setup WiringPi using GPIO pin numbering
    let pi = wiringpi::setup_gpio();

    //Use WiringPi pin 12 as output
    let pin = pi.output_pin(12);

    let interval = time::Duration::from_millis(1000);

    loop {
        //Set pin 0 to high and wait one second
        pin.digital_write(High);
        thread::sleep(interval);

        //Set pin 0 to low and wait one second
        pin.digital_write(Low);
        thread::sleep(interval);
    }
}