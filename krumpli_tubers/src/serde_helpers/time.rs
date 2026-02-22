// SPDX-License-Identifier: GPL-3.0-or-later

//! Helpers to deserialize [`Duration`].
//!
//! These are designed to be fallible since deserializing the correct times are not too important.
//! I hope.

use std::time::Duration;

use jiff::{Timestamp, Zoned, tz::TimeZone};
use serde::{Deserialize, Deserializer, de::Error as DeErrorT};

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

/// Deserialize i64 to [`Timestamp`] and then make it [`Zoned`].
pub fn timestamp_to_zoned<'de, D>(deserializer: D) -> Result<Zoned, D::Error>
where
    D: Deserializer<'de>,
{
    i64::deserialize(deserializer).and_then(|timestamp| {
        Timestamp::new(timestamp, 0)
            .map(|timestamp| timestamp.to_zoned(TimeZone::UTC))
            .map_err(DeErrorT::custom)
    })
}

/// Deserialize [`Option<i64>`] to [`Option<Zoned>`].
pub fn timestamp_to_zoned_opt<'de, D>(deserializer: D) -> Result<Option<Zoned>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<i64>::deserialize(deserializer)? {
        Some(timestamp) => Ok(Some(
            Timestamp::new(timestamp, 0)
                .map_err(DeErrorT::custom)?
                .to_zoned(TimeZone::UTC),
        )),
        None => Ok(None),
    }
}
