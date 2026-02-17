// SPDX-License-Identifier: GPL-3.0-or-later

//! Types for Invidious' annotations endpoint.
//!
//! This is entirely unused in KrumpliTube but I want to mock it out anyways.

use url::Url;

use crate::{BuildApiUrl, serde_helpers::video_id::YouTubeVideoId};

pub const ENDPOINT_ANNOT: &str = "api/v1/annotations/";

/// [`BuildApiUrl`] implementation for the annotations endpoint.
#[derive(Clone, Copy)]
pub struct AnnotUrl<'url> {
    pub url: &'url Url,
    pub id: YouTubeVideoId,
    pub source: Option<AnnotationSource>,
}

impl BuildApiUrl for AnnotUrl<'_> {
    type Item = String;
}

impl TryFrom<AnnotUrl<'_>> for Url {
    type Error = url::ParseError;

    fn try_from(builder: AnnotUrl<'_>) -> Result<Self, Self::Error> {
        let mut url = builder
            .url
            .join(ENDPOINT_ANNOT)?
            .join(builder.id.as_ref())?;

        if let Some(source) = builder.source {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("source", source.as_str());
            query_pairs.finish();
        }

        Ok(url)
    }
}

#[derive(Clone, Copy)]
pub enum AnnotationSource {
    Archive,
    YouTube,
}

impl AnnotationSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Archive => "archive",
            Self::YouTube => "youtube",
        }
    }
}
