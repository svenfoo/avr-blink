#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_nano_every as board;
use board::prelude::*;

#[board::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();
    let mut pins = board::Pins::new(dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output(&mut pins.ddr);

    led.set_high().void_unwrap();

    loop {
        led.toggle().void_unwrap();
        board::delay_ms(100);
    }
}

