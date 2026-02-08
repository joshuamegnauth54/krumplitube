// SPDX-License-Identifier: GPL-3.0-or-later

//! Network handlers for KrumpliTube.

pub mod client;
pub use client::KrumpliClient;

pub mod invidious;
pub use invidious::InvidiousClient;
