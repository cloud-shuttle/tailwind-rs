//! Border radius utilities
//!
//! This module provides utilities for border radius values.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Border radius values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderRadius {
    /// No radius
    None,
    /// Small radius
    Sm,
    /// Default radius
    Default,
    /// Medium radius
    Md,
    /// Large radius
    Lg,
    /// Extra large radius
    Xl,
    /// 2X large radius
    Xl2,
    /// 3X large radius
    Xl3,
    /// Full radius (circular)
    Full,
}

impl BorderRadius {
    /// Get the CSS value for the border radius
    pub fn css_value(&self) -> &'static str {
        match self {
            BorderRadius::None => "0",
            BorderRadius::Sm => "0.125rem",
            BorderRadius::Default => "0.25rem",
            BorderRadius::Md => "0.375rem",
            BorderRadius::Lg => "0.5rem",
            BorderRadius::Xl => "0.75rem",
            BorderRadius::Xl2 => "1rem",
            BorderRadius::Xl3 => "1.5rem",
            BorderRadius::Full => "9999px",
        }
    }

    /// Get the Tailwind class for the border radius
    pub fn class_name(&self) -> &'static str {
        match self {
            BorderRadius::None => "rounded-none",
            BorderRadius::Sm => "rounded-sm",
            BorderRadius::Default => "rounded",
            BorderRadius::Md => "rounded-md",
            BorderRadius::Lg => "rounded-lg",
            BorderRadius::Xl => "rounded-xl",
            BorderRadius::Xl2 => "rounded-2xl",
            BorderRadius::Xl3 => "rounded-3xl",
            BorderRadius::Full => "rounded-full",
        }
    }
}

impl fmt::Display for BorderRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_value())
    }
}

impl ClassBuilder {
    /// Set border radius
    pub fn border_radius(&mut self, radius: BorderRadius) -> &mut Self {
        *self = self.clone().class(radius.class_name());
        self
    }

    /// Set border radius to none
    pub fn rounded_none(&mut self) -> &mut Self {
        self.border_radius(BorderRadius::None)
    }

    /// Set border radius to small
    pub fn rounded_sm(&mut self) -> &mut Self {
        self.border_radius(BorderRadius::Sm)
    }

    /// Set border radius to default
    pub fn rounded(&mut self) -> &mut Self {
        self.border_radius(BorderRadius::Default)
    }

    /// Set border radius to medium
    pub fn rounded_md(&mut self) -> &mut Self {
        self.border_radius(BorderRadius::Md)
    }

    /// Set border radius to large
    pub fn rounded_lg(&mut self) -> &mut Self {
        self.border_radius(BorderRadius::Lg)
    }

    /// Set border radius to extra large
    pub fn rounded_xl(&mut self) -> &mut Self {
        self.border_radius(BorderRadius::Xl)
    }

    /// Set border radius to 2X large
    pub fn rounded_2xl(&mut self) -> &mut Self {
        self.border_radius(BorderRadius::Xl2)
    }

    /// Set border radius to 3X large
    pub fn rounded_3xl(&mut self) -> &mut Self {
        self.border_radius(BorderRadius::Xl3)
    }

    /// Set border radius to full (circular)
    pub fn rounded_full(&mut self) -> &mut Self {
        self.border_radius(BorderRadius::Full)
    }
}
