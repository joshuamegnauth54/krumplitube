// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use smol_str::SmolStr;

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileType {
    pub size: Size,
    #[serde(flatten)]
    pub extensions: Extensions,
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub file: FileType,
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub image: FileType,
    pub file: Extensions,
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoCaption {
    pub file: FileType,
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    pub max: u32,
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    pub extensions: Vec<SmolStr>,
}
