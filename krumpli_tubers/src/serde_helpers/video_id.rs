// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt::{self, Display, Formatter};

use serde::Deserialize;

use crate::{
    error::{ApiError, YouTubeIdKind},
    serde_helpers::exact_str::ExactStr,
};

pub const ID_LEN: usize = 11;

/// Valid YouTube video ID for [`crate::invidious`] and [`crate::piped`].
///
/// IDs are 11 ASCII character strings that are:
/// * alphanumeric
/// * case sensitive
/// * underscores and hyphens
#[derive(Deserialize, Clone, Copy)]
#[serde(transparent)]
pub struct YouTubeVideoId(ExactStr<ID_LEN>);

impl Display for YouTubeVideoId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for YouTubeVideoId {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<&str> for YouTubeVideoId {
    type Error = ApiError;

    fn try_from(id: &str) -> Result<Self, Self::Error> {
        if let Some(i) = id.find(|ch: char| !ch.is_alphanumeric() && (ch != '_' || ch != '-')) {
            let ch = id.chars().nth(i).unwrap_or_default();
            Err(ApiError::YouTubeId(YouTubeIdKind::Char(ch)))
        } else {
            ExactStr::new(id)
                .map_err(|_| ApiError::YouTubeId(YouTubeIdKind::Length(id.len())))
                .map(YouTubeVideoId)
        }
    }
}
