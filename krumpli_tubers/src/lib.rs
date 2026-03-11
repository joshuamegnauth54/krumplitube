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

/// Return a [`Url`] that can be used to fetch and deserialize this type.
///
/// Types implementing this trait should include all of the context needed to actually build the
/// URL. The base URL of the instance, if needed, is one such context.
pub trait BuildApiUrl: TryInto<Url, Error: std::error::Error + 'static> {
    /// Deserialized data from the endpoint.
    type Item: for<'de> Deserialize<'de>;
}
