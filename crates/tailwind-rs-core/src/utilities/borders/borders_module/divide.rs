//! Divide utilities
//!
//! This module provides utilities for divide values.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Divide values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Divide {
    /// No divide
    None,
    /// Thin divide
    Thin,
    /// Default divide
    Default,
    /// Medium divide
    Medium,
    /// Thick divide
    Thick,
}

impl Divide {
    /// Get the CSS value for the divide
    pub fn css_value(&self) -> &'static str {
        match self {
            Divide::None => "0",
            Divide::Thin => "1px",
            Divide::Default => "1px",
            Divide::Medium => "2px",
            Divide::Thick => "4px",
        }
    }

    /// Get the Tailwind class for the divide
    pub fn class_name(&self) -> &'static str {
        match self {
            Divide::None => "divide-x-0",
            Divide::Thin => "divide-x",
            Divide::Default => "divide-x",
            Divide::Medium => "divide-x-2",
            Divide::Thick => "divide-x-4",
        }
    }
}

impl fmt::Display for Divide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_value())
    }
}

impl ClassBuilder {
    /// Set divide
    pub fn divide(&mut self, divide: Divide) -> &mut Self {
        *self = self.clone().class(divide.class_name());
        self
    }

    /// Set divide to none
    pub fn divide_none(&mut self) -> &mut Self {
        self.divide(Divide::None)
    }

    /// Set divide to thin
    pub fn divide_thin(&mut self) -> &mut Self {
        self.divide(Divide::Thin)
    }

    /// Set divide to default
    pub fn divide_default(&mut self) -> &mut Self {
        self.divide(Divide::Default)
    }

    /// Set divide to medium
    pub fn divide_medium(&mut self) -> &mut Self {
        self.divide(Divide::Medium)
    }

    /// Set divide to thick
    pub fn divide_thick(&mut self) -> &mut Self {
        self.divide(Divide::Thick)
    }
}
