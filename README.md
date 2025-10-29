# 24 bit integer arithmetic for AVR

This library provides a 24-bit signed integer type, `Int24`, for Rust.
It is designed for use on AVR microcontrollers.

For more information see the [crate README](avr-int24/README.md).

## Unit testing

There are two ways to unit test the `avr-int24` crate:

1. Run the `cargo test` from the `avr-int24` crate.
2. Build the native test runner and run it on an AtMega microcontroller.

### Cargo test

The `cargo test` unit test is just a very basic smoke test.
It runs the whole test suite with a generic backend instead of the actual AVR assembly implementation.
Therefore, this test can only catch problems in non-AVR specific code.

### Native test on hardware

This test builds a test application for the AVR target hardware and runs this test on an actual AVR microcontroller.

The required test hardware consists of:

- An `AtMega328p` microcontroller connected to a 5V power supply.
- An `avrisp2` ISP programmer to load the test program.
- A serial connection (direction AVR to PC only).

The microcontroller runs with internal 8 MHz clock.
Therefore, the only peripheral that needs to be connected to the microcontroller is the serial connection, for example via some kind of UART-TTL to USB converter cable.

To build the test program, run the build command in the main directory of this git repository:

```sh
make
```

Then connect the `avrisp2` programmer to the ISP port of the microcontroller and run the following command to write the fuse bits:

```sh
make isp-fuses
```

Then connect the serial cable to your PC and run the `console.py` script from the git repository.
This script will wait for messages from the microcontroller and display them on screen.

```sh
./console.py /dev/ttyUSB0
```

Then flash the test program to the microcontroller:

```sh
make isp-flash
```

After successful flashing the test will immediately begin executing and the results will be shown in the `console.py` output.
This is a shortened example output:

```
Begin tests
Begin: conv_i16
line 25: Ok
line 29: Ok
line 33: Ok
line 37: Ok
  <snip>
Done!
```

If you see `Done!` then all tests finished successfully.
