extern crate sysfs_gpio;


const BUTTON_PIN: u32 = 25;

pub fn run() {
    let led_pin = Pin::new(18);
    let button_pin = Pin::new(25);
    led_pin
        .with_exported(|| {
            button_pin.with_exported ( ||
            {
led_pin.set_direction(Direction::Out).unwrap();
            loop {
                led_pin.set_value(0).unwrap();
                sleep(Duration::from_millis(200));
                led_pin.set_value(1).unwrap();
                sleep(Duration::from_millis(200));
            }
            }
            )
            
        })
        .unwrap();
}