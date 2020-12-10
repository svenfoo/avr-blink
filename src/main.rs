#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_nano_every as board;
use board::prelude::*;

#[board::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();

    let portb = dp.PORTB.split();
    let mut tx0 = portb.pb4.into_output(&portb.ddr);
    let mut rx0 = portb.pb5.into_output(&portb.ddr);

    // Digital pin 13 is connected to the onboard LED
    // let mut pins = board::Pins::new(dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);
    // let mut led = pins.d13.into_output(&mut pins.ddr);

    tx0.set_high().void_unwrap();
    rx0.set_low().void_unwrap();

    loop {
        tx0.toggle().void_unwrap();
        rx0.toggle().void_unwrap();

        board::delay_ms(100);
    }
}
