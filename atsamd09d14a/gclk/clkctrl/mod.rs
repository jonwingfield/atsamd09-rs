#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CLKCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `ID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDR {
    #[doc = "DFLL48"]
    DFLL48,
    #[doc = "FDPLL"]
    FDPLL,
    #[doc = "FDPLL32K"]
    FDPLL32K,
    #[doc = "WDT"]
    WDT,
    #[doc = "RTC"]
    RTC,
    #[doc = "EIC"]
    EIC,
    #[doc = "EVSYS_0"]
    EVSYS_0,
    #[doc = "EVSYS_1"]
    EVSYS_1,
    #[doc = "EVSYS_2"]
    EVSYS_2,
    #[doc = "EVSYS_3"]
    EVSYS_3,
    #[doc = "EVSYS_4"]
    EVSYS_4,
    #[doc = "EVSYS_5"]
    EVSYS_5,
    #[doc = "SERCOMX_SLOW"]
    SERCOMX_SLOW,
    #[doc = "SERCOM0_CORE"]
    SERCOM0_CORE,
    #[doc = "SERCOM1_CORE"]
    SERCOM1_CORE,
    #[doc = "SERCOM2_CORE"]
    SERCOM2_CORE,
    #[doc = "TC1_TC2"]
    TC1_TC2,
    #[doc = "ADC"]
    ADC,
    #[doc = "AC_DIG"]
    AC_DIG,
    #[doc = "AC_ANA"]
    AC_ANA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDR::DFLL48 => 0,
            IDR::FDPLL => 1,
            IDR::FDPLL32K => 2,
            IDR::WDT => 3,
            IDR::RTC => 4,
            IDR::EIC => 5,
            IDR::EVSYS_0 => 7,
            IDR::EVSYS_1 => 8,
            IDR::EVSYS_2 => 9,
            IDR::EVSYS_3 => 10,
            IDR::EVSYS_4 => 11,
            IDR::EVSYS_5 => 12,
            IDR::SERCOMX_SLOW => 13,
            IDR::SERCOM0_CORE => 14,
            IDR::SERCOM1_CORE => 15,
            IDR::SERCOM2_CORE => 16,
            IDR::TC1_TC2 => 18,
            IDR::ADC => 19,
            IDR::AC_DIG => 20,
            IDR::AC_ANA => 21,
            IDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDR {
        match value {
            0 => IDR::DFLL48,
            1 => IDR::FDPLL,
            2 => IDR::FDPLL32K,
            3 => IDR::WDT,
            4 => IDR::RTC,
            5 => IDR::EIC,
            7 => IDR::EVSYS_0,
            8 => IDR::EVSYS_1,
            9 => IDR::EVSYS_2,
            10 => IDR::EVSYS_3,
            11 => IDR::EVSYS_4,
            12 => IDR::EVSYS_5,
            13 => IDR::SERCOMX_SLOW,
            14 => IDR::SERCOM0_CORE,
            15 => IDR::SERCOM1_CORE,
            16 => IDR::SERCOM2_CORE,
            18 => IDR::TC1_TC2,
            19 => IDR::ADC,
            20 => IDR::AC_DIG,
            21 => IDR::AC_ANA,
            i => IDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DFLL48`"]
    #[inline]
    pub fn is_dfll48(&self) -> bool {
        *self == IDR::DFLL48
    }
    #[doc = "Checks if the value of the field is `FDPLL`"]
    #[inline]
    pub fn is_fdpll(&self) -> bool {
        *self == IDR::FDPLL
    }
    #[doc = "Checks if the value of the field is `FDPLL32K`"]
    #[inline]
    pub fn is_fdpll32k(&self) -> bool {
        *self == IDR::FDPLL32K
    }
    #[doc = "Checks if the value of the field is `WDT`"]
    #[inline]
    pub fn is_wdt(&self) -> bool {
        *self == IDR::WDT
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline]
    pub fn is_rtc(&self) -> bool {
        *self == IDR::RTC
    }
    #[doc = "Checks if the value of the field is `EIC`"]
    #[inline]
    pub fn is_eic(&self) -> bool {
        *self == IDR::EIC
    }
    #[doc = "Checks if the value of the field is `EVSYS_0`"]
    #[inline]
    pub fn is_evsys_0(&self) -> bool {
        *self == IDR::EVSYS_0
    }
    #[doc = "Checks if the value of the field is `EVSYS_1`"]
    #[inline]
    pub fn is_evsys_1(&self) -> bool {
        *self == IDR::EVSYS_1
    }
    #[doc = "Checks if the value of the field is `EVSYS_2`"]
    #[inline]
    pub fn is_evsys_2(&self) -> bool {
        *self == IDR::EVSYS_2
    }
    #[doc = "Checks if the value of the field is `EVSYS_3`"]
    #[inline]
    pub fn is_evsys_3(&self) -> bool {
        *self == IDR::EVSYS_3
    }
    #[doc = "Checks if the value of the field is `EVSYS_4`"]
    #[inline]
    pub fn is_evsys_4(&self) -> bool {
        *self == IDR::EVSYS_4
    }
    #[doc = "Checks if the value of the field is `EVSYS_5`"]
    #[inline]
    pub fn is_evsys_5(&self) -> bool {
        *self == IDR::EVSYS_5
    }
    #[doc = "Checks if the value of the field is `SERCOMX_SLOW`"]
    #[inline]
    pub fn is_sercomx_slow(&self) -> bool {
        *self == IDR::SERCOMX_SLOW
    }
    #[doc = "Checks if the value of the field is `SERCOM0_CORE`"]
    #[inline]
    pub fn is_sercom0_core(&self) -> bool {
        *self == IDR::SERCOM0_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM1_CORE`"]
    #[inline]
    pub fn is_sercom1_core(&self) -> bool {
        *self == IDR::SERCOM1_CORE
    }
    #[doc = "Checks if the value of the field is `SERCOM2_CORE`"]
    #[inline]
    pub fn is_sercom2_core(&self) -> bool {
        *self == IDR::SERCOM2_CORE
    }
    #[doc = "Checks if the value of the field is `TC1_TC2`"]
    #[inline]
    pub fn is_tc1_tc2(&self) -> bool {
        *self == IDR::TC1_TC2
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline]
    pub fn is_adc(&self) -> bool {
        *self == IDR::ADC
    }
    #[doc = "Checks if the value of the field is `AC_DIG`"]
    #[inline]
    pub fn is_ac_dig(&self) -> bool {
        *self == IDR::AC_DIG
    }
    #[doc = "Checks if the value of the field is `AC_ANA`"]
    #[inline]
    pub fn is_ac_ana(&self) -> bool {
        *self == IDR::AC_ANA
    }
}
#[doc = "Possible values of the field `GEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENR {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GENR::GCLK0 => 0,
            GENR::GCLK1 => 1,
            GENR::GCLK2 => 2,
            GENR::GCLK3 => 3,
            GENR::GCLK4 => 4,
            GENR::GCLK5 => 5,
            GENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GENR {
        match value {
            0 => GENR::GCLK0,
            1 => GENR::GCLK1,
            2 => GENR::GCLK2,
            3 => GENR::GCLK3,
            4 => GENR::GCLK4,
            5 => GENR::GCLK5,
            i => GENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline]
    pub fn is_gclk0(&self) -> bool {
        *self == GENR::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline]
    pub fn is_gclk1(&self) -> bool {
        *self == GENR::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline]
    pub fn is_gclk2(&self) -> bool {
        *self == GENR::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline]
    pub fn is_gclk3(&self) -> bool {
        *self == GENR::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline]
    pub fn is_gclk4(&self) -> bool {
        *self == GENR::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline]
    pub fn is_gclk5(&self) -> bool {
        *self == GENR::GCLK5
    }
}
#[doc = r" Value of the field"]
pub struct CLKENR {
    bits: bool,
}
impl CLKENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct WRTLOCKR {
    bits: bool,
}
impl WRTLOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `ID`"]
pub enum IDW {
    #[doc = "DFLL48"]
    DFLL48,
    #[doc = "FDPLL"]
    FDPLL,
    #[doc = "FDPLL32K"]
    FDPLL32K,
    #[doc = "WDT"]
    WDT,
    #[doc = "RTC"]
    RTC,
    #[doc = "EIC"]
    EIC,
    #[doc = "EVSYS_0"]
    EVSYS_0,
    #[doc = "EVSYS_1"]
    EVSYS_1,
    #[doc = "EVSYS_2"]
    EVSYS_2,
    #[doc = "EVSYS_3"]
    EVSYS_3,
    #[doc = "EVSYS_4"]
    EVSYS_4,
    #[doc = "EVSYS_5"]
    EVSYS_5,
    #[doc = "SERCOMX_SLOW"]
    SERCOMX_SLOW,
    #[doc = "SERCOM0_CORE"]
    SERCOM0_CORE,
    #[doc = "SERCOM1_CORE"]
    SERCOM1_CORE,
    #[doc = "SERCOM2_CORE"]
    SERCOM2_CORE,
    #[doc = "TC1_TC2"]
    TC1_TC2,
    #[doc = "ADC"]
    ADC,
    #[doc = "AC_DIG"]
    AC_DIG,
    #[doc = "AC_ANA"]
    AC_ANA,
}
impl IDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDW::DFLL48 => 0,
            IDW::FDPLL => 1,
            IDW::FDPLL32K => 2,
            IDW::WDT => 3,
            IDW::RTC => 4,
            IDW::EIC => 5,
            IDW::EVSYS_0 => 7,
            IDW::EVSYS_1 => 8,
            IDW::EVSYS_2 => 9,
            IDW::EVSYS_3 => 10,
            IDW::EVSYS_4 => 11,
            IDW::EVSYS_5 => 12,
            IDW::SERCOMX_SLOW => 13,
            IDW::SERCOM0_CORE => 14,
            IDW::SERCOM1_CORE => 15,
            IDW::SERCOM2_CORE => 16,
            IDW::TC1_TC2 => 18,
            IDW::ADC => 19,
            IDW::AC_DIG => 20,
            IDW::AC_ANA => 21,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDW<'a> {
    w: &'a mut W,
}
impl<'a> _IDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DFLL48"]
    #[inline]
    pub fn dfll48(self) -> &'a mut W {
        self.variant(IDW::DFLL48)
    }
    #[doc = "FDPLL"]
    #[inline]
    pub fn fdpll(self) -> &'a mut W {
        self.variant(IDW::FDPLL)
    }
    #[doc = "FDPLL32K"]
    #[inline]
    pub fn fdpll32k(self) -> &'a mut W {
        self.variant(IDW::FDPLL32K)
    }
    #[doc = "WDT"]
    #[inline]
    pub fn wdt(self) -> &'a mut W {
        self.variant(IDW::WDT)
    }
    #[doc = "RTC"]
    #[inline]
    pub fn rtc(self) -> &'a mut W {
        self.variant(IDW::RTC)
    }
    #[doc = "EIC"]
    #[inline]
    pub fn eic(self) -> &'a mut W {
        self.variant(IDW::EIC)
    }
    #[doc = "EVSYS_0"]
    #[inline]
    pub fn evsys_0(self) -> &'a mut W {
        self.variant(IDW::EVSYS_0)
    }
    #[doc = "EVSYS_1"]
    #[inline]
    pub fn evsys_1(self) -> &'a mut W {
        self.variant(IDW::EVSYS_1)
    }
    #[doc = "EVSYS_2"]
    #[inline]
    pub fn evsys_2(self) -> &'a mut W {
        self.variant(IDW::EVSYS_2)
    }
    #[doc = "EVSYS_3"]
    #[inline]
    pub fn evsys_3(self) -> &'a mut W {
        self.variant(IDW::EVSYS_3)
    }
    #[doc = "EVSYS_4"]
    #[inline]
    pub fn evsys_4(self) -> &'a mut W {
        self.variant(IDW::EVSYS_4)
    }
    #[doc = "EVSYS_5"]
    #[inline]
    pub fn evsys_5(self) -> &'a mut W {
        self.variant(IDW::EVSYS_5)
    }
    #[doc = "SERCOMX_SLOW"]
    #[inline]
    pub fn sercomx_slow(self) -> &'a mut W {
        self.variant(IDW::SERCOMX_SLOW)
    }
    #[doc = "SERCOM0_CORE"]
    #[inline]
    pub fn sercom0_core(self) -> &'a mut W {
        self.variant(IDW::SERCOM0_CORE)
    }
    #[doc = "SERCOM1_CORE"]
    #[inline]
    pub fn sercom1_core(self) -> &'a mut W {
        self.variant(IDW::SERCOM1_CORE)
    }
    #[doc = "SERCOM2_CORE"]
    #[inline]
    pub fn sercom2_core(self) -> &'a mut W {
        self.variant(IDW::SERCOM2_CORE)
    }
    #[doc = "TC1_TC2"]
    #[inline]
    pub fn tc1_tc2(self) -> &'a mut W {
        self.variant(IDW::TC1_TC2)
    }
    #[doc = "ADC"]
    #[inline]
    pub fn adc(self) -> &'a mut W {
        self.variant(IDW::ADC)
    }
    #[doc = "AC_DIG"]
    #[inline]
    pub fn ac_dig(self) -> &'a mut W {
        self.variant(IDW::AC_DIG)
    }
    #[doc = "AC_ANA"]
    #[inline]
    pub fn ac_ana(self) -> &'a mut W {
        self.variant(IDW::AC_ANA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GEN`"]
pub enum GENW {
    #[doc = "Generic clock generator 0"]
    GCLK0,
    #[doc = "Generic clock generator 1"]
    GCLK1,
    #[doc = "Generic clock generator 2"]
    GCLK2,
    #[doc = "Generic clock generator 3"]
    GCLK3,
    #[doc = "Generic clock generator 4"]
    GCLK4,
    #[doc = "Generic clock generator 5"]
    GCLK5,
}
impl GENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GENW::GCLK0 => 0,
            GENW::GCLK1 => 1,
            GENW::GCLK2 => 2,
            GENW::GCLK3 => 3,
            GENW::GCLK4 => 4,
            GENW::GCLK5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GENW<'a> {
    w: &'a mut W,
}
impl<'a> _GENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Generic clock generator 0"]
    #[inline]
    pub fn gclk0(self) -> &'a mut W {
        self.variant(GENW::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline]
    pub fn gclk1(self) -> &'a mut W {
        self.variant(GENW::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline]
    pub fn gclk2(self) -> &'a mut W {
        self.variant(GENW::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline]
    pub fn gclk3(self) -> &'a mut W {
        self.variant(GENW::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline]
    pub fn gclk4(self) -> &'a mut W {
        self.variant(GENW::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline]
    pub fn gclk5(self) -> &'a mut W {
        self.variant(GENW::GCLK5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRTLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _WRTLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline]
    pub fn id(&self) -> IDR {
        IDR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline]
    pub fn gen(&self) -> GENR {
        GENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline]
    pub fn clken(&self) -> CLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        CLKENR { bits }
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline]
    pub fn wrtlock(&self) -> WRTLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        WRTLOCKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Generic Clock Selection ID"]
    #[inline]
    pub fn id(&mut self) -> _IDW {
        _IDW { w: self }
    }
    #[doc = "Bits 8:11 - Generic Clock Generator"]
    #[inline]
    pub fn gen(&mut self) -> _GENW {
        _GENW { w: self }
    }
    #[doc = "Bit 14 - Clock Enable"]
    #[inline]
    pub fn clken(&mut self) -> _CLKENW {
        _CLKENW { w: self }
    }
    #[doc = "Bit 15 - Write Lock"]
    #[inline]
    pub fn wrtlock(&mut self) -> _WRTLOCKW {
        _WRTLOCKW { w: self }
    }
}
