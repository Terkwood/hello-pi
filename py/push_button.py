import RPi.GPIO as GPIO
import time

GPIO.setwarnings(True)
GPIO.setmode(GPIO.BCM)

blue_pin = 18
yellow_pin = 5
button_pin = 25

GPIO.setup(blue_pin, GPIO.OUT)
GPIO.setup(yellow_pin, GPIO.OUT)

GPIO.setup(button_pin, GPIO.IN)

while True:
    if GPIO.input(button_pin):
        GPIO.output(blue_pin, False)
    else:
        GPIO.output(blue_pin, True)
