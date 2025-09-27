//! Border style utilities
//!
//! This module provides utilities for border style values.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Border style values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderStyle {
    /// Solid border
    Solid,
    /// Dashed border
    Dashed,
    /// Dotted border
    Dotted,
    /// Double border
    Double,
    /// Hidden border
    Hidden,
    /// None border
    None,
}

impl BorderStyle {
    /// Get the CSS value for the border style
    pub fn css_value(&self) -> &'static str {
        match self {
            BorderStyle::Solid => "solid",
            BorderStyle::Dashed => "dashed",
            BorderStyle::Dotted => "dotted",
            BorderStyle::Double => "double",
            BorderStyle::Hidden => "hidden",
            BorderStyle::None => "none",
        }
    }

    /// Get the Tailwind class for the border style
    pub fn class_name(&self) -> &'static str {
        match self {
            BorderStyle::Solid => "border-solid",
            BorderStyle::Dashed => "border-dashed",
            BorderStyle::Dotted => "border-dotted",
            BorderStyle::Double => "border-double",
            BorderStyle::Hidden => "border-hidden",
            BorderStyle::None => "border-none",
        }
    }
}

impl fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_value())
    }
}

impl ClassBuilder {
    /// Set border style
    pub fn border_style(&mut self, style: BorderStyle) -> &mut Self {
        *self = self.clone().class(style.class_name());
        self
    }

    /// Set border style to solid
    pub fn border_solid(&mut self) -> &mut Self {
        self.border_style(BorderStyle::Solid)
    }

    /// Set border style to dashed
    pub fn border_dashed(&mut self) -> &mut Self {
        self.border_style(BorderStyle::Dashed)
    }

    /// Set border style to dotted
    pub fn border_dotted(&mut self) -> &mut Self {
        self.border_style(BorderStyle::Dotted)
    }

    /// Set border style to double
    pub fn border_double(&mut self) -> &mut Self {
        self.border_style(BorderStyle::Double)
    }

    /// Set border style to hidden
    pub fn border_hidden(&mut self) -> &mut Self {
        self.border_style(BorderStyle::Hidden)
    }

    /// Set border style to none
    pub fn border_none(&mut self) -> &mut Self {
        self.border_style(BorderStyle::None)
    }
}
