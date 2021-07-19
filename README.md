# AI plays Trackmania Nation United Forever (TMNF)

This is an experiment to see if I can create an AI playing TMNF.

The idea is simple:

- In a big loop take a screen shot, extract the speed (for now and in the future some raytracing to detect the border of the road)
- Use a crate which simulate pressing keys
- Profit

For now the AI only reads the speed of the car and presses <kbd>up</kbd> when it is lower than a threshold and release it when the threshold is crossed.


