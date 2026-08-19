// SPDX-License-Identifier: GPL-3.0-or-later

//! PeerTube API driver.

use krumpli_tubers::peertube::BearerToken;
use reqwest::Url;

use crate::{
    Error,
    client::{BackendType, KrumpliClient},
    error::ErrorKind,
};

pub struct PeerTubeClient {
    client: KrumpliClient,
    instance: Option<Instance>,
}

impl PeerTubeClient {
    pub async fn set_instance(&mut self, url: Url) -> Result<(), Error> {
        todo!()
    }
}

pub struct Instance {
    url: Url,
    token: BearerToken,
}
