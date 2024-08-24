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
        assert!(n <= 8);

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
                value = (value << 8) + digit;
                n -= 1;
            }
        }

        Self(value)
    }

    ///
    /// Returns the next thread in the sequence
    /// 
    /// * i - digit index
    /// * width - sequence width in bits
    /// 
    pub fn next(&self, i: usize, width: usize) -> Self {
        assert!(i < 8);
        assert!(width <= 6);

        let digit = (self.0 >> (i * 8)) & 0x3f;

        if digit == 0 {
            return Self(self.0);
        }

        let mask = 0xff << (i * 8);
        let submask = (1 << width) - 1;

        let seq = digit & submask;
        let digit = if seq == submask {
            0
        } else {
            (digit & !submask) + seq + 1
        };

        Self((self.0 & !mask) + (digit << (i * 8)))
    }
}

impl fmt::Display for Thread64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.0;
        let mut is_digit = f.sign_aware_zero_pad();
        for b in v.to_be_bytes() {
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
        let v = self.0;
        let mut is_digit = f.sign_aware_zero_pad();
        for b in v.to_be_bytes() {
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

impl FromStr for Thread64 {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 8 {
            return Err(fmt::Error);
        }

        let mut value : u64 = 0;

        for ch in s.chars() {
            value = (value << 8) + base64_rev(ch)? as u64;
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
        assert_eq!("00000000", format!("{:0}", Thread64(0)));
        assert_eq!("10", format!("{}", Thread64(0x0100)));
        assert_eq!("00000010", format!("{:0}", Thread64(0x0100)));
        assert_eq!("76543210", format!("{}", Thread64(0x0706050_403020100)));
        assert_eq!("fedcba98", format!("{}", Thread64(0x0f0e0d0_c0b0a0908)));
        assert_eq!("nmlkjihg", format!("{}", Thread64(0x17161514_13121110)));
        assert_eq!("vutsrqpo", format!("{}", Thread64(0x1f1e1d1c_1b1a1918)));
        assert_eq!("DCBAzyxw", format!("{}", Thread64(0x27262524_23222120)));
        assert_eq!("LKJIHGFE", format!("{}", Thread64(0x2f2e2d2c_2b2a2928)));
        assert_eq!("TSRQPONM", format!("{}", Thread64(0x37363534_33323130)));
        assert_eq!("#$ZYXWVU", format!("{}", Thread64(0x3f3e3d3c_3b3a3938)));
    }

    #[test]
    fn thread64_debug() {
        assert_eq!("Thread64(0)", format!("{:?}", Thread64(0)));
        assert_eq!("Thread64(00000000)", format!("{:0?}", Thread64(0)));
        assert_eq!("Thread64(10)", format!("{:?}", Thread64(0x0100)));
        assert_eq!("Thread64(00000010)", format!("{:0?}", Thread64(0x0100)));
        assert_eq!("Thread64(76543210)", format!("{:?}", Thread64(0x0706050_403020100)));
        assert_eq!("Thread64(fedcba98)", format!("{:?}", Thread64(0x0f0e0d0_c0b0a0908)));
        assert_eq!("Thread64(nmlkjihg)", format!("{:?}", Thread64(0x17161514_13121110)));
        assert_eq!("Thread64(vutsrqpo)", format!("{:?}", Thread64(0x1f1e1d1c_1b1a1918)));
        assert_eq!("Thread64(DCBAzyxw)", format!("{:?}", Thread64(0x27262524_23222120)));
        assert_eq!("Thread64(LKJIHGFE)", format!("{:?}", Thread64(0x2f2e2d2c_2b2a2928)));
        assert_eq!("Thread64(TSRQPONM)", format!("{:?}", Thread64(0x37363534_33323130)));
        assert_eq!("Thread64(#$ZYXWVU)", format!("{:?}", Thread64(0x3f3e3d3c_3b3a3938)));
    }

    #[test]
    fn thread64_from_str() {
        assert_eq!(Thread64(0), Thread64::from(""));
        assert_eq!(Thread64(0), Thread64::from("0"));
        assert_eq!(Thread64(0x01), Thread64::from("1"));
        assert_eq!(Thread64(0x3f), Thread64::from("#"));
        assert_eq!(Thread64(0x3f00), Thread64::from("0#0"));

        assert_eq!(Thread64(0x0001020304050607), Thread64::from("01234567"));
        assert_eq!(Thread64(0x08090a0b0c0d0e0f), Thread64::from("89abcdef"));
        assert_eq!(Thread64(0x1011121314151617), Thread64::from("ghijklmn"));
        assert_eq!(Thread64(0x18191a1b1c1d1e1f), Thread64::from("opqrstuv"));
        assert_eq!(Thread64(0x2021222324252627), Thread64::from("wxyzABCD"));
        assert_eq!(Thread64(0x28292a2b2c2d2e2f), Thread64::from("EFGHIJKL"));
        assert_eq!(Thread64(0x3031323334353637), Thread64::from("MNOPQRST"));
        assert_eq!(Thread64(0x38393a3b3c3d3e3f), Thread64::from("UVWXYZ$#"));
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

        assert_eq!(Thread64(0x01020304050607_11), Thread64(0x01020304050607_10).next(0, 1));
        assert_eq!(Thread64(0x01020304050607_00), Thread64(0x01020304050607_11).next(0, 1));
        assert_eq!(Thread64(0x01020304050607_13), Thread64(0x01020304050607_12).next(0, 1));
        assert_eq!(Thread64(0x01020304050607_00), Thread64(0x01020304050607_13).next(0, 1));
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

        assert_eq!(Thread64(0x31323334353637_3d), Thread64(0x31323334353637_3c).next(0, 6));
        assert_eq!(Thread64(0x31323334353637_3e), Thread64(0x31323334353637_3d).next(0, 6));
        assert_eq!(Thread64(0x31323334353637_3f), Thread64(0x31323334353637_3e).next(0, 6));
        assert_eq!(Thread64(0x31323334353637_00), Thread64(0x31323334353637_3f).next(0, 6));
    }

    #[test]
    fn thread64_next_1_1() {
        assert_eq!(Thread64(0x3f_00_3f), Thread64(0x3f_00_3f).next(1, 1));
        assert_eq!(Thread64(0x3f_00_3f), Thread64(0x3f_01_3f).next(1, 1));

        assert_eq!(Thread64(0x3f_03_3f), Thread64(0x3f_02_3f).next(1, 1));
        assert_eq!(Thread64(0x3f_00_3f), Thread64(0x3f_03_3f).next(1, 1));
        assert_eq!(Thread64(0x3f_05_3f), Thread64(0x3f_04_3f).next(1, 1));
        assert_eq!(Thread64(0x3f_00_3f), Thread64(0x3f_05_3f).next(1, 1));
        assert_eq!(Thread64(0x3f_21_3f), Thread64(0x3f_20_3f).next(1, 1));
        assert_eq!(Thread64(0x3f_00_3f), Thread64(0x3f_21_3f).next(1, 1));

        assert_eq!(Thread64(0), Thread64(0x0).next(1, 1));
        assert_eq!(Thread64(0x1), Thread64(0x1).next(1, 1));

        assert_eq!(Thread64(0x313233343536_11_38), Thread64(0x313233343536_10_38).next(1, 1));
        assert_eq!(Thread64(0x313233343536_00_38), Thread64(0x313233343536_11_38).next(1, 1));
        assert_eq!(Thread64(0x313233343536_13_38), Thread64(0x313233343536_12_38).next(1, 1));
        assert_eq!(Thread64(0x313233343536_00_38), Thread64(0x313233343536_13_38).next(1, 1));
    }

    #[test]
    fn thread64_next_7_1() {
        assert_eq!(Thread64(0x00_3f3f3f_3f3f3f3f), Thread64(0x00_3f3f3f_3f3f3f3f).next(7, 1));
        assert_eq!(Thread64(0x00_3f3f3f_3f3f3f3f), Thread64(0x00_3f3f3f_3f3f3f3f).next(7, 1));

        assert_eq!(Thread64(0x03_3f3f3f_3f3f3f3f), Thread64(0x02_3f3f3f_3f3f3f3f).next(7, 1));
        assert_eq!(Thread64(0x00_3f3f3f_3f3f3f3f), Thread64(0x03_3f3f3f_3f3f3f3f).next(7, 1));
        assert_eq!(Thread64(0x05_3f3f3f_3f3f3f3f), Thread64(0x04_3f3f3f_3f3f3f3f).next(7, 1));
        assert_eq!(Thread64(0x00_3f3f3f_3f3f3f3f), Thread64(0x05_3f3f3f_3f3f3f3f).next(7, 1));
        assert_eq!(Thread64(0x21_3f3f3f_3f3f3f3f), Thread64(0x20_3f3f3f_3f3f3f3f).next(7, 1));
        assert_eq!(Thread64(0x00_3f3f3f_3f3f3f3f), Thread64(0x21_3f3f3f_3f3f3f3f).next(7, 1));

        assert_eq!(Thread64(0), Thread64(0x0).next(1, 1));
        assert_eq!(Thread64(0x1), Thread64(0x1).next(1, 1));

        assert_eq!(Thread64(0x11_3f3f3f_3f3f3f3f), Thread64(0x10_3f3f3f_3f3f3f3f).next(1, 1));
        assert_eq!(Thread64(0x00_3f3f3f_3f3f3f3f), Thread64(0x11_3f3f3f_3f3f3f3f).next(1, 1));
        assert_eq!(Thread64(0x13_3f3f3f_3f3f3f3f), Thread64(0x12_3f3f3f_3f3f3f3f).next(1, 1));
        assert_eq!(Thread64(0x00_3f3f3f_3f3f3f3f), Thread64(0x13_3f3f3f_3f3f3f3f).next(1, 1));
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
    }

    #[test]
    fn thread64_rand_radix() {
        let mut rand = Rand32(42);

        assert_eq!(Thread64::from("pCPx#l#j"), Thread64::rand(&mut rand, 8, 6, 0));
        assert_eq!(Thread64::from("FpzXVIfC"), Thread64::rand(&mut rand, 8, 6, 0));
        assert_eq!(Thread64::from("zkyj7YNK"), Thread64::rand(&mut rand, 8, 6, 0));
        assert_eq!(Thread64::from("l8iobxeA"), Thread64::rand(&mut rand, 8, 6, 0));

        assert_eq!(Thread64::from("n#x92kIn"), Thread64::rand(&mut rand, 8, 6, 0));
        assert_eq!(Thread64::from("A3rlJRoA"), Thread64::rand(&mut rand, 8, 6, 0));
        assert_eq!(Thread64::from("FqRBNUHO"), Thread64::rand(&mut rand, 8, 6, 0));
        assert_eq!(Thread64::from("i3CD3EHF"), Thread64::rand(&mut rand, 8, 6, 0));

        assert_eq!(Thread64::from("11111111"), Thread64::rand(&mut rand, 8, 1, 0));

        assert_eq!(Thread64::from("13311321"), Thread64::rand(&mut rand, 8, 2, 0));
        assert_eq!(Thread64::from("33312111"), Thread64::rand(&mut rand, 8, 2, 0));

        assert_eq!(Thread64::from("37613527"), Thread64::rand(&mut rand, 8, 3, 0));
        assert_eq!(Thread64::from("52761115"), Thread64::rand(&mut rand, 8, 3, 0));
        assert_eq!(Thread64::from("73112557"), Thread64::rand(&mut rand, 8, 3, 0));
        assert_eq!(Thread64::from("74367512"), Thread64::rand(&mut rand, 8, 3, 0));

        assert_eq!(Thread64::from("29d7d1cf"), Thread64::rand(&mut rand, 8, 4, 0));
        assert_eq!(Thread64::from("edc413f9"), Thread64::rand(&mut rand, 8, 4, 0));
        assert_eq!(Thread64::from("ca9e3495"), Thread64::rand(&mut rand, 8, 4, 0));
        assert_eq!(Thread64::from("a1c9e76f"), Thread64::rand(&mut rand, 8, 4, 0));
    }

    #[test]
    fn thread64_rand_seq() {
        let mut rand = Rand32(42);

        assert_eq!(Thread64::from("11111111"), Thread64::rand(&mut rand, 8, 1, 0));
        assert_eq!(Thread64::from("22222222"), Thread64::rand(&mut rand, 8, 2, 1));

        assert_eq!(Thread64::from("46242264"), Thread64::rand(&mut rand, 8, 3, 1));
        assert_eq!(Thread64::from("66244664"), Thread64::rand(&mut rand, 8, 3, 1));
        assert_eq!(Thread64::from("44444444"), Thread64::rand(&mut rand, 8, 3, 2));

        assert_eq!(Thread64::from("8a8244ee"), Thread64::rand(&mut rand, 8, 4, 1));
        assert_eq!(Thread64::from("a8684c26"), Thread64::rand(&mut rand, 8, 4, 1));
        assert_eq!(Thread64::from("c48c4488"), Thread64::rand(&mut rand, 8, 4, 2));
        assert_eq!(Thread64::from("88888888"), Thread64::rand(&mut rand, 8, 4, 3));

        assert_eq!(Thread64::from("8242m4qm"), Thread64::rand(&mut rand, 8, 5, 1));
        assert_eq!(Thread64::from("8g4quig8"), Thread64::rand(&mut rand, 8, 5, 1));
        assert_eq!(Thread64::from("sssg8k4o"), Thread64::rand(&mut rand, 8, 5, 2));
        assert_eq!(Thread64::from("8o88og88"), Thread64::rand(&mut rand, 8, 5, 3));
        assert_eq!(Thread64::from("gggggggg"), Thread64::rand(&mut rand, 8, 5, 4));

        assert_eq!(Thread64::from("IaU$OAEk"), Thread64::rand(&mut rand, 8, 6, 1));
        assert_eq!(Thread64::from("qMYouS6$"), Thread64::rand(&mut rand, 8, 6, 1));
        assert_eq!(Thread64::from("gkIoYgII"), Thread64::rand(&mut rand, 8, 6, 2));
        assert_eq!(Thread64::from("gEogEUog"), Thread64::rand(&mut rand, 8, 6, 3));
        assert_eq!(Thread64::from("MMwgggMg"), Thread64::rand(&mut rand, 8, 6, 4));
        assert_eq!(Thread64::from("wwwwwwww"), Thread64::rand(&mut rand, 8, 6, 5));
    }
}