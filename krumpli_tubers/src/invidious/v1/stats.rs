// SPDX-License-Identifier: GPL-3.0-or-later

//! Types for Invidious' stats endpoint.
//!
//! Version 1.

use serde::Deserialize;
use url::Url;

use crate::{BuildApiUrl, common::NodeInfo};

pub const ENDPOINT_STATS: &str = "api/v1/stats/";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    #[serde(flatten)]
    pub node_info: NodeInfo<Metadata>,
    #[serde(default)]
    pub playback: Playback,
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
