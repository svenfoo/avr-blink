#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_nano_every as board;
use board::prelude::*;

#[board::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();

    let mut pins = board::Pins::new(dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);

    let mut led = pins.d13.into_output(&mut pins.ddr);
    let mut tx = pins.led_tx.into_output(&mut pins.ddr);
    let mut rx = pins.led_rx.into_output(&mut pins.ddr);

    led.set_high().void_unwrap();
    tx.set_high().void_unwrap();
    rx.set_low().void_unwrap();

    loop {
        tx.toggle().void_unwrap();
        rx.toggle().void_unwrap();

        board::delay_ms(100);
    }
}
