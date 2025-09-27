//! Prefix Generator for Autoprefixer
//!
//! This module provides prefix generation functionality for autoprefixer.

use std::collections::HashMap;

/// Prefix generator for CSS properties
pub struct PrefixGenerator {
    prefixes: HashMap<String, Vec<String>>,
    config: GeneratorConfig,
}

impl PrefixGenerator {
    pub fn new() -> Self {
        Self {
            prefixes: HashMap::new(),
            config: GeneratorConfig::default(),
        }
    }

    pub fn generate_prefixes(&self, property: &str, browsers: &[String]) -> Vec<String> {
        let mut prefixes = Vec::new();

        if self.needs_webkit_prefix(property) {
            prefixes.push(format!("-webkit-{}", property));
        }
        if self.needs_moz_prefix(property) {
            prefixes.push(format!("-moz-{}", property));
        }
        if self.needs_ms_prefix(property) {
            prefixes.push(format!("-ms-{}", property));
        }
        if self.needs_o_prefix(property) {
            prefixes.push(format!("-o-{}", property));
        }

        prefixes
    }

    fn needs_webkit_prefix(&self, property: &str) -> bool {
        match property {
            "transform" | "flex" | "grid" | "filter" | "backdrop-filter" => true,
            "mask" | "clip-path" | "shape-outside" => true,
            "text-decoration" | "text-emphasis" => true,
            _ => false,
        }
    }

    fn needs_moz_prefix(&self, property: &str) -> bool {
        match property {
            "transform" | "flex" | "grid" | "filter" | "backdrop-filter" => true,
            "mask" | "clip-path" | "shape-outside" => true,
            "text-decoration" | "text-emphasis" => true,
            _ => false,
        }
    }

    fn needs_ms_prefix(&self, property: &str) -> bool {
        match property {
            "transform" | "flex" | "grid" | "filter" | "backdrop-filter" => true,
            "mask" | "clip-path" | "shape-outside" => true,
            "text-decoration" | "text-emphasis" => true,
            _ => false,
        }
    }

    fn needs_o_prefix(&self, property: &str) -> bool {
        match property {
            "transform" | "flex" | "grid" | "filter" | "backdrop-filter" => true,
            "mask" | "clip-path" | "shape-outside" => true,
            "text-decoration" | "text-emphasis" => true,
            _ => false,
        }
    }
}

/// Configuration for prefix generator
pub struct GeneratorConfig {
    pub enable_webkit: bool,
    pub enable_moz: bool,
    pub enable_ms: bool,
    pub enable_o: bool,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            enable_webkit: true,
            enable_moz: true,
            enable_ms: true,
            enable_o: true,
        }
    }
}
