// SPDX-License-Identifier: GPL-3.0-or-later

//! Media Type (MIME) zero copy deserialization wrapper.

use std::{
    fmt::{self, Formatter},
    str::FromStr,
};

use mime::Mime;
use serde::{
    Deserializer,
    de::{Error as DeError, Visitor},
};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Mime, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(MimeVisitor)
}

struct MimeVisitor;

impl<'de> Visitor<'de> for MimeVisitor {
    type Value = Mime;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "a MIME type string")
    }

    fn visit_borrowed_str<E>(self, mime: &'de str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Mime::from_str(mime).map_err(DeError::custom)
    }
}
