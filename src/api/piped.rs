// SPDX-License-Identifier: GPL-3.0-or-later

//! Piped API types.
//!
//! See [docs](https://docs.piped.video/docs/api-documentation/) for more info.

use serde::Deserialize;

use crate::api::common::{AudioCodec, AudioFormat};

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
    pub codec: AudioCodec,
    pub format: AudioFormat,
}
