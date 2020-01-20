# Gravity simulator

[![Join the chat at https://gitter.im/gravity-sim/community](https://badges.gitter.im/gravity-sim/community.svg)](https://gitter.im/gravity-sim/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

A simple program simulating gravity in 2 dimensions.

The purpose of this program is to provide some educational features, like:

- defining arbitrary sets of interacting bodies
- changing the law of gravity (modifying the gravitational constant, changing the exponent for the distance)
- turning off gravity between selected pairs of bodies

## Interface

The features described above can only be accessed via a config file, which has to be called `config.yaml` at the moment. An example file showcasing all the features is provided.

Apart from that, the application window itself allows for zooming and panning the view, as well as selecting a reference body.

Selecting a reference body acts like you might expect: the view begins to move in exactly the same way as the selected body.

Panning the view is done by holding the left mouse button and moving the cursor.

To zoom, hold the right mouse button and move the cursor up/down.

Have fun!
