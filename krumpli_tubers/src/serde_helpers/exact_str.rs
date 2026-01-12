// SPDX-License-Identifier: GPL-3.0-or-later

//! Strings for which we know the exact length.
//!
//! Exact sized strings are useful to avoid allocating when we know the length of a string
//! beforehand. It also acts as a parser because invalid lengths are rejected. Unlike
//! [`smol_str::SmolStr`], exact sized strings can be even smaller because accounting for possible
//! heap allocs isn't needed.

use core::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use serde::{
    Deserialize, Deserializer,
    de::{Error as DeError, Visitor},
};

/// Emoji flags length.
///
/// Additional regions, like U.S. states, are represented differently. I'm pretty sure I'll never
/// have to deal with that for these APIs.
///
/// https://en.wikipedia.org/wiki/Regional_indicator_symbol
pub const FLAG_LEN: usize = {
    let us = "\u{1f1fa}\u{1f1f8}";
    us.len()
};

pub const LANG_LEN: usize = 2;
pub const REGION_LEN: usize = 2;

/// A string that is always `LEN` bytes and stack allocated.
pub struct ExactStr<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> ExactStr<LEN> {
    /// Create a new [`ExactStr`] from a string.
    pub const fn new(s: &str) -> Result<Self, ExactStrErr> {
        if s.len() != LEN {
            Err(ExactStrErr::InvalidLen {
                expected: LEN,
                actual: s.len(),
            })
        } else {
            let mut buf = [0u8; LEN];
            buf.copy_from_slice(s.as_bytes());
            Ok(Self(buf))
        }
    }

    /// Return a [`str`] slice of this [`ExactStr`].
    pub const fn as_str(&self) -> &str {
        // SAFETY:
        // * The internal buffer is private and always checked to be unicode.
        unsafe { str::from_utf8_unchecked(&self.0) }
    }
}

impl<const LEN: usize> Display for ExactStr<LEN> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl<const LEN: usize> Debug for ExactStr<LEN> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl<const LEN: usize> FromStr for ExactStr<LEN> {
    type Err = ExactStrErr;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl<const LEN: usize> AsRef<str> for ExactStr<LEN> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'de, const LEN: usize> Deserialize<'de> for ExactStr<LEN> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExactSizeVisitor<const LEN: usize>;
        impl<'de, const LEN: usize> Visitor<'de> for ExactSizeVisitor<LEN> {
            type Value = ExactStr<LEN>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(formatter, "a string with length {LEN}")
            }

            fn visit_borrowed_str<E>(self, s: &'de str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                ExactStr::new(s).map_err(DeError::custom)
            }
        }

        deserializer.deserialize_str(ExactSizeVisitor)
    }
}

/// Errors that can occur while creating an [`ExactStr`].
#[derive(Debug, PartialEq)]
pub enum ExactStrErr {
    InvalidLen { expected: usize, actual: usize },
    InvalidUnicode,
}

impl Error for ExactStrErr {}

impl Display for ExactStrErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUnicode => write!(f, "Bytes are invalid Unicode"),
            Self::InvalidLen { expected, actual } => {
                write!(f, "Expected a string of length {expected} but got {actual}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ExactStr, ExactStrErr};

    #[test]
    fn valid_size_parses() -> Result<(), ExactStrErr> {
        let expected = "potato";
        let actual: ExactStr<6> = expected.parse()?;

        assert_eq!(expected, actual.as_str());
        Ok(())
    }

    #[test]
    fn too_small_len_fails() {
        let potato = "krumpli";
        let err = potato
            .parse::<ExactStr<0>>()
            .expect_err("'krumpli' can't fit into a buffer of length 0 so parsing should fail");

        assert_eq!(
            ExactStrErr::InvalidLen {
                expected: 0,
                actual: potato.len()
            },
            err
        );
    }

    #[test]
    fn too_large_len_fails() {
        let olive = "olviabogyo";
        let err = olive.parse::<ExactStr<128>>().expect_err(
            "The buffer is larger than needed to store 'olviabogyo' so parsing should fail",
        );

        assert_eq!(
            ExactStrErr::InvalidLen {
                expected: 128,
                actual: olive.len()
            },
            err
        );
    }
}
