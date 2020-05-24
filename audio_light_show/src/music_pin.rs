use wiringpi::pin::*;
use wiringpi::WiringPi;

pub fn note_to_led(c: i8, num_leds: usize) -> u16 {
    c.modulo(num_leds as i8) as u16
}
///
/// Modulo that handles negative numbers, works the same as Python's `%`.
///
/// eg: `(a + b).modulo(c)`
/// from https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
pub trait ModuloSignedExt {
    fn modulo(&self, n: Self) -> Self;
}
macro_rules! modulo_signed_ext_impl {
    ($($t:ty)*) => ($(
        impl ModuloSignedExt for $t {
            #[inline]
            fn modulo(&self, n: Self) -> Self {
                (self % n + n) % n
            }
        }
    )*)
}
modulo_signed_ext_impl! { i8 i16 i32 i64 }

pub trait MusicPin {
    fn write(&self, velocity: i32);
}

pub struct DigitalMusicPin {
    pub pin: OutputPin<Gpio>,
}

impl DigitalMusicPin {
    pub fn new(gpio: &WiringPi<Gpio>, pin_num: u16) -> DigitalMusicPin {
        DigitalMusicPin {
            pin: gpio.output_pin(pin_num),
        }
    }
}

impl MusicPin for DigitalMusicPin {
    fn write(&self, velocity: i32) {
        let v = if velocity > 0 {
            Value::High
        } else {
            Value::Low
        };
        self.pin.digital_write(v)
    }
}
