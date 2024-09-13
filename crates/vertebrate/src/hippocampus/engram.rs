use core::fmt;
use std::{fmt::Write, str::FromStr};

use util::random::{Rand32, Rand64};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Engram64(u64);

impl Engram64 {
    ///
    /// Create a random engram with n digits and a sequence bit width 
    /// 
    pub fn rand(rand: &mut Rand32, n: usize, radix: usize, seq: usize) -> Self {
        assert!(radix <= 6);
        assert!(seq < radix);
        assert!(n <= 10);

        let mask = ((1 << radix) - 1) & !((1 << seq) - 1);

        let mut n = n;
        let mut value = 0;
        let mut rand_value = 0;
        
        while n > 0 {
            if rand_value == 0 {
                rand_value = rand.next() as u64 & 0x3fff_ffff;
            }

            let digit = rand_value & mask;
            rand_value >>= 6;

            if digit != 0 {
                value = (value << 6) + digit;
                n -= 1;
            }
        }

        Self(value)
    }

    ///
    /// Returns the next engram in the sequence
    /// 
    /// * `i` - index of the digit to change
    /// * `width` - sequence width in bits
    /// 
    pub fn next(&self, i: usize, width: usize) -> Self {
        assert!(i < 10);
        assert!(width <= 6);

        let digit = (self.0 >> (i * 6)) & 0x3f;

        if digit == 0 {
            return Self(self.0);
        }

        let mask = 0x3f << (i * 6);
        let submask = (1 << width) - 1;

        let seq = digit & submask;
        let digit = if seq == submask {
            0
        } else {
            (digit & !submask) + seq + 1
        };

        Self((self.0 & !mask) + (digit << (i * 6)))
    }
}

impl fmt::Display for Engram64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_digit = f.sign_aware_zero_pad();
        for i in 0..10 {
            let b = ((self.0 >> 6 * (9 - i)) & 0x3f) as u8;
            if is_digit || b != 0 {
                is_digit = true;
                f.write_char(base64_unchecked(b))?;
            }
        }

        if ! is_digit {
            f.write_str("0")?;
        }

        Ok(())
    }
}

impl fmt::Debug for Engram64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Thread64(")?;

        let mut is_digit = f.sign_aware_zero_pad();
        for i in 0..10 {
            let b = ((self.0 >> 6 * (9 - i)) & 0x3f) as u8;
            if is_digit || b != 0 {
                is_digit = true;
                f.write_char(base64_unchecked(b))?;
            }
        }

        if ! is_digit {
            f.write_str("0")?;
        }

        f.write_str(")")
    }
}

impl<const N : usize> From<[u8; N]> for Engram64 {
    fn from(value: [u8; N]) -> Self {
        assert!(N <= 10);

        let mut result = 0;
        for i in 0..N {
            result = (result << 6) + value[i] as u64;
        }

        Self(result)
    }
}

impl FromStr for Engram64 {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value : u64 = 0;

        for ch in s.chars() {
            if ch == '_' {
                continue;
            }

            value = (value << 6) + base64_rev(ch)? as u64;
        }

        Ok(Self(value))
    }
}

impl From<&str> for Engram64 {
    fn from(s: &str) -> Engram64 {
        Engram64::from_str(s).unwrap()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Engram128(u128);

impl Engram128 {
    ///
    /// Create a random engram with n digits and a sequence bit width 
    /// 
    pub fn rand(rand: &mut Rand64, n: usize, radix: usize, seq: usize) -> Self {
        assert!(radix <= 6);
        assert!(seq < radix);
        assert!(n <= 21);

        let mask = ((1 << radix) - 1) & !((1 << seq) - 1);

        let mut n = n;
        let mut value = 0;
        let mut rand_value = 0;
        
        while n > 0 {
            if rand_value == 0 {
                rand_value = rand.next() & 0x0fff_ffff_ffff_ffff;
            }

            let digit = rand_value & mask;
            rand_value >>= 6;

            if digit != 0 {
                value = (value << 6) + digit as u128;
                n -= 1;
            }
        }

        Self(value)
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    ///
    /// Returns the next engram in the sequence
    /// 
    /// * `i` - index of the digit to change
    /// * `width` - sequence width in bits
    /// 
    pub fn next(&self, i: usize, width: usize) -> Self {
        assert!(i < 21);
        assert!(width <= 6);

        let digit = (self.0 >> (i * 6)) & 0x3f;

        if digit == 0 {
            return Self(self.0);
        }

        let mask = 0x3f << (i * 6);
        let submask = (1 << width) - 1;

        let seq = digit & submask;
        let digit = if seq == submask {
            0
        } else {
            (digit & !submask) + seq + 1
        };

        Self((self.0 & !mask) + (digit << (i * 6)))
    }
}

impl fmt::Display for Engram128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_digit = f.sign_aware_zero_pad();
        for i in 0..21 {
            let b = ((self.0 >> 6 * (20 - i)) & 0x3f) as u8;

            if is_digit && (20 - i) % 5 == 4 {
                f.write_char('_')?;
            }

            if is_digit || b != 0 {
                is_digit = true;
                f.write_char(base64_unchecked(b))?;
            }
        }

        if ! is_digit {
            f.write_str("0")?;
        }

        Ok(())
    }
}

impl fmt::Debug for Engram128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Engram128(")?;

        (&self as &dyn fmt::Display).fmt(f)?;
        /*
        let mut is_digit = f.sign_aware_zero_pad();
        for i in 0..21 {
            let b = ((self.0 >> 6 * (20 - i)) & 0x3f) as u8;

            if is_digit && (20 - i) % 5 == 4 {
                f.write_char('_')?;
            }

            if is_digit || b != 0 {
                is_digit = true;
                f.write_char(base64_unchecked(b))?;
            }
        }

        if ! is_digit {
            f.write_str("0")?;
        }
        */

        f.write_str(")")
    }
}

