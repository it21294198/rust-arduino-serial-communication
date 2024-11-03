#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _; // Halts on panic, useful for debugging

#[arduino_hal::entry]
fn main() -> ! {
    // Initialize peripherals
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    
    // Configure the LED pin (assuming it's on D13)
    let mut led = pins.d13.into_output();
    
    // Set up serial interface with a baud rate of 57600
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 57600);
    
    // Print a message to indicate startup
    ufmt::uwriteln!(&mut serial, "Serial communication started!\r").unwrap();
    
    // Main loop
    loop {
        // Read a byte if available
        if let Ok(byte) = serial.read() {
            // Check if the byte is the ASCII code for 'a'
            if byte == b'a' {
                led.set_high();
            } else {
                led.set_low();
            }
            
            // Print the received byte as a character
            ufmt::uwriteln!(&mut serial, "Received: {}\r", byte as char).unwrap();
        }
    }
}

