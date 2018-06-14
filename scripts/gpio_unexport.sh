#!/bin/bash

# USAGE:
#    gpio_unexport.sh 5
#    gpio_unexport.sh 5 17 26
#
#    Unexports the given GPIO pins from user space 
#    Useful when clearing the value of pins after
#      killing a program

for arg in "$@"
do
    echo $arg > /sys/class/gpio/unexport
done
