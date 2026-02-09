// SPDX-License-Identifier: GPL-3.0-or-later

//! Types for Invidious' stats endpoint.
//!
//! Version 1.

use serde::Deserialize;
use smol_str::SmolStr;
use url::Url;

use crate::BuildApiUrl;

pub const ENDPOINT_STATS: &str = "api/v1/stats/";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub version: SmolStr,
    pub software: Software,
    pub open_registrations: bool,
    pub usage: Usage,
    pub metadata: Metadata,
    pub playback: Playback,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Software {
    /// Always "invidious."
    pub name: SmolStr,
    /// Version of Invidious. May be a git commit hash.
    pub version: SmolStr,
    pub branch: SmolStr,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Usage {
    pub users: Users,
}

#[derive(Default, Deserialize)]
#[serde(default, rename = "camelCase")]
pub struct Users {
    pub total: u32,
    pub active_half_year: u32,
    pub active_month: u32,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Metadata {
    pub updated_at: u64,
    pub last_channel_refreshed_at: u64,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Playback {
    pub total_requests: u32,
    pub successful_requests: u32,
    pub ratio: f32,
}

/// [`BuildApiUrl`] implementation for [`Stats`].
#[derive(Clone, Copy)]
pub struct StatsUrl<'base>(pub &'base Url);

impl BuildApiUrl for StatsUrl<'_> {
    type Item = Stats;
}

impl TryFrom<StatsUrl<'_>> for Url {
    type Error = url::ParseError;

    #[inline]
    fn try_from(builder: StatsUrl<'_>) -> Result<Self, Self::Error> {
        builder.0.join(ENDPOINT_STATS)
    }
}
