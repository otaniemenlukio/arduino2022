#![no_std]
#![no_main]
#![allow(non_snake_case)]

use panic_halt as _;
use ufmt::*;
use embedded_hal::serial::Read;
use arduino_hal::Pins; 
use arduino_hal::usart::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);    

    let mut serial = arduino_hal::default_serial!(dp, pins, 100000);
    let mut serialList :[u8;50] = [0;50];
    uwriteln!(&mut serial, "Welcome to the cum zone");
    let mut serialCount :usize =0;
    loop {
        let l = nb::block!(serial.read()).unwrap();
        match l{
            0=>{
                serialCount=0;
                let print: &str = core::str::from_utf8(&serialList).unwrap();
                uwriteln!(&mut serial,"Data: {}", print);
            },
            _ => {
                serialList[serialCount]=l;
                serialCount+=1;
            }
        }

        
    }
}

