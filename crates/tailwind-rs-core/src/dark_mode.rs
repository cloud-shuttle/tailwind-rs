//! Dark mode variant support for tailwind-rs
//!
//! This module provides support for Tailwind CSS dark mode variants,
//! allowing users to specify styles that apply only in dark mode.
//! Examples: dark:bg-gray-800, dark:text-white, dark:hover:bg-gray-700, etc.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a dark mode variant in Tailwind CSS
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DarkModeVariant {
    /// The base class name (e.g., "bg-gray-800", "text-white")
    pub class: String,
    /// Whether this is a hover variant (dark:hover:bg-gray-700)
    pub is_hover: bool,
    /// Whether this is a focus variant (dark:focus:bg-gray-700)
    pub is_focus: bool,
    /// Whether this is an active variant (dark:active:bg-gray-700)
    pub is_active: bool,
    /// Whether this is a disabled variant (dark:disabled:bg-gray-700)
    pub is_disabled: bool,
    /// Whether this is a checked variant (dark:checked:bg-gray-700)
    pub is_checked: bool,
    /// Whether this is a group hover variant (dark:group-hover:bg-gray-700)
    pub is_group_hover: bool,
    /// Whether this is a group focus variant (dark:group-focus:bg-gray-700)
    pub is_group_focus: bool,
}

impl DarkModeVariant {
    /// Create a new dark mode variant
    pub fn new(class: impl Into<String>) -> Self {
        Self {
            class: class.into(),
            is_hover: false,
            is_focus: false,
            is_active: false,
            is_disabled: false,
            is_checked: false,
            is_group_hover: false,
            is_group_focus: false,
        }
    }

    /// Create a dark mode hover variant
    pub fn hover(class: impl Into<String>) -> Self {
        Self {
            class: class.into(),
            is_hover: true,
            is_focus: false,
            is_active: false,
            is_disabled: false,
            is_checked: false,
            is_group_hover: false,
            is_group_focus: false,
        }
    }

    /// Create a dark mode focus variant
    pub fn focus(class: impl Into<String>) -> Self {
        Self {
            class: class.into(),
            is_hover: false,
            is_focus: true,
            is_active: false,
            is_disabled: false,
            is_checked: false,
            is_group_hover: false,
            is_group_focus: false,
        }
    }

    /// Create a dark mode active variant
    pub fn active(class: impl Into<String>) -> Self {
        Self {
            class: class.into(),
            is_hover: false,
            is_focus: false,
            is_active: true,
            is_disabled: false,
            is_checked: false,
            is_group_hover: false,
            is_group_focus: false,
        }
    }

    /// Create a dark mode disabled variant
    pub fn disabled(class: impl Into<String>) -> Self {
        Self {
            class: class.into(),
            is_hover: false,
            is_focus: false,
            is_active: false,
            is_disabled: true,
            is_checked: false,
            is_group_hover: false,
            is_group_focus: false,
        }
    }

    /// Create a dark mode checked variant
    pub fn checked(class: impl Into<String>) -> Self {
        Self {
            class: class.into(),
            is_hover: false,
            is_focus: false,
            is_active: false,
            is_disabled: false,
            is_checked: true,
            is_group_hover: false,
            is_group_focus: false,
        }
    }

    /// Create a dark mode group hover variant
    pub fn group_hover(class: impl Into<String>) -> Self {
        Self {
            class: class.into(),
            is_hover: false,
            is_focus: false,
            is_active: false,
            is_disabled: false,
            is_checked: false,
            is_group_hover: true,
            is_group_focus: false,
        }
    }

    /// Create a dark mode group focus variant
    pub fn group_focus(class: impl Into<String>) -> Self {
        Self {
            class: class.into(),
            is_hover: false,
            is_focus: false,
            is_active: false,
            is_disabled: false,
            is_checked: false,
            is_group_hover: false,
            is_group_focus: true,
        }
    }

    /// Convert to Tailwind CSS class name
    pub fn to_class_name(&self) -> String {
        let mut prefix = "dark:".to_string();
        
        if self.is_group_hover {
            prefix.push_str("group-hover:");
        } else if self.is_group_focus {
            prefix.push_str("group-focus:");
        } else if self.is_hover {
            prefix.push_str("hover:");
        } else if self.is_focus {
            prefix.push_str("focus:");
        } else if self.is_active {
            prefix.push_str("active:");
        } else if self.is_disabled {
            prefix.push_str("disabled:");
        } else if self.is_checked {
            prefix.push_str("checked:");
        }
        
        format!("{}{}", prefix, self.class)
    }