impl<const N : usize> From<[u8; N]> for Engram128 {
    fn from(value: [u8; N]) -> Self {
        assert!(N <= 21);

        let mut result = 0;
        for i in 0..N {
            result = (result << 6) + value[i] as u128;
        }

        Self(result)
    }
}

impl FromStr for Engram128 {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = 0;

        for ch in s.chars() {
            if ch == '_' {
                continue;
            }

            value = (value << 6) + base64_rev(ch)? as u128;
        }

        Ok(Self(value))
    }
}

impl From<&str> for Engram128 {
    fn from(s: &str) -> Engram128 {
        Engram128::from_str(s).unwrap()
    }
}

fn base64_unchecked(value: u8) -> char {
    match value {
        0..=9 => ('0' as u8 + value) as char,
        10..=35 => ('a' as u8 + value - 10) as char,
        36..=61 => ('A' as u8 + value - 36) as char,
        62 => '$',
        63 => '#',

        _ => '?'
    }
}

fn base64_rev(value: char) -> Result<u8, fmt::Error> {
    match value {
        '0'..='9' => Ok(value as u8 - '0' as u8),
        'a'..='z' => Ok(value as u8 - 'a' as u8 + 10),
        'A'..='Z' => Ok(value as u8 - 'A' as u8 + 36),
        '$' => Ok(62),
        '#' => Ok(63),

        _ => Err(fmt::Error)
    }
}

#[cfg(test)]
mod test {
    use util::random::{Rand32, Rand64};

    use crate::hippocampus::engram::Engram128;

    use super::Engram64;

    #[test]
    fn thread64_format() {
        assert_eq!("0", format!("{}", Engram64(0)));
        assert_eq!("0000000000", format!("{:0}", Engram64(0)));

        assert_eq!("10", format!("{}", Engram64(0x40)));
        assert_eq!("0000000010", format!("{:0}", Engram64(0x40)));

        assert_eq!("9876543210", format!("{}", Engram64::from([
            0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ])));
        assert_eq!("76543210", format!("{}", Engram64::from([
            0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ])));
        assert_eq!("fedcba98", format!("{}", Engram64::from([
            0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08,
        ])));
        assert_eq!("nmlkjihg", format!("{}", Engram64::from([
            0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10
        ])));
        assert_eq!("vutsrqpo", format!("{}", Engram64::from([
            0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18,
        ])));
        assert_eq!("DCBAzyxw", format!("{}", Engram64::from([
            0x27, 0x26, 0x25, 0x24, 0x23, 0x22, 0x21, 0x20
        ])));
        assert_eq!("LKJIHGFE", format!("{}", Engram64::from([
            0x2f, 0x2e, 0x2d, 0x2c, 0x2b, 0x2a, 0x29, 0x28,
        ])));
        assert_eq!("TSRQPONM", format!("{}", Engram64::from([
            0x37, 0x36, 0x35, 0x34, 0x33, 0x32, 0x31, 0x30
        ])));
        assert_eq!("#$ZYXWVU", format!("{}", Engram64::from([
            0x3f, 0x3e, 0x3d, 0x3c, 0x3b, 0x3a, 0x39, 0x38,
        ])));
    }

    #[test]
    fn thread64_debug() {
        assert_eq!("Thread64(0)", format!("{:?}", Engram64(0)));
        assert_eq!("Thread64(0000000000)", format!("{:0?}", Engram64(0)));

        assert_eq!("Thread64(10)", format!("{:?}", Engram64(0x40)));
        assert_eq!("Thread64(0000000010)", format!("{:0?}", Engram64(0x040)));


        assert_eq!("Thread64(9876543210)", format!("{:?}", Engram64::from([
            0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ])));
        assert_eq!("Thread64(76543210)", format!("{:?}", Engram64::from([
            0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ])));
        assert_eq!("Thread64(fedcba98)", format!("{:?}", Engram64::from([
            0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08,
        ])));
        assert_eq!("Thread64(nmlkjihg)", format!("{:?}", Engram64::from([
            0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10
        ])));
        assert_eq!("Thread64(vutsrqpo)", format!("{:?}", Engram64::from([
            0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18,
        ])));
        assert_eq!("Thread64(DCBAzyxw)", format!("{:?}", Engram64::from([
            0x27, 0x26, 0x25, 0x24, 0x23, 0x22, 0x21, 0x20
        ])));
        assert_eq!("Thread64(LKJIHGFE)", format!("{:?}", Engram64::from([
            0x2f, 0x2e, 0x2d, 0x2c, 0x2b, 0x2a, 0x29, 0x28,
        ])));
        assert_eq!("Thread64(TSRQPONM)", format!("{:?}", Engram64::from([
            0x37, 0x36, 0x35, 0x34, 0x33, 0x32, 0x31, 0x30
        ])));
        assert_eq!("Thread64(#$ZYXWVU)", format!("{:?}", Engram64::from([
            0x3f, 0x3e, 0x3d, 0x3c, 0x3b, 0x3a, 0x39, 0x38,
        ])));
    }

