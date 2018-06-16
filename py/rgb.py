# Assumed setup:
# - Raspberry Pi 3 B+
# - Button on GPIO 25
# - Common cathode RGB LED
#       First lead: RED, GPIO 12
#       Second lead: CATHODE, GND
#       Third lead: GREEN, GPIO 16
#       Fourth lead: BLUE, GPIO 20

# Also see the diagram rgb.jpg in the docs folder.

from gpiozero import PWMLED
from gpiozero import Button
import random
import time

SLEEP_SECS = 0.333

red_led = PWMLED(12)
green_led = PWMLED(16)
blue_led = PWMLED(20)

button = Button(25)

state = 0

def reroll():
    change_color(rgb_rand(), rgb_rand(), rgb_rand())
    
def pulse(r = True, g = True, b = True):
    if r:
        red_led.pulse()
    else:
        red_led.off()
    if g:
        green_led.pulse()
    else:
        green_led.off()
    if b:
        blue_led.pulse()
    else:
        blue_led.off()

def state_change():
    global state
    state = (state+1)%8
    if not state:
        print("\nG  O      W  I  L  D\n")
        reroll()
    else:
        time.sleep(0.1)
        r = state & 1 > 0
        g = state & 2 > 0
        b = state & 4 > 0
        print("R {}\tG {}\tB {}".format(int(r), int(g), int(b)))
        pulse(r, g, b)

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
    if not state:
        time.sleep(0.1)
        reroll()
