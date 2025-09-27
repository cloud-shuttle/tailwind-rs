//! Border width utilities
//!
//! This module provides utilities for border width values.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Border width values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderWidth {
    /// No border
    Zero,
    /// Thin border
    Thin,
    /// Default border
    Default,
    /// Medium border
    Medium,
    /// Thick border
    Thick,
}

impl BorderWidth {
    /// Get the CSS value for the border width
    pub fn css_value(&self) -> &'static str {
        match self {
            BorderWidth::Zero => "0",
            BorderWidth::Thin => "1px",
            BorderWidth::Default => "1px",
            BorderWidth::Medium => "2px",
            BorderWidth::Thick => "4px",
        }
    }

    /// Get the Tailwind class for the border width
    pub fn class_name(&self) -> &'static str {
        match self {
            BorderWidth::Zero => "border-0",
            BorderWidth::Thin => "border",
            BorderWidth::Default => "border",
            BorderWidth::Medium => "border-2",
            BorderWidth::Thick => "border-4",
        }
    }
}

impl fmt::Display for BorderWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_value())
    }
}

impl ClassBuilder {
    /// Set border width
    pub fn border_width(&mut self, width: BorderWidth) -> &mut Self {
        *self = self.clone().class(width.class_name());
        self
    }

    /// Set border width to zero
    pub fn border_0(&mut self) -> &mut Self {
        self.border_width(BorderWidth::Zero)
    }

    /// Set border width to thin
    pub fn border(&mut self) -> &mut Self {
        self.border_width(BorderWidth::Thin)
    }

    /// Set border width to medium
    pub fn border_2(&mut self) -> &mut Self {
        self.border_width(BorderWidth::Medium)
    }

    /// Set border width to thick
    pub fn border_4(&mut self) -> &mut Self {
        self.border_width(BorderWidth::Thick)
    }
}
