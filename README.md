# Hello Raspberry Pi 

Examples which demonstrate the use of Raspberry Pi and GPIO.

Of interest are those written in [rust](rs).

## Linux GPIO hints

Unexport pin 5 (useful after killing your process):

```bash
echo 5 > /sys/class/gpio/unexport 
```

Export pin 17 and give it some juice

```bash
echo 17 > /sys/class/gpio/export && \
  echo out > /sys/class/gpio/gpio17/direction && \  # not in
  echo 1 > /sys/class/gpio/gpio17/value
```

![blink freely](img/flashy.jpg)
