// SPDX-License-Identifier: GPL-3.0-or-later

//! Invidious captions endpoint implementation.

use serde::Deserialize;
use smol_str::SmolStr;
use url::Url;

use crate::{
    BuildApiUrl,
    serde_helpers::{
        exact_str::{ExactStr, LANG_LEN},
        video_id::YouTubeVideoId,
    },
};

pub const ENDPOINT_CAPTIONS: &str = "/api/v1/captions/";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Captions {
    pub captions: Vec<Caption>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Caption {
    pub label: SmolStr,
    pub language_code: ExactStr<LANG_LEN>,
    pub url: Url,
}

#[derive(Clone)]
pub struct CaptionsUrl<'url> {
    pub url: &'url Url,
    pub id: YouTubeVideoId,
    pub label: Option<SmolStr>,
    pub lang: Option<ExactStr<LANG_LEN>>,
    pub tlang: Option<ExactStr<LANG_LEN>>,
}

impl BuildApiUrl for CaptionsUrl<'_> {
    type Item = Captions;
}

impl TryFrom<CaptionsUrl<'_>> for Url {
    type Error = url::ParseError;

    fn try_from(builder: CaptionsUrl<'_>) -> Result<Self, Self::Error> {
        let mut url = builder
            .url
            .join(ENDPOINT_CAPTIONS)?
            .join(builder.id.as_ref())?;

        if builder.label.is_some() || builder.lang.is_some() || builder.tlang.is_some() {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(label) = builder.label {
                query_pairs.append_pair("label", &label);
            }
            if let Some(lang) = builder.lang {
                query_pairs.append_pair("lang", lang.as_str());
            }
            if let Some(tlang) = builder.tlang {
                query_pairs.append_pair("tlang", tlang.as_str());
            }
            query_pairs.finish();
        }

        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use smol_str::SmolStr;
    use url::Url;

    use crate::{invidious::v1::captions::CaptionsUrl, serde_helpers::exact_str::ExactStr};

    #[test]
    fn captions_url_no_parameters() {
        let url = "https://yewtu.be".parse().expect("Valid URL should parse");
        let id = "5zY8W9L7AFs"
            .try_into()
            .expect("Valid YouTube ID from YouTube itself should parse");

        let builder = CaptionsUrl {
            url: &url,
            id,
            label: None,
            lang: None,
            tlang: None,
        };
        let actual: Url = builder
            .try_into()
            .expect("Converting CaptionsUrl to URL should work since the parts are validated");
        let expected: Url = format!("https://yewtu.be/api/v1/captions/{id}")
            .parse()
            .expect("Valid URL should parse");

        assert_eq!(expected, actual);
    }

    #[test]
    fn captions_url_all_parameters() {
        let url = "https://yewtu.be".parse().expect("Valid URL should parse");
        let id = "hsug4crnvSU"
            .try_into()
            .expect("Valid YouTube ID from YouTube itself should parse");

        let label: Option<SmolStr> = SmolStr::new_static("taco").into();
        let lang = ExactStr::new("jp")
            .expect("Valid ExactStr should parse")
            .into();
        let tlang = ExactStr::new("en")
            .expect("Valid ExactStr should parse")
            .into();
        let builder = CaptionsUrl {
            url: &url,
            id,
            label: label.clone(),
            lang,
            tlang,
        };
        let actual: Url = builder
            .try_into()
            .expect("Converting CaptionsUrl to URL should work since the parts are validated");
        let expected: Url = format!(
            "https://yewtu.be/api/v1/captions/{}?label={}&lang={}&tlang={}",
            id,
            label.unwrap(),
            lang.unwrap(),
            tlang.unwrap()
        )
        .parse()
        .expect("Valid URL should parse");

        assert_eq!(expected, actual);
    }
}
