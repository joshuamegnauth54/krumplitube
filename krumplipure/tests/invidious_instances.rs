// SPDX-License-Identifier: GPL-3.0-or-later

use krumplipure::{Error, InvidiousClient, KrumpliClient};
use test_log::test;
use tracing::info;

#[test(tokio::test)]
async fn fetch_invidious_instances() -> Result<(), Error> {
    InvidiousClient::new(KrumpliClient::default(), None)
        .instances()
        .await
        .inspect(|instances| info!("Fetched {} instances", instances.iter().count()))
        .map(|_| ())
}
