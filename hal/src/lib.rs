#![no_std]

#[cfg(feature = "samd09d14a")]
pub extern crate atsamd09d14a;
#[cfg(feature = "samd09d14a")]
pub use atsamd09d14a as target_device;

#[cfg(feature = "samd10")]
pub extern crate atsamd10;
#[cfg(feature = "samd10")]
pub use atsamd10 as target_device;

#[cfg(feature = "samd11")]
pub extern crate atsamd11;
#[cfg(feature = "samd11")]
pub use atsamd11 as target_device;

extern crate cortex_m;
pub extern crate embedded_hal as hal;
extern crate nb;
extern crate void;

mod calibration;
pub mod clock;
pub mod delay;
pub mod gpio;
pub mod prelude;
pub mod sercom;
pub mod time;
// TODO:
// pub mod timer;
#[cfg(feature = "samd11")]
pub mod usb;
