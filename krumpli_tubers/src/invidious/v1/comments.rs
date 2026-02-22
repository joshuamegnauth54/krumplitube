// SPDX-License-Identifier: GPL-3.0-or-later

//! Comments endpoint for Invidious.

use jiff::Zoned;
use serde::Deserialize;
use url::Url;

use crate::{
    invidious::v1::videos::AuthorMetadata,
    serde_helpers::{self, time, video_id::YouTubeVideoId},
};

pub const ENDPOINT_COMMENTS: &str = "/api/v1/comments/";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comments {
    #[serde(with = "serde_helpers::option_t")]
    pub comment_count: u32,
    pub video_id: YouTubeVideoId,
    pub comments: Vec<Comment>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    #[serde(flatten)]
    pub author: AuthorMetadata,
    pub is_edited: bool,
    pub is_pinned: bool,
    #[serde(with = "serde_helpers::option_t")]
    pub is_sponsor: bool,
    #[serde(default)]
    pub sponsor_icon_url: Option<Url>,
    pub content: String,
    pub content_html: String,
    #[serde(deserialize_with = "time::timestamp_to_zoned")]
    pub published: Zoned,
}
