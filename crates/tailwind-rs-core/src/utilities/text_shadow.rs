//! Text shadow utilities for tailwind-rs
//!
//! This module provides utilities for text-shadow CSS property.
//! It includes all Tailwind CSS text-shadow variants.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Text shadow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextShadow {
    /// No text shadow
    None,
    /// Small text shadow (0 1px 2px 0 rgb(0 0 0 / 0.05))
    Sm,
    /// Medium text shadow (0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1))
    Md,
    /// Default text shadow (0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1))
    Default,
    /// Large text shadow (0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1))
    Lg,
    /// Extra large text shadow (0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1))
    Xl,
    /// 2XL text shadow (0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1))
    Xl2,
    /// Inner text shadow (inset 0 2px 4px 0 rgb(0 0 0 / 0.05))
    Inner,
}

impl fmt::Display for TextShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextShadow::None => write!(f, "none"),
            TextShadow::Sm => write!(f, "0 1px 2px 0 rgb(0 0 0 / 0.05)"),
            TextShadow::Md => write!(
                f,
                "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)"
            ),
            TextShadow::Default => write!(
                f,
                "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)"
            ),
            TextShadow::Lg => write!(
                f,
                "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)"
            ),
            TextShadow::Xl => write!(
                f,
                "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)"
            ),
            TextShadow::Xl2 => write!(f, "0 25px 50px -12px rgb(0 0 0 / 0.25)"),
            TextShadow::Inner => write!(f, "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)"),
        }
    }
}

impl TextShadow {
    /// Get the CSS class name for this text shadow
    pub fn to_class_name(&self) -> String {
        match self {
            TextShadow::None => "text-shadow-none".to_string(),
            TextShadow::Sm => "text-shadow-sm".to_string(),
            TextShadow::Md => "text-shadow-md".to_string(),
            TextShadow::Default => "text-shadow".to_string(),
            TextShadow::Lg => "text-shadow-lg".to_string(),
            TextShadow::Xl => "text-shadow-xl".to_string(),
            TextShadow::Xl2 => "text-shadow-2xl".to_string(),
            TextShadow::Inner => "text-shadow-inner".to_string(),
        }
    }

    /// Get the CSS value for this text shadow
    pub fn to_css_value(&self) -> String {
        self.to_string()
    }
}

/// Trait for adding text shadow utilities to ClassBuilder
pub trait TextShadowUtilities {
    /// Set text shadow to none
    fn text_shadow_none(self) -> Self;
    /// Set text shadow to small
    fn text_shadow_sm(self) -> Self;
    /// Set text shadow to medium
    fn text_shadow_md(self) -> Self;
    /// Set text shadow to default
    fn text_shadow(self) -> Self;
    /// Set text shadow to large
    fn text_shadow_lg(self) -> Self;
    /// Set text shadow to extra large
    fn text_shadow_xl(self) -> Self;
    /// Set text shadow to 2XL
    fn text_shadow_2xl(self) -> Self;
    /// Set text shadow to inner
    fn text_shadow_inner(self) -> Self;
    /// Set text shadow with custom value
    fn text_shadow_custom(self, shadow: TextShadow) -> Self;
}

impl TextShadowUtilities for ClassBuilder {
    fn text_shadow_none(self) -> Self {
        self.class("text-shadow-none")
    }

    fn text_shadow_sm(self) -> Self {
        self.class("text-shadow-sm")
    }

    fn text_shadow_md(self) -> Self {
        self.class("text-shadow-md")
    }

    fn text_shadow(self) -> Self {
        self.class("text-shadow")
    }

    fn text_shadow_lg(self) -> Self {
        self.class("text-shadow-lg")
    }

    fn text_shadow_xl(self) -> Self {
        self.class("text-shadow-xl")
    }

    fn text_shadow_2xl(self) -> Self {
        self.class("text-shadow-2xl")
    }

    fn text_shadow_inner(self) -> Self {
        self.class("text-shadow-inner")
    }

