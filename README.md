Hardware Control on the Raspberry Pi with the Rust Language
===========================================================

A work in progress...

Introduction
------------

1. hello.rs

    Hello world - to prove the cross-compiler and deployment to the Pi is working.

2. blink.rs

    Blink an LED via GPIO.  The hello world of physical computing!

3. button.rs

    Read the state of a pushbutton via GPIO.

4. blink-button.rs

    Press the button to make the LED blink. Naive method - the button is not very 
    responsive. Demonstrates the need for interrupt-driven GPIO and concurrency

5. blink-button-tasks.rs

    Use Rust's tasks to make the blink-button program respond rapidly to the
    push-button.

6. fast-gpio.rs

    Memory map the GPIO device into process memory and rely on Rust's type system
    to stop bugs smashing kernel memory.  Bit-bang a simple protocol.


