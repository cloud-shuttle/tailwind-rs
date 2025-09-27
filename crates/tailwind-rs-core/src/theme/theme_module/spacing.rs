//! Spacing system for tailwind-rs
//!
//! This module provides spacing utilities and definitions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Spacing values
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SpacingValue {
    /// Pixel value
    Pixels(u32),
    /// Rem value
    Rem(f32),
    /// Em value
    Em(f32),
    /// Percentage value
    Percentage(f32),
    /// Auto value
    Auto,
}

impl SpacingValue {
    /// Get the CSS value for the spacing
    pub fn css_value(&self) -> String {
        match self {
            SpacingValue::Pixels(px) => format!("{}px", px),
            SpacingValue::Rem(rem) => format!("{}rem", rem),
            SpacingValue::Em(em) => format!("{}em", em),
            SpacingValue::Percentage(pct) => format!("{}%", pct),
            SpacingValue::Auto => "auto".to_string(),
        }
    }

    /// Create a pixel spacing value
    pub fn px(value: u32) -> Self {
        Self::Pixels(value)
    }

    /// Create a rem spacing value
    pub fn rem(value: f32) -> Self {
        Self::Rem(value)
    }

    /// Create an em spacing value
    pub fn em(value: f32) -> Self {
        Self::Em(value)
    }

    /// Create a percentage spacing value
    pub fn pct(value: f32) -> Self {
        Self::Percentage(value)
    }

    /// Create an auto spacing value
    pub fn auto() -> Self {
        Self::Auto
    }
}

/// Spacing scale definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingScale {
    /// Base spacing unit (usually 0.25rem = 4px)
    pub base_unit: f32,
    /// Spacing scale values
    pub values: HashMap<String, SpacingValue>,
}

impl Default for SpacingScale {
    fn default() -> Self {
        let mut values = HashMap::new();
        values.insert("0".to_string(), SpacingValue::px(0));
        values.insert("1".to_string(), SpacingValue::rem(0.25));
        values.insert("2".to_string(), SpacingValue::rem(0.5));
        values.insert("3".to_string(), SpacingValue::rem(0.75));
        values.insert("4".to_string(), SpacingValue::rem(1.0));
        values.insert("5".to_string(), SpacingValue::rem(1.25));
        values.insert("6".to_string(), SpacingValue::rem(1.5));
        values.insert("8".to_string(), SpacingValue::rem(2.0));
        values.insert("10".to_string(), SpacingValue::rem(2.5));
        values.insert("12".to_string(), SpacingValue::rem(3.0));
        values.insert("16".to_string(), SpacingValue::rem(4.0));
        values.insert("20".to_string(), SpacingValue::rem(5.0));
        values.insert("24".to_string(), SpacingValue::rem(6.0));
        values.insert("32".to_string(), SpacingValue::rem(8.0));
        values.insert("40".to_string(), SpacingValue::rem(10.0));
        values.insert("48".to_string(), SpacingValue::rem(12.0));
        values.insert("56".to_string(), SpacingValue::rem(14.0));
        values.insert("64".to_string(), SpacingValue::rem(16.0));

        Self {
            base_unit: 0.25,
            values,
        }
    }
}

impl SpacingScale {
    /// Create a new spacing scale
    pub fn new(base_unit: f32) -> Self {
        Self {
            base_unit,
            values: HashMap::new(),
        }
    }

    /// Add a spacing value
    pub fn add_spacing(&mut self, name: impl Into<String>, value: SpacingValue) {
        self.values.insert(name.into(), value);
    }

    /// Get a spacing value
    pub fn get_spacing(&self, name: &str) -> Option<&SpacingValue> {
        self.values.get(name)
    }

    /// Get all spacing values
    pub fn values(&self) -> &HashMap<String, SpacingValue> {
        &self.values
    }
}
