// SPDX-License-Identifier: GPL-3.0-or-later

//! Types that are common between APIs.
//!
//! * https://datatracker.ietf.org/doc/html/rfc6381
//! * https://developer.mozilla.org/en-US/docs/Web/Media/Guides/Formats/codecs_parameter

use std::str::FromStr;

use serde::{
    Deserialize, Deserializer,
    de::{Error as DeErrorT, Unexpected},
};
use serde_bytes::Bytes;

use crate::api::error::ApiError;

pub struct MediaContainer {
    pub container: ContainerType,
    pub codec: Codec,
}

#[derive(Default)]
pub struct Codec {
    pub video: Option<VideoFormat>,
    pub audio: Option<AudioFormat>,
}

impl FromStr for Codec {
    type Err = ApiError;

    fn from_str(codec: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

/// A container that holds either [`AudioFormat`] or [`VideoFormat`].
#[derive(Clone, Copy)]
pub enum ContainerType {
    /// Audio Data Transport Stream.
    Adts,
    /// Free Lossless Audio Codec. Audio only.
    Flac,
    /// Matroska.
    Mkv,
    /// Moving Picture Experts Group 1.
    Mpeg,
    /// Moving Picture Experts Group 2.
    Mpeg2,
    /// Moving Picture Experts Group 4.
    Mpeg4,
    /// Ogg.
    Ogg,
    /// QuickTime.
    QuickTime,
    /// Third Generation Partnership 2.
    ThirdGen3gp2,
    /// Third Generation Partnership.
    ThirdGen3gpp,
    /// Web Media.
    WebM,
}

#[derive(Clone, Copy)]
pub enum VideoFormat {
    AomAv1,
    Vp8,
}

impl FromStr for VideoFormat {
    type Err = ApiError;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl<'de> Deserialize<'de> for VideoFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // See [`AudioFormat::from_str`]
        let unparsed: &Bytes = serde_bytes::deserialize(deserializer)?;
        str::from_utf8(unparsed.as_ref())
            .map_err(|_| {
                DeErrorT::invalid_value(Unexpected::Other("invalid UTF-8"), &"valid ASCII")
            })?
            .parse()
            .map_err(DeErrorT::custom)
    }
}

/// Audio formats subset.
#[derive(Clone, Copy)]
pub enum AudioFormat {
    Opus,
    Vorbis,
    Mp4a,
}

impl FromStr for AudioFormat {
    type Err = ApiError;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format.trim_ascii().split_once('.').unwrap_or((format, "")) {
            ("mp4a", oti) => {
                unimplemented!()
            }
            _ => unreachable!(),
        }
    }
}

impl<'de> Deserialize<'de> for AudioFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // An exact sized slice can't be used because the Object Type Indication (OTI) is variable
        // length.
        // Also...I'm deserializing a byte slice because I can't figure out how to deserialize a
        // &str so this is the path of least resistance.
        let unparsed: &Bytes = serde_bytes::deserialize(deserializer)?;
        str::from_utf8(unparsed.as_ref())
            .map_err(|_| {
                DeErrorT::invalid_value(Unexpected::Other("invalid UTF-8"), &"valid ASCII")
            })?
            .parse()
            .map_err(DeErrorT::custom)
    }
}
