#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate microbit;
extern crate panic_abort;

use cortex_m_rt::entry;
use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let mut gpio = p.GPIO.split();
        let mut delay = Delay::new(p.TIMER0);
        let mut pin = gpio.pin14.into_push_pull_output();
        let _ = gpio.pin6.into_push_pull_output();
        loop {
            pin.set_high();
            delay.delay_ms(1_000_u16);
            pin.set_low();
            delay.delay_ms(500_u16);
        }
    }
    panic!();
}
