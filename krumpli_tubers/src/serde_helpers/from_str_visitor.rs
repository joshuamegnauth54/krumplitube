// SPDX-License-Identifier: GPL-3.0-or-later

use core::{fmt, marker::PhantomData, str::FromStr};

use serde::de::{Error as DeError, Visitor};

/// Deserialize a struct implementing [`FromStr`] without allocating a temporary String.
///
/// (I copied this from a PR I submitted to another project).
pub(crate) struct FromStrVisitor<F>(pub PhantomData<F>);

impl<'de, F> Visitor<'de> for FromStrVisitor<F>
where
    F: FromStr,
    F::Err: fmt::Display,
    F: Expecting,
{
    type Value = F;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", F::EXPECTING)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        F::from_str(v).map_err(DeError::custom)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        F::from_str(v).map_err(DeError::custom)
    }
}

pub trait Expecting {
    const EXPECTING: &'static str;
}
