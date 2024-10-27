#![no_std]
#![no_main]

use panic_halt as _;

use embedded_hal::digital::v2::OutputPin;  // Import OutputPin trait

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Define output pins
    let mut pin13_led = pins.d13.into_output();
    let _ = pin13_led.set_low();

    let mut ds = pins.d2.into_output();       // Data pin for 74HC595
    let mut latch = pins.d4.into_output();    // Latch pin for 74HC595
    let mut clock = pins.d3.into_output();    // Clock pin for 74HC595

    // Ensure initial states are low
    let _ = ds.set_low();
    let _ = clock.set_low();
    let _ = latch.set_low();

    let mut daysWithoutIngesting = 9;

    // Segment patterns for digits 0-9 (assuming common cathode)
    const SEGMENT_PATTERNS: [u8; 10] = [
        0b00111111, 
        0b00000110,
        0b01011011,
        0b01001111,
        0b01100110,
        0b01101101,
        0b01111101,
        0b00000111,
        0b01111111,
        0b01101111,
    ];

    loop {
        // Indicate shift started
        let _ = pin13_led.set_high();
        // Bring latch low before shifting data
        let _ = latch.set_low();
        // Send the digit pattern
        shift_out(&mut ds, &mut clock, SEGMENT_PATTERNS[daysWithoutIngesting]);
        // Bring latch high to latch the data
        let _ = latch.set_high();
        // Indicate shift completed
        let _ = pin13_led.set_low();
        // Wait before displaying the next digit
        arduino_hal::delay_ms(1000);
    }
}

// Function to shift out a byte to the 74HC595
fn shift_out<DPIN, CPIN>(ds: &mut DPIN, clock: &mut CPIN, data: u8)
where DPIN: OutputPin, CPIN: OutputPin, {
    // Ensure clock starts low
    let _ = clock.set_low();

    for i in (0..8).rev() {
        // Set data pin based on the current bit
        if (data & (1 << i)) != 0 {
            let _ = ds.set_high();   // Ignoring Result
        } else {
            let _ = ds.set_low();    // Ignoring Result
        }
        // Pulse the clock pin to shift in this bit
        let _ = clock.set_high();   // Ignoring Result
        arduino_hal::delay_us(10);
        let _ = clock.set_low();    // Ignoring Result
    }
}
