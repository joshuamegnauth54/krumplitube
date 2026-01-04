// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Deserializer};

/// Deserialize [`Option<bool>`] into [`bool`].
#[inline]
pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<bool>::deserialize(deserializer).map(Option::unwrap_or_default)
}
