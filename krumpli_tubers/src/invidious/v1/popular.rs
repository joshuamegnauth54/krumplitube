// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use smol_str::SmolStr;
use url::Url;

use crate::{
    BuildApiUrl,
    invidious::v1::videos::{Published, VideoMetadata},
};

const ENDPOINT_POPULAR: &str = "api/v1/popular";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Popular {
    #[serde(rename = "type")]
    pub video_type: SmolStr,
    #[serde(flatten)]
    pub video_metadata: VideoMetadata,
    #[serde(flatten)]
    pub published: Published,
}

/// [`BuildApiUrl`] implementation for [`Popular`].
#[derive(Clone, Copy)]
pub struct PopularUrl<'url>(pub &'url Url);

impl BuildApiUrl for PopularUrl<'_> {
    type Item = Popular;
}

impl TryFrom<PopularUrl<'_>> for Url {
    type Error = url::ParseError;

    fn try_from(builder: PopularUrl<'_>) -> Result<Self, Self::Error> {
        builder.0.join(ENDPOINT_POPULAR)
    }
}
