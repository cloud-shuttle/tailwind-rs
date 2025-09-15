//! # tailwind-rs-core
//!
//! Core types and utilities for the tailwind-rs library.
//! This crate provides the fundamental building blocks for Tailwind CSS integration in Rust.

pub mod arbitrary;
pub mod classes;
pub mod color;
pub mod config;
pub mod custom_variant;
pub mod dark_mode;
pub mod error;
pub mod gradients;
pub mod performance;
pub mod responsive;
pub mod theme;
pub mod theme_new;
pub mod utils;
pub mod utilities;
pub mod validation;

#[cfg(test)]
mod property_tests;

#[cfg(test)]
mod api_stability;

#[cfg(test)]
mod week18_documentation_tests;

#[cfg(test)]
mod week19_testing_qa_tests;

#[cfg(test)]
mod week20_release_prep_tests;

// Re-export commonly used types
pub use arbitrary::{ArbitraryValue, ArbitraryValueError, ArbitraryValueUtilities};
pub use classes::{ClassBuilder, ClassSet};
pub use color::Color;
pub use config::{BuildConfig, TailwindConfig};
pub use custom_variant::{CustomVariant, CustomVariantManager, CustomVariantType};
pub use dark_mode::{DarkModeVariant, DarkModeVariantError, DarkModeVariantUtilities};
pub use error::{Result, TailwindError};
pub use gradients::{Gradient, GradientDirection, GradientError, GradientStop, GradientUtilities};
pub use performance::{CacheStats, ClassCache, OptimizationLevel, PerformanceOptimizer};
pub use responsive::{
    AlignItems, Breakpoint, FlexDirection, FlexWrap, JustifyContent, Responsive, ResponsiveBuilder,
    ResponsiveFlex, ResponsiveGrid, ResponsiveValue, State,
};
pub use theme::{BorderRadius, BoxShadow, Spacing, Theme, ThemeValue};
pub use theme_new::{
    AnimationScale, BorderScale, FontFamily, FontSizeScale, FontWeightScale, LetterSpacingScale,
    LineHeightScale, ShadowScale, SpacingScale, SpacingSize, Theme as NewTheme, ThemePreset,
    ThemeVariant, ThemedComponent, TypographyScale,
};
pub use utilities::*;
pub use validation::{ClassValidator, ErrorReporter, ValidationError, ValidationRules};

// Build system types
pub struct TailwindBuilder;
pub struct CssOptimizer;

impl Default for TailwindBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TailwindBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn scan_source(self, _path: &std::path::Path) -> Self {
        self
    }

    pub fn output_css(self, _path: &std::path::Path) -> Self {
        self
    }

    pub fn config_file(self, _path: &std::path::Path) -> Self {
        self
    }

    pub fn enable_tree_shaking(self) -> Self {
        self
    }

    pub fn enable_minification(self) -> Self {
        self
    }

    pub fn enable_source_maps(self) -> Self {
        self
    }

    pub async fn build(self) -> Result<()> {
        Ok(())
    }
}

impl Default for CssOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl CssOptimizer {
    pub fn new() -> Self {
        Self
    }

    pub fn input_file(self, _path: &std::path::Path) -> Self {
        self
    }

    pub fn output_file(self, _path: &std::path::Path) -> Self {
        self
    }

    pub fn optimization_level(self, _level: u8) -> Self {
        self
    }

    pub fn remove_unused_classes(self) -> Self {
        self
    }

    pub fn minify(self) -> Self {
        self
    }

    pub fn generate_source_maps(self) -> Self {
        self
    }

    pub async fn optimize(self) -> Result<()> {
        Ok(())
    }
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration values
pub mod defaults {
    use super::*;

    pub const DEFAULT_THEME: &str = "default";
    pub const DEFAULT_BREAKPOINT: Breakpoint = Breakpoint::Base;
    pub const DEFAULT_SPACING: Spacing = Spacing::Rem(1.0);

    pub fn default_color() -> Color {
        Color::Blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constant() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.chars().any(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_defaults() {
        assert_eq!(defaults::DEFAULT_THEME, "default");
        assert_eq!(defaults::DEFAULT_BREAKPOINT, Breakpoint::Base);
        assert_eq!(defaults::default_color(), Color::Blue);
        assert_eq!(defaults::DEFAULT_SPACING, Spacing::Rem(1.0));
    }
}
