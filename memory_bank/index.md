# Purpose

We are creating a moving string artwork. The idea is that we have a circular board with pins on the outside. It will be completely automated so that it ties the string between the pins. 

# Mechanism
We are using the greedy algorithm to find the best path for the strings. Eventually the strings will resemble an image over time the more we tie. 

The canvas will be a white background with moveable pins on the outside. There will be 720 pins (2 per degree of motion) and there will be a motor that spins the canvas. 

Where the motor is, there will be a mechanism that pushes a select pin out and wraps the string around it. This will be how the artwork "paints" itself.

When it is complete there will be another mechanism to spin the pins so that they release their strings. 

The pins are basically long cylinders with a groove on the one side. When the pin is spun the string looses the groove and can come free.