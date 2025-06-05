avr-modern-rust
===============

Rust project for the modern AVR CPUs like _attiny402_, _attiny1614_, _avr128db28_ which all share compatible peripherals).

It implements parts of the embedded_hal and embedded_io traits.

The example code uses the serial USART and flashes leds nad some dummy calculations.

Also contains sleep code for the RTC realtime counter.

Credits to the author of https://github.com/Rahix/avr-device for his Rust crates and examples!

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Flash with the UPDI tool of your choice (see ./flash.sh)

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
