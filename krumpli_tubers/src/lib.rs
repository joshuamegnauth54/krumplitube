// SPDX-License-Identifier: GPL-3.0-or-later

//! KrumpliTube is a small, minimalist player for Invidious and Piped. KrumpliTubers consists of
//! API wrappers for the services. And no, I didn't realize that KrumpliTube and KrumpliTubers
//! sound too alike when I renamed the APIs crate to KrumpliTubers.

pub mod error;
pub mod invidious;
pub mod peertube;
pub mod serde_helpers;
