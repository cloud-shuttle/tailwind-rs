//! Color shade utilities for tailwind-rs

use serde::{Deserialize, Serialize};
use std::fmt;

/// Color shade values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorShade {
    Shade50,
    Shade100,
    Shade200,
    Shade300,
    Shade400,
    Shade500,
    Shade600,
    Shade700,
    Shade800,
    Shade900,
    Shade950,
}

impl fmt::Display for ColorShade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorShade::Shade50 => write!(f, "50"),
            ColorShade::Shade100 => write!(f, "100"),
            ColorShade::Shade200 => write!(f, "200"),
            ColorShade::Shade300 => write!(f, "300"),
            ColorShade::Shade400 => write!(f, "400"),
            ColorShade::Shade500 => write!(f, "500"),
            ColorShade::Shade600 => write!(f, "600"),
            ColorShade::Shade700 => write!(f, "700"),
            ColorShade::Shade800 => write!(f, "800"),
            ColorShade::Shade900 => write!(f, "900"),
            ColorShade::Shade950 => write!(f, "950"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_shade_display() {
        assert_eq!(ColorShade::Shade50.to_string(), "50");
        assert_eq!(ColorShade::Shade500.to_string(), "500");
        assert_eq!(ColorShade::Shade950.to_string(), "950");
    }

    #[test]
    fn test_color_shade_serialization() {
        let shade = ColorShade::Shade500;
        let serialized = serde_json::to_string(&shade).unwrap();
        let deserialized: ColorShade = serde_json::from_str(&serialized).unwrap();
        assert_eq!(shade, deserialized);
    }
}
