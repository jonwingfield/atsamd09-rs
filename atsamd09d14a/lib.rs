#![doc = "Peripheral access API for ATSAMD09C13A microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(untagged_unions)]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn PM_INTREQ();
    fn SYSCTRL_INTREQ();
    fn WDT_INTREQ();
    fn RTC_INTREQ();
    fn EIC_INTREQ();
    fn NVMCTRL_INTREQ();
    fn DMAC_INTREQ();
    fn EVSYS_INTREQ();
    fn SERCOM0_INTREQ();
    fn SERCOM1_INTREQ();
    fn TC1_INTREQ();
    fn TC2_INTREQ();
    fn ADC_INTREQ();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 16] = [
    Vector {
        _handler: PM_INTREQ,
    },
    Vector {
        _handler: SYSCTRL_INTREQ,
    },
    Vector {
        _handler: WDT_INTREQ,
    },
    Vector {
        _handler: RTC_INTREQ,
    },
    Vector {
        _handler: EIC_INTREQ,
    },
    Vector {
        _handler: NVMCTRL_INTREQ,
    },
    Vector {
        _handler: DMAC_INTREQ,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: EVSYS_INTREQ,
    },
    Vector {
        _handler: SERCOM0_INTREQ,
    },
    Vector {
        _handler: SERCOM1_INTREQ,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: TC1_INTREQ,
    },
    Vector {
        _handler: TC2_INTREQ,
    },
    Vector {
        _handler: ADC_INTREQ,
    },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ($Name:ident, $handler:path,state: $State:ty = $initial_state:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ($Name:ident, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - PM_INTREQ"]
    PM_INTREQ,
    #[doc = "1 - SYSCTRL_INTREQ"]
    SYSCTRL_INTREQ,
    #[doc = "2 - WDT_INTREQ"]
    WDT_INTREQ,
    #[doc = "3 - RTC_INTREQ"]
    RTC_INTREQ,
    #[doc = "4 - EIC_INTREQ"]
    EIC_INTREQ,
    #[doc = "5 - NVMCTRL_INTREQ"]
    NVMCTRL_INTREQ,
    #[doc = "6 - DMAC_INTREQ"]
    DMAC_INTREQ,
    #[doc = "8 - EVSYS_INTREQ"]
    EVSYS_INTREQ,
    #[doc = "9 - SERCOM0_INTREQ"]
    SERCOM0_INTREQ,
    #[doc = "10 - SERCOM1_INTREQ"]
    SERCOM1_INTREQ,
    #[doc = "13 - TC1_INTREQ"]
    TC1_INTREQ,
    #[doc = "14 - TC2_INTREQ"]
    TC2_INTREQ,
    #[doc = "15 - ADC_INTREQ"]
    ADC_INTREQ,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PM_INTREQ => 0,
            Interrupt::SYSCTRL_INTREQ => 1,
            Interrupt::WDT_INTREQ => 2,
            Interrupt::RTC_INTREQ => 3,
            Interrupt::EIC_INTREQ => 4,
            Interrupt::NVMCTRL_INTREQ => 5,
            Interrupt::DMAC_INTREQ => 6,
            Interrupt::EVSYS_INTREQ => 8,
            Interrupt::SERCOM0_INTREQ => 9,
            Interrupt::SERCOM1_INTREQ => 10,
            Interrupt::TC1_INTREQ => 13,
            Interrupt::TC2_INTREQ => 14,
            Interrupt::ADC_INTREQ => 15,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Analog Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1107304448 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog Digital Converter"]
pub mod adc;
#[doc = "Direct Memory Access Controller"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmac::RegisterBlock {
        1090537472 as *const _
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    fn deref(&self) -> &dmac::RegisterBlock {
        unsafe { &*DMAC::ptr() }
    }
}
#[doc = "Direct Memory Access Controller"]
pub mod dmac;
#[doc = "Device Service Unit"]
pub struct DSU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSU {}
impl DSU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dsu::RegisterBlock {
        1090527232 as *const _
    }
}
impl Deref for DSU {
    type Target = dsu::RegisterBlock;
    fn deref(&self) -> &dsu::RegisterBlock {
        unsafe { &*DSU::ptr() }
    }
}
#[doc = "Device Service Unit"]
pub mod dsu;
#[doc = "External Interrupt Controller"]
pub struct EIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EIC {}
impl EIC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eic::RegisterBlock {
        1073747968 as *const _
    }
}
impl Deref for EIC {
    type Target = eic::RegisterBlock;
    fn deref(&self) -> &eic::RegisterBlock {
        unsafe { &*EIC::ptr() }
    }
}
#[doc = "External Interrupt Controller"]
pub mod eic;
#[doc = "Event System Interface"]
pub struct EVSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVSYS {}
impl EVSYS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const evsys::RegisterBlock {
        1107297280 as *const _
    }
}
impl Deref for EVSYS {
    type Target = evsys::RegisterBlock;
    fn deref(&self) -> &evsys::RegisterBlock {
        unsafe { &*EVSYS::ptr() }
    }
}
#[doc = "Event System Interface"]
pub mod evsys;
#[doc = "Generic Clock Generator"]
pub struct GCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GCLK {}
impl GCLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gclk::RegisterBlock {
        1073744896 as *const _
    }
}
impl Deref for GCLK {
    type Target = gclk::RegisterBlock;
    fn deref(&self) -> &gclk::RegisterBlock {
        unsafe { &*GCLK::ptr() }
    }
}
#[doc = "Generic Clock Generator"]
pub mod gclk;
#[doc = "HSB Matrix"]
pub struct HMATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HMATRIX {}
impl HMATRIX {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hmatrix::RegisterBlock {
        1090547712 as *const _
    }
}
impl Deref for HMATRIX {
    type Target = hmatrix::RegisterBlock;
    fn deref(&self) -> &hmatrix::RegisterBlock {
        unsafe { &*HMATRIX::ptr() }
    }
}
#[doc = "HSB Matrix"]
pub mod hmatrix;
#[doc = "Cortex-M0+ Micro-Trace Buffer"]
pub struct MTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB {}
impl MTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mtb::RegisterBlock {
        1090543616 as *const _
    }
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    fn deref(&self) -> &mtb::RegisterBlock {
        unsafe { &*MTB::ptr() }
    }
}
#[doc = "Cortex-M0+ Micro-Trace Buffer"]
pub mod mtb;
#[doc = "Non-Volatile Memory Controller"]
pub struct NVMCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMCTRL {}
impl NVMCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvmctrl::RegisterBlock {
        1090535424 as *const _
    }
}
impl Deref for NVMCTRL {
    type Target = nvmctrl::RegisterBlock;
    fn deref(&self) -> &nvmctrl::RegisterBlock {
        unsafe { &*NVMCTRL::ptr() }
    }
}
#[doc = "Non-Volatile Memory Controller"]
pub mod nvmctrl;
#[doc = "Peripheral Access Controller 0"]
pub struct PAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC0 {}
impl PAC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pac0::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for PAC0 {
    type Target = pac0::RegisterBlock;
    fn deref(&self) -> &pac0::RegisterBlock {
        unsafe { &*PAC0::ptr() }
    }
}
#[doc = "Peripheral Access Controller 0"]
pub mod pac0;
#[doc = "Peripheral Access Controller 1"]
pub struct PAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC1 {}
impl PAC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pac0::RegisterBlock {
        1090519040 as *const _
    }
}
impl Deref for PAC1 {
    type Target = pac0::RegisterBlock;
    fn deref(&self) -> &pac0::RegisterBlock {
        unsafe { &*PAC1::ptr() }
    }
}
#[doc = "Peripheral Access Controller 2"]
pub struct PAC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PAC2 {}
impl PAC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pac0::RegisterBlock {
        1107296256 as *const _
    }
}
impl Deref for PAC2 {
    type Target = pac0::RegisterBlock;
    fn deref(&self) -> &pac0::RegisterBlock {
        unsafe { &*PAC2::ptr() }
    }
}
#[doc = "Power Manager"]
pub struct PM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PM {}
impl PM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pm::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for PM {
    type Target = pm::RegisterBlock;
    fn deref(&self) -> &pm::RegisterBlock {
        unsafe { &*PM::ptr() }
    }
}
#[doc = "Power Manager"]
pub mod pm;
#[doc = "Port Module"]
pub struct PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT {}
impl PORT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port::RegisterBlock {
        1090536448 as *const _
    }
}
impl Deref for PORT {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &port::RegisterBlock {
        unsafe { &*PORT::ptr() }
    }
}
#[doc = "Port Module"]
pub mod port;
#[doc = "Port Module (IOBUS)"]
pub struct PORT_IOBUS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_IOBUS {}
impl PORT_IOBUS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port::RegisterBlock {
        1610612736 as *const _
    }
}
impl Deref for PORT_IOBUS {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &port::RegisterBlock {
        unsafe { &*PORT_IOBUS::ptr() }
    }
}
#[doc = "Real-Time Counter"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073746944 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-Time Counter"]
pub mod rtc;
#[doc = "Serial Communication Interface 0"]
pub struct SERCOM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM0 {}
impl SERCOM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sercom0::RegisterBlock {
        1107298304 as *const _
    }
}
impl Deref for SERCOM0 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &sercom0::RegisterBlock {
        unsafe { &*SERCOM0::ptr() }
    }
}
#[doc = "Serial Communication Interface 0"]
pub mod sercom0;
#[doc = "Serial Communication Interface 1"]
pub struct SERCOM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SERCOM1 {}
impl SERCOM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sercom0::RegisterBlock {
        1107299328 as *const _
    }
}
impl Deref for SERCOM1 {
    type Target = sercom0::RegisterBlock;
    fn deref(&self) -> &sercom0::RegisterBlock {
        unsafe { &*SERCOM1::ptr() }
    }
}
#[doc = "System Control"]
pub struct SYSCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCTRL {}
impl SYSCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sysctrl::RegisterBlock {
        1073743872 as *const _
    }
}
impl Deref for SYSCTRL {
    type Target = sysctrl::RegisterBlock;
    fn deref(&self) -> &sysctrl::RegisterBlock {
        unsafe { &*SYSCTRL::ptr() }
    }
}
#[doc = "System Control"]
pub mod sysctrl;
#[doc = "Basic Timer Counter 1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc1::RegisterBlock {
        1107302400 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc1::RegisterBlock;
    fn deref(&self) -> &tc1::RegisterBlock {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "Basic Timer Counter 1"]
