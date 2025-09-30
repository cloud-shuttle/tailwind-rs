//! Blur Filter Utilities Module
//!
//! Handles blur filter utilities:
//! - Blur enum for different blur sizes
//! - CSS value generation and class name utilities

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Blur values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Blur {
    /// No blur
    None,
    /// Small blur
    Sm,
    /// Default blur
    Default,
    /// Medium blur
    Md,
    /// Large blur
    Lg,
    /// Extra large blur
    Xl,
    /// 2x large blur
    Xl2,
    /// 3x large blur
    Xl3,
}

impl Blur {
    /// Returns the CSS value for the blur filter
    pub fn css_value(&self) -> &'static str {
        match self {
            Blur::None => "blur(0)",
            Blur::Sm => "blur(4px)",
            Blur::Default => "blur(8px)",
            Blur::Md => "blur(12px)",
            Blur::Lg => "blur(16px)",
            Blur::Xl => "blur(24px)",
            Blur::Xl2 => "blur(40px)",
            Blur::Xl3 => "blur(64px)",
        }
    }

    /// Returns the class name suffix for the blur filter
    pub fn class_suffix(&self) -> &'static str {
        match self {
            Blur::None => "none",
            Blur::Sm => "sm",
            Blur::Default => "",
            Blur::Md => "md",
            Blur::Lg => "lg",
            Blur::Xl => "xl",
            Blur::Xl2 => "2xl",
            Blur::Xl3 => "3xl",
        }
    }

    /// Returns the full Tailwind class name
    pub fn class_name(&self) -> String {
        if self.class_suffix().is_empty() {
            "blur".to_string()
        } else {
            format!("blur-{}", self.class_suffix())
        }
    }

    /// Check if this blur has no effect
    pub fn is_none(&self) -> bool {
        matches!(self, Blur::None)
    }

    /// Check if this is a large blur
    pub fn is_large(&self) -> bool {
        matches!(self, Blur::Xl | Blur::Xl2 | Blur::Xl3)
    }

    /// Returns all available blur variants
    pub fn variants() -> &'static [Blur] {
        &[
            Blur::None,
            Blur::Sm,
            Blur::Default,
            Blur::Md,
            Blur::Lg,
            Blur::Xl,
            Blur::Xl2,
            Blur::Xl3,
        ]
    }
}

impl fmt::Display for Blur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

impl Default for Blur {
    fn default() -> Self {
        Blur::None
    }
}

/// Blur filter utilities trait for extending ClassBuilder
pub trait BlurUtilities {
    /// Add blur filter class
    fn blur(self, blur: Blur) -> Self;

    /// Add blur-none class
    fn blur_none(self) -> Self;

    /// Add blur-sm class
    fn blur_sm(self) -> Self;

    /// Add blur class (default)
    fn blur_default(self) -> Self;

    /// Add blur-md class
    fn blur_md(self) -> Self;

    /// Add blur-lg class
    fn blur_lg(self) -> Self;

    /// Add blur-xl class
    fn blur_xl(self) -> Self;

    /// Add blur-2xl class
    fn blur_2xl(self) -> Self;

    /// Add blur-3xl class
    fn blur_3xl(self) -> Self;
}

impl BlurUtilities for ClassBuilder {
    fn blur(self, blur: Blur) -> Self {
        self.class(&blur.class_name())
    }

    fn blur_none(self) -> Self {
        self.class("blur-none")
    }

    fn blur_sm(self) -> Self {
        self.class("blur-sm")
    }

    fn blur_default(self) -> Self {
        self.class("blur")
    }

    fn blur_md(self) -> Self {
        self.class("blur-md")
    }

    fn blur_lg(self) -> Self {
        self.class("blur-lg")
    }

    fn blur_xl(self) -> Self {
        self.class("blur-xl")
    }

    fn blur_2xl(self) -> Self {
        self.class("blur-2xl")
    }

    fn blur_3xl(self) -> Self {
        self.class("blur-3xl")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blur_css_values() {
        assert_eq!(Blur::None.css_value(), "blur(0)");
        assert_eq!(Blur::Sm.css_value(), "blur(4px)");
        assert_eq!(Blur::Default.css_value(), "blur(8px)");
        assert_eq!(Blur::Xl3.css_value(), "blur(64px)");
    }

    #[test]
    fn blur_class_names() {
        assert_eq!(Blur::None.class_name(), "blur-none");
        assert_eq!(Blur::Sm.class_name(), "blur-sm");
        assert_eq!(Blur::Default.class_name(), "blur");
        assert_eq!(Blur::Xl.class_name(), "blur-xl");
    }

    #[test]
    fn blur_properties() {
        assert!(Blur::None.is_none());
        assert!(!Blur::Sm.is_none());

        assert!(Blur::Xl.is_large());
        assert!(Blur::Xl3.is_large());
        assert!(!Blur::Sm.is_large());
    }

    #[test]
    fn blur_variants() {
        let variants = Blur::variants();
        assert_eq!(variants.len(), 8);
        assert!(variants.contains(&Blur::None));
        assert!(variants.contains(&Blur::Xl3));
    }

    #[test]
    fn blur_display() {
        assert_eq!(Blur::Default.to_string(), "blur");
        assert_eq!(Blur::Sm.to_string(), "blur-sm");
    }

    #[test]
    fn blur_utilities_trait() {
        let builder = ClassBuilder::new();

        // Test trait methods (simplified - would need actual ClassBuilder implementation)
        let _result = builder.blur(Blur::Sm).blur_none().blur_xl();
        // In a real implementation, this would add classes to the builder
    }
}
