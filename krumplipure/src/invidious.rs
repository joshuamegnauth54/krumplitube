// SPDX-License-Identifier: GPL-3.0-or-later

//! Invidious API driver.

use krumpli_tubers::{
    BuildApiUrl,
    invidious::{BaseVideosUrl, Instances, InstancesUrl, Stats, StatsUrl},
};
use reqwest::Url;
use tracing::{info, instrument};

use crate::{
    Error,
    client::{BackendType, KrumpliClient},
    error::ErrorKind,
};

pub struct InvidiousClient {
    client: KrumpliClient,
    instance: Option<Url>,
}

impl InvidiousClient {
    #[inline]
    pub const fn new(client: KrumpliClient, instance: Option<Url>) -> Self {
        Self { client, instance }
    }

    #[inline]
    pub const fn backend(&self) -> BackendType {
        BackendType::Invidious
    }

    /// Fetch and deserialize list of Invidious [`Instances`].
    pub async fn instances(&self) -> Result<Instances, Error> {
        self.client
            .inner()
            .get(Url::try_from(InstancesUrl).expect("Infallible"))
            .send()
            .await?
            .error_for_status()?
            .json::<<InstancesUrl as BuildApiUrl>::Item>()
            .await
            .inspect(|instances| {
                for instance in instances.iter() {
                    info!(%instance.uri, "Found instance");
                }
            })
            .map_err(From::from)
    }

    #[instrument(skip_all, fields(backend = %self.backend(), url = ?self.instance))]
    pub async fn stats(&self) -> Result<Stats, Error> {
        let url = self.instance.as_ref().ok_or(ErrorKind::MissingApiUrl)?;
        let url = StatsUrl(url);
        self.get_stats(&url).await
    }

    #[instrument(skip_all, fields(backend = %self.backend(), url = %url.0))]
    async fn get_stats(&self, url: &StatsUrl<'_>) -> Result<Stats, Error> {
        self.client
            .inner()
            .get(Url::try_from(*url).map_err(|e| ErrorKind::Other(e.into()))?)
            .send()
            .await?
            .error_for_status()?
            .json::<<StatsUrl as BuildApiUrl>::Item>()
            .await
            .map_err(From::from)
    }

    /// Set the current instance regardless if the API is enabled.
    ///
    /// This still checks that the instance is valid.
    #[instrument(skip(self), fields(backend = %self.backend()))]
    pub async fn set_instance(&mut self, instance: Url) -> Result<(), Error> {
        let url = StatsUrl(&instance);
        // Test that this is an Invidious instance by hitting the stats endpoint.
        self.get_stats(&url).await?;
        self.instance = Some(instance);
        Ok(())
    }

    /// Set the current instance only if the API is enabled.
    #[instrument(skip(self), fields(backend = %self.backend()))]
    pub async fn try_set_instance(&mut self, instance: Url) -> Result<(), Error> {
        let url = StatsUrl(&instance);
        // Test that this is an Invidious instance by hitting the stats endpoint.
        self.get_stats(&url).await?;

        // Check that the API is available. Many instances disable the API due to abuse.
        let url = BaseVideosUrl(&instance);
        self.get_base_videos(&url).await?;

        self.instance = Some(instance);
        Ok(())
    }

    // Try to GET the videos endpoint.
    //
    // The videos endpoint is part of the API. It's often disabled due to abuse unlike the
    // stats API. Checking the videos endpoint is a good proxy for testing if an Invidious
    // instance has the API enabled.
    #[instrument(skip_all, fields(backend = %self.backend(), url = %url.0))]
    async fn get_base_videos(&self, url: &BaseVideosUrl<'_>) -> Result<(), Error> {
        self.client
            .inner()
            .get(Url::try_from(*url).map_err(|e| ErrorKind::Other(e.into()))?)
            .send()
            .await?
            .error_for_status()
            .map(|_| ())
            .map_err(From::from)
    }
}
