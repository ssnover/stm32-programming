# stm32-programming
This repo is meant to hold a number of small examples demonstrating how to use Rust to implement a number of smaller bare metal embedded programs. All of these are set up to compile for the STM32F302x8 Nucleo development board, but they should be trivially modifiable to other ARM boards with a decent HAL crate.

Right now I'm working through the C examples in __Make: AVR Programming__ by Elliot Williams. The source of the C programs can be found in the [official repository here](https://github.com/hexagon5un/AVR-Programming). If you're new to embedded systems programming, I recommend buying the book and going through the exercises as they've got excellent explanations of various aspects of embedded development.

## Examples List
* `blink-led`: Demonstrates blocking delays and toggling a GPIO pin.
* `pov-toy`: Demonstrates toggling output pins with embedded HAL traits.