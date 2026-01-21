// SPDX-License-Identifier: GPL-3.0-or-later

use krumpli_tubers::{BuildApiUrl, invidious::Instances};
use krumplipure::KrumpliClient;

#[tokio::test]
async fn fetch_invidious_instances() -> reqwest::Result<()> {
    // KrumpliClient::default().inner().get(Instances::build_url(b, context))
    todo!()
}
