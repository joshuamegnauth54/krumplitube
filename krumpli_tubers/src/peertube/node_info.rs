// SPDX-License-Identifier: GPL-3.0-or-later

pub mod client_script;
pub mod file;
pub mod plugins;
pub mod themes;

use serde::Deserialize;
use smallvec::SmallVec;
use smol_str::SmolStr;

use crate::{
    common,
    peertube::node_info::{
        file::{Avatar, Video, VideoCaption},
        plugins::Plugins,
        themes::Themes,
    },
};

pub const HEADER: (&str, &str) = ("x-powered-by", "peertube");

pub type NodeInfo = common::NodeInfo<Metadata>;

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Metadata {
    pub taxonomy: Taxonomy,
    pub node_name: SmolStr,
    pub node_description: String,
    pub node_config: NodeConfig,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Taxonomy {
    pub posts_name: SmolStr,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfig {
    pub search: Search,
    pub plugin: Plugins,
    pub theme: Themes,
    pub email: Enabled,
    pub contact_form: Enabled,
    pub transcoding: Transcoding,
    pub live: Live,
    pub import: Import,
    pub auto_blacklist: AutoBlacklist,
    pub avatar: Avatar,
    pub video: Video,
    pub video_caption: VideoCaption,
    pub user: User,
    pub trending: Trending,
    pub tracker: Enabled,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub remote_uri: RemoteUri,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteUri {
    pub users: bool,
    pub anonymous: bool,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enabled {
    pub enabled: bool,
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transcoding {
    #[serde(default)]
    pub hls: Enabled,
    #[serde(default)]
    pub web_videos: Enabled,
    pub enabled_resolutions: SmallVec<u16, 6>,
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Live {
    pub enabled: bool,
    pub transcoding: Transcoding,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Import {
    pub videos: ImportVideos,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportVideos {
    pub http: Enabled,
    pub torrent: Enabled,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoBlacklist {
    pub videos: AutoblacklistVideos,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoblacklistVideos {
    pub of_users: Enabled,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub video_quota: u64,
    pub video_quota_daily: u64,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trending {
    pub videos: TrendingVideos,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendingVideos {
    pub interval_days: u8,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::NodeInfo;

    #[test]
    fn peertube_node_info_parses() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_data/peertube/nodeinfo/2.0.json"
        );
        let json = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("PeerTube Node Info test data not found at: {path}\n{e}"));
        let _info: NodeInfo = serde_json::from_str(&json)
            .expect("PeerTube Node Info JSON should parse unless the schema changed");
    }
}
