//! Drop shadow utilities for tailwind-rs
//!
//! This module provides utilities for drop shadow effects.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Drop shadow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DropShadow {
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
    /// 3x large shadow
    Xl3,
    // Colored drop shadows
    /// Red drop shadow
    Red,
    /// Blue drop shadow
    Blue,
    /// Green drop shadow
    Green,
    /// Yellow drop shadow
    Yellow,
    /// Purple drop shadow
    Purple,
    /// Pink drop shadow
    Pink,
    /// Orange drop shadow
    Orange,
    /// Indigo drop shadow
    Indigo,
    /// Cyan drop shadow
    Cyan,
    /// Teal drop shadow
    Teal,
    /// Lime drop shadow
    Lime,
    /// Emerald drop shadow
    Emerald,
    /// Rose drop shadow
    Rose,
    /// Violet drop shadow
    Violet,
    /// Fuchsia drop shadow
    Fuchsia,
    /// Sky drop shadow
    Sky,
    /// Amber drop shadow
    Amber,
    /// Stone drop shadow
    Stone,
    /// Neutral drop shadow
    Neutral,
    /// Zinc drop shadow
    Zinc,
    /// Gray drop shadow
    Gray,
    /// Slate drop shadow
    Slate,
}

impl fmt::Display for DropShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DropShadow::None => write!(f, "drop-shadow-none"),
            DropShadow::Sm => write!(f, "drop-shadow-sm"),
            DropShadow::Default => write!(f, "drop-shadow"),
            DropShadow::Md => write!(f, "drop-shadow-md"),
            DropShadow::Lg => write!(f, "drop-shadow-lg"),
            DropShadow::Xl => write!(f, "drop-shadow-xl"),
            DropShadow::Xl2 => write!(f, "drop-shadow-2xl"),
            DropShadow::Xl3 => write!(f, "drop-shadow-3xl"),
            // Colored drop shadows
            DropShadow::Red => write!(f, "drop-shadow-red"),
            DropShadow::Blue => write!(f, "drop-shadow-blue"),
            DropShadow::Green => write!(f, "drop-shadow-green"),
            DropShadow::Yellow => write!(f, "drop-shadow-yellow"),
            DropShadow::Purple => write!(f, "drop-shadow-purple"),
            DropShadow::Pink => write!(f, "drop-shadow-pink"),
            DropShadow::Orange => write!(f, "drop-shadow-orange"),
            DropShadow::Indigo => write!(f, "drop-shadow-indigo"),
            DropShadow::Cyan => write!(f, "drop-shadow-cyan"),
            DropShadow::Teal => write!(f, "drop-shadow-teal"),
            DropShadow::Lime => write!(f, "drop-shadow-lime"),
            DropShadow::Emerald => write!(f, "drop-shadow-emerald"),
            DropShadow::Rose => write!(f, "drop-shadow-rose"),
            DropShadow::Violet => write!(f, "drop-shadow-violet"),
            DropShadow::Fuchsia => write!(f, "drop-shadow-fuchsia"),
            DropShadow::Sky => write!(f, "drop-shadow-sky"),
            DropShadow::Amber => write!(f, "drop-shadow-amber"),
            DropShadow::Stone => write!(f, "drop-shadow-stone"),
            DropShadow::Neutral => write!(f, "drop-shadow-neutral"),
            DropShadow::Zinc => write!(f, "drop-shadow-zinc"),
            DropShadow::Gray => write!(f, "drop-shadow-gray"),
            DropShadow::Slate => write!(f, "drop-shadow-slate"),
        }
    }
}

/// Trait for adding drop shadow utilities to a class builder
pub trait DropShadowUtilities {
    fn drop_shadow(self, shadow: DropShadow) -> Self;
    