    #[test]
    fn thread64_from_str() {
        assert_eq!(Engram64(0), Engram64::from(""));
        assert_eq!(Engram64(0), Engram64::from("0"));
        assert_eq!(Engram64(0x01), Engram64::from("1"));
        assert_eq!(Engram64(0x3f), Engram64::from("#"));
        assert_eq!(Engram64::from([0x3f, 0x00]), Engram64::from("0#0"));

        assert_eq!(Engram64::from([
            0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ]), Engram64::from("9876543210"));

        assert_eq!(Engram64::from([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07
        ]), Engram64::from("01234567"));
        assert_eq!(Engram64::from([
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f
        ]), Engram64::from("89abcdef"));

        assert_eq!(Engram64::from([
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17
        ]), Engram64::from("ghijklmn"));
        assert_eq!(Engram64::from([
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f
        ]), Engram64::from("opqrstuv"));
        
        assert_eq!(Engram64::from([
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27
        ]), Engram64::from("wxyzABCD"));
        assert_eq!(Engram64::from([
            0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f
        ]), Engram64::from("EFGHIJKL"));
        
        assert_eq!(Engram64::from([
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37
        ]), Engram64::from("MNOPQRST"));
        assert_eq!(Engram64::from([
            0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f
        ]), Engram64::from("UVWXYZ$#"));
    }

