//! Theme Variants Module
//!
//! Handles theme variant enums and their associated properties:
//! - ThemeVariant enum for different component styles
//! - Color associations and utility methods

use crate::color::Color;

/// Theme variant for different component styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeVariant {
    Primary,
    Secondary,
    Danger,
    Success,
    Warning,
    Info,
    Light,
    Dark,
}

impl ThemeVariant {
    /// Returns the associated color for the variant
    pub fn color(&self) -> Color {
        match self {
            ThemeVariant::Primary => Color::Blue,
            ThemeVariant::Secondary => Color::Gray,
            ThemeVariant::Danger => Color::Red,
            ThemeVariant::Success => Color::Green,
            ThemeVariant::Warning => Color::Yellow,
            ThemeVariant::Info => Color::Blue,
            ThemeVariant::Light => Color::Gray,
            ThemeVariant::Dark => Color::Gray,
        }
    }

    /// Returns the CSS class name for the variant
    pub fn class_name(&self) -> String {
        match self {
            ThemeVariant::Primary => "primary".to_string(),
            ThemeVariant::Secondary => "secondary".to_string(),
            ThemeVariant::Danger => "danger".to_string(),
            ThemeVariant::Success => "success".to_string(),
            ThemeVariant::Warning => "warning".to_string(),
            ThemeVariant::Info => "info".to_string(),
            ThemeVariant::Light => "light".to_string(),
            ThemeVariant::Dark => "dark".to_string(),
        }
    }

    /// Returns all available theme variants
    pub fn variants() -> &'static [ThemeVariant] {
        &[
            ThemeVariant::Primary,
            ThemeVariant::Secondary,
            ThemeVariant::Danger,
            ThemeVariant::Success,
            ThemeVariant::Warning,
            ThemeVariant::Info,
            ThemeVariant::Light,
            ThemeVariant::Dark,
        ]
    }

    /// Check if variant is considered "light"
    pub fn is_light(&self) -> bool {
        matches!(self, ThemeVariant::Light)
    }

    /// Check if variant is considered "dark"
    pub fn is_dark(&self) -> bool {
        matches!(self, ThemeVariant::Dark)
    }

    /// Check if variant represents a status (success, warning, danger, info)
    pub fn is_status(&self) -> bool {
        matches!(self, ThemeVariant::Success | ThemeVariant::Warning | ThemeVariant::Danger | ThemeVariant::Info)
    }
}

impl std::fmt::Display for ThemeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_variant_colors() {
        assert_eq!(ThemeVariant::Primary.color(), Color::Blue);
        assert_eq!(ThemeVariant::Danger.color(), Color::Red);
        assert_eq!(ThemeVariant::Success.color(), Color::Green);
        assert_eq!(ThemeVariant::Warning.color(), Color::Yellow);
    }

    #[test]
    fn theme_variant_class_names() {
        assert_eq!(ThemeVariant::Primary.class_name(), "primary");
        assert_eq!(ThemeVariant::Danger.class_name(), "danger");
        assert_eq!(ThemeVariant::Light.class_name(), "light");
        assert_eq!(ThemeVariant::Dark.class_name(), "dark");
    }

    #[test]
    fn theme_variant_properties() {
        assert!(ThemeVariant::Light.is_light());
        assert!(!ThemeVariant::Dark.is_light());

        assert!(ThemeVariant::Dark.is_dark());
        assert!(!ThemeVariant::Light.is_dark());

        assert!(ThemeVariant::Success.is_status());
        assert!(ThemeVariant::Danger.is_status());
        assert!(!ThemeVariant::Primary.is_status());
    }

    #[test]
    fn theme_variant_variants() {
        let variants = ThemeVariant::variants();
        assert_eq!(variants.len(), 8);
        assert!(variants.contains(&ThemeVariant::Primary));
        assert!(variants.contains(&ThemeVariant::Dark));
    }

    #[test]
    fn theme_variant_display() {
        assert_eq!(ThemeVariant::Primary.to_string(), "primary");
        assert_eq!(ThemeVariant::Success.to_string(), "success");
    }
}
