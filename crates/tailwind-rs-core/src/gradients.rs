//! Gradient support for tailwind-rs
//!
//! This module provides comprehensive support for Tailwind CSS gradients,
//! including background gradients, gradient directions, and gradient stops.
//! Examples: bg-gradient-to-r, from-blue-500, via-purple-500, to-pink-500, etc.

use crate::classes::ClassBuilder;
use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents gradient directions in Tailwind CSS
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
    /// Convert to Tailwind CSS class name
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
}

impl fmt::Display for GradientDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Represents gradient stops in Tailwind CSS
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GradientStop {
    /// From stop (starting color)
    From(Color),
    /// Via stop (middle color)
    Via(Color),
    /// To stop (ending color)
    To(Color),
}

impl GradientStop {
    /// Create a from stop
    pub fn from(color: Color) -> Self {
        Self::From(color)
    }

    /// Create a via stop
    pub fn via(color: Color) -> Self {
        Self::Via(color)
    }

    /// Create a to stop
    pub fn to(color: Color) -> Self {
        Self::To(color)
    }

    /// Convert to Tailwind CSS class name
    pub fn to_class_name(&self) -> String {
        match self {
            GradientStop::From(color) => format!("from-{}", color.to_class_name()),
            GradientStop::Via(color) => format!("via-{}", color.to_class_name()),
            GradientStop::To(color) => format!("to-{}", color.to_class_name()),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            GradientStop::From(color) => color.to_css_value(),
            GradientStop::Via(color) => color.to_css_value(),
            GradientStop::To(color) => color.to_css_value(),
        }
    }
}

impl fmt::Display for GradientStop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Represents a complete gradient configuration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Gradient {
    /// Gradient direction
    pub direction: GradientDirection,
    /// Gradient stops
    pub stops: Vec<GradientStop>,
}

impl Gradient {
    /// Create a new gradient
    pub fn new(direction: GradientDirection) -> Self {
        Self {
            direction,
            stops: Vec::new(),
        }
    }

    /// Add a from stop
    pub fn from(mut self, color: Color) -> Self {
        self.stops.push(GradientStop::from(color));
        self
    }

    /// Add a via stop
    pub fn via(mut self, color: Color) -> Self {
        self.stops.push(GradientStop::via(color));
        self
    }

    /// Add a to stop
    pub fn to(mut self, color: Color) -> Self {
        self.stops.push(GradientStop::to(color));
        self
    }

    /// Convert to Tailwind CSS class names
    pub fn to_class_names(&self) -> Vec<String> {
        let mut classes = vec![format!("bg-gradient-{}", self.direction.to_class_name())];
        
        for stop in &self.stops {
            classes.push(stop.to_class_name());
        }
        
        classes
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        let mut css = format!("linear-gradient({}, ", self.direction.to_css_value());
        
        let stop_values: Vec<String> = self.stops.iter()
            .map(|stop| stop.to_css_value())
            .collect();
        
        css.push_str(&stop_values.join(", "));
        css.push(')');
        css
    }

    /// Validate the gradient
    pub fn validate(&self) -> Result<(), GradientError> {
        if self.stops.is_empty() {
            return Err(GradientError::NoStops);
        }

        // Check for at least one from and one to stop
        let has_from = self.stops.iter().any(|stop| matches!(stop, GradientStop::From(_)));
        let has_to = self.stops.iter().any(|stop| matches!(stop, GradientStop::To(_)));

        if !has_from {
            return Err(GradientError::MissingFromStop);
        }

        if !has_to {
            return Err(GradientError::MissingToStop);
        }

        Ok(())
    }
}

/// Errors that can occur when working with gradients
#[derive(Debug, thiserror::Error)]
pub enum GradientError {
    #[error("No gradient stops defined")]
    NoStops,
    
    #[error("Missing 'from' stop")]
    MissingFromStop,
    
