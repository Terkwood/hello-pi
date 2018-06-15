from gpiozero import PWMLED

red_led = PWMLED(12)
green_led = PWMLED(16)
blue_led = PWMLED(20)

def bounded(n):
    return min(max(n, 0.0), 1.0)

def change_color(r, g, b):
    red_led.value = bounded(r)
    green_led.value = bounded(g)
    blue_led.value = bounded(b)
    return


