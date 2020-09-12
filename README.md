# aevum
A CLI utility to calculate timer register values for STM32 microcontrollers. Given the desired length of the timer, aevum will calculate and spit out the prescaler and auto-reload register values.

## Usage
Run aevum with `./aevum --help` for more information.

When using aevum, you must always provide the desired period of time to run the timer. By default the units are in seconds. The program assumes that the prescaler and auto-reload registers are both 16-bit.

### Options
* `-h, --help`: Display help information
* `-c, --clock`: Specify the timer clock speed

### Examples
`aevum 10 -c 16000000`

## Installation
Precompiled binaries are not yet available. You can compile the project 
from source if your machine has [Rust](https://www.rust-lang.org/tools/install).
```
$ git clone https://github.com/dylanwishner/aevum.git
$ cd aevum
$ cargo build
```

## To-do List
- [ ] Support alternative time units (milliseconds, microseconds, etc.)
- [ ] Specify how many possibilities to calculate (or only output first match to reduce run time)
- [ ] Allow the default parameters to be changed in case you frequently work with the same MCU
- [ ] Better integer parsing to allow for 8,000,000 or 8_000_000 style integers
- [ ] Handle cases where the values exceed the 16-bit maximum (using clock dividers)