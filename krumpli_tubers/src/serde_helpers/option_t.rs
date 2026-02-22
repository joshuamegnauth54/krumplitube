// SPDX-License-Identifier: GPL-3.0-or-later

//! Helpers for JSON null.
//!
//! `#[serde(default)]` doesn't handle explicit "null"s because it expects an Option.

use serde::{Deserialize, Deserializer};

/// Deserialize [`Option<T>`] into [`Default`].
#[inline]
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Option::unwrap_or_default)
}