    #[error("Missing 'to' stop")]
    MissingToStop,
}

impl fmt::Display for Gradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_names().join(" "))
    }
}

/// Trait for adding gradient utilities to a class builder
pub trait GradientUtilities {
    /// Add a gradient direction
    fn gradient_direction(self, direction: GradientDirection) -> Self;
    
    /// Add a gradient from stop
    fn gradient_from(self, color: Color) -> Self;
    
    /// Add a gradient via stop
    fn gradient_via(self, color: Color) -> Self;
    
    /// Add a gradient to stop
    fn gradient_to(self, color: Color) -> Self;
    
    /// Add a complete gradient
    fn gradient(self, gradient: Gradient) -> Self;
    
    /// Add a simple two-color gradient
    fn gradient_simple(self, direction: GradientDirection, from: Color, to: Color) -> Self;
    
    /// Add a three-color gradient
    fn gradient_three(self, direction: GradientDirection, from: Color, via: Color, to: Color) -> Self;
    
    /// Add a gradient from color names
    fn gradient_from_names(self, direction: GradientDirection, from: &str, to: &str) -> Self;
    
    /// Add a gradient from color names with via
    fn gradient_from_names_with_via(self, direction: GradientDirection, from: &str, via: &str, to: &str) -> Self;
}

impl GradientUtilities for ClassBuilder {
    fn gradient_direction(self, direction: GradientDirection) -> Self {
        self.class(format!("bg-gradient-{}", direction.to_class_name()))
    }
    
    fn gradient_from(self, color: Color) -> Self {
        self.class(format!("from-{}", color.to_class_name()))
    }
    
    fn gradient_via(self, color: Color) -> Self {
        self.class(format!("via-{}", color.to_class_name()))
    }
    
    fn gradient_to(self, color: Color) -> Self {
        self.class(format!("to-{}", color.to_class_name()))
    }
    
    fn gradient(self, gradient: Gradient) -> Self {
        let mut builder = self;
        let class_names = gradient.to_class_names();
        
        for class_name in class_names {
            builder = builder.class(class_name);
        }
        
        builder
    }
    
    fn gradient_simple(self, direction: GradientDirection, from: Color, to: Color) -> Self {
        let gradient = Gradient::new(direction)
            .from(from)
            .to(to);
        
        self.gradient(gradient)
    }
    
    fn gradient_three(self, direction: GradientDirection, from: Color, via: Color, to: Color) -> Self {
        let gradient = Gradient::new(direction)
            .from(from)
            .via(via)
            .to(to);
        
        self.gradient(gradient)
    }
    
    fn gradient_from_names(self, direction: GradientDirection, from: &str, to: &str) -> Self {
        // Parse color names (simplified - in real implementation, you'd want more robust parsing)
        let from_color = parse_color_name(from);
        let to_color = parse_color_name(to);
        
        self.gradient_simple(direction, from_color, to_color)
    }
    
    fn gradient_from_names_with_via(self, direction: GradientDirection, from: &str, via: &str, to: &str) -> Self {
        // Parse color names (simplified - in real implementation, you'd want more robust parsing)
        let from_color = parse_color_name(from);
        let via_color = parse_color_name(via);
        let to_color = parse_color_name(to);
        
        self.gradient_three(direction, from_color, via_color, to_color)
    }
}

