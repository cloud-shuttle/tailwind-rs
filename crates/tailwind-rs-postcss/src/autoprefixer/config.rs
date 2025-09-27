//! Configuration for Autoprefixer
//!
//! This module provides configuration options for autoprefixer.

/// Configuration for autoprefixer
pub struct AutoprefixerConfig {
    pub browsers: Vec<String>,
    pub cascade: bool,
    pub remove: bool,
    pub add: bool,
    pub flexbox: bool,
    pub grid: bool,
    pub stats: bool,
}

impl Default for AutoprefixerConfig {
    fn default() -> Self {
        Self {
            browsers: vec![
                "last 2 versions".to_string(),
                "> 1%".to_string(),
                "not dead".to_string(),
            ],
            cascade: true,
            remove: true,
            add: true,
            flexbox: true,
            grid: true,
            stats: false,
        }
    }
}
