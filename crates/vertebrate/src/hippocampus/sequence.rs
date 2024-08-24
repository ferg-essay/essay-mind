use core::fmt;
use std::{fmt::Write, str::FromStr};

use util::random::Rand32;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Thread64(u64);

impl Thread64 {
    ///
    /// Create a random thread with n digits and a sequence bit width 
    /// 
    pub fn rand(rand: &mut Rand32, n: usize, radix: usize, seq: usize) -> Self {
        assert!(radix <= 6);
        assert!(seq < radix);
        assert!(n <= 10);

        let mask = ((1 << radix) - 1) & !((1 << seq) - 1);

        let mut n = n;
        let mut value : u64 = 0;
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
    /// Returns the next thread in the sequence
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

impl fmt::Display for Thread64 {
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

impl fmt::Debug for Thread64 {
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

impl<const N : usize> From<[u8; N]> for Thread64 {
    fn from(value: [u8; N]) -> Self {
        assert!(N <= 10);

        let mut result = 0;
        for i in 0..N {
            result = (result << 6) + value[i] as u64;
        }

        Self(result)
    }
}

impl FromStr for Thread64 {
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

impl From<&str> for Thread64 {
    fn from(s: &str) -> Thread64 {
        Thread64::from_str(s).unwrap()
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
    use util::random::Rand32;

    use super::Thread64;

    #[test]
    fn thread64_format() {
        assert_eq!("0", format!("{}", Thread64(0)));
        assert_eq!("0000000000", format!("{:0}", Thread64(0)));

        assert_eq!("10", format!("{}", Thread64(0x40)));
        assert_eq!("0000000010", format!("{:0}", Thread64(0x40)));

        assert_eq!("9876543210", format!("{}", Thread64::from([
            0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ])));
        assert_eq!("76543210", format!("{}", Thread64::from([
            0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ])));
        assert_eq!("fedcba98", format!("{}", Thread64::from([
            0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08,
        ])));
        assert_eq!("nmlkjihg", format!("{}", Thread64::from([
            0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10
        ])));
        assert_eq!("vutsrqpo", format!("{}", Thread64::from([
            0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18,
        ])));
        assert_eq!("DCBAzyxw", format!("{}", Thread64::from([
            0x27, 0x26, 0x25, 0x24, 0x23, 0x22, 0x21, 0x20
        ])));
        assert_eq!("LKJIHGFE", format!("{}", Thread64::from([
            0x2f, 0x2e, 0x2d, 0x2c, 0x2b, 0x2a, 0x29, 0x28,
        ])));
        assert_eq!("TSRQPONM", format!("{}", Thread64::from([
            0x37, 0x36, 0x35, 0x34, 0x33, 0x32, 0x31, 0x30
        ])));
        assert_eq!("#$ZYXWVU", format!("{}", Thread64::from([
            0x3f, 0x3e, 0x3d, 0x3c, 0x3b, 0x3a, 0x39, 0x38,
        ])));
    }

    #[test]
    fn thread64_debug() {
        assert_eq!("Thread64(0)", format!("{:?}", Thread64(0)));
        assert_eq!("Thread64(0000000000)", format!("{:0?}", Thread64(0)));

        assert_eq!("Thread64(10)", format!("{:?}", Thread64(0x40)));
        assert_eq!("Thread64(0000000010)", format!("{:0?}", Thread64(0x040)));


        assert_eq!("Thread64(9876543210)", format!("{:?}", Thread64::from([
            0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ])));
        assert_eq!("Thread64(76543210)", format!("{:?}", Thread64::from([
            0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ])));
        assert_eq!("Thread64(fedcba98)", format!("{:?}", Thread64::from([
            0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08,
        ])));
        assert_eq!("Thread64(nmlkjihg)", format!("{:?}", Thread64::from([
            0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10
        ])));
        assert_eq!("Thread64(vutsrqpo)", format!("{:?}", Thread64::from([
            0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18,
        ])));
        assert_eq!("Thread64(DCBAzyxw)", format!("{:?}", Thread64::from([
            0x27, 0x26, 0x25, 0x24, 0x23, 0x22, 0x21, 0x20
        ])));
        assert_eq!("Thread64(LKJIHGFE)", format!("{:?}", Thread64::from([
            0x2f, 0x2e, 0x2d, 0x2c, 0x2b, 0x2a, 0x29, 0x28,
        ])));
        assert_eq!("Thread64(TSRQPONM)", format!("{:?}", Thread64::from([
            0x37, 0x36, 0x35, 0x34, 0x33, 0x32, 0x31, 0x30
        ])));
        assert_eq!("Thread64(#$ZYXWVU)", format!("{:?}", Thread64::from([
            0x3f, 0x3e, 0x3d, 0x3c, 0x3b, 0x3a, 0x39, 0x38,
        ])));
    }

    #[test]
    fn thread64_from_str() {
        assert_eq!(Thread64(0), Thread64::from(""));
        assert_eq!(Thread64(0), Thread64::from("0"));
        assert_eq!(Thread64(0x01), Thread64::from("1"));
        assert_eq!(Thread64(0x3f), Thread64::from("#"));
        assert_eq!(Thread64::from([0x3f, 0x00]), Thread64::from("0#0"));

        assert_eq!(Thread64::from([
            0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
        ]), Thread64::from("9876543210"));

        assert_eq!(Thread64::from([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07
        ]), Thread64::from("01234567"));
        assert_eq!(Thread64::from([
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f
        ]), Thread64::from("89abcdef"));

        assert_eq!(Thread64::from([
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17
        ]), Thread64::from("ghijklmn"));
        assert_eq!(Thread64::from([
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f
        ]), Thread64::from("opqrstuv"));
        
        assert_eq!(Thread64::from([
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27
        ]), Thread64::from("wxyzABCD"));
        assert_eq!(Thread64::from([
            0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f
        ]), Thread64::from("EFGHIJKL"));
        
        assert_eq!(Thread64::from([
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37
        ]), Thread64::from("MNOPQRST"));
        assert_eq!(Thread64::from([
            0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f
        ]), Thread64::from("UVWXYZ$#"));
    }

    #[test]
    fn thread64_next_0_1() {
        assert_eq!(Thread64(0), Thread64(0x0).next(0, 1));
        assert_eq!(Thread64(0), Thread64(0x1).next(0, 1));
        assert_eq!(Thread64(0x3), Thread64(0x2).next(0, 1));
        assert_eq!(Thread64(0), Thread64(0x3).next(0, 1));
        assert_eq!(Thread64(0x5), Thread64(0x4).next(0, 1));
        assert_eq!(Thread64(0), Thread64(0x5).next(0, 1));
        assert_eq!(Thread64(0x21), Thread64(0x20).next(0, 1));
        assert_eq!(Thread64(0), Thread64(0x21).next(0, 1));

        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x11
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x10
        ]).next(0, 1));
        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x00
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x11
        ]).next(0, 1));
        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x13
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x12
        ]).next(0, 1));
        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x00
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x13
        ]).next(0, 1));
    }

    #[test]
    fn thread64_next_0_6() {
        assert_eq!(Thread64(0), Thread64(0x0).next(0, 6));
        assert_eq!(Thread64(0x2), Thread64(0x1).next(0, 6));
        assert_eq!(Thread64(0x3), Thread64(0x2).next(0, 6));
        assert_eq!(Thread64(0x4), Thread64(0x3).next(0, 6));

        assert_eq!(Thread64(0x5), Thread64(0x4).next(0, 6));
        assert_eq!(Thread64(0x6), Thread64(0x5).next(0, 6));
        assert_eq!(Thread64(0x7), Thread64(0x6).next(0, 6));
        assert_eq!(Thread64(0x8), Thread64(0x7).next(0, 6));

        assert_eq!(Thread64(0xd), Thread64(0xc).next(0, 6));
        assert_eq!(Thread64(0xe), Thread64(0xd).next(0, 6));
        assert_eq!(Thread64(0xf), Thread64(0xe).next(0, 6));
        assert_eq!(Thread64(0x10), Thread64(0xf).next(0, 6));

        assert_eq!(Thread64(0x1d), Thread64(0x1c).next(0, 6));
        assert_eq!(Thread64(0x1e), Thread64(0x1d).next(0, 6));
        assert_eq!(Thread64(0x1f), Thread64(0x1e).next(0, 6));
        assert_eq!(Thread64(0x20), Thread64(0x1f).next(0, 6));

        assert_eq!(Thread64(0x2d), Thread64(0x2c).next(0, 6));
        assert_eq!(Thread64(0x2e), Thread64(0x2d).next(0, 6));
        assert_eq!(Thread64(0x2f), Thread64(0x2e).next(0, 6));
        assert_eq!(Thread64(0x30), Thread64(0x2f).next(0, 6));

        assert_eq!(Thread64(0x3d), Thread64(0x3c).next(0, 6));
        assert_eq!(Thread64(0x3e), Thread64(0x3d).next(0, 6));
        assert_eq!(Thread64(0x3f), Thread64(0x3e).next(0, 6));
        assert_eq!(Thread64(0x00), Thread64(0x3f).next(0, 6));

        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3d
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3c
        ]).next(0, 6));
        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3e
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3d
        ]).next(0, 6));
        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3e
        ]).next(0, 6));
        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x00
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f
        ]).next(0, 6));
    }

    #[test]
    fn thread64_next_1_1() {
        assert_eq!(
            Thread64::from([0x3f, 0x00, 0x3f]), 
            Thread64::from([0x3f, 0x00, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Thread64::from([0x3f, 0x00, 0x3f]),
            Thread64::from([0x3f, 0x01, 0x3f]).next(1, 1)
        );

        assert_eq!(
            Thread64::from([0x3f, 0x03, 0x3f]), 
            Thread64::from([0x3f, 0x02, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Thread64::from([0x3f, 0x00, 0x3f]),
            Thread64::from([0x3f, 0x03, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Thread64::from([0x3f, 0x05, 0x3f]),
            Thread64::from([0x3f, 0x04, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Thread64::from([0x3f, 0x00, 0x3f]),
            Thread64::from([0x3f, 0x05, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Thread64::from([0x3f, 0x21, 0x3f]),
            Thread64::from([0x3f, 0x20, 0x3f]).next(1, 1)
        );
        assert_eq!(
            Thread64::from([0x3f, 0x00, 0x3f]),
            Thread64::from([0x3f, 0x21, 0x3f]).next(1, 1)
        );

        assert_eq!(Thread64(0), Thread64(0x0).next(1, 1));
        assert_eq!(Thread64(0x1), Thread64(0x1).next(1, 1));

        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x11, 0x3f
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x10, 0x3f
        ]).next(1, 1));
        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x00, 0x3f
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x11, 0x3f
        ]).next(1, 1));
        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x13, 0x3f
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x12, 0x3f
        ]).next(1, 1));
        assert_eq!(Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x00, 0x3f
        ]), Thread64::from([
            0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x13, 0x3f
        ]).next(1, 1));
    }

    #[test]
    fn thread64_next_9_1() {
        assert_eq!(Thread64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Thread64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x01, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));

        assert_eq!(Thread64::from([
            0x03, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x02, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Thread64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x03, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Thread64::from([
            0x05, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x04, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Thread64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x05, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Thread64::from([
            0x21, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x20, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Thread64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x21, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));

        assert_eq!(Thread64(0), Thread64(0x0).next(1, 1));
        assert_eq!(Thread64(0x1), Thread64(0x1).next(1, 1));

        assert_eq!(Thread64::from([
            0x11, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x10, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Thread64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x11, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Thread64::from([
            0x13, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x12, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
        assert_eq!(Thread64::from([
            0x00, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]), Thread64::from([
            0x13, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f, 0x3f,
        ]).next(9, 1));
    }

    #[test]
    fn thread64_rand_n() {
        let mut rand = Rand32(42);

        assert_eq!(Thread64::from("0"), Thread64::rand(&mut rand, 0, 6, 0));

        assert_eq!(Thread64::from("p"), Thread64::rand(&mut rand, 1, 6, 0));
        assert_eq!(Thread64::from("l"), Thread64::rand(&mut rand, 1, 6, 0));
        assert_eq!(Thread64::from("F"), Thread64::rand(&mut rand, 1, 6, 0));
        assert_eq!(Thread64::from("I"), Thread64::rand(&mut rand, 1, 6, 0));

        assert_eq!(Thread64::from("zk"), Thread64::rand(&mut rand, 2, 6, 0));
        assert_eq!(Thread64::from("YNK"), Thread64::rand(&mut rand, 3, 6, 0));
        assert_eq!(Thread64::from("l8io"), Thread64::rand(&mut rand, 4, 6, 0));
        assert_eq!(Thread64::from("xeApq"), Thread64::rand(&mut rand, 5, 6, 0));
        assert_eq!(Thread64::from("n#x92k"), Thread64::rand(&mut rand, 6, 6, 0));
        assert_eq!(Thread64::from("A3rlJRo"), Thread64::rand(&mut rand, 7, 6, 0));
        assert_eq!(Thread64::from("FqRBNUHO"), Thread64::rand(&mut rand, 8, 6, 0));
        assert_eq!(Thread64::from("i3CD3EHFh"), Thread64::rand(&mut rand, 9, 6, 0));
        assert_eq!(Thread64::from("O4kfw$fJcu"), Thread64::rand(&mut rand, 10, 6, 0));
    }

    #[test]
    fn thread64_rand_radix() {
        let mut rand = Rand32(42);

        assert_eq!(Thread64::from("pCPx#l#jjH"), Thread64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Thread64::from("FpzXVIfCi6"), Thread64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Thread64::from("zkyj7YNKil"), Thread64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Thread64::from("xeApqn#x92"), Thread64::rand(&mut rand, 10, 6, 0));

        assert_eq!(Thread64::from("A3rlJRoArw"), Thread64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Thread64::from("FqRBNUHOsB"), Thread64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Thread64::from("i3CD3EHFhO"), Thread64::rand(&mut rand, 10, 6, 0));
        assert_eq!(Thread64::from("$fJcuGVnpl"), Thread64::rand(&mut rand, 10, 6, 0));

        assert_eq!(Thread64::from("1111111111"), Thread64::rand(&mut rand, 10, 1, 0));

        assert_eq!(Thread64::from("3331211123"), Thread64::rand(&mut rand, 10, 2, 0));
        assert_eq!(Thread64::from("3321312312"), Thread64::rand(&mut rand, 10, 2, 0));

        assert_eq!(Thread64::from("1115273112"), Thread64::rand(&mut rand, 10, 3, 0));
        assert_eq!(Thread64::from("5577436751"), Thread64::rand(&mut rand, 10, 3, 0));
        assert_eq!(Thread64::from("2157514761"), Thread64::rand(&mut rand, 10, 3, 0));
        assert_eq!(Thread64::from("6544137124"), Thread64::rand(&mut rand, 10, 3, 0));

        assert_eq!(Thread64::from("49574a1c9e"), Thread64::rand(&mut rand, 10, 4, 0));
        assert_eq!(Thread64::from("76f967d8ef"), Thread64::rand(&mut rand, 10, 4, 0));
        assert_eq!(Thread64::from("598738cc18"), Thread64::rand(&mut rand, 10, 4, 0));
        assert_eq!(Thread64::from("c45e8d34ee"), Thread64::rand(&mut rand, 10, 4, 0));
    }

    #[test]
    fn thread64_rand_seq() {
        let mut rand = Rand32(42);

        assert_eq!(Thread64::from("1111111111"), Thread64::rand(&mut rand, 10, 1, 0));
        assert_eq!(Thread64::from("2222222222"), Thread64::rand(&mut rand, 10, 2, 1));

        assert_eq!(Thread64::from("4226426624"), Thread64::rand(&mut rand, 10, 3, 1));
        assert_eq!(Thread64::from("4224444224"), Thread64::rand(&mut rand, 10, 3, 1));
        assert_eq!(Thread64::from("2244226622"), Thread64::rand(&mut rand, 10, 3, 1));
        assert_eq!(Thread64::from("4444444444"), Thread64::rand(&mut rand, 10, 3, 2));

        assert_eq!(Thread64::from("c26e48e46a"), Thread64::rand(&mut rand, 10, 4, 1));
        assert_eq!(Thread64::from("826c48a8e2"), Thread64::rand(&mut rand, 10, 4, 1));
        assert_eq!(Thread64::from("44844c8484"), Thread64::rand(&mut rand, 10, 4, 2));
        assert_eq!(Thread64::from("8888888888"), Thread64::rand(&mut rand, 10, 4, 3));

        assert_eq!(Thread64::from("8m4oi6e4q2"), Thread64::rand(&mut rand, 10, 5, 1));
        assert_eq!(Thread64::from("gceuesgskg"), Thread64::rand(&mut rand, 10, 5, 1));
        assert_eq!(Thread64::from("c8osg48k4k"), Thread64::rand(&mut rand, 10, 5, 2));
        assert_eq!(Thread64::from("ogooogo8gg"), Thread64::rand(&mut rand, 10, 5, 3));
        assert_eq!(Thread64::from("gggggggggg"), Thread64::rand(&mut rand, 10, 5, 4));

        assert_eq!(Thread64::from("e8sikMueg6"), Thread64::rand(&mut rand, 10, 6, 1));
        assert_eq!(Thread64::from("8aGW2kqUww"), Thread64::rand(&mut rand, 10, 6, 1));
        assert_eq!(Thread64::from("gU8IUgYsMA"), Thread64::rand(&mut rand, 10, 6, 2));
        assert_eq!(Thread64::from("oMwUMwgEME"), Thread64::rand(&mut rand, 10, 6, 3));
        assert_eq!(Thread64::from("wwMMggwMMw"), Thread64::rand(&mut rand, 10, 6, 4));
        assert_eq!(Thread64::from("wwwwwwwwww"), Thread64::rand(&mut rand, 10, 6, 5));
    }
}