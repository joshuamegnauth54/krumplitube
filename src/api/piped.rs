// SPDX-License-Identifier: GPL-3.0-or-later

//! Piped API types.
//!
//! See [docs](https://docs.piped.video/docs/api-documentation/) for more info.

use mime::Mime;
use serde::Deserialize;
use smol_str::SmolStr;
use url::Url;

/// /streams:video_id
#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct Streams {
    pub audio_streams: Vec<AudioStream>,
    pub index_start: u8,
    pub dash: Option<Url>,
    pub description: String,
    pub dislikes: u32,
    pub likes: u32,
    pub duration: u32,
    pub hls: Option<Url>,
    #[serde(skip)]
    pub lbry_id: Option<Url>,
    pub livestream: bool,
    pub proxy_url: Url,
    pub related_streams: Vec<RelatedStream>,
    pub subtitles: Vec<Subtitles>,
    pub thumbnail_url: Url,
    pub title: String,
    pub upload_date: SmolStr,
    pub uploader: SmolStr,
    pub uploader_url: Url,
    pub uploader_verified: bool,
    pub video_streams: Vec<VideoStream>,
    pub views: u32,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct AudioStream {
    pub bitrate: u32,
    pub codec: SmolStr,
    pub format: SmolStr,
    pub index_end: u32,
    pub index_start: u32,
    pub init_end: u32,
    pub init_start: u32,
    #[serde(with = "crate::api::mime")]
    pub mime_type: Mime,
    pub url: Url,
    pub video_only: bool,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct RelatedStream {
    pub duration: u32,
    pub thumbnail: Url,
    pub title: String,
    pub uploaded_date: SmolStr,
    pub uploaded_avatar: Url,
    pub uploader_url: Url,
    pub uploader_verified: bool,
    pub url: Url,
    pub views: u32,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct Subtitles {
    pub auto_generated: bool,
    pub code: SmolStr,
    #[serde(with = "crate::api::mime")]
    pub mime_type: Mime,
    pub url: Url,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct VideoStream {
    pub bitrate: u32,
    pub codec: SmolStr,
    pub format: SmolStr,
    pub fps: u16,
    pub height: u16,
    pub width: u16,
    pub index_end: u32,
    pub index_start: u32,
    pub init_end: u32,
    pub init_start: u32,
    #[serde(with = "crate::api::mime")]
    pub mime_type: Mime,
    pub url: Url,
    pub quality: SmolStr,
}
