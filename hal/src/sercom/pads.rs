use gpio::{self, IntoFunction, Port};

/// The PadPin trait makes it more ergonomic to convert a
/// pin into a Sercom pad.  You should not implement this
/// trait for yourself; only the implementations in the
/// sercom module make sense.
pub trait PadPin<T> {
    fn into_pad(self, port: &mut Port) -> T;
}

/// The pad macro helps to define enums for pads and makes it
/// a little more convenient to initialize them.
macro_rules! pad {
    ($(pub enum $PadType:ident {
        $( $PinType:ident ($new:ident, $Pf:ident),)+
    })+
    ) => {
$(
/// Represents a numbered pad for the associated sercom instance
pub enum $PadType {
    $(
        $PinType(gpio::$PinType<gpio::$Pf>),
    )+
}

impl $PadType {
    $(
    /// Construct pad from the appropriate pin in any mode.
    /// You may find it more convenient to use the `into_pad` trait
    /// and avoid referencing the pad type.
    pub fn $new<MODE>(pin: gpio::$PinType<MODE>, port: &mut Port) -> Self {
        $PadType::$PinType(pin.into_function(port))
    }

    )+
}

$(
impl<MODE> PadPin<$PadType> for gpio::$PinType<MODE> {
    fn into_pad(self, port: &mut Port) -> $PadType {
        $PadType::$new(self, port)
    }
}
)+

)+
    };
}

pad!(

pub enum Sercom0Pad0 {
    Pa4(pa4, PfD),
    Pa14(pa14, PfC),
}

pub enum Sercom0Pad1 {
    Pa5(pa5, PfD),
    Pa15(pa15, PfC),
}

pub enum Sercom0Pad2 {
    Pa6(pa6, PfD),
    Pa10(pa10, PfC),
}

pub enum Sercom0Pad3 {
    Pa7(pa7, PfD),
    Pa11(pa11, PfC),
}

pub enum Sercom1Pad0 {
    Pa22(pa22, PfC),
}

pub enum Sercom1Pad1 {
    Pa23(pa23, PfC),
}

pub enum Sercom1Pad2 {
    Pa16(pa16, PfC),
}

pub enum Sercom1Pad3 {
    Pa17(pa17, PfC),
}
);

#[cfg(feature = "samd21g18a")]
pad!(
// sercom4[0]:  PA12:D   PB08:D   PB12:C
// sercom4[1]:  PA13:D   PB09:D   PB13:C
// sercom4[2]:  PA14:D   PB10:D   PB14:C
// sercom4[3]:  PA15:D   PB11:D   PB15:C

pub enum Sercom4Pad0 {
    Pa12(pa12, PfD),
    Pb8(pb8, PfD),
    Pb12(pb12, PfC),
}

pub enum Sercom4Pad1 {
    Pa13(pa13, PfD),
    Pb9(pb9, PfD),
    Pb13(pb13, PfC),
}

pub enum Sercom4Pad2 {
    Pa14(pa14, PfD),
    Pb10(pb10, PfD),
    Pb14(pb14, PfC),
}

pub enum Sercom4Pad3 {
    Pa15(pa15, PfD),
    Pb11(pb11, PfD),
    Pb15(pb15, PfC),
}

// sercom5[0]:  PA22:D   PB02:D   PB16:C  PB30:D
// sercom5[1]:  PA23:D   PB03:D   PB17:C  PB31:D
// sercom5[2]:  PA24:D   PB00:D   PA20:C  PB22:D
// sercom5[3]:  PA25:D   PB01:D   PA21:C  PB23:D

pub enum Sercom5Pad0 {
    Pa22(pa22, PfD),
    Pb2(pb2, PfD),
    Pb16(pb16, PfC),
    Pb30(pb30, PfD),
}

pub enum Sercom5Pad1 {
    Pa23(pa23, PfD),
    Pb3(pb3, PfD),
    Pb17(pb17, PfC),
    Pb31(pb31, PfD),
}

pub enum Sercom5Pad2 {
    Pa24(pa24, PfD),
    Pb0(pb0, PfD),
    Pa20(pa20, PfC),
    Pb22(pb22, PfD),
}

pub enum Sercom5Pad3 {
    Pa25(pa25, PfD),
    Pb1(pb1, PfD),
    Pa21(pa21, PfC),
    Pb23(pb23, PfD),
}
);
