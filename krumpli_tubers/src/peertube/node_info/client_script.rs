// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use smallvec::SmallVec;

#[derive(Deserialize)]
pub struct ClientScript {
    pub script: String,
    pub scopes: SmallVec<ScriptScope, 2>,
}

/// https://docs.joinpeertube.org/api/plugins
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScriptScope {
    Common,
    VideoWatch,
    Search,
    Signup,
    Login,
    Embed,
    VideoEdit,
    AdminPlugin,
    MyLibrary,
    VideoChannel,
    MyAccount,
    AdminUsers,
    AdminComments,
    Moderation,
}
