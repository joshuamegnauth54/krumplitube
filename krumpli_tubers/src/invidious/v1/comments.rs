// SPDX-License-Identifier: GPL-3.0-or-later

//! Comments endpoint for Invidious.

use jiff::Zoned;
use serde::Deserialize;
use smol_str::SmolStr;
use url::Url;

use crate::{
    BuildApiUrl,
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
    pub comment_id: SmolStr,
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
    pub published_text: SmolStr,
    pub like_count: u32,
    pub author_is_channel_owner: bool,
    #[serde(default)]
    pub creator_heart: Option<CreatorHeart>,
    #[serde(default)]
    pub continuation: Option<SmolStr>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatorHeart {
    pub creator_thumbnail: Url,
    pub creator_name: SmolStr,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortBy {
    New,
    #[default]
    Top,
}

impl SortBy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Top => "top",
        }
    }
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Source {
    Reddit,
    #[default]
    YouTube,
}

impl Source {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reddit => "reddit",
            Self::YouTube => "youtube",
        }
    }
}

/// [`BuildApiUrl`] implementation for [`Comments`].
#[derive(Clone, Copy)]
pub struct CommentsUrl<'url> {
    pub url: &'url Url,
    pub id: YouTubeVideoId,
    pub sort_by: Option<SortBy>,
    pub source: Option<Source>,
}

impl BuildApiUrl for CommentsUrl<'_> {
    type Item = Comments;
}

impl TryFrom<CommentsUrl<'_>> for Url {
    type Error = url::ParseError;

    fn try_from(builder: CommentsUrl<'_>) -> Result<Self, Self::Error> {
        let mut url = builder
            .url
            .join(ENDPOINT_COMMENTS)?
            .join(builder.id.as_ref())?;

        // Unconditionally calling query_pairs_mut() causes the URL to have Some("") as the query.
        // It's nicer for tests and consistency to avoid that.
        if builder.sort_by.is_some() || builder.source.is_some() {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(sort_by) = builder.sort_by {
                query_pairs.append_pair("sort_by", sort_by.as_str());
            }
            if let Some(source) = builder.source {
                query_pairs.append_pair("source", source.as_str());
            }
            query_pairs.finish();
        }

        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::invidious::v1::comments::{CommentsUrl, SortBy, Source};

    #[test]
    fn comments_url_no_parameters() {
        let url = "https://yewtu.be".parse().expect("Valid URL should parse");
        let id = "wX9Sc88qreg"
            .try_into()
            .expect("Valid YouTube ID from YouTube itself should parse");

        let builder = CommentsUrl {
            url: &url,
            id,
            sort_by: None,
            source: None,
        };
        let actual: Url = builder
            .try_into()
            .expect("Converting CommentsUrl to URL should work since the parts are validated");
        let expected: Url = format!("https://yewtu.be/api/v1/comments/{id}")
            .parse()
            .expect("Valid URL should parse");

        assert_eq!(expected, actual);
    }

    #[test]
    fn comments_url_all_parameters() {
        let url = "https://yewtu.be".parse().expect("Valid URL should parse");
        let id = "geNMz0J9TEQ"
            .try_into()
            .expect("Valid YouTube ID from YouTube itself should parse");
        let sort_by = Some(SortBy::New);
        let source = Some(Source::YouTube);

        let builder = CommentsUrl {
            url: &url,
            id,
            sort_by,
            source,
        };
        let actual: Url = builder
            .try_into()
            .expect("Converting CommentsUrl to URL should work since the parts are validated");
        #[allow(clippy::unnecessary_literal_unwrap)]
        let expected: Url = format!(
            "https://yewtu.be/api/v1/comments/{}?sort_by={}&source={}",
            id,
            sort_by.unwrap().as_str(),
            source.unwrap().as_str()
        )
        .parse()
        .expect("Valid URL should parse");

        assert_eq!(expected, actual);
    }
}
