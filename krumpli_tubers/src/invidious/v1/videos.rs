// SPDX-License-Identifier: GPL-3.0-or-later

use std::num::NonZeroU32;

use mime::Mime;
use serde::Deserialize;
use smol_str::SmolStr;
use url::Url;

use crate::serde_helpers::{
    self,
    exact_str::{ExactStr, LANG_LEN, REGION_LEN},
    video_id::YouTubeVideoId,
};

pub const ENDPOINT_VIDEOS: &str = "/api/v1/videos/";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    #[serde(flatten)]
    pub metadata: VideoMetadata,
    #[serde(rename = "type")]
    pub video_type: VideoType,
    pub storyboards: Vec<StoryBoard>,
    pub description: String,
    pub description_html: String,
    pub published: u64,
    pub published_text: SmolStr,
    pub keywords: Vec<SmolStr>,
    pub like_count: u32,
    pub dislike_count: u32,
    pub paid: bool,
    pub premium: bool,
    pub is_family_friendly: bool,
    pub allowed_regions: Vec<ExactStr<REGION_LEN>>,
    pub genre: SmolStr,
    pub genre_url: Url,
    pub sub_count_text: SmolStr,
    pub allow_ratings: bool,
    pub rating: f32,
    pub is_listed: u32,
    pub live_now: bool,
    pub is_posted_live_dvr: bool,
    pub is_upcoming: bool,
    pub dash_url: Url,
    #[serde(default)]
    pub premiere_timestamp: u64,
    pub hls_url: Url,
    pub adaptive_formats: Vec<AdaptiveFormat>,
    pub format_streams: Vec<FormatStream>,
    #[serde(default)]
    pub captions: Vec<Caption>,
    #[serde(default)]
    pub music_tracks: Vec<MusicTrack>,
    #[serde(default)]
    pub recommended_videos: Vec<VideoMetadata>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMetadata {
    pub video_id: YouTubeVideoId,
    pub title: String,
    // XXX: How many thumbnails per video is normal?
    pub video_thumbnails: Vec<VideoThumbnail>,
    pub author: SmolStr,
    #[serde(default)]
    pub author_id: Option<SmolStr>,
    pub author_url: Url,
    #[serde(default)]
    pub author_verified: bool,
    pub author_thumbnails: Vec<AuthorThumbnail>,
    pub length_seconds: u32,
    pub view_count: u32,
    pub view_count_text: SmolStr,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoType {
    Video,
    Published,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoThumbnail {
    pub quality: ThumbnailQuality,
    pub url: Url,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThumbnailQuality {
    Default,
    Medium,
    High,
    Standard,
    MaxRes,
    MaxResDefault,
    SdDefault,
    Start,
    Middle,
    End,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoryBoard {
    pub url: Url,
    pub template_url: Url,
    pub width: u32,
    pub height: u32,
    pub count: u32,
    pub interval: u32,
    pub storyboard_width: u32,
    pub storyboard_height: u32,
    pub storyboard_count: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorThumbnail {
    pub url: Url,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveFormat {
    pub index: SmolStr,
    pub bitrate: u32,
    pub init: SmolStr,
    pub url: Url,
    /// A tag representing the quality of the stream. This could be parsed into an enum.
    pub itag: u32,
    #[serde(rename = "type", with = "serde_helpers::mime")]
    pub mime: Mime,
    pub clen: u32,
    pub lmt: u64,
    pub projection_type: SmolStr,
    pub container: SmolStr,
    pub encoding: SmolStr,
    #[serde(default)]
    pub quality_label: Option<SmolStr>,
    #[serde(default)]
    pub resolution: Option<SmolStr>,
    pub fps: u32,
    pub size: SmolStr,
    pub target_duration_sec: Option<u64>,
    pub max_dvr_duration_sec: Option<u64>,
    #[serde(default)]
    pub audio_quality: Option<SmolStr>,
    #[serde(default)]
    pub audio_sample_rate: Option<NonZeroU32>,
    #[serde(default)]
    pub audio_channels: Option<NonZeroU32>,
    pub color_info: Option<ColorInfo>,
    pub caption_track: SmolStr,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorInfo {
    pub primaries: SmolStr,
    pub transfer_characteristics: SmolStr,
    pub matrix_coefficients: SmolStr,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatStream {
    pub url: Url,
    pub itag: SmolStr,
    #[serde(rename = "type", with = "serde_helpers::mime")]
    pub mime: Mime,
    pub quality: SmolStr,
    #[serde(default)]
    pub bitrate: Option<NonZeroU32>,
    pub container: SmolStr,
    pub encoding: SmolStr,
    pub quality_label: SmolStr,
    pub resolution: SmolStr,
    pub size: SmolStr,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Caption {
    pub label: SmolStr,
    pub language_code: ExactStr<LANG_LEN>,
    pub url: Url,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicTrack {
    pub song: String,
    pub artist: String,
    pub album: String,
    pub license: SmolStr,
}