    /// Validate the dark mode variant
    pub fn validate(&self) -> Result<(), DarkModeVariantError> {
        if self.class.is_empty() {
            return Err(DarkModeVariantError::EmptyClass);
        }

        // Check for conflicting states
        let state_count = [
            self.is_hover,
            self.is_focus,
            self.is_active,
            self.is_disabled,
            self.is_checked,
            self.is_group_hover,
            self.is_group_focus,
        ]
        .iter()
        .filter(|&&state| state)
        .count();

        if state_count > 1 {
            return Err(DarkModeVariantError::ConflictingStates);
        }

        Ok(())
    }
}

/// Errors that can occur when working with dark mode variants
#[derive(Debug, thiserror::Error)]
pub enum DarkModeVariantError {
    #[error("Empty class name")]
    EmptyClass,
    
    #[error("Conflicting states: only one state modifier can be used at a time")]
    ConflictingStates,
}

impl fmt::Display for DarkModeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding dark mode variant utilities to a class builder
pub trait DarkModeVariantUtilities {
    /// Add a dark mode variant
    fn dark_mode(self, class: impl Into<String>) -> Self;
    
    /// Add a dark mode hover variant
    fn dark_hover(self, class: impl Into<String>) -> Self;
    
    /// Add a dark mode focus variant
    fn dark_focus(self, class: impl Into<String>) -> Self;
    
    /// Add a dark mode active variant
    fn dark_active(self, class: impl Into<String>) -> Self;
    
    /// Add a dark mode disabled variant
    fn dark_disabled(self, class: impl Into<String>) -> Self;
    
    /// Add a dark mode checked variant
    fn dark_checked(self, class: impl Into<String>) -> Self;
    
    /// Add a dark mode group hover variant
    fn dark_group_hover(self, class: impl Into<String>) -> Self;
    
    /// Add a dark mode group focus variant
    fn dark_group_focus(self, class: impl Into<String>) -> Self;
    
    /// Add a dark mode background color
    fn dark_bg(self, color: impl Into<String>) -> Self;
    
    /// Add a dark mode text color
    fn dark_text(self, color: impl Into<String>) -> Self;
    
    /// Add a dark mode border color
    fn dark_border(self, color: impl Into<String>) -> Self;
    
    /// Add a dark mode hover background color
    fn dark_hover_bg(self, color: impl Into<String>) -> Self;
    
    /// Add a dark mode hover text color
    fn dark_hover_text(self, color: impl Into<String>) -> Self;
    
    /// Add a dark mode focus background color
    fn dark_focus_bg(self, color: impl Into<String>) -> Self;
    
    /// Add a dark mode focus text color
    fn dark_focus_text(self, color: impl Into<String>) -> Self;
}

impl DarkModeVariantUtilities for ClassBuilder {
    fn dark_mode(self, class: impl Into<String>) -> Self {
        let variant = DarkModeVariant::new(class);
        self.class(variant.to_class_name())
    }
    
    fn dark_hover(self, class: impl Into<String>) -> Self {
        let variant = DarkModeVariant::hover(class);
        self.class(variant.to_class_name())
    }
    
    fn dark_focus(self, class: impl Into<String>) -> Self {
        let variant = DarkModeVariant::focus(class);
        self.class(variant.to_class_name())
    }
    
    fn dark_active(self, class: impl Into<String>) -> Self {
        let variant = DarkModeVariant::active(class);
        self.class(variant.to_class_name())
    }
    
    fn dark_disabled(self, class: impl Into<String>) -> Self {
        let variant = DarkModeVariant::disabled(class);
        self.class(variant.to_class_name())
    }
    
    fn dark_checked(self, class: impl Into<String>) -> Self {
        let variant = DarkModeVariant::checked(class);
        self.class(variant.to_class_name())
    }
    
    fn dark_group_hover(self, class: impl Into<String>) -> Self {
        let variant = DarkModeVariant::group_hover(class);
        self.class(variant.to_class_name())
    }
    
    fn dark_group_focus(self, class: impl Into<String>) -> Self {
        let variant = DarkModeVariant::group_focus(class);
        self.class(variant.to_class_name())
    }
    
    fn dark_bg(self, color: impl Into<String>) -> Self {
        self.dark_mode(format!("bg-{}", color.into()))
    }
    
    fn dark_text(self, color: impl Into<String>) -> Self {
        self.dark_mode(format!("text-{}", color.into()))
    }
    
    fn dark_border(self, color: impl Into<String>) -> Self {
        self.dark_mode(format!("border-{}", color.into()))
    }
    
    fn dark_hover_bg(self, color: impl Into<String>) -> Self {
        self.dark_hover(format!("bg-{}", color.into()))
    }
    
