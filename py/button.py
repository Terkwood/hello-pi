import RPi.GPIO as GPIO
import time

button_pin = 26 

GPIO.setwarnings(True)
GPIO.setmode(GPIO.BCM) # use GPIO pin numbers
GPIO.setup(button_pin, GPIO.IN, pull_up_down=GPIO.PUD_DOWN) # Important: set initial value for pin low 

while True:
    if GPIO.input(button_pin) == GPIO.HIGH:
        print("CLIMBING NOW!")
