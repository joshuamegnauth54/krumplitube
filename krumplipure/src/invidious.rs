// SPDX-License-Identifier: GPL-3.0-or-later

//! Invidious API driver.

use krumpli_tubers::{
    BuildApiUrl,
    invidious::{Instances, InstancesUrl},
};
use reqwest::Url;

use crate::client::KrumpliClient;

pub struct InvidiousClient {
    client: KrumpliClient,
    instance: Option<Url>,
}

impl InvidiousClient {
    pub const fn new(client: KrumpliClient, instance: Option<Url>) -> Self {
        Self { client, instance }
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
}
