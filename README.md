Hardware Control on the Raspberry Pi with the Rust Language
===========================================================

A work in progress...

Current plan...


1. Getting Started: intro to the Raspberry Pi, building a cross-compiler and setting up the development environment, parts for hardware tinkering. Rust learning: program entry point, the prelude
2. Getting Physical: simple hardware control blinking LEDs and reacting to button presses via GPIO. Rust learning: I/O, error handling, enums, structs, traits, splitting a program into cooperating tasks, separating I/O from coordination
3. Getting Fast: map GPIO device registers into memory for direct access, bit-bang a simple radio protocol. Rust learning: memory mapping, unsafe blocks, use Rust's type system to ensure safe access.
4. Getting Connected: using the I2C bus to read from analogue-to-digital converters. Rust learning: using C functions and data structures. Native vs green runtimes.
5. Getting Mobile: motor control or control of an RC vehicle.

This plan may change...