pub mod tc1;
#[doc = "Basic Timer Counter 2"]
pub struct TC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC2 {}
impl TC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc1::RegisterBlock {
        1107303424 as *const _
    }
}
impl Deref for TC2 {
    type Target = tc1::RegisterBlock;
    fn deref(&self) -> &tc1::RegisterBlock {
        unsafe { &*TC2::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "DSU"]
    pub DSU: DSU,
    #[doc = "EIC"]
    pub EIC: EIC,
    #[doc = "EVSYS"]
    pub EVSYS: EVSYS,
    #[doc = "GCLK"]
    pub GCLK: GCLK,
    #[doc = "HMATRIX"]
    pub HMATRIX: HMATRIX,
    #[doc = "MTB"]
    pub MTB: MTB,
    #[doc = "NVMCTRL"]
    pub NVMCTRL: NVMCTRL,
    #[doc = "PAC0"]
    pub PAC0: PAC0,
    #[doc = "PAC1"]
    pub PAC1: PAC1,
    #[doc = "PAC2"]
    pub PAC2: PAC2,
    #[doc = "PM"]
    pub PM: PM,
    #[doc = "PORT"]
    pub PORT: PORT,
    #[doc = "PORT_IOBUS"]
    pub PORT_IOBUS: PORT_IOBUS,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SERCOM0"]
    pub SERCOM0: SERCOM0,
    #[doc = "SERCOM1"]
    pub SERCOM1: SERCOM1,
    #[doc = "SYSCTRL"]
    pub SYSCTRL: SYSCTRL,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TC2"]
    pub TC2: TC2,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ADC: ADC {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            DSU: DSU {
                _marker: PhantomData,
            },
            EIC: EIC {
                _marker: PhantomData,
            },
            EVSYS: EVSYS {
                _marker: PhantomData,
            },
            GCLK: GCLK {
                _marker: PhantomData,
            },
            HMATRIX: HMATRIX {
                _marker: PhantomData,
            },
            MTB: MTB {
                _marker: PhantomData,
            },
            NVMCTRL: NVMCTRL {
                _marker: PhantomData,
            },
            PAC0: PAC0 {
                _marker: PhantomData,
            },
            PAC1: PAC1 {
                _marker: PhantomData,
            },
            PAC2: PAC2 {
                _marker: PhantomData,
            },
            PM: PM {
                _marker: PhantomData,
            },
            PORT: PORT {
                _marker: PhantomData,
            },
            PORT_IOBUS: PORT_IOBUS {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SERCOM0: SERCOM0 {
                _marker: PhantomData,
            },
            SERCOM1: SERCOM1 {
                _marker: PhantomData,
            },
            SYSCTRL: SYSCTRL {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TC2: TC2 {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
