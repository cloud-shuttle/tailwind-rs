//! Device variant utilities for tailwind-rs
//!
//! This module provides utilities for device-specific media queries like pointer variants,
//! motion preferences, and other device capabilities for better accessibility and device targeting.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Pointer capability variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointerVariant {
    /// Coarse pointer (touch, stylus)
    Coarse,
    /// Fine pointer (mouse, trackpad)
    Fine,
    /// Any coarse pointer
    AnyCoarse,
    /// Any fine pointer
    AnyFine,
}

/// Motion preference variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MotionVariant {
    /// Reduced motion preference
    Reduced,
    /// No motion preference
    NoPreference,
}

/// Color scheme variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorSchemeVariant {
    /// Light color scheme
    Light,
    /// Dark color scheme
    Dark,
}

impl PointerVariant {
    pub fn to_class_name(&self) -> String {
        match self {
            PointerVariant::Coarse => "pointer-coarse".to_string(),
            PointerVariant::Fine => "pointer-fine".to_string(),
            PointerVariant::AnyCoarse => "any-pointer-coarse".to_string(),
            PointerVariant::AnyFine => "any-pointer-fine".to_string(),
        }
    }
    
    pub fn to_media_query(&self) -> String {
        match self {
            PointerVariant::Coarse => "@media (pointer: coarse)".to_string(),
            PointerVariant::Fine => "@media (pointer: fine)".to_string(),
            PointerVariant::AnyCoarse => "@media (any-pointer: coarse)".to_string(),
            PointerVariant::AnyFine => "@media (any-pointer: fine)".to_string(),
        }
    }
}

impl MotionVariant {
    pub fn to_class_name(&self) -> String {
        match self {
            MotionVariant::Reduced => "motion-reduce".to_string(),
            MotionVariant::NoPreference => "motion-safe".to_string(),
        }
    }
    
    pub fn to_media_query(&self) -> String {
        match self {
            MotionVariant::Reduced => "@media (prefers-reduced-motion: reduce)".to_string(),
            MotionVariant::NoPreference => "@media (prefers-reduced-motion: no-preference)".to_string(),
        }
    }
}

impl ColorSchemeVariant {
    pub fn to_class_name(&self) -> String {
        match self {
            ColorSchemeVariant::Light => "light".to_string(),
            ColorSchemeVariant::Dark => "dark".to_string(),
        }
    }
    
    pub fn to_media_query(&self) -> String {
        match self {
            ColorSchemeVariant::Light => "@media (prefers-color-scheme: light)".to_string(),
            ColorSchemeVariant::Dark => "@media (prefers-color-scheme: dark)".to_string(),
        }
    }
}

impl fmt::Display for PointerVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for MotionVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for ColorSchemeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding pointer variant utilities to a class builder
pub trait PointerVariantUtilities {
    /// Add pointer coarse variant
    fn pointer_coarse(self) -> Self;
    /// Add pointer fine variant
    fn pointer_fine(self) -> Self;
    /// Add any pointer coarse variant
    fn any_pointer_coarse(self) -> Self;
    /// Add any pointer fine variant
    fn any_pointer_fine(self) -> Self;
}

impl PointerVariantUtilities for ClassBuilder {
    fn pointer_coarse(self) -> Self {
        self.class("pointer-coarse")
    }
    
    fn pointer_fine(self) -> Self {
        self.class("pointer-fine")
    }
    
    fn any_pointer_coarse(self) -> Self {
        self.class("any-pointer-coarse")
    }
    
    fn any_pointer_fine(self) -> Self {
        self.class("any-pointer-fine")
    }
}

/// Trait for adding motion variant utilities to a class builder
pub trait MotionVariantUtilities {
    /// Add motion reduce variant
    fn motion_reduce(self) -> Self;
    /// Add motion safe variant
    fn motion_safe(self) -> Self;
}

impl MotionVariantUtilities for ClassBuilder {
    fn motion_reduce(self) -> Self {
        self.class("motion-reduce")
    }
    
    fn motion_safe(self) -> Self {
        self.class("motion-safe")
    }
}

