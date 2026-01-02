// SPDX-License-Identifier: GPL-3.0-or-later

//! Piped API types.
//!
//! See [docs](https://docs.piped.video/docs/api-documentation/) for more info.

use mime::Mime;
use serde::Deserialize;
use smol_str::SmolStr;
use url::Url;

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct Streams {
    pub audio_streams: Vec<AudioStream>,
    pub index_start: u8,
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub struct AudioStream {
    pub bitrate: u16,
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
