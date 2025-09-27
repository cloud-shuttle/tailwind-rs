//! Browser Data for Autoprefixer
//!
//! This module provides browser compatibility data for autoprefixer functionality.

use std::collections::HashMap;

/// Browser data for compatibility checking
pub struct BrowserData {
    browsers: HashMap<String, BrowserInfo>,
    features: HashMap<String, FeatureSupport>,
    versions: HashMap<String, Vec<String>>,
}

impl BrowserData {
    pub fn new() -> Self {
        Self {
            browsers: HashMap::new(),
            features: HashMap::new(),
            versions: HashMap::new(),
        }
    }

    pub fn get_feature_support(&self, feature: &str, browser: &str) -> Option<SupportLevel> {
        self.features.get(feature)
            .and_then(|support| support.browsers.get(browser))
            .copied()
    }

    pub fn supports_feature(&self, _feature: &str, browser: &str, _version: &str) -> bool {
        // Simplified support check
        match browser {
            "chrome" | "firefox" | "safari" | "edge" => true,
            _ => false,
        }
    }
}

/// Browser information
pub struct BrowserInfo {
    pub name: String,
    pub version: String,
    pub support_level: SupportLevel,
}

/// Feature support information
pub struct FeatureSupport {
    pub browsers: HashMap<String, SupportLevel>,
    pub caniuse_id: String,
    pub description: String,
}

/// Support level for a feature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportLevel {
    None,
    Partial,
    Full,
}
