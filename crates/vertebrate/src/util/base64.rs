use core::fmt;


pub fn base64_unchecked(value: u8) -> char {
    match value {
        0..=9 => ('0' as u8 + value) as char,
        10..=35 => ('a' as u8 + value - 10) as char,
        36..=61 => ('A' as u8 + value - 36) as char,
        62 => '$',
        63 => '#',

        _ => '?'
    }
}

pub fn base64_rev(value: char) -> Result<u8, fmt::Error> {
    match value {
        '0'..='9' => Ok(value as u8 - '0' as u8),
        'a'..='z' => Ok(value as u8 - 'a' as u8 + 10),
        'A'..='Z' => Ok(value as u8 - 'A' as u8 + 36),
        '$' => Ok(62),
        '#' => Ok(63),

        _ => Err(fmt::Error)
    }
}
