
=============================================================================
Comment to myself "KITER": 

The command to flash the pro-nano is:

`avrdude -p m328p -c arduino -b 57600 -P /dev/cu.usbserial-0001 -U flash:w:target/avr-atmega328p/debug/vitamins-uno.elf`

Remember to change /dev/--- to the actual port

check the port by using:
`ls /dev/cu.*`
   
===================================================================================

to change the RAVEDUDE port to know which device to flash use in the terminal: 

`export RAVEDUDE_PORT=/dev//dev/cu.usbmodem10` <- you can find the device name using: `ls /dev/cu.*`

================================================================================




vitamins-uno
============

Rust project for the _Arduino Uno_.

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

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

