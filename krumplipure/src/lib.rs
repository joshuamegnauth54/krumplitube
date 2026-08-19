// SPDX-License-Identifier: GPL-3.0-or-later

//! Network handlers for KrumpliTube.

pub mod client;
pub use client::KrumpliClient;

pub mod error;
pub use error::Error;

pub mod invidious;
pub use invidious::InvidiousClient;

pub mod peertube;
pub use peertube::PeerTubeClient;
