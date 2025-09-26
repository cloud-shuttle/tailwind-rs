//! Core types for CSS generation
//!
//! This module contains the fundamental types used throughout the CSS generation system.

use crate::error::{Result, TailwindError};
use crate::responsive::Breakpoint;
use std::collections::HashMap;

/// Represents a CSS rule with selector and properties
#[derive(Debug, Clone, PartialEq)]
pub struct CssRule {
    /// CSS selector (e.g., ".p-4", ".md:bg-blue-500")
    pub selector: String,
    /// CSS properties for this rule
    pub properties: Vec<CssProperty>,
    /// Media query for responsive rules
    pub media_query: Option<String>,
    /// CSS specificity score
    pub specificity: u32,
}

/// Represents a CSS property
#[derive(Debug, Clone, PartialEq)]
pub struct CssProperty {
    /// Property name (e.g., "padding", "background-color")
    pub name: String,
    /// Property value (e.g., "1rem", "#3b82f6")
    pub value: String,
    /// Whether the property is marked as !important
    pub important: bool,
}

/// CSS generation configuration
#[derive(Debug, Clone)]
pub struct CssGenerationConfig {
    /// Whether to include responsive variants
    pub include_responsive: bool,
    /// Whether to include dark mode variants
    pub include_dark_mode: bool,
    /// Whether to include hover/focus variants
    pub include_interactive: bool,
    /// Whether to include device variants
    pub include_device_variants: bool,
    /// Custom breakpoints
    pub custom_breakpoints: HashMap<Breakpoint, String>,
}

impl Default for CssGenerationConfig {
    fn default() -> Self {
        Self {
            include_responsive: true,
            include_dark_mode: true,
            include_interactive: true,
            include_device_variants: true,
            custom_breakpoints: HashMap::new(),
        }
    }
}

impl CssRule {
    /// Create a new CSS rule
    pub fn new(selector: String, properties: Vec<CssProperty>) -> Self {
        Self {
            selector,
            properties,
            media_query: None,
            specificity: 0,
        }
    }

    /// Create a responsive CSS rule
    pub fn new_responsive(
        selector: String,
        properties: Vec<CssProperty>,
        media_query: String,
    ) -> Self {
        Self {
            selector,
            properties,
            media_query: Some(media_query),
            specificity: 20, // Higher specificity for responsive rules
        }
    }
}

impl CssProperty {
    /// Create a new CSS property
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            important: false,
        }
    }

    /// Create a new CSS property with importance
    pub fn new_important(name: String, value: String) -> Self {
        Self {
            name,
            value,
            important: true,
        }
    }
}
