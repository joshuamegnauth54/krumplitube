// SPDX-License-Identifier: GPL-3.0-or-later

//! Common errors that may occur during API operations.

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum ApiError {
    MissingCodec,
    MissingContainer,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCodec => write!(f, "Missing codec string"),
            Self::MissingContainer => write!(f, "Missing container name (e.g. mp4)"),
        }
    }
}

impl Error for ApiError {}
