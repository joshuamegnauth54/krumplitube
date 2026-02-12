// SPDX-License-Identifier: GPL-3.0-or-later

use krumpli_tubers::invidious::StatsUrl;
use krumplipure::{Error, InvidiousClient, KrumpliClient};
use test_log::test;
use tracing::{error, info};

#[test(tokio::test)]
async fn fetch_invidious_stats() -> Result<(), Error> {
    let mut client = InvidiousClient::new(KrumpliClient::default(), None);

    for instance in client.instances().await?.iter() {
        info!("Testing stats endpoint for instance: {}", instance.uri);
        client
            .set_instance(instance.uri.clone())
            .await
            .inspect_err(|err| error!(url = %instance.uri, "{err}"))?;

        info!(url = %instance.uri, "Instance set. Testing stats endpoint");
        if let Err(err) = client.stats().await {
            error!(url = %instance.uri, %err, "Fetching stats endpoint failed");
            let url = StatsUrl(&instance.uri);
            client.inner().body_debug_trace(url).await?;
        }
    }

    Ok(())
}
