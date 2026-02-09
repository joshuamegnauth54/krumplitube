// SPDX-License-Identifier: GPL-3.0-or-later

//! Retrieve Invidious instances.

use std::convert::Infallible;

use serde::{Deserialize, Deserializer};
use smol_str::SmolStr;
use url::Url;

use crate::{
    BuildApiUrl,
    invidious::v1::stats::Stats,
    serde_helpers::{
        self,
        exact_str::{ExactStr, FLAG_LEN, REGION_LEN},
    },
};

/// List of public Invidious instances.
pub const INSTANCES: &str = "https://api.invidious.io/instances.json";

/// Public instances as parsed from the Invidious API.
///
/// See: https://api.invidious.io
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

impl<'de> Deserialize<'de> for Instances {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Delegate {
            #[serde(rename = "0")]
            name: SmolStr,
            #[serde(rename = "1")]
            data: Instance,
        }

        let instances = Vec::<Delegate>::deserialize(deserializer)?
            .into_iter()
            .map(
                |Delegate {
                     name,
                     data:
                         Instance {
                             flag,
                             region,
                             stats,
                             cors,
                             api,
                             protocol_type,
                             uri,
                             monitor,
                             ..
                         },
                 }| Instance {
                    name,
                    flag,
                    region,
                    stats,
                    cors,
                    api,
                    protocol_type,
                    uri,
                    monitor,
                },
            )
            .collect();

        Ok(Self { instances })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    #[serde(skip)]
    pub name: SmolStr,
    pub flag: ExactStr<FLAG_LEN>,
    pub region: ExactStr<REGION_LEN>,
    pub stats: Option<Stats>,
    #[serde(with = "serde_helpers::option_bool")]
    pub cors: bool,
    #[serde(with = "serde_helpers::option_bool")]
    pub api: bool,
    #[serde(rename = "type")]
    pub protocol_type: ProtocolType,
    pub uri: Url,
    #[serde(skip)]
    pub monitor: Monitor,
}

/// Invidious host's protocol.
///
/// The few remaining instances are HTTPS but there is at least one Tor node.
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProtocolType {
    Http,
    Https,
    Onion,
    I2p,
}

/// Uptime monitor stats.
///
/// Invidious uses: https://updown.io/
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Monitor;

/// [`BuildApiUrl`] implementation for [`Instances`].
pub struct InstancesUrl;

impl BuildApiUrl for InstancesUrl {
    type Item = Instances;
}

#[allow(clippy::infallible_try_from)]
impl TryFrom<InstancesUrl> for Url {
    type Error = Infallible;

    #[inline]
    fn try_from(_: InstancesUrl) -> Result<Self, Self::Error> {
        Ok(INSTANCES
            .parse()
            .unwrap_or_else(|e| panic!("{INSTANCES} is a URL: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::Instances;

    const INSTANCES_CACHED: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/test_data/invidious/instances.json"
    );

    #[test]
    fn instances_deserialize() {
        let raw_json = fs::read_to_string(INSTANCES_CACHED)
            .expect("Should be able to read cached instances.json");
        let _: Instances = serde_json::from_str(&raw_json)
            .expect("Should be able to deserialize instances from JSON");
    }
}
