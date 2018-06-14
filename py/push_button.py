import RPi.GPIO as GPIO
import time

GPIO.setwarnings(False)
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
        GPIO.output(yellow_pin, True)
    else:
        GPIO.output(blue_pin, True)
        GPIO.output(yellow_pin, False)
