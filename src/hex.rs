// Copyright (c) 2013-2019 The Rust Project Developers.
use std::fmt;

/// A trait for converting a value to hexadecimal encoding
pub trait ToHex {
    /// Converts the value of `self` to a hex value, returning the owned
    /// string.
    fn to_hex(&self) -> String;
}

const CHARS: &[u8] = b"0123456789abcdef";

// const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";
// const HEX_CHARS_UPPER: &[u8; 16] = b"0123456789ABCDEF";

impl ToHex for [u8] {
    fn to_hex(&self) -> String {
        let mut v = Vec::with_capacity(self.len() * 2);
        for &byte in self {
            v.push(CHARS[(byte >> 4) as usize]);
            v.push(CHARS[(byte & 0xf) as usize]);
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}

pub trait FromHex: Sized {
    type Error;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error>;
}

/// Errors that can occur when decoding a hex encoded string
#[derive(Copy, Clone, Debug)]
pub enum FromHexError {
    /// The input contained a character not part of the hex format
    InvalidHexCharacter {
        c: char,
        index: usize,
    },
    /// The input had an invalid length
    InvalidHexLength,
    OddLength,
}

impl std::error::Error for FromHexError {
    fn description(&self) -> &str {
        match *self {
            Self::InvalidHexCharacter { .. } => "invalid character",
            Self::OddLength => "odd number of digits",
            Self::InvalidHexLength => "invalid hex length",
        }
    }
}

impl fmt::Display for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidHexCharacter { c, index } => {
                write!(f, "Invalid character '{}' at position {}", c, index)
            }
            Self::OddLength => write!(f, "Odd number of digits"),
            Self::InvalidHexLength => write!(f, "Invalid hex length"),
        }
    }
}

fn val(c: u8, idx: usize) -> Result<u8, FromHexError> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(FromHexError::InvalidHexCharacter {
            c: c as char,
            index: idx,
        }),
    }
}

impl FromHex for Vec<u8> {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, FromHexError> {
        let hex = hex.as_ref();
        if hex.len() % 2 != 0 {
            return Err(FromHexError::OddLength);
        }

        hex.chunks(2)
            .enumerate()
            .map(|(i, pair)| Ok(val(pair[0], 2 * i)? << 4 | val(pair[1], 2 * i + 1)?))
            .collect()
    }
}
