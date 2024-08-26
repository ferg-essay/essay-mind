use std::fmt;

use util::random::Rand64;

use super::Engram128;

#[derive(Clone, Copy)]
pub struct Sequence128 {
    engram: Engram128,

    digits: usize,
    seq: usize,

    offset: usize,
}

impl Sequence128 {
    pub fn new(
        engram: impl Into<Engram128>, 
        digits: usize,
        seq: usize,
    ) -> Self {
        Self {
            engram: engram.into(),
            digits,
            seq,
            offset: 0,
        }
    }

    pub fn rand(
        rand: &mut Rand64,
        digits: usize,
        radix: usize,
        seq: usize,
    ) -> Self {
        Self {
            engram: Engram128::rand(rand, digits, radix, seq),
            digits,
            seq,
            offset: 0,
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.engram.is_zero()
    }

    #[inline]
    pub fn next(&mut self) -> &mut Self {
        self.engram = self.engram.next(self.offset, self.seq);
        self.offset = (self.offset + 1) % self.digits.max(1);

        self
    }
}

impl fmt::Display for Sequence128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.engram.fmt(f)
    }
}

impl fmt::Debug for Sequence128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Sequence128(")?;
        (&self.engram as &dyn fmt::Display).fmt(f)?;
        f.write_str(")")
    }
}

impl PartialEq for Sequence128 {
    fn eq(&self, other: &Self) -> bool {
        self.engram == other.engram 
            && self.digits == other.digits
            && self.seq == other.seq
    }
}

pub struct Sequence128Builder {
    rand: Rand64,

    digits: usize,
    radix: usize,
    seq: usize,
}

impl Sequence128Builder {
    pub fn new(rand: Rand64, digits: usize, radix: usize, seq: usize) -> Self {
        Self {
            rand,
            digits,
            radix,
            seq
        }
    }

    #[inline]
    pub fn next(&mut self) -> Sequence128 {
        Sequence128::rand(&mut self.rand, self.digits, self.radix, self.seq)
    }
}

#[cfg(test)]
mod test {
    use util::random::Rand64;

    use crate::hippocampus::Sequence128;

    #[test]
    fn sequence128_display() {
        assert_eq!(
            "1_23456_789ab_cdefg_hijkl", 
            format!("{}", Sequence128::new("1_23456_789ab_cdefg_hijkl", 21, 0))
        );
    }

    #[test]
    fn sequence128_debug() {
        assert_eq!(
            "Sequence128(1_23456_789ab_cdefg_hijkl)", 
            format!("{:?}", Sequence128::new("1_23456_789ab_cdefg_hijkl", 21, 0))
        );
    }

    #[test]
    fn sequence128_rand() {
        let mut rand = Rand64(42);
        
        assert_eq!(
            Sequence128::new("k_r1Zfm_ATC#P_P65vA_HG1lX", 21, 0), 
            Sequence128::rand(&mut rand, 21, 6, 0),
        );
        
        assert_eq!(
            Sequence128::new("8_51728_8f9e4_a5516_945ad", 21, 0), 
            Sequence128::rand(&mut rand, 21, 4, 0),
        );
        
        assert_eq!(
            Sequence128::new("7bd96_9f688", 10, 0), 
            Sequence128::rand(&mut rand, 10, 4, 0),
        );
        
        assert_eq!(
            Sequence128::new("c_84cc8_88c48_44c8c_cc48c", 21, 2), 
            Sequence128::rand(&mut rand, 21, 4, 2),
        );
    }

    #[test]
    fn sequence128_next() {
        let mut seq = Sequence128::new("0", 1, 2);
        assert_eq!(Sequence128::new("0", 1, 2), *seq.next());
        assert_eq!(Sequence128::new("0", 1, 2), *seq.next());

        let mut seq = Sequence128::new("4", 1, 3);
        assert_eq!(Sequence128::new("5", 1, 3), *seq.next());
        assert_eq!(Sequence128::new("6", 1, 3), *seq.next());
        assert_eq!(Sequence128::new("7", 1, 3), *seq.next());
        assert_eq!(Sequence128::new("0", 1, 3), *seq.next());

        let mut seq = Sequence128::new("8", 1, 3);
        assert_eq!(Sequence128::new("9", 1, 3), *seq.next());
        assert_eq!(Sequence128::new("a", 1, 3), *seq.next());
        assert_eq!(Sequence128::new("b", 1, 3), *seq.next());
        assert_eq!(Sequence128::new("c", 1, 3), *seq.next());
        assert_eq!(Sequence128::new("d", 1, 3), *seq.next());
        assert_eq!(Sequence128::new("e", 1, 3), *seq.next());
        assert_eq!(Sequence128::new("f", 1, 3), *seq.next());
        assert_eq!(Sequence128::new("0", 1, 3), *seq.next());

        let mut seq = Sequence128::new("88", 2, 2);
        assert_eq!(Sequence128::new("89", 2, 2), *seq.next());
        assert_eq!(Sequence128::new("99", 2, 2), *seq.next());
        assert_eq!(Sequence128::new("9a", 2, 2), *seq.next());
        assert_eq!(Sequence128::new("aa", 2, 2), *seq.next());
        assert_eq!(Sequence128::new("ab", 2, 2), *seq.next());
        assert_eq!(Sequence128::new("bb", 2, 2), *seq.next());
        assert_eq!(Sequence128::new("b0", 2, 2), *seq.next());
        assert_eq!(Sequence128::new("0", 2, 2), *seq.next());
        assert_eq!(Sequence128::new("0", 2, 2), *seq.next());

        let mut seq = Sequence128::new("44444_44444", 10, 1);
        assert_eq!(Sequence128::new("44444_44445", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("44444_44455", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("44444_44555", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("44444_45555", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("44444_55555", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("44445_55555", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("44455_55555", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("44555_55555", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("45555_55555", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("55555_55555", 10, 1), *seq.next());

        assert_eq!(Sequence128::new("55555_55550", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("55555_55500", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("55555_55000", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("55555_50000", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("55555_00000", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("55550_00000", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("55500_00000", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("55000_00000", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("50000_00000", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("0", 10, 1), *seq.next());
        assert_eq!(Sequence128::new("0", 10, 1), *seq.next());
    }
}