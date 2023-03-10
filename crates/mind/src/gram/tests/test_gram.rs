use crate::gram::{Gram,Digit::*, gram};

const NIL: u8 = 0x3f; 
const LOW: u8 = 0x00; 
const MED: u8 = 0x40; 
const HIGH: u8 = 0x80; 
const MAX: u8 = 0xc0; 

#[test]
fn basic_format() {
    let mut gram = Gram::new();
    gram.push_u8(0);
    assert_eq!(format!("{}", gram), "?0");
    assert_eq!(format!("{:?}", gram), "Gram(\"?0\")");

    let mut gram = Gram::new();
    gram.push_u8(0x40);
    assert_eq!(format!("{}", gram), "0");
    assert_eq!(format!("{:?}", gram), "Gram(\"0\")");

    let mut gram = Gram::new();
    gram.push_u8(0x80);
    assert_eq!(format!("{}", gram), "+0");
    assert_eq!(format!("{:?}", gram), "Gram(\"+0\")");

    let mut gram = Gram::new();
    gram.push_u8(0xc0);
    assert_eq!(format!("{}", gram), "!0");
    assert_eq!(format!("{:?}", gram), "Gram(\"!0\")");
}

#[test]
fn display_single_digit() {
    for i in 0..0xff {
        let gram = from_u8(u8::try_from(i).expect("invalid index"));

        assert_eq!(format!("{}", gram), format!("{}", tostr(i)));
    }
}

#[test]
fn debug_single_digit() {
    for i in 0..0xff {
        let gram = from_u8(u8::try_from(i).expect("invalid index"));

        assert_eq!(format!("{:?}", gram), format!("Gram(\"{}\")", tostr(i)));
    }
}

#[test]
fn as_bytes_single_digit() {
    for i in 0..=0xff {
        //let gram = from_u8(u8::try_from(i).expect("invalid index"));
        let gram = Gram::from([i]);

        assert_eq!(gram.as_bytes(), [i]);
    }
}

#[test]
fn from_weight() {
    assert_eq!(Gram::from(Nil).as_bytes(), [NIL]);

    assert_eq!(Gram::from(Low(0)).as_bytes(), [0 + LOW]);
    assert_eq!(Gram::from(Low(0x3e)).as_bytes(), [0x3e + LOW]);

    assert_eq!(Gram::from(Med(0)).as_bytes(), [0 + MED]);
    assert_eq!(Gram::from(Med(0x3e)).as_bytes(), [0x3e + MED]);

    assert_eq!(Gram::from(High(0)).as_bytes(), [0 + HIGH]);
    assert_eq!(Gram::from(High(0x3e)).as_bytes(), [0x3e + HIGH]);

    assert_eq!(Gram::from(Max(0)).as_bytes(), [0 + MAX]);
    assert_eq!(Gram::from(Max(0x3e)).as_bytes(), [0x3e + MAX]);
}

#[test]
fn from_vec_u8() {
    for i in 0..=0xff {
        let gram = Gram::from([i]);
        let vec = Vec::<u8>::from([i]);

        assert_eq!(gram.as_bytes(), vec);
    }
}

#[test]
fn from_vec_obj_u8() {
    for i in 0..=0xff {
        let vec_src = Vec::<u8>::from([i]);
        let gram = Gram::from(vec_src);
        let vec = Vec::<u8>::from([i]);

        assert_eq!(gram.as_bytes(), vec);
    }
}

#[test]
fn string_from() {
    for i in 0..=0xff {
        let gram = Gram::from([i]);

        assert_eq!(String::from(gram), tostr(i));
    }
}

#[test]
fn from_string_single() {
    for i in 0..=0xff {
        if i == 0xff || i == 0x7f || i == 0xbf {
            continue
        }

        let gram = Gram::from(tostr(i));

        assert_eq!(gram.as_bytes(), [i]);
    }
}

#[test]
fn from_str_single() {
    for i in 0..=0xff {
        if i == 0xff || i == 0x7f || i == 0xbf {
            continue
        }

        let val: &str = &tostr(i);

        let gram = Gram::from(val);

        assert_eq!(gram.as_bytes(), [i]);
    }
}

#[test]
fn from_str_slice() {
    assert_eq!(Gram::from(&"01234"[1..=3]).as_bytes(), [65, 66, 67]);
}

#[test]
fn eq() {
    assert_eq!(Gram::from("a"), Gram::from("a"));
    assert_eq!(Gram::from("0"), Gram::from(0));
    assert_eq!(Gram::from("."), Gram::from("."));
    assert_eq!(Gram::from("09azAZ-"), Gram::from("09azAZ-"));

    assert_ne!(Gram::from("a"), Gram::from("b"));
    assert_ne!(Gram::from("a"), Gram::from("."));
    assert_ne!(Gram::from("."), Gram::from("Z"));
    assert_ne!(Gram::from("0123"), Gram::from("01234"));
    assert_ne!(Gram::from("01234"), Gram::from("0123"));

    assert_ne!(Gram::from("?0"), Gram::from("0"));
    assert_ne!(Gram::from("0"), Gram::from("?0"));
    assert_ne!(Gram::from("+0"), Gram::from("0"));
    assert_ne!(Gram::from("!0"), Gram::from("+0"));

    assert_ne!(Gram::from(0), Gram::from(32));
}

