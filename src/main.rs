#![no_std]
#![no_main]

use panic_halt as _;

use ufmt::uwriteln;

use embedded_hal::digital::v2::OutputPin;  // Import OutputPin trait

// Allow NaiveDate and NaiveTime, because we use it to set the time when we run the first time. 
#[allow(unused_imports)]
use ds1307::{DateTimeAccess, Ds1307, NaiveDate, NaiveTime, Timelike};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600); // Set baud rate as required

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

    let reset_hour = 5;
    let mut days_without_ingesting = 1; // Start with 1 to display "1" initially
    let mut last_date_checked: Option<NaiveDate> = None;

    // RTC logic
    let scl = pins.a5.into_pull_up_input();
    let sda = pins.a4.into_pull_up_input();
    let i2c = arduino_hal::I2c::new(dp.TWI, sda, scl, 50000);

    let mut rtc = Ds1307::new(i2c);

    /* 
        This is for setting the time, only needs to run once, please comment after using 
         ------------ and do not include in production !!!!! --------------
        also set the time manually as constants
        let date = NaiveDate::from_ymd(2025, 5, 30);
        let time = NaiveTime::from_hms(10, 16, 59);
        rtc.set_datetime(&date.and_time(time)).unwrap();
    */

    // Segment patterns for digits 0-9 (assuming common cathode)
    const SEGMENT_PATTERNS: [u8; 10] = [
        0b00000000, // 0: all segments off
        0b00000110, // 1
        0b01011011, // 2
        0b01001111, // 3
        0b01100110, // 4
        0b01101101, // 5
        0b01111101, // 6
        0b00000111, // 7
        0b01111111, // 8
        0b01101111, // 9
    ];

    let reset_button = pins.d7.into_pull_up_input();
    let test_button = pins.d8.into_pull_up_input();

    // Variables to track button states and last displayed digit
    let mut prev_reset_pressed = false;
    let mut prev_test_pressed = false;
    let mut last_displayed = 255; // Invalid initial value to force first update
    

    loop {
        let datetime = rtc.datetime().unwrap();
        let current_date = datetime.date();
        let current_hour = datetime.hour();

        uwriteln!(&mut serial, "current time: {}:{}:{}, date: {}\r", current_hour, datetime.minute(), datetime.second());

        // Auto-increment at reset hour
        if current_hour == reset_hour {
            if last_date_checked != Some(current_date) {
                days_without_ingesting = (days_without_ingesting + 1) % 10;
                last_date_checked = Some(current_date);
                uwriteln!(&mut serial, "Incremented days_without_ingesting to: {}\r", days_without_ingesting);
            }
        }

        // Read current button states
        let reset_pressed = reset_button.is_low();
        let test_pressed = test_button.is_low();

        // Reset button: trigger only on press (high to low transition)
        if !prev_reset_pressed && reset_pressed {
            days_without_ingesting = 0;
            uwriteln!(&mut serial, "days_without_ingesting reset to 0 by button press.\r");
            arduino_hal::delay_ms(100); // Longer debounce delay
        }

        // Test button: trigger only on press (high to low transition)
        if !prev_test_pressed && test_pressed {
            days_without_ingesting = (days_without_ingesting + 1) % 10;
            uwriteln!(&mut serial, "days_without_ingesting incremented to: {}\r", days_without_ingesting);
            arduino_hal::delay_ms(100); // Longer debounce delay
        }

        // Update previous button states
        prev_reset_pressed = reset_pressed;
        prev_test_pressed = test_pressed;

        // Update display only if the value has changed
        if days_without_ingesting != last_displayed {
            uwriteln!(&mut serial, "Updating display to: {}\r", days_without_ingesting);
            last_displayed = days_without_ingesting;
            let _ = latch.set_low();
            shift_out(&mut ds, &mut clock, SEGMENT_PATTERNS[days_without_ingesting]);
            let _ = latch.set_high();
        }

        arduino_hal::delay_ms(150);
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