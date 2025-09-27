//! CanIUse Data for Autoprefixer
//!
//! This module provides CanIUse compatibility data for autoprefixer functionality.

use std::collections::HashMap;
use crate::autoprefixer::SupportLevel;

/// CanIUse data for compatibility checking
pub struct CanIUseData {
    features: HashMap<String, FeatureSupport>,
}

impl CanIUseData {
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
        }
    }

    pub fn get_support(&self, _feature: &str, browser: &str) -> Option<SupportLevel> {
        // Simplified support check
        match browser {
            "chrome" | "firefox" | "safari" | "edge" => Some(SupportLevel::Full),
            _ => Some(SupportLevel::None),
        }
    }
}

/// Feature support information
pub struct FeatureSupport {
    pub browsers: HashMap<String, SupportLevel>,
    pub caniuse_id: String,
    pub description: String,
}
