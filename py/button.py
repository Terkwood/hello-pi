import RPi.GPIO as GPIO
import time

led_pin = 18
button_pin = 25 

GPIO.setwarnings(True)
GPIO.setmode(GPIO.BCM)   # use GPIO pin numbers
GPIO.setup(led_pin, GPIO.OUT)  
GPIO.setup(button_pin, GPIO.IN)

while True:
    if GPIO.input(button_pin):
        GPIO.output(led_pin, True)
    else:
        GPIO.output(led_pin, False)
