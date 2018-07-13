#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRBS {
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
#[doc = r" Value of the field"]
pub struct M8PRR {
    bits: u8,
}
impl M8PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M9PRR {
    bits: u8,
}
impl M9PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M10PRR {
    bits: u8,
}
impl M10PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M11PRR {
    bits: u8,
}
impl M11PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M12PRR {
    bits: u8,
}
impl M12PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M13PRR {
    bits: u8,
}
impl M13PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M14PRR {
    bits: u8,
}
impl M14PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M15PRR {
    bits: u8,
}
impl M15PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _M8PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M8PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M9PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M9PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M10PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M10PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M11PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M11PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M12PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M12PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M13PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M13PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M14PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M14PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M15PRW<'a> {
    w: &'a mut W,
}
impl<'a> _M15PRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Master 8 Priority"]
    #[inline]
    pub fn m8pr(&self) -> M8PRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M8PRR { bits }
    }
    #[doc = "Bits 4:7 - Master 9 Priority"]
    #[inline]
    pub fn m9pr(&self) -> M9PRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M9PRR { bits }
    }
    #[doc = "Bits 8:11 - Master 10 Priority"]
    #[inline]
    pub fn m10pr(&self) -> M10PRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M10PRR { bits }
    }
    #[doc = "Bits 12:15 - Master 11 Priority"]
    #[inline]
    pub fn m11pr(&self) -> M11PRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M11PRR { bits }
    }
    #[doc = "Bits 16:19 - Master 12 Priority"]
    #[inline]
    pub fn m12pr(&self) -> M12PRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M12PRR { bits }
    }
    #[doc = "Bits 20:23 - Master 13 Priority"]
    #[inline]
    pub fn m13pr(&self) -> M13PRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M13PRR { bits }
    }
    #[doc = "Bits 24:27 - Master 14 Priority"]
    #[inline]
    pub fn m14pr(&self) -> M14PRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M14PRR { bits }
    }
    #[doc = "Bits 28:31 - Master 15 Priority"]
    #[inline]
    pub fn m15pr(&self) -> M15PRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M15PRR { bits }
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Master 8 Priority"]
    #[inline]
    pub fn m8pr(&mut self) -> _M8PRW {
        _M8PRW { w: self }
    }
    #[doc = "Bits 4:7 - Master 9 Priority"]
    #[inline]
    pub fn m9pr(&mut self) -> _M9PRW {
        _M9PRW { w: self }
    }
    #[doc = "Bits 8:11 - Master 10 Priority"]
    #[inline]
    pub fn m10pr(&mut self) -> _M10PRW {
        _M10PRW { w: self }
    }
    #[doc = "Bits 12:15 - Master 11 Priority"]
    #[inline]
    pub fn m11pr(&mut self) -> _M11PRW {
        _M11PRW { w: self }
    }
    #[doc = "Bits 16:19 - Master 12 Priority"]
    #[inline]
    pub fn m12pr(&mut self) -> _M12PRW {
        _M12PRW { w: self }
    }
    #[doc = "Bits 20:23 - Master 13 Priority"]
    #[inline]
    pub fn m13pr(&mut self) -> _M13PRW {
        _M13PRW { w: self }
    }
    #[doc = "Bits 24:27 - Master 14 Priority"]
    #[inline]
    pub fn m14pr(&mut self) -> _M14PRW {
        _M14PRW { w: self }
    }
    #[doc = "Bits 28:31 - Master 15 Priority"]
    #[inline]
    pub fn m15pr(&mut self) -> _M15PRW {
        _M15PRW { w: self }
    }
}
