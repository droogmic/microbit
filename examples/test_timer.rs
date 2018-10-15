#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate microbit;
extern crate panic_abort;

use core::time::Duration;
use cortex_m_rt::entry;
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;
use microbit::nb::block;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let mut gpio = p.GPIO.split();
        let mut timer = Timer::new(p.TIMER0);
        let mut pin = gpio.pin14.into_push_pull_output();
        let _ = gpio.pin6.into_push_pull_output();
        timer.start(Duration::from_millis(500));
        loop {
            pin.set_high();
            block!(timer.wait());
            block!(timer.wait());
            pin.set_low();
            block!(timer.wait());
        }
    }
    panic!();
}
