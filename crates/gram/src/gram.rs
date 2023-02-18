use std::{fmt};

use crate::{Digit, digit::{NIL, MED, LOW, HIGH, MAX}};

pub struct Gram {
    vec: Vec<u8>,
}

impl Gram {
    pub fn new() -> Self {
        Gram { 
            vec: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn push(&mut self, digit: Digit) {
        self.vec.push(digit.as_u8());
    }

    pub fn push_u8(&mut self, digit: u8) {
        self.vec.push(digit);
    }

    pub fn push_unit(&mut self, value: f32, radix: u8) {
        self.push(Digit::try_from_unit(value, radix))
    }

    pub fn get_unit(&self, index: usize, radix: u8) -> f32 {
        Digit::f32_unit(self.vec[index], radix)
    }

    pub fn get_sunit(&self, index: usize, radix: u8) -> f32 {
        Digit::f32_sunit(self.vec[index], radix)
    }

    fn from_str(value: &str) -> Gram {
        let mut vec = Vec::<u8>::new();
    
        let mut size: u8 = 0x40;
    
        for ch in value.chars() {
            match ch {
                '.' => { vec.push(NIL); size = MED; },
                '?' => { assert!(size == 0); size = LOW; },
                '+' => { assert!(size == 0); size = HIGH; },
                '!' => { assert!(size == 0); size = MAX; },
                '0'..='9' => { vec.push(ch as u8 - b'0' + size); size = MED; },
                'a'..='z' => { vec.push(ch as u8 - b'a' + size + 10); size = MED; },
                'A'..='Z' => { vec.push(ch as u8 - b'A' + size + 36); size = MED; },
                '-' => { vec.push(size + 62); size = 0; },
                '_' => {},
                _ => { panic!("{} is an invalid Gram character.", ch)}
            }
    
        }
    
        Gram { vec: vec }
    }
    
    pub fn as_bytes(&self) -> &[u8] {
        &self.vec
    }
}

impl fmt::Display for Gram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for data in &self.vec {
            f.write_str(U8_TO_STR[usize::from(*data)])?;
        }

        Ok(())
    }
}

impl fmt::Debug for Gram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Gram(\"")?;

        for data in &self.vec {
            f.write_str(U8_TO_STR[usize::from(*data)])?;
        }

        f.write_str("\")")?;

        Ok(())
    }
}

impl Clone for Gram {
    fn clone(&self) -> Self {
        Self { vec: self.vec.clone() }
    }
}

impl PartialEq for Gram {
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
    }
}

impl From<Digit> for Gram {
    fn from(value: Digit) -> Self {
        let mut gram = Gram::new();
        gram.push(Digit::from(value));
        gram
    }
}

impl From<u8> for Gram {
    fn from(value: u8) -> Self {
        let mut gram = Gram::new();
        gram.push(Digit::from(value));
        gram
    }
}

impl<const N: usize> From<[u8; N]> for Gram {
    fn from(value: [u8; N]) -> Self {
        Gram { vec: Vec::from(value) }
    }
}

impl From<Vec<u8>> for Gram {
    fn from(value: Vec<u8>) -> Self {
        Gram { vec: Vec::from(value) }
    }
}

impl From<&str> for Gram {
    fn from(value: &str) -> Self {
        Gram::from_str(&value)
    }
}

impl From<String> for Gram {
    fn from(value: String) -> Self {
        Gram::from_str(&value)
    }
}

impl From<Gram> for String {
    fn from(value: Gram) -> Self {
        format!("{}", value)
    }
}

pub(crate) const U8_TO_STR: &'static [&str] = &[
    "?0", "?1", "?2", "?3", "?4", "?5", "?6", "?7",
    "?8", "?9", "?a", "?b", "?c", "?d", "?e", "?f",
    "?g", "?h", "?i", "?j", "?k", "?l", "?m", "?n",
    "?o", "?p", "?q", "?r", "?s", "?t", "?u", "?v",
    "?w", "?x", "?y", "?z", "?A", "?B", "?C", "?D",
    "?E", "?F", "?G", "?H", "?I", "?J", "?K", "?L",
    "?M", "?N", "?O", "?P", "?Q", "?R", "?S", "?T",
    "?U", "?V", "?W", "?X", "?Y", "?Z", "?-", ".",

    "0", "1", "2", "3", "4", "5", "6", "7",
    "8", "9", "a", "b", "c", "d", "e", "f",
    "g", "h", "i", "j", "k", "l", "m", "n",
    "o", "p", "q", "r", "s", "t", "u", "v",
    "w", "x", "y", "z", "A", "B", "C", "D",
    "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T",
    "U", "V", "W", "X", "Y", "Z", "-", ".",

    "+0", "+1", "+2", "+3", "+4", "+5", "+6", "+7",
    "+8", "+9", "+a", "+b", "+c", "+d", "+e", "+f",
    "+g", "+h", "+i", "+j", "+k", "+l", "+m", "+n",
    "+o", "+p", "+q", "+r", "+s", "+t", "+u", "+v",
    "+w", "+x", "+y", "+z", "+A", "+B", "+C", "+D",
    "+E", "+F", "+G", "+H", "+I", "+J", "+K", "+L",
    "+M", "+N", "+O", "+P", "+Q", "+R", "+S", "+T",
    "+U", "+V", "+W", "+X", "+Y", "+Z", "+-", ".",

    "!0", "!1", "!2", "!3", "!4", "!5", "!6", "!7",
    "!8", "!9", "!a", "!b", "!c", "!d", "!e", "!f",
    "!g", "!h", "!i", "!j", "!k", "!l", "!m", "!n",
    "!o", "!p", "!q", "!r", "!s", "!t", "!u", "!v",
    "!w", "!x", "!y", "!z", "!A", "!B", "!C", "!D",
    "!E", "!F", "!G", "!H", "!I", "!J", "!K", "!L",
    "!M", "!N", "!O", "!P", "!Q", "!R", "!S", "!T",
    "!U", "!V", "!W", "!X", "!Y", "!Z", "!-", ".",
];
