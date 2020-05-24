use wiringpi::pin::*;
use wiringpi::WiringPi;

fn note_to_led(c: u8, num_leds: usize) -> usize {
    ((60 - c as i8).modulo(num_leds as i8)) as usize
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

trait MusicPin {
    fn write(&self, velocity: i32);
}

struct DigitalMusicPin {
    pin: OutputPin<Gpio>,
}

impl DigitalMusicPin {
    fn new(gpio: &WiringPi<Gpio>, pin_num: u16) -> DigitalMusicPin {
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