/// Parse a color name string into a Color struct
/// This is a simplified implementation - in production, you'd want more robust parsing
fn parse_color_name(color_name: &str) -> Color {
    // Handle common color patterns like "blue-500", "red-600", etc.
    let parts: Vec<&str> = color_name.split('-').collect();
    
    if parts.len() == 2 {
        let palette = match parts[0] {
            "slate" => ColorPalette::Slate,
            "gray" => ColorPalette::Gray,
            "zinc" => ColorPalette::Zinc,
            "neutral" => ColorPalette::Neutral,
            "stone" => ColorPalette::Stone,
            "red" => ColorPalette::Red,
            "orange" => ColorPalette::Orange,
            "amber" => ColorPalette::Amber,
            "yellow" => ColorPalette::Yellow,
            "lime" => ColorPalette::Lime,
            "green" => ColorPalette::Green,
            "emerald" => ColorPalette::Emerald,
            "teal" => ColorPalette::Teal,
            "cyan" => ColorPalette::Cyan,
            "sky" => ColorPalette::Sky,
            "blue" => ColorPalette::Blue,
            "indigo" => ColorPalette::Indigo,
            "violet" => ColorPalette::Violet,
            "purple" => ColorPalette::Purple,
            "fuchsia" => ColorPalette::Fuchsia,
            "pink" => ColorPalette::Pink,
            "rose" => ColorPalette::Rose,
            _ => ColorPalette::Blue, // Default fallback
        };
        
        let shade = match parts[1] {
            "50" => ColorShade::Shade50,
            "100" => ColorShade::Shade100,
            "200" => ColorShade::Shade200,
            "300" => ColorShade::Shade300,
            "400" => ColorShade::Shade400,
            "500" => ColorShade::Shade500,
            "600" => ColorShade::Shade600,
            "700" => ColorShade::Shade700,
            "800" => ColorShade::Shade800,
            "900" => ColorShade::Shade900,
            "950" => ColorShade::Shade950,
            _ => ColorShade::Shade500, // Default fallback
        };
        
        Color::new(palette, shade)
    } else {
        // Default to blue-500 if parsing fails
        Color::new(ColorPalette::Blue, ColorShade::Shade500)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gradient_direction() {
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
    fn test_gradient_direction_css() {
        assert_eq!(GradientDirection::ToRight.to_css_value(), "to right");
        assert_eq!(GradientDirection::ToLeft.to_css_value(), "to left");
        assert_eq!(GradientDirection::ToTop.to_css_value(), "to top");
        assert_eq!(GradientDirection::ToBottom.to_css_value(), "to bottom");
    }
    
    #[test]
    fn test_gradient_stop() {
        let from_stop = GradientStop::from(Color::new(ColorPalette::Blue, ColorShade::Shade500));
        assert_eq!(from_stop.to_class_name(), "from-blue-500");
        
        let via_stop = GradientStop::via(Color::new(ColorPalette::Purple, ColorShade::Shade500));
        assert_eq!(via_stop.to_class_name(), "via-purple-500");
        
        let to_stop = GradientStop::to(Color::new(ColorPalette::Pink, ColorShade::Shade500));
        assert_eq!(to_stop.to_class_name(), "to-pink-500");
    }
    
    #[test]
    fn test_gradient_creation() {
        let gradient = Gradient::new(GradientDirection::ToRight)
            .from(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .to(Color::new(ColorPalette::Red, ColorShade::Shade500));
        
        let class_names = gradient.to_class_names();
        assert!(class_names.contains(&"bg-gradient-to-r".to_string()));
        assert!(class_names.contains(&"from-blue-500".to_string()));
        assert!(class_names.contains(&"to-red-500".to_string()));
    }
    
    #[test]
    fn test_gradient_three_colors() {
        let gradient = Gradient::new(GradientDirection::ToRight)
            .from(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .via(Color::new(ColorPalette::Purple, ColorShade::Shade500))
            .to(Color::new(ColorPalette::Pink, ColorShade::Shade500));
        
        let class_names = gradient.to_class_names();
        assert!(class_names.contains(&"bg-gradient-to-r".to_string()));
        assert!(class_names.contains(&"from-blue-500".to_string()));
        assert!(class_names.contains(&"via-purple-500".to_string()));
        assert!(class_names.contains(&"to-pink-500".to_string()));
    }
    
    #[test]
    fn test_gradient_validation() {
        // Valid gradient
        let valid_gradient = Gradient::new(GradientDirection::ToRight)
            .from(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .to(Color::new(ColorPalette::Red, ColorShade::Shade500));
        assert!(valid_gradient.validate().is_ok());
        
        // Invalid gradient - no stops
        let invalid_gradient = Gradient::new(GradientDirection::ToRight);
        assert!(invalid_gradient.validate().is_err());
        
        // Invalid gradient - missing from
        let invalid_gradient2 = Gradient::new(GradientDirection::ToRight)
            .to(Color::new(ColorPalette::Red, ColorShade::Shade500));
        assert!(invalid_gradient2.validate().is_err());
        
        // Invalid gradient - missing to
        let invalid_gradient3 = Gradient::new(GradientDirection::ToRight)
            .from(Color::new(ColorPalette::Blue, ColorShade::Shade500));
        assert!(invalid_gradient3.validate().is_err());
    }
    
    #[test]
    fn test_gradient_utilities() {
        let classes = ClassBuilder::new()
            .gradient_direction(GradientDirection::ToRight)
            .gradient_from(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .gradient_to(Color::new(ColorPalette::Red, ColorShade::Shade500))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-gradient-to-r"));
        assert!(css_classes.contains("from-blue-500"));
        assert!(css_classes.contains("to-red-500"));
    }
    
    #[test]
    fn test_gradient_simple() {
        let classes = ClassBuilder::new()
            .gradient_simple(
                GradientDirection::ToRight,
                Color::new(ColorPalette::Blue, ColorShade::Shade500),
                Color::new(ColorPalette::Red, ColorShade::Shade500)
            )
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-gradient-to-r"));
        assert!(css_classes.contains("from-blue-500"));
        assert!(css_classes.contains("to-red-500"));
    }
    
    #[test]
    fn test_gradient_three_colors_utility() {
        let classes = ClassBuilder::new()
            .gradient_three(
                GradientDirection::ToRight,
                Color::new(ColorPalette::Blue, ColorShade::Shade500),
                Color::new(ColorPalette::Purple, ColorShade::Shade500),
                Color::new(ColorPalette::Pink, ColorShade::Shade500)
            )
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-gradient-to-r"));
        assert!(css_classes.contains("from-blue-500"));
        assert!(css_classes.contains("via-purple-500"));
        assert!(css_classes.contains("to-pink-500"));
    }
    
    #[test]
    fn test_gradient_from_names() {
        let classes = ClassBuilder::new()
            .gradient_from_names(GradientDirection::ToRight, "blue-500", "red-500")
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-gradient-to-r"));
        assert!(css_classes.contains("from-blue-500"));
        assert!(css_classes.contains("to-red-500"));
    }
    
    #[test]
    fn test_gradient_from_names_with_via() {
        let classes = ClassBuilder::new()
            .gradient_from_names_with_via(GradientDirection::ToRight, "blue-500", "purple-500", "pink-500")
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-gradient-to-r"));
        assert!(css_classes.contains("from-blue-500"));
        assert!(css_classes.contains("via-purple-500"));
        assert!(css_classes.contains("to-pink-500"));
    }
    
    #[test]
    fn test_gradient_display() {
        let gradient = Gradient::new(GradientDirection::ToRight)
            .from(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .to(Color::new(ColorPalette::Red, ColorShade::Shade500));
        
        let display = format!("{}", gradient);
        assert!(display.contains("bg-gradient-to-r"));
        assert!(display.contains("from-blue-500"));
        assert!(display.contains("to-red-500"));
    }
    
    // #[test]
    // fn test_gradient_css_value() {
    //     let gradient = Gradient::new(GradientDirection::ToRight)
    //         .from(Color::Blue)
    //         .to(Color::Red);
    //     
    //     let css_value = gradient.to_css_value();
    //     assert!(css_value.contains("linear-gradient(to right"));
    //     assert!(css_value.contains("blue"));
    //     assert!(css_value.contains("red"));
    // }
}
