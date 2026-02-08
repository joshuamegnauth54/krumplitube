// SPDX-License-Identifier: GPL-3.0-or-later

use krumplipure::{InvidiousClient, KrumpliClient};
use test_log::test;

#[test(tokio::test)]
async fn fetch_invidious_instances() -> reqwest::Result<()> {
    InvidiousClient::new(KrumpliClient::default(), None)
        .instances()
        .await
        .map(|_| ())
}
