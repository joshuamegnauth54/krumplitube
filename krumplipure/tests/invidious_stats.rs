// SPDX-License-Identifier: GPL-3.0-or-later

use krumplipure::{Error, InvidiousClient, KrumpliClient};
use test_log::test;

#[test(tokio::test)]
async fn fetch_invidious_stats() -> Result<(), Error> {
    let mut client = InvidiousClient::new(KrumpliClient::default(), None);

    for instance in client.instances().await?.iter() {
        client.set_instance(instance.uri.clone()).await?;
        client.stats().await?;
    }

    Ok(())
}
