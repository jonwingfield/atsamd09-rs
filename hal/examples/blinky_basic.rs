#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

extern crate cortex_m;
#[cfg(feature = "use_semihosting")]
extern crate cortex_m_semihosting;
extern crate embedded_hal;
extern crate hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_abort;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

use hal::atsamd09d14a::{CorePeripherals, Peripherals};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;

use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::new(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = peripherals.PORT.split();
    let mut red_led = pins.pa27.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        delay.delay_ms(250u8);
        delay.delay_ms(250u8);
        red_led.set_high();
        delay.delay_ms(250u8);
        delay.delay_ms(250u8);
        red_led.set_low();
    }
}

// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("");
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("");
    // fmt should be avoided if possible since it greatly bloats the binary.
    // and forget debug formatting!
    // panic!("unhandled exception (IRQn={})", irqn);
}
