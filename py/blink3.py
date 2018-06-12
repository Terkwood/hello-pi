import RPi.GPIO as GPIO
import time

GPIO.setwarnings(False)
GPIO.setmode(GPIO.BCM)

GPIO.setup(17, GPIO.OUT)
GPIO.setup(5, GPIO.OUT)
GPIO.setup(26, GPIO.OUT)

duration = 0.125

while True:
    GPIO.output(17, True)
    GPIO.output(5, True)
    GPIO.output(26, True)
    time.sleep(duration)
    GPIO.output(17, False)
    GPIO.output(5, False)
    GPIO.output(26, False)
    time.sleep(duration)
