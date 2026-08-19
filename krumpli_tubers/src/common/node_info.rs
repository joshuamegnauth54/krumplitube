// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use smallvec::SmallVec;
use smol_str::SmolStr;

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct NodeInfo<M: Default> {
    pub version: SmolStr,
    pub software: Software,
    pub protocols: SmallVec<SmolStr, 2>,
    pub services: Services,
    pub open_registrations: bool,
    pub usage: Usage,
    pub metadata: M,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Software {
    /// "invidious", "peertube", etc.
    pub name: SmolStr,
    /// Version; may be a git commit hash.
    pub version: SmolStr,
    pub branch: SmolStr,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Services {
    pub inbound: Vec<SmolStr>,
    pub outbound: Vec<SmolStr>,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Usage {
    pub users: Users,
    pub local_posts: u32,
    pub local_comments: u32,
}

#[derive(Default, Deserialize)]
#[serde(default, rename = "camelCase")]
pub struct Users {
    pub total: u32,
    pub active_half_year: u32,
    pub active_month: u32,
}
