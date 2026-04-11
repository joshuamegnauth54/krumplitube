// SPDX-License-Identifier: GPL-3.0-or-later

//! Wrapper for Invidious' trending endpoint.

use serde::Deserialize;
use url::Url;

use crate::{
    BuildApiUrl,
    invidious::v1::videos::{AuthorMetadata, Description, Published, VideoMetadata},
    serde_helpers::exact_str::{ExactStr, LANG_LEN},
};

pub const ENDPOINT_TRENDING: &str = "api/v1/trending";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trending {
    #[serde(flatten)]
    pub video: VideoMetadata,
    #[serde(flatten)]
    pub author: AuthorMetadata,
    #[serde(flatten)]
    pub published: Published,
    #[serde(flatten)]
    pub description: Description,
}

/// [`BuildApiUrl`] implementation for trending videos.
#[derive(Clone, Copy)]
pub struct TrendingUrl<'url> {
    pub url: &'url Url,
    pub type_: Option<TrendingType>,
    pub region: Option<ExactStr<LANG_LEN>>,
}

impl BuildApiUrl for TrendingUrl<'_> {
    type Item = Trending;
}

impl TryFrom<TrendingUrl<'_>> for Url {
    type Error = url::ParseError;

    fn try_from(builder: TrendingUrl<'_>) -> Result<Self, Self::Error> {
        let mut url = builder.url.join(ENDPOINT_TRENDING)?;

        if builder.type_.is_some() || builder.region.is_some() {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(type_) = builder.type_ {
                query_pairs.append_pair("type", type_.as_str());
            }
            if let Some(region) = builder.region {
                query_pairs.append_pair("region", region.as_str());
            }
            query_pairs.finish();
        }

        Ok(url)
    }
}

/// [`Trending`] category filter.
///
/// Invidious' API only supports the variants listed below. YouTube itself supports dozens of
/// variants.
#[derive(Clone, Copy)]
pub enum TrendingType {
    Default,
    Gaming,
    Movies,
    Music,
}

impl TrendingType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Gaming => "gaming",
            Self::Movies => "movies",
            Self::Music => "music",
        }
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::invidious::v1::trending::{TrendingType, TrendingUrl};

    #[test]
    fn trending_url_no_parameters() {
        let url = "https://yewtu.be".parse().expect("Valid URL should parse");
        // let id = "lASs908RJoU"
        //     .try_into()
        //     .expect("Valid YouTube ID from YouTube itself should parse");

        let builder = TrendingUrl {
            url: &url,
            type_: None,
            region: None,
        };
        let actual: Url = builder
            .try_into()
            .expect("Converting TrendingUrl to URL should work since the parts are validated");
        let expected: Url = "https://yewtu.be/api/v1/trending"
            .parse()
            .expect("Valid URL should parse");

        assert_eq!(expected, actual);
    }

    #[test]
    fn trending_url_all_parameters() {
        let url = "https://yewtu.be".parse().expect("Valid URL should parse");
        // let id = "lASs908RJoU"
        //     .try_into()
        //     .expect("Valid YouTube ID from YouTube itself should parse");

        let builder = TrendingUrl {
            url: &url,
            type_: Some(TrendingType::Gaming),
            region: Some("us".parse().unwrap()),
        };
        let actual: Url = builder
            .try_into()
            .expect("Converting TrendingUrl to URL should work since the parts are validated");
        let expected: Url = format!(
            "https://yewtu.be/api/v1/trending?type={}&region={}",
            builder.type_.unwrap().as_str(),
            builder.region.unwrap()
        )
        .parse()
        .expect("Valid URL should parse");

        assert_eq!(expected, actual);
    }
}
