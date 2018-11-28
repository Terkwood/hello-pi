# shamelessly interpreted from https://projects.raspberrypi.org/en/projects/getting-started-with-picamera/5
# thanks :-D

from picamera import PiCamera
from time import sleep

camera = PiCamera()

camera.start_preview()
sleep(5)
camera.capture('/tmp/image.jpg')
camera.stop_preview()

