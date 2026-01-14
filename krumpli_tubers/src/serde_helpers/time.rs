// SPDX-License-Identifier: GPL-3.0-or-later

//! Helpers to deserialize [`Duration`].
//!
//! These are designed to be fallible since deserializing the correct times are not too important.
//! I hope.

use std::time::Duration;

use serde::{Deserialize, Deserializer};

/// Deserialize seconds into a [`Duration`].
pub fn from_secs<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    u64::deserialize(deserializer)
        .map(Duration::from_secs)
        .or(Ok(Duration::ZERO))
}

/// Deserialize seconds into an [`Option<Duration>`].
pub fn from_secs_opt<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<u64>::deserialize(deserializer).map(|opt| Duration::from_secs(opt.unwrap_or_default()))
}
