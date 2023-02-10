use mind::Gram;

#[test]
fn basic_format() {
    let mut gram = Gram::new();
    gram.push(0);
    
    assert_eq!(format!("{}", gram), "0");
    assert_eq!(format!("{:?}", gram), "g\"0\"");
}

#[test]
fn as_bytes_single_digit() {
    for i in 0..0xff {
        let gram = from_u8(u8::try_from(i).expect("invalid index"));

        assert_eq!(gram.as_bytes(), Vec::<u8>::from([i]));
    }
}

#[test]
fn from_vec_u8() {
    for i in 0..0xff {
        let gram = Gram::from([i]);
        let vec = Vec::<u8>::from([i]);

        assert_eq!(gram.as_bytes(), vec);
    }
}

#[test]
fn from_vec_obj_u8() {
    for i in 0..0xff {
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
        if i == 0x3f || i == 0x7f || i == 0xbf {
            continue
        }

        let gram = Gram::from(tostr(i));

        assert_eq!(gram.as_bytes(), [i]);
    }
}

#[test]
fn from_str_single() {
    for i in 0..=0xff {
        if i == 0x3f || i == 0x7f || i == 0xbf {
            continue
        }

        let val: &str = &tostr(i);

        let gram = Gram::from(val);

        assert_eq!(gram.as_bytes(), [i]);
    }
}

#[test]
fn from_str_slice() {
    assert_eq!(Gram::from(&"01234"[1..=3]).as_bytes(), [1, 2, 3]);
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

        assert_eq!(format!("{:?}", gram), format!("g\"{}\"", tostr(i)));
    }
}

#[test]
fn eq() {
    assert_eq!(Gram::from("a"), Gram::from("a"));
    assert_eq!(Gram::from("0"), Gram::from([0]));
    assert_eq!(Gram::from("."), Gram::from("."));
    assert_eq!(Gram::from("09azAZ-"), Gram::from("09azAZ-"));

    assert_ne!(Gram::from("a"), Gram::from("b"));
    assert_ne!(Gram::from("a"), Gram::from("."));
    assert_ne!(Gram::from("."), Gram::from("Z"));
    assert_ne!(Gram::from("0123"), Gram::from("01234"));
    assert_ne!(Gram::from("01234"), Gram::from("0123"));

    assert_ne!(Gram::from("+0"), Gram::from("0"));
    assert_ne!(Gram::from("=0"), Gram::from("0"));
    assert_ne!(Gram::from(":0"), Gram::from("0"));
    assert_ne!(Gram::from("0"), Gram::from("+0"));
}

#[test]
fn clone() {
    assert_eq!(Gram::from("a").clone(), Gram::from("a"));
}

fn from_u8(v: u8) -> Gram {
    let mut gram = Gram::new();

    gram.push(v);

    gram
}

fn tostr(v: u8) -> String {
    let weight = v >> 6;
    let digit = v & 0x3f;

    let prefix = match weight {
        0 => "",
        1 => "+",
        2 => "=",
        3 => ":",
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
    } else if weight == 3 {
        String::from(".")
    } else {
        format!("{}?", prefix)
    }
}
