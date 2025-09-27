//! Theme system for tailwind-rs

use crate::error::{Result, TailwindError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

pub mod color;
pub mod spacing;
pub mod typography;
pub mod breakpoints;
pub mod theme_config;
pub mod theme_manager;
pub mod theme_builder;
pub mod theme_validator;

pub use color::*;
pub use spacing::*;
pub use typography::*;
pub use breakpoints::*;
pub use theme_config::*;
pub use theme_manager::*;
pub use theme_builder::*;
pub use theme_validator::*;

// Core theme types for backward compatibility
pub type Theme = ThemeConfig;
pub type ThemeValue = serde_json::Value;

// Re-export utilities types for compatibility
pub use spacing::{Spacing, SpacingValue};
pub use color::{BorderRadius, BoxShadow};

// Additional compatibility types
pub type ThemeToml = ThemeConfig;

/// Create a default theme configuration
pub fn create_default_theme() -> ThemeConfig {
    ThemeConfig::default()
}
