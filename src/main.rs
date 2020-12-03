#![feature(llvm_asm)]
#![no_std]
#![no_main]

use ruduino::cores::atmega328p as avr_core;
use ruduino::Register;

use avr_core::{DDRB, PORTB};

#[no_mangle]
pub extern "C" fn main() {
    // Set all PORTB pins up as outputs
    DDRB::set_mask_raw(0xFFu8);

    loop {
        PORTB::set_mask_raw(0xFF);
        delay(40);
        PORTB::unset_mask_raw(0xFF);
        delay(80);
    }
}

/// A small busy loop.
fn delay(usec: u32) {
    let cycles = usec * 1000;
    for _ in 0..cycles {
        unsafe { llvm_asm!("" :::: "volatile") }
    }
}
