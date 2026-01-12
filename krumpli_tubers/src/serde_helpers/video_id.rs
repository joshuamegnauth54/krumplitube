// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;

use crate::serde_helpers::exact_str::ExactStr;

const ID_LEN: usize = 11;

/// Valid YouTube video ID for [`crate::invidious`] and [`crate::piped`].
///
/// IDs are 11 ASCII character strings that are:
/// * alphanumeric
/// * case sensitive
/// * underscores and hyphens
#[derive(Deserialize)]
#[serde(transparent)]
pub struct YouTubeVideoId(ExactStr<ID_LEN>);

impl AsRef<str> for YouTubeVideoId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
