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
use smallvec::SmallVec;
use smol_str::SmolStr;

use crate::{
    peertube::node_info::client_script::ClientScript,
    serde_helpers::from_str_visitor::{Expecting, FromStrVisitor},
};

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Themes {
    pub registered: Vec<Theme>,
    pub default: SmolStr,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub npm_name: ThemeNpmName,
    pub name: SmolStr,
    pub version: SmolStr,
    pub description: String,
    pub css: SmallVec<SmolStr, 2>,
    pub client_scripts: HashMap<String, ClientScript>,
}

#[derive(Default)]
pub struct ThemeNpmName(SmolStr);

impl FromStr for ThemeNpmName {
    type Err = DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // https://docs.joinpeertube.org/contribute/plugins#write-a-plugin-theme
        s.strip_prefix("peertube-theme-")
            .ok_or_else(|| {
                DeError::invalid_value(
                    Unexpected::Other("invalid theme"),
                    &"plugin starting with peertube-theme-",
                )
            })
            .map(|s| Self(SmolStr::new(s)))
    }
}

impl Display for ThemeNpmName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.0.is_empty() {
            write!(f, "peertube-theme-{}", self.0)
        } else {
            write!(f, "")
        }
    }
}

impl Expecting for ThemeNpmName {
    const EXPECTING: &'static str = "NPM theme name starting with peertube-theme-";
}

impl<'de> Deserialize<'de> for ThemeNpmName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FromStrVisitor(PhantomData))
    }
}
