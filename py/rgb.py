from gpiozero import PWMLED
from gpiozero import Button
import random
import time

SLEEP_SECS = 0.333

red_led = PWMLED(12)
green_led = PWMLED(16)
blue_led = PWMLED(20)

button = Button(25)

rand_mode = False

def reroll():
    change_color(rgb_rand(), rgb_rand(), rgb_rand())
    
def pulse():
    red_led.pulse()
    green_led.pulse()
    blue_led.pulse()

def state_change():
    global rand_mode
    rand_mode = not rand_mode
    print(rand_mode)
    if rand_mode:
        reroll()
    else:
        time.sleep(0.1)
        pulse()

button.when_pressed = state_change

def bounded(n):
    return min(max(n, 0.0), 1.0)

def change_color(r, g, b):
    red_led.value = bounded(r)
    green_led.value = bounded(g)
    blue_led.value = bounded(b)
    return

def rgb_rand():
    return random.uniform(0.0, 0.5)

pulse()

while True:
    if rand_mode:
        time.sleep(0.1)
        reroll()
