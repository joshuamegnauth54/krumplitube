// SPDX-License-Identifier: GPL-3.0-or-later

//! Retrieve Invidious instances.

use serde::Deserialize;
use smol_str::SmolStr;
use url::Url;

pub use super::v1::stats::Stats;

pub const INSTANCES: &str = "https://api.invidious.io/instances.json";

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Instances {
    pub instances: Vec<Instance>,
}

impl Instances {
    /// Iterator over instances with the API enabled.
    ///
    /// Most instances disable the API due to abuse. Only instances with the API enabled are useful
    /// for this app.
    #[inline]
    pub fn enabled_api(self) -> impl Iterator<Item = Instance> {
        self.instances.into_iter().filter(|instance| instance.api)
    }
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct Instance {
    pub flag: SmolStr,
    pub region: SmolStr,
    pub stats: Stats,
    pub cors: bool,
    pub api: bool,
    #[serde(rename = "type")]
    pub protocol_type: SmolStr,
    pub uri: Url,
    #[serde(skip)]
    pub monitor: Monitor,
}

/// Uptime monitor stats.
///
/// Invidious uses: https://updown.io/
#[derive(Deserialize, Default)]
#[serde(rename = "camelCase")]
pub struct Monitor;