/// Trait for adding color scheme variant utilities to a class builder
pub trait ColorSchemeVariantUtilities {
    /// Add light color scheme variant
    fn light(self) -> Self;
    /// Add dark color scheme variant
    fn dark(self) -> Self;
}

impl ColorSchemeVariantUtilities for ClassBuilder {
    fn light(self) -> Self {
        self.class("light")
    }
    
    fn dark(self) -> Self {
        self.class("dark")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_variant_class_names() {
        assert_eq!(PointerVariant::Coarse.to_class_name(), "pointer-coarse");
        assert_eq!(PointerVariant::Fine.to_class_name(), "pointer-fine");
        assert_eq!(PointerVariant::AnyCoarse.to_class_name(), "any-pointer-coarse");
        assert_eq!(PointerVariant::AnyFine.to_class_name(), "any-pointer-fine");
    }

    #[test]
    fn test_pointer_variant_media_queries() {
        assert_eq!(PointerVariant::Coarse.to_media_query(), "@media (pointer: coarse)");
        assert_eq!(PointerVariant::Fine.to_media_query(), "@media (pointer: fine)");
        assert_eq!(PointerVariant::AnyCoarse.to_media_query(), "@media (any-pointer: coarse)");
        assert_eq!(PointerVariant::AnyFine.to_media_query(), "@media (any-pointer: fine)");
    }

    #[test]
    fn test_motion_variant_class_names() {
        assert_eq!(MotionVariant::Reduced.to_class_name(), "motion-reduce");
        assert_eq!(MotionVariant::NoPreference.to_class_name(), "motion-safe");
    }

    #[test]
    fn test_motion_variant_media_queries() {
        assert_eq!(MotionVariant::Reduced.to_media_query(), "@media (prefers-reduced-motion: reduce)");
        assert_eq!(MotionVariant::NoPreference.to_media_query(), "@media (prefers-reduced-motion: no-preference)");
    }

    #[test]
    fn test_color_scheme_variant_class_names() {
        assert_eq!(ColorSchemeVariant::Light.to_class_name(), "light");
        assert_eq!(ColorSchemeVariant::Dark.to_class_name(), "dark");
    }

    #[test]
    fn test_color_scheme_variant_media_queries() {
        assert_eq!(ColorSchemeVariant::Light.to_media_query(), "@media (prefers-color-scheme: light)");
        assert_eq!(ColorSchemeVariant::Dark.to_media_query(), "@media (prefers-color-scheme: dark)");
    }

    #[test]
    fn test_pointer_variant_utilities() {
        let classes = ClassBuilder::new()
            .pointer_coarse()
            .pointer_fine()
            .any_pointer_coarse()
            .any_pointer_fine()
            .build();
        
        assert!(classes.classes.contains("pointer-coarse"));
        assert!(classes.classes.contains("pointer-fine"));
        assert!(classes.classes.contains("any-pointer-coarse"));
        assert!(classes.classes.contains("any-pointer-fine"));
    }

    #[test]
    fn test_motion_variant_utilities() {
        let classes = ClassBuilder::new()
            .motion_reduce()
            .motion_safe()
            .build();
        
        assert!(classes.classes.contains("motion-reduce"));
        assert!(classes.classes.contains("motion-safe"));
    }

    #[test]
    fn test_color_scheme_variant_utilities() {
        let classes = ClassBuilder::new()
            .light()
            .dark()
            .build();
        
        assert!(classes.classes.contains("light"));
        assert!(classes.classes.contains("dark"));
    }

    #[test]
    fn test_device_variants_comprehensive() {
        let classes = ClassBuilder::new()
            .pointer_coarse()
            .pointer_fine()
            .motion_reduce()
            .motion_safe()
            .light()
            .dark()
            .build();
        
        assert!(classes.classes.contains("pointer-coarse"));
        assert!(classes.classes.contains("pointer-fine"));
        assert!(classes.classes.contains("motion-reduce"));
        assert!(classes.classes.contains("motion-safe"));
        assert!(classes.classes.contains("light"));
        assert!(classes.classes.contains("dark"));
    }
}
