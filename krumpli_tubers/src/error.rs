// SPDX-License-Identifier: GPL-3.0-or-later

//! Common errors that may occur during API operations.

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::serde_helpers::video_id;

#[derive(Debug)]
pub enum ApiError {
    YouTubeId(YouTubeIdKind),
    MissingCodec,
    MissingContainer,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::YouTubeId(invalid) => write!(f, "Invalid YouTube ID: {invalid}"),
            Self::MissingCodec => write!(f, "Missing codec string"),
            Self::MissingContainer => write!(f, "Missing container name (e.g. mp4)"),
        }
    }
}

impl Error for ApiError {}

#[derive(Debug)]
pub enum YouTubeIdKind {
    Length(usize),
    Char(char),
}

impl Display for YouTubeIdKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length(len) => write!(
                f,
                "expected length {}, actual length {}",
                video_id::ID_LEN,
                len
            ),
            Self::Char(ch) => write!(f, "unexpected char {ch} (alphanumeric, -, _ only)"),
        }
    }
}
