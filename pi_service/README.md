# Pi Service

A service which executes instructions against Raspberry Pi based on messages received via Redis pub/sub.

## Caution

Always use caution when wiring Raspberry Pi, and especially when executing arbitrary scripts that you find on the internet.  This project won't work with your Raspberry Pi wiring unless you've taken extreme care to match the configuration shown here.  This project is intended as an educational example only :fire:

## Acknowledgements

[rust-wiringpi](https://github.com/Ogeon/rust-wiringpi/blob/master/src/bindings.rs) was extremely helpful here.  Thank you.

## References

* http://wiringpi.com/
* https://github.com/Ogeon/rust-wiringpi/blob/master/src/bindings.rs
* https://www.admfactory.com/breathing-light-led-on-raspberry-pi-using-c/
* http://www.bristolwatch.com/rpi/pwmRpi.htm
