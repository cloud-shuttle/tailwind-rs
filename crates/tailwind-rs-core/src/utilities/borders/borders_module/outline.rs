//! Outline utilities
//!
//! This module provides utilities for outline values.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Outline values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Outline {
    /// No outline
    None,
    /// Thin outline
    Thin,
    /// Default outline
    Default,
    /// Medium outline
    Medium,
    /// Thick outline
    Thick,
}

impl Outline {
    /// Get the CSS value for the outline
    pub fn css_value(&self) -> &'static str {
        match self {
            Outline::None => "none",
            Outline::Thin => "2px solid",
            Outline::Default => "2px solid",
            Outline::Medium => "4px solid",
            Outline::Thick => "8px solid",
        }
    }

    /// Get the Tailwind class for the outline
    pub fn class_name(&self) -> &'static str {
        match self {
            Outline::None => "outline-none",
            Outline::Thin => "outline",
            Outline::Default => "outline",
            Outline::Medium => "outline-2",
            Outline::Thick => "outline-4",
        }
    }
}

impl fmt::Display for Outline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_value())
    }
}

impl ClassBuilder {
    /// Set outline
    pub fn outline(&mut self, outline: Outline) -> &mut Self {
        *self = self.clone().class(outline.class_name());
        self
    }

    /// Set outline to none
    pub fn outline_none(&mut self) -> &mut Self {
        self.outline(Outline::None)
    }

    /// Set outline to thin
    pub fn outline_thin(&mut self) -> &mut Self {
        self.outline(Outline::Thin)
    }

    /// Set outline to default
    pub fn outline_default(&mut self) -> &mut Self {
        self.outline(Outline::Default)
    }

    /// Set outline to medium
    pub fn outline_medium(&mut self) -> &mut Self {
        self.outline(Outline::Medium)
    }

    /// Set outline to thick
    pub fn outline_thick(&mut self) -> &mut Self {
        self.outline(Outline::Thick)
    }
}
