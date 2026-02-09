// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    error,
    fmt::{self, Display, Formatter},
};

use http::StatusCode;
use reqwest::Url;

#[derive(Debug)]
pub struct Error {
    pub url: Option<Url>,
    pub kind: ErrorKind,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::ApiDisabled => write!(f, "API is either disabled or requires auth"),
            ErrorKind::MissingApiUrl => write!(
                f,
                "A URL, like an instance URL or endpoint, is needed for this operation"
            ),
            ErrorKind::Other(message) => write!(f, "{message}"),
            ErrorKind::StatusCode(code) => write!(f, "Status code: {code}"),
            ErrorKind::TimedOut => write!(f, "Connection timed out"),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    ApiDisabled,
    MissingApiUrl,
    Other(Box<dyn error::Error>),
    StatusCode(StatusCode),
    TimedOut,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { url: None, kind }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        let url = error.url().cloned();
        if let Some(code) = error.status() {
            Self {
                url,
                kind: ErrorKind::StatusCode(code),
            }
        } else if error.is_timeout() {
            Self {
                url,
                kind: ErrorKind::TimedOut,
            }
        } else {
            Self {
                url,
                kind: ErrorKind::Other(error.into()),
            }
        }
    }
}