    fn text_shadow_custom(self, shadow: TextShadow) -> Self {
        self.class(&shadow.to_class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ClassBuilder;

    #[test]
    fn test_text_shadow_enum_values() {
        assert_eq!(TextShadow::None.to_string(), "none");
        assert_eq!(TextShadow::Sm.to_string(), "0 1px 2px 0 rgb(0 0 0 / 0.05)");
        assert_eq!(
            TextShadow::Default.to_string(),
            "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)"
        );
        assert_eq!(
            TextShadow::Lg.to_string(),
            "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)"
        );
        assert_eq!(
            TextShadow::Xl.to_string(),
            "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)"
        );
        assert_eq!(
            TextShadow::Xl2.to_string(),
            "0 25px 50px -12px rgb(0 0 0 / 0.25)"
        );
        assert_eq!(
            TextShadow::Inner.to_string(),
            "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)"
        );
    }

    #[test]
    fn test_text_shadow_class_names() {
        assert_eq!(TextShadow::None.to_class_name(), "text-shadow-none");
        assert_eq!(TextShadow::Sm.to_class_name(), "text-shadow-sm");
        assert_eq!(TextShadow::Default.to_class_name(), "text-shadow");
        assert_eq!(TextShadow::Lg.to_class_name(), "text-shadow-lg");
        assert_eq!(TextShadow::Xl.to_class_name(), "text-shadow-xl");
        assert_eq!(TextShadow::Xl2.to_class_name(), "text-shadow-2xl");
        assert_eq!(TextShadow::Inner.to_class_name(), "text-shadow-inner");
    }

    #[test]
    fn test_text_shadow_utilities() {
        let classes = ClassBuilder::new()
            .text_shadow_none()
            .text_shadow_sm()
            .text_shadow()
            .text_shadow_lg()
            .text_shadow_xl()
            .text_shadow_2xl()
            .text_shadow_inner();

        let result = classes.build();
        assert!(result.classes.contains("text-shadow-none"));
        assert!(result.classes.contains("text-shadow-sm"));
        assert!(result.classes.contains("text-shadow"));
        assert!(result.classes.contains("text-shadow-lg"));
        assert!(result.classes.contains("text-shadow-xl"));
        assert!(result.classes.contains("text-shadow-2xl"));
        assert!(result.classes.contains("text-shadow-inner"));
    }

    #[test]
    fn test_text_shadow_css_values() {
        assert_eq!(TextShadow::None.to_css_value(), "none");
        assert_eq!(
            TextShadow::Sm.to_css_value(),
            "0 1px 2px 0 rgb(0 0 0 / 0.05)"
        );
        assert_eq!(
            TextShadow::Default.to_css_value(),
            "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)"
        );
        assert_eq!(
            TextShadow::Lg.to_css_value(),
            "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)"
        );
        assert_eq!(
            TextShadow::Xl.to_css_value(),
            "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)"
        );
        assert_eq!(
            TextShadow::Xl2.to_css_value(),
            "0 25px 50px -12px rgb(0 0 0 / 0.25)"
        );
        assert_eq!(
            TextShadow::Inner.to_css_value(),
            "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)"
        );
    }

    #[test]
    fn test_text_shadow_serialization() {
        let shadow = TextShadow::Lg;
        let serialized = serde_json::to_string(&shadow).unwrap();
        let deserialized: TextShadow = serde_json::from_str(&serialized).unwrap();
        assert_eq!(shadow, deserialized);
    }

    #[test]
    fn test_text_shadow_comprehensive_usage() {
        let classes = ClassBuilder::new().text_shadow_sm().text_shadow_lg();

        let result = classes.build();
        assert!(result.classes.contains("text-shadow-sm"));
        assert!(result.classes.contains("text-shadow-lg"));
    }

    #[test]
    fn test_text_shadow_custom_usage() {
        let classes = ClassBuilder::new()
            .text_shadow_custom(TextShadow::Xl)
            .text_shadow_custom(TextShadow::Inner);

        let result = classes.build();
        assert!(result.classes.contains("text-shadow-xl"));
        assert!(result.classes.contains("text-shadow-inner"));
    }
}
