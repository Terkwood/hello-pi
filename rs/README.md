# Hello Rustberry Pi

Three small examples which you can use to test GPIO
on a Raspberry Pi.  

This work makes use of the [rust-sysfs-gpio](https://github.com/rust-embedded/rust-sysfs-gpio) library.

## Usage

Blink pin 17:

`cargo run blink`

Blink three pins at the same time:

`cargo run blink3`

Count from 0..8 in binary:

`cargo run flashy`

Blink wildly after a given number of seconds:

`cargo run timer 10`

Turn off pin 18 when you push a button on pin 25:

`cargo run button`

## Caveat Emptor: Hardcoded Pin Numbers

Please note that the GPIO pin numbers are hardcoded in
[lights.rs](src/lights.rs) and [button.rs](src/button.rs).

### Sleepy Thread Heuristic After First Export

We found that we needed to give "the system" about 50 millis
to catch up to the GPIO pins changing from an _unexported_
to an _exported_ state.  To that end, we've defined the
`on_export::wait()` function in [on_export.rs](src/on_export.rs).
