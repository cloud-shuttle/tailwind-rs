//! Theme configuration
//!
//! This module provides theme configuration functionality.

use super::{ColorPalette, SpacingScale, TypographyScale, BreakpointSystem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Theme name
    pub name: String,
    /// Color palettes
    pub color_palettes: Vec<ColorPalette>,
    /// Spacing scale
    pub spacing: SpacingScale,
    /// Typography scale
    pub typography: TypographyScale,
    /// Breakpoint system
    pub breakpoints: BreakpointSystem,
    /// Custom CSS variables
    pub custom_variables: HashMap<String, String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            color_palettes: vec![],
            spacing: SpacingScale::default(),
            typography: TypographyScale::default(),
            breakpoints: BreakpointSystem::default(),
            custom_variables: HashMap::new(),
        }
    }
}

impl ThemeConfig {
    /// Create a new theme configuration
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Add a color palette
    pub fn add_color_palette(&mut self, palette: ColorPalette) {
        self.color_palettes.push(palette);
    }

    /// Add a custom CSS variable
    pub fn add_custom_variable(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.custom_variables.insert(name.into(), value.into());
    }

    /// Get theme name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get color palettes
    pub fn color_palettes(&self) -> &[ColorPalette] {
        &self.color_palettes
    }

    /// Get spacing scale
    pub fn spacing(&self) -> &SpacingScale {
        &self.spacing
    }

    /// Get typography scale
    pub fn typography(&self) -> &TypographyScale {
        &self.typography
    }

    /// Get breakpoint system
    pub fn breakpoints(&self) -> &BreakpointSystem {
        &self.breakpoints
    }

    /// Get custom variables
    pub fn custom_variables(&self) -> &HashMap<String, String> {
        &self.custom_variables
    }

    /// Validate the theme configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        // Validate theme name
        if self.name.is_empty() {
            return Err(crate::error::TailwindError::Theme {
                message: "Theme name cannot be empty".to_string()
            });
        }

        // Validate color palettes
        for palette in &self.color_palettes {
            if palette.name.is_empty() {
                return Err(crate::error::TailwindError::Theme {
                    message: "Color palette name cannot be empty".to_string()
                });
            }
            if palette.colors.is_empty() {
                return Err(crate::error::TailwindError::Theme {
                    message: format!("Color palette '{}' is empty", palette.name)
                });
            }
        }

        // Validate spacing scale
        if self.spacing.values().is_empty() {
            return Err(crate::error::TailwindError::Theme {
                message: "Spacing scale cannot be empty".to_string()
            });
        }

        // Validate typography scale
        if self.typography.font_sizes.is_empty() {
            return Err(crate::error::TailwindError::Theme {
                message: "Typography scale cannot be empty".to_string()
            });
        }

        // Validate breakpoints
        if self.breakpoints.breakpoints.is_empty() {
            return Err(crate::error::TailwindError::Theme {
                message: "Breakpoint system cannot be empty".to_string()
            });
        }

        Ok(())
    }
}
