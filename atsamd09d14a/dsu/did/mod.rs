#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DEVSELR {
    bits: u8,
}
impl DEVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REVISIONR {
    bits: u8,
}
impl REVISIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIER {
    bits: u8,
}
impl DIER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SERIES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SERIESR {
    #[doc = "Cortex-M0+ processor, basic feature set"]
    _0,
    #[doc = "Cortex-M0+ processor, USB"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SERIESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SERIESR::_0 => 0,
            SERIESR::_1 => 1,
            SERIESR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SERIESR {
        match value {
            0 => SERIESR::_0,
            1 => SERIESR::_1,
            i => SERIESR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SERIESR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SERIESR::_1
    }
}
#[doc = "Possible values of the field `FAMILY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAMILYR {
    #[doc = "General purpose microcontroller"]
    _0,
    #[doc = "PicoPower"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FAMILYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FAMILYR::_0 => 0,
            FAMILYR::_1 => 1,
            FAMILYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FAMILYR {
        match value {
            0 => FAMILYR::_0,
            1 => FAMILYR::_1,
            i => FAMILYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAMILYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAMILYR::_1
    }
}
#[doc = "Possible values of the field `PROCESSOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROCESSORR {
    #[doc = "Cortex-M0"]
    _0,
    #[doc = "Cortex-M0+"]
    _1,
    #[doc = "Cortex-M3"]
    _2,
    #[doc = "Cortex-M4"]
    _3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PROCESSORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PROCESSORR::_0 => 0,
            PROCESSORR::_1 => 1,
            PROCESSORR::_2 => 2,
            PROCESSORR::_3 => 3,
            PROCESSORR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PROCESSORR {
        match value {
            0 => PROCESSORR::_0,
            1 => PROCESSORR::_1,
            2 => PROCESSORR::_2,
            3 => PROCESSORR::_3,
            i => PROCESSORR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PROCESSORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PROCESSORR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == PROCESSORR::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == PROCESSORR::_3
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Device Select"]
    #[inline]
    pub fn devsel(&self) -> DEVSELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEVSELR { bits }
    }
    #[doc = "Bits 8:11 - Revision Number"]
    #[inline]
    pub fn revision(&self) -> REVISIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVISIONR { bits }
    }
    #[doc = "Bits 12:15 - Die Number"]
    #[inline]
    pub fn die(&self) -> DIER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIER { bits }
    }
    #[doc = "Bits 16:21 - Series"]
    #[inline]
    pub fn series(&self) -> SERIESR {
        SERIESR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 23:27 - Family"]
    #[inline]
    pub fn family(&self) -> FAMILYR {
        FAMILYR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Processor"]
    #[inline]
    pub fn processor(&self) -> PROCESSORR {
        PROCESSORR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
