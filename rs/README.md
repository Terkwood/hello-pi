# Hello Rustberry Pi

Three small examples which you can use to test GPIO
on a Raspberry Pi.  

PLEASE NOTE that the pin numbers are hardcoded in
[lights.rs](src/lights.rs) and [button.rs](src/button.rs).

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
