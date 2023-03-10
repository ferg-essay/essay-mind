use std::{fmt, cmp};

use crate::gram::gram::U8_TO_STR;


pub enum Digit {
    Nil,
    Med(u8),
    High(u8),
    Low(u8),
    Max(u8),
}
pub const NIL: u8 = 0x3f;
pub(crate) const LOW: u8 = 0x00;
pub(crate) const MED: u8 = 0x40;
pub(crate) const HIGH: u8 = 0x80;
pub(crate) const MAX: u8 = 0xc0;
pub(crate) const WEIGHT_MASK: u8 = 0xc0;
pub(crate) const WEIGHT_SHIFT: u8 = 6;
pub(crate) const DIGIT_MASK: u8 = 0x3f;


impl Digit {
    pub fn try_from_f32(value: f32, base: u8) -> Digit {
        let base_max = base / 2;
        let index_f = value.floor();

        if index_f >= 0.0 {
            let index = index_f as usize;
            assert!(index < base_max as usize);

            Digit::Med(index as u8)
        }  else {
            let index = (-index_f) as usize;
            assert!(index <= base_max as usize);

            Digit::Med(base - index as u8)
        }
    }

    pub fn try_from_unit(value: f32, base: u8) -> Digit {
        assert!(0.0 <= value); //  && value <= 1.0);

        let value = cmp::min(base - 1, (value * base as f32) as u8);

        Digit::Med(value)
    }

    pub fn f32_unit(digit: u8, base: u8) -> f32 {
        let value = digit & DIGIT_MASK;

        assert!(value < base);

        value as f32 / base as f32
    }

    pub fn f32_sunit(digit: u8, base: u8) -> f32 {
        let value = digit & DIGIT_MASK;

        assert!(value < base);

        let base_2 = base / 2;

        if value < base_2 {
            value as f32 / base_2 as f32
        } else {
            (value as f32 - base as f32) / base_2 as f32
        }

    }

    pub fn digit(&self) -> u8 {
        match *self {
            Digit::Nil => NIL,
            Digit::Low(digit) => digit,
            Digit::Med(digit) => digit,
            Digit::High(digit) => digit,
            Digit::Max(digit) => digit
        }
    }

    pub fn as_u8(&self) -> u8 {
        match *self {
            Digit::Nil => NIL,
            Digit::Low(digit) => digit + LOW,
            Digit::Med(digit) => digit + MED,
            Digit::High(digit) => digit + HIGH,
            Digit::Max(digit) => digit + MAX,
        }
    }

    pub fn weight(&self) -> u8 {
        match *self {
            Digit::Nil => 0,
            Digit::Low(_) => 1,
            Digit::Med(_) => 2,
            Digit::High(_) => 3,
            Digit::Max(_) => 4,
        }
    }
}

impl fmt::Debug for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Digit::Nil => write!(f, "Nil"),
            Digit::Med(digit) => {
                write!(f, "Med('{}')", U8_TO_STR[(digit + MED) as usize])
            },
            Digit::Max(digit) => {
                write!(f, "High('{}')", U8_TO_STR[(digit + MAX) as usize])
            },
            Digit::High(digit) => {
                write!(f, "High('{}')", U8_TO_STR[(digit + HIGH) as usize])
            },
            Digit::Low(digit) => {
                write!(f, "Low('{}')", U8_TO_STR[(digit + LOW) as usize])
            },
        }
    }
}

impl From<u8> for Digit {
    fn from(digit: u8) -> Self {
        assert!(digit & WEIGHT_MASK == 0 && digit != NIL);

        Digit::Med(digit)
    }
}

impl From<u32> for Digit {
    fn from(digit32: u32) -> Self {
        let digit: u8 = u8::try_from(digit32).expect(
            "Invalid integer digit, must be convertable to u8"
        );

        assert!(digit & WEIGHT_MASK == 0 && digit != NIL);

        Digit::Med(digit)
    }
}

impl TryFrom<f32> for Digit {
    type Error = String;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        assert!(0.0 <= value && value < 64.0);

        unsafe { Ok(Digit::Med(value.to_int_unchecked::<u8>())) }
    }
}
