// SPDX-License-Identifier: GPL-3.0-or-later

//! KrumpliTube is a small, minimalist player for Invidious and Piped. KrumpliTubers consists of
//! API wrappers for the services. And no, I didn't realize that KrumpliTube and KrumpliTubers
//! sound too alike when I renamed the APIs crate to KrumpliTubers.

use serde::Deserialize;
use url::Url;

pub mod error;
pub mod invidious;
pub mod peertube;
pub mod serde_helpers;

/// Return a [`Url`] that can be used to fetch this type.
///
/// `Context` provides the data needed to build the [`Url`]. `Context` can be empty if a type
/// doesn't need extra info or it could include a base URL, like for an instance.
pub trait BuildApiUrl: TryInto<Url> {
    type Item: for<'de> Deserialize<'de>;
}
