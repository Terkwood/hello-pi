import RPi.GPIO as GPIO
import time

GPIO.setwarnings(False)
GPIO.setmode(GPIO.BCM)

blue_flag = int('001', 2)
yellow_flag = int('010', 2)
red_flag = int('100', 2)

blue_pin = 17
yellow_pin = 5
red_pin = 26

GPIO.setup(blue_pin, GPIO.OUT)
GPIO.setup(yellow_pin, GPIO.OUT)
GPIO.setup(red_pin, GPIO.OUT)

slow = 1.0
fast = 0.03125
duration = slow 

count = 0

while True:
    count = (count + 1) % 8
    GPIO.output(blue_pin, count & blue_flag > 0)
    GPIO.output(yellow_pin, count & yellow_flag > 0)
    GPIO.output(red_pin, count & red_flag > 0)
    time.sleep(duration)
