#!/bin/bash

for arg in "$@"
do
    echo $arg > /sys/class/gpio/export \ 
      && echo out > /sys/class/gpio/gpio$arg/direction \
      && echo 1   > /sys/class/gpio/gpio$arg/value 
done
