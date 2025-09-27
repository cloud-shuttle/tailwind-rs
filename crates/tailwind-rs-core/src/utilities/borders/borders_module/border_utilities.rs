//! Border utilities
//!
//! This module provides general border utilities.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Border utilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderUtilities {
    /// No border
    None,
    /// Default border
    Default,
    /// All borders
    All,
    /// Top border
    Top,
    /// Right border
    Right,
    /// Bottom border
    Bottom,
    /// Left border
    Left,
    /// Horizontal borders
    Horizontal,
    /// Vertical borders
    Vertical,
}

impl BorderUtilities {
    /// Get the CSS value for the border utilities
    pub fn css_value(&self) -> &'static str {
        match self {
            BorderUtilities::None => "none",
            BorderUtilities::Default => "1px solid",
            BorderUtilities::All => "1px solid",
            BorderUtilities::Top => "1px solid",
            BorderUtilities::Right => "1px solid",
            BorderUtilities::Bottom => "1px solid",
            BorderUtilities::Left => "1px solid",
            BorderUtilities::Horizontal => "1px solid",
            BorderUtilities::Vertical => "1px solid",
        }
    }

    /// Get the Tailwind class for the border utilities
    pub fn class_name(&self) -> &'static str {
        match self {
            BorderUtilities::None => "border-0",
            BorderUtilities::Default => "border",
            BorderUtilities::All => "border",
            BorderUtilities::Top => "border-t",
            BorderUtilities::Right => "border-r",
            BorderUtilities::Bottom => "border-b",
            BorderUtilities::Left => "border-l",
            BorderUtilities::Horizontal => "border-x",
            BorderUtilities::Vertical => "border-y",
        }
    }
}

impl fmt::Display for BorderUtilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.css_value())
    }
}

impl ClassBuilder {
    /// Set border utilities
    pub fn border_utilities(&mut self, utilities: BorderUtilities) -> &mut Self {
        *self = self.clone().class(utilities.class_name());
        self
    }

    /// Set border to none
    pub fn border_0_utilities(&mut self) -> &mut Self {
        self.border_utilities(BorderUtilities::None)
    }

    /// Set border to default
    pub fn border_default(&mut self) -> &mut Self {
        self.border_utilities(BorderUtilities::Default)
    }

    /// Set border to all
    pub fn border_all(&mut self) -> &mut Self {
        self.border_utilities(BorderUtilities::All)
    }

    /// Set border to top
    pub fn border_t(&mut self) -> &mut Self {
        self.border_utilities(BorderUtilities::Top)
    }

    /// Set border to right
    pub fn border_r(&mut self) -> &mut Self {
        self.border_utilities(BorderUtilities::Right)
    }

    /// Set border to bottom
    pub fn border_b(&mut self) -> &mut Self {
        self.border_utilities(BorderUtilities::Bottom)
    }

    /// Set border to left
    pub fn border_l(&mut self) -> &mut Self {
        self.border_utilities(BorderUtilities::Left)
    }

    /// Set border to horizontal
    pub fn border_x(&mut self) -> &mut Self {
        self.border_utilities(BorderUtilities::Horizontal)
    }

    /// Set border to vertical
    pub fn border_y(&mut self) -> &mut Self {
        self.border_utilities(BorderUtilities::Vertical)
    }
}
