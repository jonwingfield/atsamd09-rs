#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate embedded_hal;
extern crate embedded_nrf24l01;
extern crate hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_abort;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

use hal::gpio;
use hal::gpio::{Floating, Input, Output, Port, PushPull};
use hal::sercom::{PadPin, SPIMaster0};
use hal::time::Hertz;

use hal::atsamd09d14a::PM;
use hal::atsamd09d14a::{CorePeripherals, Peripherals, SERCOM0};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;

use cortex_m::asm;
use embedded_hal::blocking::spi::Transfer as SPITransfer;
use embedded_hal::digital::OutputPin;
use embedded_nrf24l01::{Configuration, CrcMode, NRF24L01, PAControl};

#[cfg(feature = "use_semihosting")]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use cortex_m_semihosting::hio;
            use core::fmt::Write;
            let mut stdout = hio::hstdout().unwrap();
            writeln!(stdout, $($arg)*).ok();
        }
    };
}

#[cfg(not(feature = "use_semihosting"))]
macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

struct GarageDoorController<CE: OutputPin, CSN: OutputPin, SPI: SPITransfer<u8>> {
    nrf24: NRF24L01<CE, CSN, SPI>,
}

use rt::ExceptionFrame;

entry!(main);

pub fn spi_master<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom0: SERCOM0,
    pm: &mut PM,
    sck: gpio::Pa11<Input<Floating>>,
    mosi: gpio::Pa10<Input<Floating>>,
    miso: gpio::Pa14<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster0 {
    let gclk0 = clocks.gclk0();
    SPIMaster0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        bus_speed.into(),
        hal::hal::spi::Mode {
            phase: hal::hal::spi::Phase::CaptureOnFirstTransition,
            polarity: hal::hal::spi::Polarity::IdleLow,
        },
        sercom0,
        pm,
        hal::sercom::SPI0Pinout::Dipo0Dopo1 {
            miso: miso.into_pad(port),
            mosi: mosi.into_pad(port),
            sck: sck.into_pad(port),
        },
    )
}

fn panic_r<E, T>(_e: E) -> T {
    panic!("");
}

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

    let spi_master = spi_master(
        &mut clocks,
        1.mhz(),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        pins.pa11,
        pins.pa10,
        pins.pa14,
        &mut pins.port,
    );

    let ce = pins.pa8.into_push_pull_output(&mut pins.port);
    let mut csn = pins.pa9.into_push_pull_output(&mut pins.port);
    csn.set_high();

    let mut nrf24 = NRF24L01::new(ce, csn, spi_master).unwrap_or_else(|e| panic!("NRF::new"));

    nrf24
        .set_rf(embedded_nrf24l01::DataRate::R2Mbps, PAControl::PAMin)
        .unwrap_or_else(panic_r);
    nrf24.set_frequency(74).unwrap_or_else(panic_r);
    nrf24.set_tx_addr(b"serv1").unwrap_or_else(panic_r);
    nrf24.set_rx_addr(1, b"clie1").unwrap_or_else(panic_r);

    nrf24
        .set_crc(Some(CrcMode::TwoBytes))
        .unwrap_or_else(panic_r);
    nrf24
        .set_pipes_rx_lengths(&[Some(32), Some(32), Some(2), Some(2), Some(2), Some(2)])
        .unwrap_or_else(panic_r);
    nrf24
        .set_pipes_rx_enable(&[true, true, false, false, false, false])
        .unwrap_or_else(panic_r);
    nrf24
        .set_auto_ack(&[true, true, true, true, true, true])
        .unwrap_or_else(panic_r);
    nrf24.set_auto_retransmit(15, 15).unwrap_or_else(panic_r);
    nrf24.flush_tx().unwrap_or_else(panic_r);
    nrf24.flush_rx().unwrap_or_else(panic_r);

    // dbgprint!("Channel {}\n", nrf24.get_frequency().unwrap());
    // dbgprint!("AutoAck {:?}\n", nrf24.get_auto_ack().unwrap()[1]);
    // dbgprint!("Register {:?}\n", nrf24.get_address_width().unwrap());

    let mut rx = nrf24.rx().unwrap_or_else(|e| panic!(""));

    let mut red_led = pins.pa27.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    delay.delay_us(130u8);

    loop {
        if let Some(_) = rx.can_read().unwrap_or_else(panic_r) {
            let mut payload = rx.read().unwrap_or_else(panic_r);
            let data = payload[0];

            let mut standby = rx.standby();
            let mut tx = standby.tx().unwrap_or_else(panic_r);
            delay.delay_ms(1u8);
            // let mut payload: [u8; 4] = [1, 0, 1, 0];

            let result = tx.send_sync(&payload).unwrap_or_else(panic_r);
            if !result {
                red_led.set_high();
            } else {
                red_led.set_low();
            }

            rx = tx.standby()
                .unwrap_or_else(panic_r)
                .rx()
                .unwrap_or_else(panic_r);
            delay.delay_us(130u8);
            // delay.delay_ms(200u8);
        }

        // let mut standby = rx.standby();
        // let mut tx = standby.tx().unwrap_or_else(panic_r);
        // let mut payload: [u8; 4] = [1, 0, 1, 0];

        // let result = tx.send_sync(&payload).unwrap_or_else(panic_r);
        // if !result {
        //     red_led.set_high();
        // }

        // rx = tx.standby()
        //     .unwrap_or_else(panic_r)
        //     .rx()
        //     .unwrap_or_else(panic_r);
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
