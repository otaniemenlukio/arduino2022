#![no_std]
#![no_main]

use panic_halt as _;
use ufmt::*;
use embedded_hal::serial::Read;


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();
    
    uwriteln!(&mut serial, "Welcome to the cum zone");
    
    loop {
        let line = nb::block!(serial.read()).unwrap();
        if line == 0{
            led.toggle();
        }
    }
}
