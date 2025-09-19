//! Box shadow utilities for tailwind-rs
//!
//! This module provides utilities for box shadow effects.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Box shadow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoxShadow {
    /// No shadow
    None,
    /// Small shadow
    Sm,
    /// Default shadow
    Default,
    /// Medium shadow
    Md,
    /// Large shadow
    Lg,
    /// Extra large shadow
    Xl,
    /// 2x large shadow
    Xl2,
    /// Inner shadow
    Inner,
}

impl fmt::Display for BoxShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoxShadow::None => write!(f, "shadow-none"),
            BoxShadow::Sm => write!(f, "shadow-sm"),
            BoxShadow::Default => write!(f, "shadow"),
            BoxShadow::Md => write!(f, "shadow-md"),
            BoxShadow::Lg => write!(f, "shadow-lg"),
            BoxShadow::Xl => write!(f, "shadow-xl"),
            BoxShadow::Xl2 => write!(f, "shadow-2xl"),
            BoxShadow::Inner => write!(f, "shadow-inner"),
        }
    }
}

/// Trait for adding box shadow utilities to a class builder
pub trait BoxShadowUtilities {
    fn box_shadow(self, shadow: BoxShadow) -> Self;
}

impl BoxShadowUtilities for ClassBuilder {
    fn box_shadow(self, shadow: BoxShadow) -> Self {
        self.class(shadow.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_shadow_display() {
        assert_eq!(BoxShadow::None.to_string(), "shadow-none");
        assert_eq!(BoxShadow::Sm.to_string(), "shadow-sm");
        assert_eq!(BoxShadow::Default.to_string(), "shadow");
        assert_eq!(BoxShadow::Md.to_string(), "shadow-md");
        assert_eq!(BoxShadow::Lg.to_string(), "shadow-lg");
        assert_eq!(BoxShadow::Xl.to_string(), "shadow-xl");
        assert_eq!(BoxShadow::Xl2.to_string(), "shadow-2xl");
        assert_eq!(BoxShadow::Inner.to_string(), "shadow-inner");
    }

    #[test]
    fn test_box_shadow_utilities() {
        let classes = ClassBuilder::new()
            .box_shadow(BoxShadow::Lg)
            .build();
        
        assert!(classes.to_css_classes().contains("shadow-lg"));
    }

    #[test]
    fn test_box_shadow_serialization() {
        let box_shadow = BoxShadow::Md;
        let serialized = serde_json::to_string(&box_shadow).unwrap();
        let deserialized: BoxShadow = serde_json::from_str(&serialized).unwrap();
        assert_eq!(box_shadow, deserialized);
    }

    #[test]
    fn test_box_shadow_equality_and_hash() {
        let box_shadow1 = BoxShadow::Lg;
        let box_shadow2 = BoxShadow::Lg;
        let box_shadow3 = BoxShadow::Md;
        
        assert_eq!(box_shadow1, box_shadow2);
        assert_ne!(box_shadow1, box_shadow3);
        
        // Test that equal effects have the same hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        box_shadow1.hash(&mut hasher1);
        box_shadow2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}