    // Convenience methods for colored drop shadows
    fn drop_shadow_red(self) -> Self;
    fn drop_shadow_blue(self) -> Self;
    fn drop_shadow_green(self) -> Self;
    fn drop_shadow_yellow(self) -> Self;
    fn drop_shadow_purple(self) -> Self;
    fn drop_shadow_pink(self) -> Self;
    fn drop_shadow_orange(self) -> Self;
    fn drop_shadow_indigo(self) -> Self;
    fn drop_shadow_cyan(self) -> Self;
    fn drop_shadow_teal(self) -> Self;
    fn drop_shadow_lime(self) -> Self;
    fn drop_shadow_emerald(self) -> Self;
    fn drop_shadow_rose(self) -> Self;
    fn drop_shadow_violet(self) -> Self;
    fn drop_shadow_fuchsia(self) -> Self;
    fn drop_shadow_sky(self) -> Self;
    fn drop_shadow_amber(self) -> Self;
    fn drop_shadow_stone(self) -> Self;
    fn drop_shadow_neutral(self) -> Self;
    fn drop_shadow_zinc(self) -> Self;
    fn drop_shadow_gray(self) -> Self;
    fn drop_shadow_slate(self) -> Self;
}

impl DropShadowUtilities for ClassBuilder {
    fn drop_shadow(self, shadow: DropShadow) -> Self {
        self.class(shadow.to_string())
    }
    
    fn drop_shadow_red(self) -> Self {
        self.drop_shadow(DropShadow::Red)
    }
    
    fn drop_shadow_blue(self) -> Self {
        self.drop_shadow(DropShadow::Blue)
    }
    
    fn drop_shadow_green(self) -> Self {
        self.drop_shadow(DropShadow::Green)
    }
    
    fn drop_shadow_yellow(self) -> Self {
        self.drop_shadow(DropShadow::Yellow)
    }
    
    fn drop_shadow_purple(self) -> Self {
        self.drop_shadow(DropShadow::Purple)
    }
    
    fn drop_shadow_pink(self) -> Self {
        self.drop_shadow(DropShadow::Pink)
    }
    
    fn drop_shadow_orange(self) -> Self {
        self.drop_shadow(DropShadow::Orange)
    }
    
    fn drop_shadow_indigo(self) -> Self {
        self.drop_shadow(DropShadow::Indigo)
    }
    
    fn drop_shadow_cyan(self) -> Self {
        self.drop_shadow(DropShadow::Cyan)
    }
    
    fn drop_shadow_teal(self) -> Self {
        self.drop_shadow(DropShadow::Teal)
    }
    
    fn drop_shadow_lime(self) -> Self {
        self.drop_shadow(DropShadow::Lime)
    }
    
    fn drop_shadow_emerald(self) -> Self {
        self.drop_shadow(DropShadow::Emerald)
    }
    
    fn drop_shadow_rose(self) -> Self {
        self.drop_shadow(DropShadow::Rose)
    }
    
    fn drop_shadow_violet(self) -> Self {
        self.drop_shadow(DropShadow::Violet)
    }
    
    fn drop_shadow_fuchsia(self) -> Self {
        self.drop_shadow(DropShadow::Fuchsia)
    }
    
    fn drop_shadow_sky(self) -> Self {
        self.drop_shadow(DropShadow::Sky)
    }
    
    fn drop_shadow_amber(self) -> Self {
        self.drop_shadow(DropShadow::Amber)
    }
    
    fn drop_shadow_stone(self) -> Self {
        self.drop_shadow(DropShadow::Stone)
    }
    
    fn drop_shadow_neutral(self) -> Self {
        self.drop_shadow(DropShadow::Neutral)
    }
    
    fn drop_shadow_zinc(self) -> Self {
        self.drop_shadow(DropShadow::Zinc)
    }
    
    fn drop_shadow_gray(self) -> Self {
        self.drop_shadow(DropShadow::Gray)
    }
    
