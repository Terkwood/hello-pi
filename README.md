# Hello Raspberry Pi 

Examples which demonstrate the use of Raspberry Pi and GPIO.

Of interest are those written in [rust](rs).

## Learnings

We learned that the simple push button included in basic 
Raspberry Pi kits has legs that are bent by default.  This
is helpful for soldering the buttons onto actual build,
but it made it difficult to get the button pushed into the
bread board.

We used a pair of pliers to straighten out the pins on the
button, and made sure that we pushed the button all the
way down into the bread board.

## Linux GPIO hints

These scripts are available in the [helpers](helpers) directory,
which you can use to unexport pins which are still held in user
space after killing your app, or to test setting the output
value for a set of pins.

![blink freely](img/flashy.jpg)
