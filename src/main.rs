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
    let mut days_without_ingesting = 0;
    let mut last_date_checked: Option<NaiveDate> = None;

    //RTC logic
    let scl = pins.a5.into_pull_up_input();
    let sda = pins.a4.into_pull_up_input();
    let i2c = arduino_hal::I2c::new(dp.TWI, sda, scl, 50000);

    let mut rtc = Ds1307::new(i2c);

    /* 
        This is for setting the time, only needs to run once, please comment after using 
         ------------ and do not include in production !!!!! --------------
        also set the time manually as constants

        let date = NaiveDate::from_ymd(2024, 11, 3);
        let time = NaiveTime::from_hms(4, 59, 40);
        rtc.set_datetime(&date.and_time(time)).unwrap();
    */

    // Segment patterns for digits 0-9 (assuming common cathode)
    const SEGMENT_PATTERNS: [u8; 10] = [
        0b00000000, 
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

    let reset_button = pins.d7.into_pull_up_input();


    loop {
        uwriteln!(&mut serial, "Test message: daysWithoutIngesting = {}\r", days_without_ingesting);

        let datetime = rtc.datetime().unwrap();
        let current_date = datetime.date();
        let current_hour = datetime.hour();

        if current_hour == reset_hour {
            if last_date_checked != Some(current_date) {
                days_without_ingesting = (days_without_ingesting + 1) % 10;
                last_date_checked = Some(current_date);
                uwriteln!(&mut serial, "Incremented days_without_ingesting to: {}\r", days_without_ingesting);
            }
        }

        if reset_button.is_low() {
            days_without_ingesting = 0;
            uwriteln!(&mut serial, "days_without_ingesting reset to 0 by button press.\r");
        }

        uwriteln!(&mut serial, "Current Date and Time: = {}\r", datetime.hour());

        // Indicate shift started
        let _ = pin13_led.set_high();
        // Bring latch low before shifting data
        let _ = latch.set_low();
        // Send the digit pattern
        shift_out(&mut ds, &mut clock, SEGMENT_PATTERNS[days_without_ingesting]);
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