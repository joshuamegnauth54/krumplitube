// SPDX-License-Identifier: GPL-3.0-or-later

use core::{
    fmt::{self, Display, Formatter},
    marker::PhantomData,
    str::FromStr,
};
use std::collections::HashMap;

use serde::{
    Deserialize, Deserializer,
    de::{Error, Unexpected, value::Error as DeError},
};
use smol_str::SmolStr;

use crate::{
    peertube::node_info::client_script::ClientScript,
    serde_helpers::from_str_visitor::{Expecting, FromStrVisitor},
};

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugins {
    pub registered: Vec<Plugin>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub npm_name: PluginNpmName,
    pub name: SmolStr,
    pub version: SmolStr,
    pub description: String,
    pub client_scripts: HashMap<String, ClientScript>,
}

#[derive(Default)]
pub struct PluginNpmName(SmolStr);

impl FromStr for PluginNpmName {
    type Err = DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // https://docs.joinpeertube.org/contribute/plugins#write-a-plugin-theme
        s.strip_prefix("peertube-plugin-")
            .ok_or_else(|| {
                DeError::invalid_value(
                    Unexpected::Other("invalid plugin"),
                    &"plugin starting with peertube-plugin-",
                )
            })
            .map(|s| Self(SmolStr::new(s)))
    }
}

impl Display for PluginNpmName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.0.is_empty() {
            write!(f, "peertube-plugin-{}", self.0)
        } else {
            write!(f, "")
        }
    }
}

impl Expecting for PluginNpmName {
    const EXPECTING: &'static str = "NPM plugin name starting with peertube-plugin-";
}

impl<'de> Deserialize<'de> for PluginNpmName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FromStrVisitor(PhantomData))
    }
}
