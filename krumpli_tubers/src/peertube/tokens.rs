// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use smol_str::SmolStr;

#[derive(Deserialize)]
pub struct BearerToken {
    pub access_token: String,
    pub token_type: SmolStr,
    pub expires_in: u64,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct BearerRequest {
    pub client_id: String,
    pub client_secret: String,
}