    #[test]
    fn thread64_next_0_1() {
        assert_eq!(Engram64(0), Engram64(0x0).next(0, 1));
        assert_eq!(Engram64(0), Engram64(0x1).next(0, 1));
        assert_eq!(Engram64(0x3), Engram64(0x2).next(0, 1));
        assert_eq!(Engram64(0), Engram64(0x3).next(0, 1));
        assert_eq!(Engram64(0x5), Engram64(0x4).next(0, 1));
        assert_eq!(Engram64(0), Engram64(0x5).next(0, 1));
        assert_eq!(Engram64(0x21), Engram64(0x20).next(0, 1));
        assert_eq!(Engram64(0), Engram64(0x21).next(0, 1));

        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x11
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x10
        ]).next(0, 1));
        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x00
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x11
        ]).next(0, 1));
        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x13
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x12
        ]).next(0, 1));
        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x00
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x13
        ]).next(0, 1));
    }

    #[test]
    fn thread64_next_0_6() {
        assert_eq!(Engram64(0), Engram64(0x0).next(0, 6));
        assert_eq!(Engram64(0x2), Engram64(0x1).next(0, 6));
        assert_eq!(Engram64(0x3), Engram64(0x2).next(0, 6));
        assert_eq!(Engram64(0x4), Engram64(0x3).next(0, 6));

        assert_eq!(Engram64(0x5), Engram64(0x4).next(0, 6));
        assert_eq!(Engram64(0x6), Engram64(0x5).next(0, 6));
        assert_eq!(Engram64(0x7), Engram64(0x6).next(0, 6));
        assert_eq!(Engram64(0x8), Engram64(0x7).next(0, 6));

        assert_eq!(Engram64(0xd), Engram64(0xc).next(0, 6));
        assert_eq!(Engram64(0xe), Engram64(0xd).next(0, 6));
        assert_eq!(Engram64(0xf), Engram64(0xe).next(0, 6));
        assert_eq!(Engram64(0x10), Engram64(0xf).next(0, 6));

        assert_eq!(Engram64(0x1d), Engram64(0x1c).next(0, 6));
        assert_eq!(Engram64(0x1e), Engram64(0x1d).next(0, 6));
        assert_eq!(Engram64(0x1f), Engram64(0x1e).next(0, 6));
        assert_eq!(Engram64(0x20), Engram64(0x1f).next(0, 6));

        assert_eq!(Engram64(0x2d), Engram64(0x2c).next(0, 6));
        assert_eq!(Engram64(0x2e), Engram64(0x2d).next(0, 6));
        assert_eq!(Engram64(0x2f), Engram64(0x2e).next(0, 6));
        assert_eq!(Engram64(0x30), Engram64(0x2f).next(0, 6));

        assert_eq!(Engram64(0x3d), Engram64(0x3c).next(0, 6));
        assert_eq!(Engram64(0x3e), Engram64(0x3d).next(0, 6));
        assert_eq!(Engram64(0x3f), Engram64(0x3e).next(0, 6));
        assert_eq!(Engram64(0x00), Engram64(0x3f).next(0, 6));

        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3d
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3c
        ]).next(0, 6));
        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3e
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3d
        ]).next(0, 6));
        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3e
        ]).next(0, 6));
        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x00
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f
        ]).next(0, 6));
    }

    #[test]
    fn thread64_next_1_1() {
        assert_eq!(
            Engram64::from([0x3f, 0x00, 0x3f]), 
            Engram64::from([0x3f, 0x00, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram64::from([0x3f, 0x00, 0x3f]),
            Engram64::from([0x3f, 0x01, 0x3f]).next(1, 1)
        );

        assert_eq!(
            Engram64::from([0x3f, 0x03, 0x3f]), 
            Engram64::from([0x3f, 0x02, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram64::from([0x3f, 0x00, 0x3f]),
            Engram64::from([0x3f, 0x03, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram64::from([0x3f, 0x05, 0x3f]),
            Engram64::from([0x3f, 0x04, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram64::from([0x3f, 0x00, 0x3f]),
            Engram64::from([0x3f, 0x05, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram64::from([0x3f, 0x21, 0x3f]),
            Engram64::from([0x3f, 0x20, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram64::from([0x3f, 0x00, 0x3f]),
            Engram64::from([0x3f, 0x21, 0x3f]).next(1, 1)
        );

        assert_eq!(Engram64(0), Engram64(0x0).next(1, 1));
        assert_eq!(Engram64(0x1), Engram64(0x1).next(1, 1));

        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x11, 0x3f
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x10, 0x3f
        ]).next(1, 1));
        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x00, 0x3f
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x11, 0x3f
        ]).next(1, 1));
        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x13, 0x3f
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x12, 0x3f
        ]).next(1, 1));
        assert_eq!(Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x00, 0x3f
        ]), Engram64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x13, 0x3f
        ]).next(1, 1));
    }

    #[test]
    fn thread64_next_9_1() {
        assert_eq!(Engram64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Engram64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x01, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));

        assert_eq!(Engram64::from([
            0x03, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x02, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Engram64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x03, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Engram64::from([
            0x05, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x04, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Engram64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x05, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Engram64::from([
            0x21, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x20, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Engram64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x21, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));

        assert_eq!(Engram64(0), Engram64(0x0).next(1, 1));
        assert_eq!(Engram64(0x1), Engram64(0x1).next(1, 1));

        assert_eq!(Engram64::from([
            0x11, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x10, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Engram64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x11, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Engram64::from([
            0x13, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x12, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Engram64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Engram64::from([
            0x13, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
    }

    #[test]
    fn thread64_rand_n() {
        let mut rand = Rand32(42);

        assert_eq!(Engram64::from("0"), Engram64::rand(&mut rand, 0, 6, 0));

        assert_eq!(Engram64::from("p"), Engram64::rand(&mut rand, 1, 6, 0));
        assert_eq!(Engram64::from("l"), Engram64::rand(&mut rand, 1, 6, 0));
        assert_eq!(Engram64::from("F"), Engram64::rand(&mut rand, 1, 6, 0));
        assert_eq!(Engram64::from("I"), Engram64::rand(&mut rand, 1, 6, 0));

        assert_eq!(Engram64::from("zk"), Engram64::rand(&mut rand, 2, 6, 0));
        assert_eq!(Engram64::from("YNK"), Engram64::rand(&mut rand, 3, 6, 0));
        assert_eq!(Engram64::from("l8io"), Engram64::rand(&mut rand, 4, 6, 0));
        assert_eq!(Engram64::from("xeApq"), Engram64::rand(&mut rand, 5, 6, 0));
        assert_eq!(Engram64::from("n#x92k"), Engram64::rand(&mut rand, 6, 6, 0));
        assert_eq!(Engram64::from("A3rlJRo"), Engram64::rand(&mut rand, 7, 6, 0));
        assert_eq!(Engram64::from("FqRBNUHO"), Engram64::rand(&mut rand, 8, 6, 0));
        assert_eq!(Engram64::from("i3CD3EHFh"), Engram64::rand(&mut rand, 9, 6, 0));
        assert_eq!(Engram64::from("O4kfw$fJcu"), Engram64::rand(&mut rand, 10, 6, 0));
    }

    #[test]
    fn thread64_rand_radix() {
        let mut rand = Rand32(42);

        assert_eq!(Engram64::from("pCPx#l#jjH"), Engram64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Engram64::from("FpzXVIfCi6"), Engram64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Engram64::from("zkyj7YNKil"), Engram64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Engram64::from("xeApqn#x92"), Engram64::rand(&mut rand, 10, 6, 0));

        assert_eq!(Engram64::from("A3rlJRoArw"), Engram64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Engram64::from("FqRBNUHOsB"), Engram64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Engram64::from("i3CD3EHFhO"), Engram64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Engram64::from("$fJcuGVnpl"), Engram64::rand(&mut rand, 10, 6, 0));

        assert_eq!(Engram64::from("1111111111"), Engram64::rand(&mut rand, 10, 1, 0));

        assert_eq!(Engram64::from("3331211123"), Engram64::rand(&mut rand, 10, 2, 0));
        assert_eq!(Engram64::from("3321312312"), Engram64::rand(&mut rand, 10, 2, 0));

        assert_eq!(Engram64::from("1115273112"), Engram64::rand(&mut rand, 10, 3, 0));
        assert_eq!(Engram64::from("5577436751"), Engram64::rand(&mut rand, 10, 3, 0));
        assert_eq!(Engram64::from("2157514761"), Engram64::rand(&mut rand, 10, 3, 0));
        assert_eq!(Engram64::from("6544137124"), Engram64::rand(&mut rand, 10, 3, 0));

        assert_eq!(Engram64::from("49574a1c9e"), Engram64::rand(&mut rand, 10, 4, 0));
        assert_eq!(Engram64::from("76f967d8ef"), Engram64::rand(&mut rand, 10, 4, 0));
        assert_eq!(Engram64::from("598738cc18"), Engram64::rand(&mut rand, 10, 4, 0));
        assert_eq!(Engram64::from("c45e8d34ee"), Engram64::rand(&mut rand, 10, 4, 0));
    }

    #[test]
    fn thread64_rand_seq() {
        let mut rand = Rand32(42);

        assert_eq!(Engram64::from("1111111111"), Engram64::rand(&mut rand, 10, 1, 0));
        assert_eq!(Engram64::from("2222222222"), Engram64::rand(&mut rand, 10, 2, 1));

        assert_eq!(Engram64::from("4226426624"), Engram64::rand(&mut rand, 10, 3, 1));
        assert_eq!(Engram64::from("4224444224"), Engram64::rand(&mut rand, 10, 3, 1));
        assert_eq!(Engram64::from("2244226622"), Engram64::rand(&mut rand, 10, 3, 1));
        assert_eq!(Engram64::from("4444444444"), Engram64::rand(&mut rand, 10, 3, 2));

        assert_eq!(Engram64::from("c26e48e46a"), Engram64::rand(&mut rand, 10, 4, 1));
        assert_eq!(Engram64::from("826c48a8e2"), Engram64::rand(&mut rand, 10, 4, 1));
        assert_eq!(Engram64::from("44844c8484"), Engram64::rand(&mut rand, 10, 4, 2));
        assert_eq!(Engram64::from("8888888888"), Engram64::rand(&mut rand, 10, 4, 3));

        assert_eq!(Engram64::from("8m4oi6e4q2"), Engram64::rand(&mut rand, 10, 5, 1));
        assert_eq!(Engram64::from("gceuesgskg"), Engram64::rand(&mut rand, 10, 5, 1));
        assert_eq!(Engram64::from("c8osg48k4k"), Engram64::rand(&mut rand, 10, 5, 2));
        assert_eq!(Engram64::from("ogooogo8gg"), Engram64::rand(&mut rand, 10, 5, 3));
        assert_eq!(Engram64::from("gggggggggg"), Engram64::rand(&mut rand, 10, 5, 4));

        assert_eq!(Engram64::from("e8sikMueg6"), Engram64::rand(&mut rand, 10, 6, 1));
        assert_eq!(Engram64::from("8aGW2kqUww"), Engram64::rand(&mut rand, 10, 6, 1));
        assert_eq!(Engram64::from("gU8IUgYsMA"), Engram64::rand(&mut rand, 10, 6, 2));
        assert_eq!(Engram64::from("oMwUMwgEME"), Engram64::rand(&mut rand, 10, 6, 3));
        assert_eq!(Engram64::from("wwMMggwMMw"), Engram64::rand(&mut rand, 10, 6, 4));
        assert_eq!(Engram64::from("wwwwwwwwww"), Engram64::rand(&mut rand, 10, 6, 5));
    }

    #[test]
    fn engram128_display() {
        assert_eq!("0", format!("{}", Engram128(0)));
        assert_eq!("0_00000_00000_00000_00000", format!("{:0}", Engram128(0)));

        assert_eq!("10", format!("{}", Engram128(0x40)));
        assert_eq!("0_00000_00000_00000_00010", format!("{:0}", Engram128(0x40)));

        assert_eq!("1_23456_789ab_cdefg_hijkl", format!("{}", Engram128::from([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
        ])));

        assert_eq!("l_mnopq_rstuv_wxyzA_BCDEF", format!("{}", Engram128::from([
            21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
        ])));

        assert_eq!("F_GHIJK_LMNOP_QRSTU_VWXYZ", format!("{}", Engram128::from([
            41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
            51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
        ])));

        assert_eq!("YZ$#0", format!("{}", Engram128::from([
            60, 61, 62, 63, 0
        ])));

        assert_eq!("765_43210", format!("{}", Engram128::from([
            0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ])));
    }

    #[test]
    fn engram128_debug() {
        assert_eq!("Engram128(0)", format!("{:?}", Engram128(0)));
        assert_eq!("Engram128(0_00000_00000_00000_00000)", format!("{:0?}", Engram128(0)));

        assert_eq!("Engram128(10)", format!("{:?}", Engram128(0x40)));
        assert_eq!("Engram128(0_00000_00000_00000_00010)", format!("{:0?}", Engram128(0x40)));

        assert_eq!("Engram128(1_23456_789ab_cdefg_hijkl)", format!("{:?}", Engram128::from([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
        ])));

        assert_eq!("Engram128(l_mnopq_rstuv_wxyzA_BCDEF)", format!("{:?}", Engram128::from([
            21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
        ])));

        assert_eq!("Engram128(F_GHIJK_LMNOP_QRSTU_VWXYZ)", format!("{:?}", Engram128::from([
            41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
            51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
        ])));

        assert_eq!("Engram128(YZ$#0)", format!("{:?}", Engram128::from([
            60, 61, 62, 63, 0
        ])));

        assert_eq!("765_43210", format!("{}", Engram128::from([
            0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ])));
    }

    #[test]
    fn engram128_from_str() {
        assert_eq!(Engram128(0), Engram128::from(""));
        assert_eq!(Engram128(0), Engram128::from("0"));
        assert_eq!(Engram128(0x01), Engram128::from("1"));
        assert_eq!(Engram128(0x3f), Engram128::from("#"));
        assert_eq!(Engram128::from([0x3f, 0x00]), Engram128::from("0#0"));

        assert_eq!(Engram128::from([
            0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ]), Engram128::from("9876543210"));

        assert_eq!(Engram128::from([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07
        ]), Engram128::from("01234567"));
        assert_eq!(Engram128::from([
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f
        ]), Engram128::from("89abcdef"));

        assert_eq!(Engram128::from([
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17
        ]), Engram128::from("ghijklmn"));
        assert_eq!(Engram128::from([
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f
        ]), Engram128::from("opqrstuv"));
        
        assert_eq!(Engram128::from([
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27
        ]), Engram128::from("wxyzABCD"));
        assert_eq!(Engram128::from([
            0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f
        ]), Engram128::from("EFGHIJKL"));
        
        assert_eq!(Engram128::from([
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37
        ]), Engram128::from("MNOPQRST"));
        assert_eq!(Engram128::from([
            0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f
        ]), Engram128::from("UVWXYZ$#"));

        assert_eq!(Engram128::from([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
        ]), Engram128::from("1_23456_789ab_cdefg_hijkl"));
    }

    #[test]
    fn engram128_next_0_1() {
        assert_eq!(Engram128(0), Engram128(0x0).next(0, 1));
        assert_eq!(Engram128(0), Engram128(0x1).next(0, 1));
        assert_eq!(Engram128(0x3), Engram128(0x2).next(0, 1));
        assert_eq!(Engram128(0), Engram128(0x3).next(0, 1));
        assert_eq!(Engram128(0x5), Engram128(0x4).next(0, 1));
        assert_eq!(Engram128(0), Engram128(0x5).next(0, 1));
        assert_eq!(Engram128(0x21), Engram128(0x20).next(0, 1));
        assert_eq!(Engram128(0), Engram128(0x21).next(0, 1));

        assert_eq!(
            Engram128::from("#_#####_#####_#####_####0"),
            Engram128::from("#_#####_#####_#####_####0").next(0, 1),
        );
        assert_eq!(
            Engram128::from("#_#####_#####_#####_####0"),
            Engram128::from("#_#####_#####_#####_####1").next(0, 1),
        );
        assert_eq!(
            Engram128::from("#_#####_#####_#####_####9"),
            Engram128::from("#_#####_#####_#####_####8").next(0, 1),
        );
        assert_eq!(
            Engram128::from("#_#####_#####_#####_####0"),
            Engram128::from("#_#####_#####_#####_####9").next(0, 1),
        );
        assert_eq!(
            Engram128::from("#_#####_#####_#####_####b"),
            Engram128::from("#_#####_#####_#####_####a").next(0, 1),
        );
        assert_eq!(
            Engram128::from("#_#####_#####_#####_####0"),
            Engram128::from("#_#####_#####_#####_####b").next(0, 1),
        );
    }

    #[test]
    fn engram128_next_0_6() {
        assert_eq!(Engram128(0), Engram128(0x0).next(0, 6));
        assert_eq!(Engram128(0x2), Engram128(0x1).next(0, 6));
        assert_eq!(Engram128(0x3), Engram128(0x2).next(0, 6));
        assert_eq!(Engram128(0x4), Engram128(0x3).next(0, 6));

        assert_eq!(Engram128(0x5), Engram128(0x4).next(0, 6));
        assert_eq!(Engram128(0x6), Engram128(0x5).next(0, 6));
        assert_eq!(Engram128(0x7), Engram128(0x6).next(0, 6));
        assert_eq!(Engram128(0x8), Engram128(0x7).next(0, 6));

        assert_eq!(Engram128(0xd), Engram128(0xc).next(0, 6));
        assert_eq!(Engram128(0xe), Engram128(0xd).next(0, 6));
        assert_eq!(Engram128(0xf), Engram128(0xe).next(0, 6));
        assert_eq!(Engram128(0x10), Engram128(0xf).next(0, 6));

        assert_eq!(Engram128(0x1d), Engram128(0x1c).next(0, 6));
        assert_eq!(Engram128(0x1e), Engram128(0x1d).next(0, 6));
        assert_eq!(Engram128(0x1f), Engram128(0x1e).next(0, 6));
        assert_eq!(Engram128(0x20), Engram128(0x1f).next(0, 6));

        assert_eq!(Engram128(0x2d), Engram128(0x2c).next(0, 6));
        assert_eq!(Engram128(0x2e), Engram128(0x2d).next(0, 6));
        assert_eq!(Engram128(0x2f), Engram128(0x2e).next(0, 6));
        assert_eq!(Engram128(0x30), Engram128(0x2f).next(0, 6));

        assert_eq!(Engram128(0x3d), Engram128(0x3c).next(0, 6));
        assert_eq!(Engram128(0x3e), Engram128(0x3d).next(0, 6));
        assert_eq!(Engram128(0x3f), Engram128(0x3e).next(0, 6));
        assert_eq!(Engram128(0x00), Engram128(0x3f).next(0, 6));

        assert_eq!(
            Engram64::from("#_#####_#####_#####_####0"),
            Engram64::from("#_#####_#####_#####_####0").next(0, 6),
        );
        assert_eq!(
            Engram64::from("#_#####_#####_#####_####Z"),
            Engram64::from("#_#####_#####_#####_####Y").next(0, 6),
        );
        assert_eq!(
            Engram64::from("#_#####_#####_#####_####$"),
            Engram64::from("#_#####_#####_#####_####Z").next(0, 6),
        );
        assert_eq!(
            Engram64::from("#_#####_#####_#####_#####"),
            Engram64::from("#_#####_#####_#####_####$").next(0, 6),
        );
        assert_eq!(
            Engram64::from("#_#####_#####_#####_####0"),
            Engram64::from("#_#####_#####_#####_#####").next(0, 6),
        );
    }

    #[test]
    fn engram128_next_1_1() {
        assert_eq!(
            Engram128::from([0x3f, 0x00, 0x3f]), 
            Engram128::from([0x3f, 0x00, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram128::from([0x3f, 0x00, 0x3f]),
            Engram128::from([0x3f, 0x01, 0x3f]).next(1, 1)
        );

        assert_eq!(
            Engram128::from([0x3f, 0x03, 0x3f]), 
            Engram128::from([0x3f, 0x02, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram128::from([0x3f, 0x00, 0x3f]),
            Engram128::from([0x3f, 0x03, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram128::from([0x3f, 0x05, 0x3f]),
            Engram128::from([0x3f, 0x04, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram128::from([0x3f, 0x00, 0x3f]),
            Engram128::from([0x3f, 0x05, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram128::from([0x3f, 0x21, 0x3f]),
            Engram128::from([0x3f, 0x20, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Engram128::from([0x3f, 0x00, 0x3f]),
            Engram128::from([0x3f, 0x21, 0x3f]).next(1, 1)
        );

        assert_eq!(Engram128(0), Engram128(0x0).next(1, 1));
        assert_eq!(Engram128(0x1), Engram128(0x1).next(1, 1));

        assert_eq!(
            Engram128::from("#_#####_#####_#####_###9#"),
            Engram128::from("#_#####_#####_#####_###8#").next(1, 1),
        );
        assert_eq!(
            Engram128::from("#_#####_#####_#####_###0#"),
            Engram128::from("#_#####_#####_#####_###9#").next(1, 1),
        );
        assert_eq!(
            Engram128::from("#_#####_#####_#####_###b#"),
            Engram128::from("#_#####_#####_#####_###a#").next(1, 1),
        );
        assert_eq!(
            Engram128::from("#_#####_#####_#####_###0#"),
            Engram128::from("#_#####_#####_#####_###b#").next(1, 1),
        );
    }

    #[test]
    fn engram128_next_20_2() {
        assert_eq!(
            Engram128::from("9_#####_#####_#####_#####"),
            Engram128::from("8_#####_#####_#####_#####").next(20, 2),
        );
        assert_eq!(
            Engram128::from("a_#####_#####_#####_#####"),
            Engram128::from("9_#####_#####_#####_#####").next(20, 2),
        );
        assert_eq!(
            Engram128::from("b_#####_#####_#####_#####"),
            Engram128::from("a_#####_#####_#####_#####").next(20, 2),
        );
        assert_eq!(
            Engram128::from("0_#####_#####_#####_#####"),
            Engram128::from("b_#####_#####_#####_#####").next(20, 2),
        );
        assert_eq!(
            Engram128::from("0_#####_#####_#####_#####"),
            Engram128::from("X_#####_#####_#####_#####").next(20, 2),
        );
        assert_eq!(
            Engram128::from("Z_#####_#####_#####_#####"),
            Engram128::from("Y_#####_#####_#####_#####").next(20, 2),
        );
        assert_eq!(
            Engram128::from("$_#####_#####_#####_#####"),
            Engram128::from("Z_#####_#####_#####_#####").next(20, 2),
        );
        assert_eq!(
            Engram128::from("#_#####_#####_#####_#####"),
            Engram128::from("$_#####_#####_#####_#####").next(20, 2),
        );
        assert_eq!(
            Engram128::from("0_#####_#####_#####_#####"),
            Engram128::from("#_#####_#####_#####_#####").next(20, 2),
        );
    }

    #[test]
    fn engram128_rand_n() {
        let mut rand = Rand64(42);

        assert_eq!(Engram128::from("k_r1Zfm_ATC#P_P65vA_HG1lX"), Engram128::rand(&mut rand, 21, 6, 0));
        assert_eq!(Engram128::from("U5hT2_ooL9e_AMalB_N6VQ5"), Engram128::rand(&mut rand, 20, 6, 0));
        assert_eq!(Engram128::from("qdtHM_CPjqj_THZFS"), Engram128::rand(&mut rand, 15, 6, 0));
        assert_eq!(Engram128::from("8UJ2K_1dBCW"), Engram128::rand(&mut rand, 10, 6, 0));
        assert_eq!(Engram128::from("#_xbkiI"), Engram128::rand(&mut rand, 6, 6, 0));
        assert_eq!(Engram128::from("LC92A"), Engram128::rand(&mut rand, 5, 6, 0));
        assert_eq!(Engram128::from("vJd4"), Engram128::rand(&mut rand, 4, 6, 0));
        assert_eq!(Engram128::from("krP"), Engram128::rand(&mut rand, 3, 6, 0));
        assert_eq!(Engram128::from("VT"), Engram128::rand(&mut rand, 2, 6, 0));
        assert_eq!(Engram128::from("S"), Engram128::rand(&mut rand, 1, 6, 0));
        assert_eq!(Engram128::from("0"), Engram128::rand(&mut rand, 0, 6, 0));
    }

    #[test]
    fn engram128_rand_radix() {
        let mut rand = Rand64(42);

        assert_eq!(Engram128::from("k_r1Zfm_ATC#P_P65vA_HG1lX"), Engram128::rand(&mut rand, 21, 6, 0));
        assert_eq!(Engram128::from("U_5hT2o_oL9eA_MalBN_6VQ5q"), Engram128::rand(&mut rand, 21, 6, 0));
        assert_eq!(Engram128::from("T_HZFSV_#S8UJ_2K1dB_CW#xb"), Engram128::rand(&mut rand, 21, 6, 0));
        assert_eq!(Engram128::from("L_C92Ag_T$a3v_Jd4z9_exisk"), Engram128::rand(&mut rand, 21, 6, 0));

        assert_eq!(Engram128::from("V_TNTis_4bpHS_yainP_RxWZr"), Engram128::rand(&mut rand, 21, 6, 0));
        assert_eq!(Engram128::from("k_l2djz_Ca51R_dLX7K_dmyhv"), Engram128::rand(&mut rand, 21, 6, 0));
        assert_eq!(Engram128::from("Z_p2yut_Jx11m_hdkbk_#TGBQ"), Engram128::rand(&mut rand, 21, 6, 0));
        assert_eq!(Engram128::from("Y_SG3Wj_O7kda_OZdyB_AhDms"), Engram128::rand(&mut rand, 21, 6, 0));

        assert_eq!(Engram128::from("1_11111_11111_11111_11111"), Engram128::rand(&mut rand, 21, 1, 0));

        assert_eq!(Engram128::from("2_21232_21223_13122_11112"), Engram128::rand(&mut rand, 21, 2, 0));
        assert_eq!(Engram128::from("3_31133_31131_22321_11211"), Engram128::rand(&mut rand, 21, 2, 0));

        assert_eq!(Engram128::from("3_26676_31176_56421_36774"), Engram128::rand(&mut rand, 21, 3, 0));
        assert_eq!(Engram128::from("4_44315_23733_61351_53726"), Engram128::rand(&mut rand, 21, 3, 0));
        assert_eq!(Engram128::from("3_42527_61116_34161_15577"), Engram128::rand(&mut rand, 21, 3, 0));
        assert_eq!(Engram128::from("7_56643_11443_47176_23716"), Engram128::rand(&mut rand, 21, 3, 0));

        assert_eq!(Engram128::from("e_32317_69a42_fb369_4bea6"), Engram128::rand(&mut rand, 21, 4, 0));
        assert_eq!(Engram128::from("3_d12f2_7481b_5e51b_87938"), Engram128::rand(&mut rand, 21, 4, 0));
        assert_eq!(Engram128::from("6_ea253_4e718_86886_58a3d"), Engram128::rand(&mut rand, 21, 4, 0));
        assert_eq!(Engram128::from("b_d966a_ed6c2_b41ee_e54ed"), Engram128::rand(&mut rand, 21, 4, 0));
    }

    #[test]
    fn engram128_rand_seq() {
        let mut rand = Rand64(42);

        assert_eq!(Engram128::from("1_11111_11111_11111_11111"), Engram128::rand(&mut rand, 21, 1, 0));
        assert_eq!(Engram128::from("2_22222_22222_22222_22222"), Engram128::rand(&mut rand, 21, 2, 1));

        assert_eq!(Engram128::from("6_62466_22644_42624_42262"), Engram128::rand(&mut rand, 21, 3, 1));
        assert_eq!(Engram128::from("6_62442_26222_62424_22222"), Engram128::rand(&mut rand, 21, 3, 1));
        assert_eq!(Engram128::from("4_42422_62444_62664_62664"), Engram128::rand(&mut rand, 21, 3, 1));
        assert_eq!(Engram128::from("4_44444_44444_44444_44444"), Engram128::rand(&mut rand, 21, 3, 2));

        assert_eq!(Engram128::from("a_2cc24_466ca_68ee6_4868c"), Engram128::rand(&mut rand, 21, 4, 1));
        assert_eq!(Engram128::from("6_ea244_a6464_4ae44_44a22"), Engram128::rand(&mut rand, 21, 4, 1));
        assert_eq!(Engram128::from("8_8c884_c844c_c4c88_84cc4"), Engram128::rand(&mut rand, 21, 4, 2));
        assert_eq!(Engram128::from("8_88888_88888_88888_88888"), Engram128::rand(&mut rand, 21, 4, 3));

        assert_eq!(Engram128::from("o_g6usg_mko8i_oooam_u8644"), Engram128::rand(&mut rand, 21, 5, 1));
        assert_eq!(Engram128::from("q_iuoas_oki8m_28emg_2e2cq"), Engram128::rand(&mut rand, 21, 5, 1));
        assert_eq!(Engram128::from("k_8c44s_ko4g8_kksk8_kc4cs"), Engram128::rand(&mut rand, 21, 5, 2));
        assert_eq!(Engram128::from("g_g8ggg_8g8go_oo88g_ggggo"), Engram128::rand(&mut rand, 21, 5, 3));
        assert_eq!(Engram128::from("g_ggggg_ggggg_ggggg_ggggg"), Engram128::rand(&mut rand, 21, 5, 4));

        assert_eq!(Engram128::from("y_EOyyo_a8GsC_KWO4O_4uCMo"), Engram128::rand(&mut rand, 21, 6, 1));
        assert_eq!(Engram128::from("I_224Qe_SqoCa_sESg6_GeICI"), Engram128::rand(&mut rand, 21, 6, 1));
        assert_eq!(Engram128::from("s_cQAoc_ksAgM_cEQcI_4goYQ"), Engram128::rand(&mut rand, 21, 6, 2));
        assert_eq!(Engram128::from("w_88Uwo_w8wUU_8ggoE_oMwEo"), Engram128::rand(&mut rand, 21, 6, 3));
        assert_eq!(Engram128::from("M_wMMwM_MMgMw_gwggg_gggwM"), Engram128::rand(&mut rand, 21, 6, 4));
        assert_eq!(Engram128::from("w_wwwww_wwwww_wwwww_wwwww"), Engram128::rand(&mut rand, 21, 6, 5));
    }
}