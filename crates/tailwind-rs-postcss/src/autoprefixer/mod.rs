//! Enhanced Autoprefixer Integration
//!
//! This module provides comprehensive vendor prefixing functionality for CSS properties
//! based on browser compatibility data, essential for cross-browser CSS support.

pub mod core;
pub mod browser_data;
pub mod caniuse_data;
pub mod prefix_generator;
pub mod prefix_cache;
pub mod config;
pub mod errors;

pub use core::{
    Autoprefixer, PrefixOptions, PrefixResult, PrefixStatistics
};
pub use browser_data::BrowserData;
pub use caniuse_data::CanIUseData;
pub use prefix_generator::PrefixGenerator;
pub use prefix_cache::PrefixCache;
pub use config::AutoprefixerConfig;
pub use errors::AutoprefixerError;
