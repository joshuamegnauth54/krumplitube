// SPDX-License-Identifier: GPL-3.0-or-later

//! A wrapper around [`reqwest::Client`] to set up the user agent and other helpful options.

use std::{
    fmt::{self, Display, Formatter},
    time::Duration,
};

use reqwest::{Client, Proxy, header::HeaderValue};

use crate::Error;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);
const DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Clone)]
pub struct KrumpliClient {
    client: Client,
}

impl KrumpliClient {
    pub fn new(
        timeout: Duration,
        user_agent: impl TryInto<HeaderValue, Error: Into<http::Error>>,
        proxy: Option<Proxy>,
    ) -> Result<Self, Error> {
        let builder = Client::builder()
            .deflate(true)
            .brotli(true)
            .gzip(true)
            .hickory_dns(true)
            .https_only(true)
            .timeout(timeout)
            .tls_backend_rustls()
            .user_agent(user_agent);

        if let Some(proxy) = proxy {
            builder.proxy(proxy)
        } else {
            builder
        }
        .build()
        .map(|client| Self { client })
        .map_err(From::from)
    }

    /// Return a clone of the inner [`Client`].
    #[inline]
    pub(crate) fn inner(&self) -> Client {
        self.client.clone()
    }

    /// Helper to get raw [reqwest::Response] on errors.
    #[cfg(any(debug_assertions, test))]
    pub async fn body_debug(
        &self,
        url: impl krumpli_tubers::BuildApiUrl,
    ) -> Result<reqwest::Response, Error> {
        use crate::error::ErrorKind;
        use reqwest::Url;

        let url: Url = url.try_into().map_err(|e| ErrorKind::Other(e.into()))?;
        self.client.get(url).send().await.map_err(Error::from)
    }

    /// Helper to print [reqwest::Response] on error.
    #[cfg(any(debug_assertions, test))]
    pub async fn body_debug_trace(
        &self,
        url: impl krumpli_tubers::BuildApiUrl,
    ) -> Result<(), Error> {
        use tracing::error;

        let response = self.body_debug(url).await?;
        let url = response.url().clone();

        error!(%url, status = %response.status(), "Status code from failed request");
        for (header, value) in response.headers() {
            error!(%url, %header, ?value, "Header from failed request");
        }
        let body = response.text().await.map_err(Error::from)?;
        error!(%url, %body, "Body from failed request");

        Ok(())
    }
}

impl Default for KrumpliClient {
    /// Create a [`KrumpliClient`] with the default user agent and timeout.
    ///
    /// Creating a client can fail. In that case, an even more generic client is used instead.
    fn default() -> Self {
        KrumpliClient::new(DEFAULT_TIMEOUT, DEFAULT_USER_AGENT, None).unwrap_or_else(|_| Self {
            client: Client::default(),
        })
    }
}

/// Helper for [`tracing`] to display the client type.
#[derive(Clone, Copy)]
pub enum BackendType {
    Invidious,
    PeerTube,
    Piped,
}

impl BackendType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Invidious => "Invidious",
            Self::PeerTube => "PeerTube",
            Self::Piped => "Piped",
        }
    }
}

impl Display for BackendType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
