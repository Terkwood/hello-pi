# Hello Raspberry Pi 

Examples which demonstrate the use of Raspberry Pi and GPIO.

Of interest are those written in [rust](rs).

## Breadboard Diagrams

We used the following configuration for the `button`
example:

![button configuration](doc/button.jpg)

There are Fritzing diagrams available for these examples
in the [docs folder](docs).

You can [download Fritzing here](http://fritzing.org/home/).

## Learnings

Basic push buttons have legs which are bent by default.
This is helpful for soldering the buttons onto actual build,
but makes it difficult to get the button pushed into the
breadboard.

We used a pair of pliers to straighten out the pins on the
button, and made sure that we pushed the button all the
way down into the bread board.  This allowed us to establish
connectivity with the button after an initial period of failure.

![Before straightening out button legs](img/bent_legs.jpg)

### Linux GPIO hints

These scripts are available in the [helpers](helpers) directory,
which you can use to unexport pins which are still held in user
space after killing your app, or to test setting the output
value for a set of pins.

## Thank You - Blink Freely!

![blink freely](img/flashy.jpg)
