//! Opacity utilities for tailwind-rs
//!
//! This module provides utilities for opacity effects.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Opacity values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Opacity {
    /// 0% opacity
    Zero,
    /// 5% opacity
    Five,
    /// 10% opacity
    Ten,
    /// 20% opacity
    Twenty,
    /// 25% opacity
    TwentyFive,
    /// 30% opacity
    Thirty,
    /// 40% opacity
    Forty,
    /// 50% opacity
    Fifty,
    /// 60% opacity
    Sixty,
    /// 70% opacity
    Seventy,
    /// 75% opacity
    SeventyFive,
    /// 80% opacity
    Eighty,
    /// 90% opacity
    Ninety,
    /// 95% opacity
    NinetyFive,
    /// 100% opacity
    Hundred,
}

impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opacity::Zero => write!(f, "opacity-0"),
            Opacity::Five => write!(f, "opacity-5"),
            Opacity::Ten => write!(f, "opacity-10"),
            Opacity::Twenty => write!(f, "opacity-20"),
            Opacity::TwentyFive => write!(f, "opacity-25"),
            Opacity::Thirty => write!(f, "opacity-30"),
            Opacity::Forty => write!(f, "opacity-40"),
            Opacity::Fifty => write!(f, "opacity-50"),
            Opacity::Sixty => write!(f, "opacity-60"),
            Opacity::Seventy => write!(f, "opacity-70"),
            Opacity::SeventyFive => write!(f, "opacity-75"),
            Opacity::Eighty => write!(f, "opacity-80"),
            Opacity::Ninety => write!(f, "opacity-90"),
            Opacity::NinetyFive => write!(f, "opacity-95"),
            Opacity::Hundred => write!(f, "opacity-100"),
        }
    }
}

/// Trait for adding opacity utilities to a class builder
pub trait OpacityUtilities {
    fn opacity(self, opacity: Opacity) -> Self;
}

impl OpacityUtilities for ClassBuilder {
    fn opacity(self, opacity: Opacity) -> Self {
        self.class(opacity.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opacity_display() {
        assert_eq!(Opacity::Zero.to_string(), "opacity-0");
        assert_eq!(Opacity::Five.to_string(), "opacity-5");
        assert_eq!(Opacity::Ten.to_string(), "opacity-10");
        assert_eq!(Opacity::Twenty.to_string(), "opacity-20");
        assert_eq!(Opacity::TwentyFive.to_string(), "opacity-25");
        assert_eq!(Opacity::Thirty.to_string(), "opacity-30");
        assert_eq!(Opacity::Forty.to_string(), "opacity-40");
        assert_eq!(Opacity::Fifty.to_string(), "opacity-50");
        assert_eq!(Opacity::Sixty.to_string(), "opacity-60");
        assert_eq!(Opacity::Seventy.to_string(), "opacity-70");
        assert_eq!(Opacity::SeventyFive.to_string(), "opacity-75");
        assert_eq!(Opacity::Eighty.to_string(), "opacity-80");
        assert_eq!(Opacity::Ninety.to_string(), "opacity-90");
        assert_eq!(Opacity::NinetyFive.to_string(), "opacity-95");
        assert_eq!(Opacity::Hundred.to_string(), "opacity-100");
    }

    #[test]
    fn test_opacity_utilities() {
        let classes = ClassBuilder::new().opacity(Opacity::Fifty).build();

        assert!(classes.to_css_classes().contains("opacity-50"));
    }

    #[test]
    fn test_opacity_serialization() {
        let opacity = Opacity::SeventyFive;
        let serialized = serde_json::to_string(&opacity).unwrap();
        let deserialized: Opacity = serde_json::from_str(&serialized).unwrap();
        assert_eq!(opacity, deserialized);
    }

    #[test]
    fn test_opacity_equality_and_hash() {
        let opacity1 = Opacity::Fifty;
        let opacity2 = Opacity::Fifty;
        let opacity3 = Opacity::SeventyFive;

        assert_eq!(opacity1, opacity2);
        assert_ne!(opacity1, opacity3);

        // Test that equal effects have the same hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        opacity1.hash(&mut hasher1);
        opacity2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}
