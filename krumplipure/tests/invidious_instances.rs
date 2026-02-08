// SPDX-License-Identifier: GPL-3.0-or-later

use krumpli_tubers::{BuildApiUrl, invidious::InstancesUrl};
use krumplipure::KrumpliClient;
use reqwest::Url;

#[tokio::test]
async fn fetch_invidious_instances() -> reqwest::Result<()> {
    KrumpliClient::default()
        .inner()
        .get(Url::try_from(InstancesUrl).expect("Infallible"))
        .send()
        .await?
        .error_for_status()?
        .json::<<InstancesUrl as BuildApiUrl>::Item>()
        .await
        .map(|_| ())
}
