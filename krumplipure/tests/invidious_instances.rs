// SPDX-License-Identifier: GPL-3.0-or-later

use krumplipure::{Error, InvidiousClient, KrumpliClient};
use test_log::test;

#[test(tokio::test)]
async fn fetch_invidious_instances() -> Result<(), Error> {
    InvidiousClient::new(KrumpliClient::default(), None)
        .instances()
        .await
        .map(|_| ())
}
