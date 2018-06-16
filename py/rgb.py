from gpiozero import PWMLED
from gpiozero import Button
import random
import time

SLEEP_SECS = 0.333

red_led = PWMLED(12)
green_led = PWMLED(16)
blue_led = PWMLED(20)

button = Button(25)

def reroll():
    change_color(rgb_rand(), rgb_rand(), rgb_rand())
    red_led.pulse()
    green_led.pulse()
    blue_led.pulse()

button.when_pressed = reroll

def bounded(n):
    return min(max(n, 0.0), 1.0)

def change_color(r, g, b):
    red_led.value = bounded(r)
    green_led.value = bounded(g)
    blue_led.value = bounded(b)
    return

def rgb_rand():
    return random.uniform(0.0, 0.5)

reroll()

while True:
    True
