//! Aspect ratio utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Aspect ratio values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AspectRatio {
    /// 1:1 aspect ratio
    Square,
    /// 16:9 aspect ratio
    Video,
    /// 4:3 aspect ratio
    Photo,
    /// 3:2 aspect ratio
    Portrait,
    /// 2:3 aspect ratio
    Landscape,
    /// Auto aspect ratio
    Auto,
}

impl fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AspectRatio::Square => write!(f, "aspect-square"),
            AspectRatio::Video => write!(f, "aspect-video"),
            AspectRatio::Photo => write!(f, "aspect-photo"),
            AspectRatio::Portrait => write!(f, "aspect-portrait"),
            AspectRatio::Landscape => write!(f, "aspect-landscape"),
            AspectRatio::Auto => write!(f, "aspect-auto"),
        }
    }
}

/// Trait for adding aspect ratio utilities to a class builder
pub trait AspectRatioUtilities {
    fn aspect_ratio(self, aspect_ratio: AspectRatio) -> Self;
}

impl AspectRatioUtilities for ClassBuilder {
    fn aspect_ratio(self, aspect_ratio: AspectRatio) -> Self {
        self.class(aspect_ratio.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_ratio_display() {
        assert_eq!(AspectRatio::Square.to_string(), "aspect-square");
        assert_eq!(AspectRatio::Video.to_string(), "aspect-video");
        assert_eq!(AspectRatio::Photo.to_string(), "aspect-photo");
    }

    #[test]
    fn test_aspect_ratio_utilities() {
        let classes = ClassBuilder::new()
            .aspect_ratio(AspectRatio::Square)
            .build();

        assert!(classes.to_css_classes().contains("aspect-square"));
    }
}
