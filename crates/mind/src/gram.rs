use std::{fmt};

pub struct Gram {
    vec: Vec<u8>,
}

impl Gram {
    pub fn new() -> Self {
        Gram { 
            vec: Vec::new(),
        }
    }

    pub fn push(&mut self, value: u8) {
        self.vec.push(value);
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
        str_to_gram(&value)
        /*
        let mut vec = Vec::<u8>::new();

        let mut size: u8 = 0;

        for ch in value.chars() {
            match ch {
                '.' => { vec.push(0xff); },
                '+' => { assert!(size == 0); size = 1 << 6; },
                '=' => { assert!(size == 0); size = 2 << 6; },
                ':' => { assert!(size == 0); size = 3 << 6; },
                '0'..='9' => { vec.push(ch as u8 - b'0' + size); size = 0; },
                'a'..='z' => { vec.push(ch as u8 - b'a' + size + 10); size = 0; },
                'A'..='Z' => { vec.push(ch as u8 - b'A' + size + 36); size = 0; },
                '-' => { vec.push(size + 62); size = 0; },
                _ => { panic!("{} is an invalid Gram character.", ch)}
            }

        }

        Gram { vec: vec }
        */
    }
}

impl From<String> for Gram {
    fn from(value: String) -> Self {
        str_to_gram(&value)
        /*
        let mut vec = Vec::<u8>::new();

        let mut size: u8 = 0;

        for ch in value.chars() {
            match ch {
                '.' => { vec.push(0xff); },
                '+' => { assert!(size == 0); size = 1 << 6; },
                '=' => { assert!(size == 0); size = 2 << 6; },
                ':' => { assert!(size == 0); size = 3 << 6; },
                '0'..='9' => { vec.push(ch as u8 - b'0' + size); size = 0; },
                'a'..='z' => { vec.push(ch as u8 - b'a' + size + 10); size = 0; },
                'A'..='Z' => { vec.push(ch as u8 - b'A' + size + 36); size = 0; },
                '-' => { vec.push(size + 62); size = 0; },
                _ => { panic!("{} is an invalid Gram character.", ch)}
            }

        }

        Gram { vec: vec }
        */
    }
}

fn str_to_gram(value: &str) -> Gram {
    let mut vec = Vec::<u8>::new();

    let mut size: u8 = 0;

    for ch in value.chars() {
        match ch {
            '.' => { vec.push(0xff); },
            '+' => { assert!(size == 0); size = 1 << 6; },
            '=' => { assert!(size == 0); size = 2 << 6; },
            ':' => { assert!(size == 0); size = 3 << 6; },
            '0'..='9' => { vec.push(ch as u8 - b'0' + size); size = 0; },
            'a'..='z' => { vec.push(ch as u8 - b'a' + size + 10); size = 0; },
            'A'..='Z' => { vec.push(ch as u8 - b'A' + size + 36); size = 0; },
            '-' => { vec.push(size + 62); size = 0; },
            _ => { panic!("{} is an invalid Gram character.", ch)}
        }

    }

    Gram { vec: vec }
}

impl From<Gram> for String {
    fn from(value: Gram) -> Self {
        format!("{}", value)
    }
}

const U8_TO_STR: &'static [&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7",
    "8", "9", "a", "b", "c", "d", "e", "f",
    "g", "h", "i", "j", "k", "l", "m", "n",
    "o", "p", "q", "r", "s", "t", "u", "v",
    "w", "x", "y", "z", "A", "B", "C", "D",
    "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T",
    "U", "V", "W", "X", "Y", "Z", "-", "?",

    "+0", "+1", "+2", "+3", "+4", "+5", "+6", "+7",
    "+8", "+9", "+a", "+b", "+c", "+d", "+e", "+f",
    "+g", "+h", "+i", "+j", "+k", "+l", "+m", "+n",
    "+o", "+p", "+q", "+r", "+s", "+t", "+u", "+v",
    "+w", "+x", "+y", "+z", "+A", "+B", "+C", "+D",
    "+E", "+F", "+G", "+H", "+I", "+J", "+K", "+L",
    "+M", "+N", "+O", "+P", "+Q", "+R", "+S", "+T",
    "+U", "+V", "+W", "+X", "+Y", "+Z", "+-", "+?",

    "=0", "=1", "=2", "=3", "=4", "=5", "=6", "=7",
    "=8", "=9", "=a", "=b", "=c", "=d", "=e", "=f",
    "=g", "=h", "=i", "=j", "=k", "=l", "=m", "=n",
    "=o", "=p", "=q", "=r", "=s", "=t", "=u", "=v",
    "=w", "=x", "=y", "=z", "=A", "=B", "=C", "=D",
    "=E", "=F", "=G", "=H", "=I", "=J", "=K", "=L",
    "=M", "=N", "=O", "=P", "=Q", "=R", "=S", "=T",
    "=U", "=V", "=W", "=X", "=Y", "=Z", "=-", "=?",

    ":0", ":1", ":2", ":3", ":4", ":5", ":6", ":7",
    ":8", ":9", ":a", ":b", ":c", ":d", ":e", ":f",
    ":g", ":h", ":i", ":j", ":k", ":l", ":m", ":n",
    ":o", ":p", ":q", ":r", ":s", ":t", ":u", ":v",
    ":w", ":x", ":y", ":z", ":A", ":B", ":C", ":D",
    ":E", ":F", ":G", ":H", ":I", ":J", ":K", ":L",
    ":M", ":N", ":O", ":P", ":Q", ":R", ":S", ":T",
    ":U", ":V", ":W", ":X", ":Y", ":Z", ":-", ".",
];
