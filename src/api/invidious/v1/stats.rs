// SPDX-License-Identifier: GPL-3.0-or-later

//! Types for Invidious' stats endpoint.
//!
//! Version 1.

use serde::Deserialize;
use smol_str::SmolStr;

pub const ENDPOINT_STATS: &str = "api/v1/stats/";

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct Stats {
    pub version: SmolStr,
    pub software: Software,
    pub open_registrations: bool,
    pub usage: Usage,
    pub metadata: Metadata,
    pub playback: Playback,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct Software {
    /// Always "invidious."
    pub name: SmolStr,
    /// Version of Invidious. May be a git commit hash.
    pub version: SmolStr,
    pub branch: SmolStr,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct Usage {
    pub users: Users,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct Users {
    pub total: u32,
    pub active_half_year: u32,
    pub active_month: u32,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct Metadata {
    pub updated_at: u64,
    pub last_channel_refreshed_at: u64,
}

#[derive(Deserialize, Default)]
#[serde(rename = "camelCase")]
pub struct Playback {
    #[serde(default)]
    pub total_requests: u32,
    #[serde(default)]
    pub successful_requests: u32,
    #[serde(default)]
    pub ratio: f32,
}
