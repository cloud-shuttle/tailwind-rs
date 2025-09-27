//! Theme builder
//!
//! This module provides theme building functionality.

use super::{ThemeConfig, ColorPalette, SpacingScale, TypographyScale, BreakpointSystem};

/// Theme builder
pub struct ThemeBuilder {
    config: ThemeConfig,
}

impl ThemeBuilder {
    /// Create a new theme builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            config: ThemeConfig::new(name),
        }
    }

    /// Add a color palette
    pub fn with_color_palette(mut self, palette: ColorPalette) -> Self {
        self.config.add_color_palette(palette);
        self
    }

    /// Set spacing scale
    pub fn with_spacing(mut self, spacing: SpacingScale) -> Self {
        self.config.spacing = spacing;
        self
    }

    /// Set typography scale
    pub fn with_typography(mut self, typography: TypographyScale) -> Self {
        self.config.typography = typography;
        self
    }

    /// Set breakpoint system
    pub fn with_breakpoints(mut self, breakpoints: BreakpointSystem) -> Self {
        self.config.breakpoints = breakpoints;
        self
    }

    /// Add custom variable
    pub fn with_custom_variable(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.add_custom_variable(name, value);
        self
    }

    /// Build the theme configuration
    pub fn build(self) -> ThemeConfig {
        self.config
    }
}

impl Default for ThemeBuilder {
    fn default() -> Self {
        Self::new("default")
    }
}
