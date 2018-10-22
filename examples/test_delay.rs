#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate microbit;
extern crate panic_semihosting;

use cortex_m_rt::entry;
use microbit::hal::delay::Timer;
use microbit::hal::prelude::*;
use microbit::nrf51::{TIMER0, TIMER1, TIMER2, RTC0, RTC1};
use microbit::hal::delay::TimerCounter;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {

        // Start the LFCLK
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

        let mut gpio = p.GPIO.split();

        let mut pin = gpio.pin14.into_push_pull_output();
        let _ = gpio.pin6.into_push_pull_output();

        // 32bits @ 1MHz = ~72 minutes
        let mut delay0 = Timer::<TIMER0>::new(p.TIMER0, 4);
        // 16bits @ 31.25kHz = ~2 seconds
        let mut delay1 = Timer::<TIMER1>::new(p.TIMER1, 9);
        // 16bits @ 31.25kHz = ~2 seconds
        let mut delay2 = Timer::<TIMER2>::new(p.TIMER2, 9);

        // 24bits @ 32.768kHz = 512 seconds
        let mut delay3 = Timer::<RTC0>::new(p.RTC0, 0);
        // 24bits @ 32.768kHz = 512 seconds
        let mut delay4 = Timer::<RTC1>::new(p.RTC1, 0);

        const LONG: u16 = 800;
        const SHORT: u16 = 400;

        for _ in 0..2 {    
            
            for _ in 0..2 {
                pin.set_high();
                delay0.delay_ms(LONG);
                pin.set_low();
                delay0.delay_ms(SHORT);
            }

            for _ in 0..2 {
                pin.set_high();
                delay1.delay_ms(LONG);
                pin.set_low();
                delay1.delay_ms(SHORT);
            }

            for _ in 0..2 {
                pin.set_high();
                delay2.delay_ms(LONG);
                pin.set_low();
                delay2.delay_ms(SHORT);
            }

            for _ in 0..2 {
                pin.set_high();
                delay3.delay_ms(LONG);
                pin.set_low();
                delay3.delay_ms(SHORT);
            }

            for _ in 0..2 {
                pin.set_high();
                delay4.delay_ms(LONG);
                pin.set_low();
                delay4.delay_ms(SHORT);
            }
        }
    }
    
    panic!("FIN");
}
