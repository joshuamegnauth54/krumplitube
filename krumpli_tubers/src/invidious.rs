// SPDX-License-Identifier: GPL-3.0-or-later

//! API wrapper for Invidious.
//!
//! See: https://docs.invidious.io/api/

pub mod instances;
pub use instances::{Instances, InstancesUrl};

pub mod v1;
pub use v1::{
    stats::{Stats, StatsUrl},
    videos::{BaseVideosUrl, Video},
};
