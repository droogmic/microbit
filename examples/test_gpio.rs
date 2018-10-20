#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate microbit;
extern crate panic_abort;

use cortex_m_rt::entry;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let mut gpio = p.GPIO.split();
        let mut pin = gpio.pin14.into_push_pull_output();
        let _ = gpio.pin6.into_push_pull_output();
        loop {
            pin.set_high();
            if cfg!(debug_assertions) {
                nop_delay(200_000);
            } else {
                nop_delay(1_000_000);
            }
            pin.set_low();
            if cfg!(debug_assertions) {
                nop_delay(10_000);
            } else {
                nop_delay(500_000);
            }
        }
    }
    panic!();
}

fn nop_delay(n: u32) {
    for _ in 0..n {
        cortex_m::asm::nop();
    }
}
