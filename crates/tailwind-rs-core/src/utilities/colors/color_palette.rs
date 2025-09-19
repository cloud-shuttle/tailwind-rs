//! Color palette utilities for tailwind-rs

use serde::{Deserialize, Serialize};
use std::fmt;

/// Color palette values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorPalette {
    // Grays
    Gray, Slate, Zinc, Neutral, Stone,
    // Reds
    Red, Rose, Pink,
    // Oranges
    Orange, Amber, Yellow,
    // Greens
    Lime, Green, Emerald, Teal, Cyan,
    // Blues
    Sky, Blue, Indigo, Violet,
    // Purples
    Purple, Fuchsia,
}

impl fmt::Display for ColorPalette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Grays
            ColorPalette::Gray => write!(f, "gray"),
            ColorPalette::Slate => write!(f, "slate"),
            ColorPalette::Zinc => write!(f, "zinc"),
            ColorPalette::Neutral => write!(f, "neutral"),
            ColorPalette::Stone => write!(f, "stone"),
            // Reds
            ColorPalette::Red => write!(f, "red"),
            ColorPalette::Rose => write!(f, "rose"),
            ColorPalette::Pink => write!(f, "pink"),
            // Oranges
            ColorPalette::Orange => write!(f, "orange"),
            ColorPalette::Amber => write!(f, "amber"),
            ColorPalette::Yellow => write!(f, "yellow"),
            // Greens
            ColorPalette::Lime => write!(f, "lime"),
            ColorPalette::Green => write!(f, "green"),
            ColorPalette::Emerald => write!(f, "emerald"),
            ColorPalette::Teal => write!(f, "teal"),
            ColorPalette::Cyan => write!(f, "cyan"),
            // Blues
            ColorPalette::Sky => write!(f, "sky"),
            ColorPalette::Blue => write!(f, "blue"),
            ColorPalette::Indigo => write!(f, "indigo"),
            ColorPalette::Violet => write!(f, "violet"),
            // Purples
            ColorPalette::Purple => write!(f, "purple"),
            ColorPalette::Fuchsia => write!(f, "fuchsia"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_palette_display() {
        assert_eq!(ColorPalette::Gray.to_string(), "gray");
        assert_eq!(ColorPalette::Red.to_string(), "red");
        assert_eq!(ColorPalette::Blue.to_string(), "blue");
        assert_eq!(ColorPalette::Green.to_string(), "green");
    }

    #[test]
    fn test_color_palette_serialization() {
        let palette = ColorPalette::Blue;
        let serialized = serde_json::to_string(&palette).unwrap();
        let deserialized: ColorPalette = serde_json::from_str(&serialized).unwrap();
        assert_eq!(palette, deserialized);
    }
}
