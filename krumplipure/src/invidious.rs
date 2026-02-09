// SPDX-License-Identifier: GPL-3.0-or-later

//! Invidious API driver.

use krumpli_tubers::{
    BuildApiUrl,
    invidious::{Instances, InstancesUrl, Stats, StatsUrl},
};
use reqwest::Url;
use tracing::instrument;

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
    pub async fn instances(&self) -> reqwest::Result<Instances> {
        self.client
            .inner()
            .get(Url::try_from(InstancesUrl).expect("Infallible"))
            .send()
            .await?
            .error_for_status()?
            .json::<<InstancesUrl as BuildApiUrl>::Item>()
            .await
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

    pub async fn try_set_instance(&self, instance: Option<Url>) -> reqwest::Result<()> {
        todo!()
    }
}
