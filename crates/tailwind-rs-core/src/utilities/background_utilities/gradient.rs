//! Background Gradient Utilities Module
//!
//! Handles gradient direction and stop enums and utilities:
//! - GradientDirection enum
//! - GradientStop enum
//! - to_class_name() and to_css_value() methods for both

use serde::{Deserialize, Serialize};

/// Gradient direction values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GradientDirection {
    /// To right
    ToRight,
    /// To left
    ToLeft,
    /// To top
    ToTop,
    /// To bottom
    ToBottom,
    /// To top right
    ToTopRight,
    /// To top left
    ToTopLeft,
    /// To bottom right
    ToBottomRight,
    /// To bottom left
    ToBottomLeft,
}

impl GradientDirection {
    /// Convert to Tailwind class name
    pub fn to_class_name(&self) -> String {
        match self {
            GradientDirection::ToRight => "to-r".to_string(),
            GradientDirection::ToLeft => "to-l".to_string(),
            GradientDirection::ToTop => "to-t".to_string(),
            GradientDirection::ToBottom => "to-b".to_string(),
            GradientDirection::ToTopRight => "to-tr".to_string(),
            GradientDirection::ToTopLeft => "to-tl".to_string(),
            GradientDirection::ToBottomRight => "to-br".to_string(),
            GradientDirection::ToBottomLeft => "to-bl".to_string(),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            GradientDirection::ToRight => "to right".to_string(),
            GradientDirection::ToLeft => "to left".to_string(),
            GradientDirection::ToTop => "to top".to_string(),
            GradientDirection::ToBottom => "to bottom".to_string(),
            GradientDirection::ToTopRight => "to top right".to_string(),
            GradientDirection::ToTopLeft => "to top left".to_string(),
            GradientDirection::ToBottomRight => "to bottom right".to_string(),
            GradientDirection::ToBottomLeft => "to bottom left".to_string(),
        }
    }

    /// Get all available direction variants
    pub fn variants() -> &'static [GradientDirection] {
        &[
            GradientDirection::ToRight,
            GradientDirection::ToLeft,
            GradientDirection::ToTop,
            GradientDirection::ToBottom,
            GradientDirection::ToTopRight,
            GradientDirection::ToTopLeft,
            GradientDirection::ToBottomRight,
            GradientDirection::ToBottomLeft,
        ]
    }
}

impl Default for GradientDirection {
    fn default() -> Self {
        GradientDirection::ToRight
    }
}

/// Gradient stop values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GradientStop {
    /// From stop
    From,
    /// Via stop
    Via,
    /// To stop
    To,
}

impl GradientStop {
    /// Convert to Tailwind class name
    pub fn to_class_name(&self) -> String {
        match self {
            GradientStop::From => "from".to_string(),
            GradientStop::Via => "via".to_string(),
            GradientStop::To => "to".to_string(),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            GradientStop::From => "from".to_string(),
            GradientStop::Via => "via".to_string(),
            GradientStop::To => "to".to_string(),
        }
    }

    /// Get all available stop variants
    pub fn variants() -> &'static [GradientStop] {
        &[
            GradientStop::From,
            GradientStop::Via,
            GradientStop::To,
        ]
    }
}

impl Default for GradientStop {
    fn default() -> Self {
        GradientStop::From
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient_direction_class_names() {
        assert_eq!(GradientDirection::ToRight.to_class_name(), "to-r");
        assert_eq!(GradientDirection::ToLeft.to_class_name(), "to-l");
        assert_eq!(GradientDirection::ToTop.to_class_name(), "to-t");
        assert_eq!(GradientDirection::ToBottom.to_class_name(), "to-b");
        assert_eq!(GradientDirection::ToTopRight.to_class_name(), "to-tr");
        assert_eq!(GradientDirection::ToTopLeft.to_class_name(), "to-tl");
        assert_eq!(GradientDirection::ToBottomRight.to_class_name(), "to-br");
        assert_eq!(GradientDirection::ToBottomLeft.to_class_name(), "to-bl");
    }

    #[test]
    fn gradient_direction_css_values() {
        assert_eq!(GradientDirection::ToRight.to_css_value(), "to right");
        assert_eq!(GradientDirection::ToLeft.to_css_value(), "to left");
        assert_eq!(GradientDirection::ToTop.to_css_value(), "to top");
        assert_eq!(GradientDirection::ToBottom.to_css_value(), "to bottom");
        assert_eq!(GradientDirection::ToTopRight.to_css_value(), "to top right");
        assert_eq!(GradientDirection::ToTopLeft.to_css_value(), "to top left");
        assert_eq!(GradientDirection::ToBottomRight.to_css_value(), "to bottom right");
        assert_eq!(GradientDirection::ToBottomLeft.to_css_value(), "to bottom left");
    }

    #[test]
    fn gradient_direction_variants() {
        let variants = GradientDirection::variants();
        assert_eq!(variants.len(), 8);
        assert!(variants.contains(&GradientDirection::ToRight));
        assert!(variants.contains(&GradientDirection::ToBottom));
        assert!(variants.contains(&GradientDirection::ToTopLeft));
    }

    #[test]
    fn gradient_direction_default() {
        assert_eq!(GradientDirection::default(), GradientDirection::ToRight);
    }

    #[test]
    fn gradient_stop_class_names() {
        assert_eq!(GradientStop::From.to_class_name(), "from");
        assert_eq!(GradientStop::Via.to_class_name(), "via");
        assert_eq!(GradientStop::To.to_class_name(), "to");
    }

    #[test]
    fn gradient_stop_css_values() {
        assert_eq!(GradientStop::From.to_css_value(), "from");
        assert_eq!(GradientStop::Via.to_css_value(), "via");
        assert_eq!(GradientStop::To.to_css_value(), "to");
    }

    #[test]
    fn gradient_stop_variants() {
        let variants = GradientStop::variants();
        assert_eq!(variants.len(), 3);
        assert!(variants.contains(&GradientStop::From));
        assert!(variants.contains(&GradientStop::Via));
        assert!(variants.contains(&GradientStop::To));
    }

    #[test]
    fn gradient_stop_default() {
        assert_eq!(GradientStop::default(), GradientStop::From);
    }
}