#[test]
fn base_eq() {
    assert!(Gram::from("a").base_eq(&Gram::from("a")));
    assert!(! Gram::from("a").base_eq(&Gram::from("0")));
    assert!(! Gram::from("0").base_eq(&Gram::from("a")));

    assert!(! Gram::from("a").base_eq(&Gram::from("ab")));
    assert!(! Gram::from("ab").base_eq(&Gram::from("a")));
    assert!(Gram::from("ab").base_eq(&Gram::from("ab")));

    assert!(Gram::from("a").base_eq(&Gram::from("a")));
    assert!(Gram::from("a").base_eq(&Gram::from("?a")));
    assert!(Gram::from("a").base_eq(&Gram::from("+a")));
    assert!(Gram::from("a").base_eq(&Gram::from("!a")));

    assert!(Gram::from("?a").base_eq(&Gram::from("a")));
    assert!(Gram::from("?a").base_eq(&Gram::from("?a")));
    assert!(Gram::from("?a").base_eq(&Gram::from("+a")));
    assert!(Gram::from("?a").base_eq(&Gram::from("!a")));

    assert!(Gram::from("+a").base_eq(&Gram::from("a")));
    assert!(Gram::from("+a").base_eq(&Gram::from("?a")));
    assert!(Gram::from("+a").base_eq(&Gram::from("+a")));
    assert!(Gram::from("+a").base_eq(&Gram::from("!a")));

    assert!(Gram::from("!a").base_eq(&Gram::from("a")));
    assert!(Gram::from("!a").base_eq(&Gram::from("?a")));
    assert!(Gram::from("!a").base_eq(&Gram::from("+a")));
    assert!(Gram::from("!a").base_eq(&Gram::from("!a")));

    assert!(! Gram::from(0).base_eq(&Gram::from(32)));
    assert!(! Gram::from(32).base_eq(&Gram::from(0)));
}

#[test]
fn clone() {
    assert_eq!(Gram::from("a").clone(), Gram::from("a"));
}

#[test]
fn weight() {
    assert_eq!(gram("").weight(), 1.);

    assert_eq!(gram("a").weight(), 1.);
    assert_eq!(gram("0").weight(), 1.);
    assert_eq!(Gram::from(0).weight(), 1.);
    assert_eq!(Gram::from(32).weight(), 1.);
    assert_eq!(gram(".").weight(), 0.);

    assert_eq!(gram("012345689").weight(), 1.);

    assert_eq!(gram("?a").weight(), 0.5);
    assert_eq!(gram("?a?b?c?d").weight(), 0.5);

    assert_eq!(gram("+a").weight(), 2.);
    assert_eq!(gram("+a+b+c+d").weight(), 2.);

    assert_eq!(gram("!a").weight(), 4.);
    assert_eq!(gram("!a!b!c!d").weight(), 4.);

    assert_eq!(gram("?abcb").weight(), 0.875);
    assert_eq!(gram("a?bcb").weight(), 0.875);
    assert_eq!(gram("ab?cb").weight(), 0.875);
    assert_eq!(gram("abc?b").weight(), 0.875);
    assert_eq!(gram("?abc?b").weight(), 0.75);
    assert_eq!(gram("?a?bc?b").weight(), 0.625);

    assert_eq!(gram("+abcb").weight(), 1.25);
    assert_eq!(gram("a+bcb").weight(), 1.25);
    assert_eq!(gram("ab+cb").weight(), 1.25);
    assert_eq!(gram("abc+b").weight(), 1.25);
    assert_eq!(gram("+abc+b").weight(), 1.5);
    assert_eq!(gram("+a+bc+b").weight(), 1.75);

    assert_eq!(gram("!abcb").weight(), 1.75);
    assert_eq!(gram("a!bcb").weight(), 1.75);
    assert_eq!(gram("ab!cb").weight(), 1.75);
    assert_eq!(gram("abc!b").weight(), 1.75);
    assert_eq!(gram("!abc!b").weight(), 2.5);
    assert_eq!(gram("!a!bc!b").weight(), 3.25);
}

fn from_u8(v: u8) -> Gram {
    let mut gram = Gram::new();

    gram.push_u8(v);

    gram
}

fn tostr(v: u8) -> String {
    let weight = v >> 6;
    let digit = v & 0x3f;

    let prefix = match weight {
        0 => "?",
        1 => "",
        2 => "+",
        3 => "!",
        _ => "*",
    };

    if digit < 10 {
        format!("{}{}", prefix, (digit + b'0') as char)
    } else if digit < 36 {
        format!("{}{}", prefix, char::from(digit - 10 + b'a'))
    } else if digit < 62 {
        format!("{}{}", prefix, char::from(digit - 36 + b'A'))
    } else if digit == 62 {
        format!("{}-", prefix)
    } else {
        String::from(".")
    }
}