    fn drop_shadow_slate(self) -> Self {
        self.drop_shadow(DropShadow::Slate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_shadow_display() {
        assert_eq!(DropShadow::None.to_string(), "drop-shadow-none");
        assert_eq!(DropShadow::Sm.to_string(), "drop-shadow-sm");
        assert_eq!(DropShadow::Default.to_string(), "drop-shadow");
        assert_eq!(DropShadow::Md.to_string(), "drop-shadow-md");
        assert_eq!(DropShadow::Lg.to_string(), "drop-shadow-lg");
        assert_eq!(DropShadow::Xl.to_string(), "drop-shadow-xl");
        assert_eq!(DropShadow::Xl2.to_string(), "drop-shadow-2xl");
        assert_eq!(DropShadow::Xl3.to_string(), "drop-shadow-3xl");
        
        // Test colored drop shadows
        assert_eq!(DropShadow::Red.to_string(), "drop-shadow-red");
        assert_eq!(DropShadow::Blue.to_string(), "drop-shadow-blue");
        assert_eq!(DropShadow::Green.to_string(), "drop-shadow-green");
        assert_eq!(DropShadow::Yellow.to_string(), "drop-shadow-yellow");
        assert_eq!(DropShadow::Purple.to_string(), "drop-shadow-purple");
        assert_eq!(DropShadow::Pink.to_string(), "drop-shadow-pink");
        assert_eq!(DropShadow::Orange.to_string(), "drop-shadow-orange");
        assert_eq!(DropShadow::Indigo.to_string(), "drop-shadow-indigo");
        assert_eq!(DropShadow::Cyan.to_string(), "drop-shadow-cyan");
        assert_eq!(DropShadow::Teal.to_string(), "drop-shadow-teal");
        assert_eq!(DropShadow::Lime.to_string(), "drop-shadow-lime");
        assert_eq!(DropShadow::Emerald.to_string(), "drop-shadow-emerald");
        assert_eq!(DropShadow::Rose.to_string(), "drop-shadow-rose");
        assert_eq!(DropShadow::Violet.to_string(), "drop-shadow-violet");
        assert_eq!(DropShadow::Fuchsia.to_string(), "drop-shadow-fuchsia");
        assert_eq!(DropShadow::Sky.to_string(), "drop-shadow-sky");
        assert_eq!(DropShadow::Amber.to_string(), "drop-shadow-amber");
        assert_eq!(DropShadow::Stone.to_string(), "drop-shadow-stone");
        assert_eq!(DropShadow::Neutral.to_string(), "drop-shadow-neutral");
        assert_eq!(DropShadow::Zinc.to_string(), "drop-shadow-zinc");
        assert_eq!(DropShadow::Gray.to_string(), "drop-shadow-gray");
        assert_eq!(DropShadow::Slate.to_string(), "drop-shadow-slate");
    }

    #[test]
    fn test_drop_shadow_utilities() {
        let classes = ClassBuilder::new()
            .drop_shadow(DropShadow::Lg)
            .build();
        
        assert!(classes.to_css_classes().contains("drop-shadow-lg"));
    }

    #[test]
    fn test_drop_shadow_convenience_methods() {
        let classes = ClassBuilder::new()
            .drop_shadow_red()
            .drop_shadow_blue()
            .drop_shadow_green()
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("drop-shadow-red"));
        assert!(css_classes.contains("drop-shadow-blue"));
        assert!(css_classes.contains("drop-shadow-green"));
    }

    #[test]
    fn test_drop_shadow_serialization() {
        let drop_shadow = DropShadow::Md;
        let serialized = serde_json::to_string(&drop_shadow).unwrap();
        let deserialized: DropShadow = serde_json::from_str(&serialized).unwrap();
        assert_eq!(drop_shadow, deserialized);
    }

    #[test]
    fn test_drop_shadow_equality_and_hash() {
        let drop_shadow1 = DropShadow::Lg;
        let drop_shadow2 = DropShadow::Lg;
        let drop_shadow3 = DropShadow::Md;
        
        assert_eq!(drop_shadow1, drop_shadow2);
        assert_ne!(drop_shadow1, drop_shadow3);
        
        // Test that equal effects have the same hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        drop_shadow1.hash(&mut hasher1);
        drop_shadow2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}
