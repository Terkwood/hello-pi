# hello rust raspberry pi world!

Three small examples which you can use to test GPIO
on a Raspberry Pi.  

PLEASE NOTE that the pin numbers are hardcoded to:

```
Blue    17
Yellow  5
Red     26
```

### Usage

Blink pin 17

`cargo run blink`

Blink three pins at the same time

`cargo run blink3`

Count from 0..8 in binary

`cargo run flashy`
