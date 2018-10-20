#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate microbit;
extern crate panic_abort;

use cortex_m_rt::entry;
use microbit::hal::delay::Timer;
use microbit::hal::prelude::*;
use microbit::nrf51::{TIMER0, TIMER1, TIMER2};

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {

        let mut gpio = p.GPIO.split();

        let mut pin = gpio.pin14.into_push_pull_output();
        let _ = gpio.pin6.into_push_pull_output();

        // 24bits @ 1MHz = ~72 minutes
        let mut delay0 = Timer::<TIMER0>::new(p.TIMER0, 4);
        // 16bits @ 31.25kHz = ~2 seconds
        let mut delay1 = Timer::<TIMER1>::new(p.TIMER1, 9);
        // 16bits @ 31.25kHz = ~2 seconds
        let mut delay2 = Timer::<TIMER2>::new(p.TIMER2, 9);

        for _ in 0..2 {    
            
            for _ in 0..2 {
                pin.set_high();
                delay0.delay_ms(1_000_u32);
                pin.set_low();
                delay0.delay_ms(500_u32);
            }

            for _ in 0..2 {
                pin.set_high();
                delay1.delay_ms(1_000_u16);
                pin.set_low();
                delay1.delay_ms(500_u16);
            }

            for _ in 0..2 {
                pin.set_high();
                delay2.delay_ms(1_000_u16);
                pin.set_low();
                delay2.delay_ms(500_u16);
            }
        }
    }
    
    panic!();
}