    fn dark_hover_text(self, color: impl Into<String>) -> Self {
        self.dark_hover(format!("text-{}", color.into()))
    }
    
    fn dark_focus_bg(self, color: impl Into<String>) -> Self {
        self.dark_focus(format!("bg-{}", color.into()))
    }
    
    fn dark_focus_text(self, color: impl Into<String>) -> Self {
        self.dark_focus(format!("text-{}", color.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dark_mode_variant_creation() {
        let variant = DarkModeVariant::new("bg-gray-800");
        assert_eq!(variant.to_class_name(), "dark:bg-gray-800");
    }
    
    #[test]
    fn test_dark_mode_hover_variant() {
        let variant = DarkModeVariant::hover("bg-gray-700");
        assert_eq!(variant.to_class_name(), "dark:hover:bg-gray-700");
    }
    
    #[test]
    fn test_dark_mode_focus_variant() {
        let variant = DarkModeVariant::focus("bg-gray-700");
        assert_eq!(variant.to_class_name(), "dark:focus:bg-gray-700");
    }
    
    #[test]
    fn test_dark_mode_active_variant() {
        let variant = DarkModeVariant::active("bg-gray-700");
        assert_eq!(variant.to_class_name(), "dark:active:bg-gray-700");
    }
    
    #[test]
    fn test_dark_mode_disabled_variant() {
        let variant = DarkModeVariant::disabled("bg-gray-700");
        assert_eq!(variant.to_class_name(), "dark:disabled:bg-gray-700");
    }
    
    #[test]
    fn test_dark_mode_checked_variant() {
        let variant = DarkModeVariant::checked("bg-gray-700");
        assert_eq!(variant.to_class_name(), "dark:checked:bg-gray-700");
    }
    
    #[test]
    fn test_dark_mode_group_hover_variant() {
        let variant = DarkModeVariant::group_hover("bg-gray-700");
        assert_eq!(variant.to_class_name(), "dark:group-hover:bg-gray-700");
    }
    
    #[test]
    fn test_dark_mode_group_focus_variant() {
        let variant = DarkModeVariant::group_focus("bg-gray-700");
        assert_eq!(variant.to_class_name(), "dark:group-focus:bg-gray-700");
    }
    
    #[test]
    fn test_dark_mode_variant_validation() {
        // Valid variants
        assert!(DarkModeVariant::new("bg-gray-800").validate().is_ok());
        assert!(DarkModeVariant::hover("bg-gray-700").validate().is_ok());
        assert!(DarkModeVariant::focus("bg-gray-700").validate().is_ok());
        
        // Invalid variants
        assert!(DarkModeVariant::new("").validate().is_err());
    }
    
    #[test]
    fn test_dark_mode_variant_display() {
        let variant = DarkModeVariant::new("bg-gray-800");
        assert_eq!(format!("{}", variant), "dark:bg-gray-800");
    }
    
    #[test]
    fn test_dark_mode_variant_utilities() {
        let classes = ClassBuilder::new()
            .dark_bg("gray-800")
            .dark_text("white")
            .dark_border("gray-700")
            .dark_hover_bg("gray-700")
            .dark_hover_text("gray-100")
            .dark_focus_bg("gray-600")
            .dark_focus_text("gray-50")
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("dark:bg-gray-800"));
        assert!(css_classes.contains("dark:text-white"));
        assert!(css_classes.contains("dark:border-gray-700"));
        assert!(css_classes.contains("dark:hover:bg-gray-700"));
        assert!(css_classes.contains("dark:hover:text-gray-100"));
        assert!(css_classes.contains("dark:focus:bg-gray-600"));
        assert!(css_classes.contains("dark:focus:text-gray-50"));
    }
    
    #[test]
    fn test_dark_mode_variant_utilities_advanced() {
        let classes = ClassBuilder::new()
            .dark_mode("bg-gray-800")
            .dark_hover("bg-gray-700")
            .dark_focus("bg-gray-600")
            .dark_active("bg-gray-500")
            .dark_disabled("bg-gray-400")
            .dark_checked("bg-gray-300")
            .dark_group_hover("bg-gray-200")
            .dark_group_focus("bg-gray-100")
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("dark:bg-gray-800"));
        assert!(css_classes.contains("dark:hover:bg-gray-700"));
        assert!(css_classes.contains("dark:focus:bg-gray-600"));
        assert!(css_classes.contains("dark:active:bg-gray-500"));
        assert!(css_classes.contains("dark:disabled:bg-gray-400"));
        assert!(css_classes.contains("dark:checked:bg-gray-300"));
        assert!(css_classes.contains("dark:group-hover:bg-gray-200"));
        assert!(css_classes.contains("dark:group-focus:bg-gray-100"));
    }
}
