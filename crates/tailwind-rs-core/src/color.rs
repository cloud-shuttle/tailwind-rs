//! Color system for tailwind-rs
//!
//! This module provides the Color enum and related functionality for generating
//! Tailwind CSS color classes with intensity-based methods.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents Tailwind color palette
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    Slate,
    Gray,
    Zinc,
    Neutral,
    Stone,
    Red,
    Orange,
    Amber,
    Yellow,
    Lime,
    Green,
    Emerald,
    Teal,
    Cyan,
    Sky,
    Blue,
    Indigo,
    Violet,
    Purple,
    Fuchsia,
    Pink,
    Rose,
}

impl Color {
    /// Get the color name as a string
    pub fn name(&self) -> &'static str {
        match self {
            Color::Slate => "slate",
            Color::Gray => "gray",
            Color::Zinc => "zinc",
            Color::Neutral => "neutral",
            Color::Stone => "stone",
            Color::Red => "red",
            Color::Orange => "orange",
            Color::Amber => "amber",
            Color::Yellow => "yellow",
            Color::Lime => "lime",
            Color::Green => "green",
            Color::Emerald => "emerald",
            Color::Teal => "teal",
            Color::Cyan => "cyan",
            Color::Sky => "sky",
            Color::Blue => "blue",
            Color::Indigo => "indigo",
            Color::Violet => "violet",
            Color::Purple => "purple",
            Color::Fuchsia => "fuchsia",
            Color::Pink => "pink",
            Color::Rose => "rose",
        }
    }

    /// Generates background color class
    pub fn background(&self, intensity: u16) -> String {
        format!("bg-{}-{}", self.name(), intensity)
    }

    /// Generates text color class
    pub fn text(&self, intensity: u16) -> String {
        format!("text-{}-{}", self.name(), intensity)
    }

    /// Generates border color class
    pub fn border(&self, intensity: u16) -> String {
        format!("border-{}-{}", self.name(), intensity)
    }

    /// Generates ring color class
    pub fn ring(&self, intensity: u16) -> String {
        format!("ring-{}-{}", self.name(), intensity)
    }

    /// Generates hover color class
    pub fn hover(&self, intensity: u16) -> String {
        format!("hover:bg-{}-{}", self.name(), intensity)
    }

    /// Generates focus color class
    pub fn focus(&self, intensity: u16) -> String {
        format!("focus:ring-{}-{}", self.name(), intensity)
    }

    /// Generates divide color class
    pub fn divide(&self, intensity: u16) -> String {
        format!("divide-{}-{}", self.name(), intensity)
    }

    /// Generates placeholder color class
    pub fn placeholder(&self, intensity: u16) -> String {
        format!("placeholder-{}-{}", self.name(), intensity)
    }

    /// Generates accent color class
    pub fn accent(&self, intensity: u16) -> String {
        format!("accent-{}-{}", self.name(), intensity)
    }

    /// Generates caret color class
    pub fn caret(&self, intensity: u16) -> String {
        format!("caret-{}-{}", self.name(), intensity)
    }

    /// Generates outline color class
    pub fn outline(&self, intensity: u16) -> String {
        format!("outline-{}-{}", self.name(), intensity)
    }

    /// Generates decoration color class
    pub fn decoration(&self, intensity: u16) -> String {
        format!("decoration-{}-{}", self.name(), intensity)
    }

    /// Generates shadow color class
    pub fn shadow(&self, intensity: u16) -> String {
        format!("shadow-{}-{}", self.name(), intensity)
    }

    /// Generates from color class (for gradients)
    pub fn from(&self, intensity: u16) -> String {
        format!("from-{}-{}", self.name(), intensity)
    }

    /// Generates via color class (for gradients)
    pub fn via(&self, intensity: u16) -> String {
        format!("via-{}-{}", self.name(), intensity)
    }

    /// Generates to color class (for gradients)
    pub fn to(&self, intensity: u16) -> String {
        format!("to-{}-{}", self.name(), intensity)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Parse a color from a string
impl std::str::FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "slate" => Ok(Color::Slate),
            "gray" => Ok(Color::Gray),
            "zinc" => Ok(Color::Zinc),
            "neutral" => Ok(Color::Neutral),
            "stone" => Ok(Color::Stone),
            "red" => Ok(Color::Red),
            "orange" => Ok(Color::Orange),
            "amber" => Ok(Color::Amber),
            "yellow" => Ok(Color::Yellow),
            "lime" => Ok(Color::Lime),
            "green" => Ok(Color::Green),
            "emerald" => Ok(Color::Emerald),
            "teal" => Ok(Color::Teal),
            "cyan" => Ok(Color::Cyan),
            "sky" => Ok(Color::Sky),
            "blue" => Ok(Color::Blue),
            "indigo" => Ok(Color::Indigo),
            "violet" => Ok(Color::Violet),
            "purple" => Ok(Color::Purple),
            "fuchsia" => Ok(Color::Fuchsia),
            "pink" => Ok(Color::Pink),
            "rose" => Ok(Color::Rose),
            _ => Err(format!("Invalid color: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_name() {
        assert_eq!(Color::Blue.name(), "blue");
        assert_eq!(Color::Red.name(), "red");
        assert_eq!(Color::Green.name(), "green");
    }

    #[test]
    fn test_color_background() {
        let color = Color::Blue;
        assert_eq!(color.background(600), "bg-blue-600");
        assert_eq!(color.background(700), "bg-blue-700");
    }

    #[test]
    fn test_color_text() {
        let color = Color::Blue;
        assert_eq!(color.text(600), "text-blue-600");
        assert_eq!(color.text(700), "text-blue-700");
    }

    #[test]
    fn test_color_border() {
        let color = Color::Blue;
        assert_eq!(color.border(600), "border-blue-600");
        assert_eq!(color.border(700), "border-blue-700");
    }

    #[test]
    fn test_color_ring() {
        let color = Color::Blue;
        assert_eq!(color.ring(600), "ring-blue-600");
        assert_eq!(color.ring(700), "ring-blue-700");
    }

    #[test]
    fn test_color_hover() {
        let color = Color::Blue;
        assert_eq!(color.hover(700), "hover:bg-blue-700");
        assert_eq!(color.hover(800), "hover:bg-blue-800");
    }

    #[test]
    fn test_color_focus() {
        let color = Color::Blue;
        assert_eq!(color.focus(500), "focus:ring-blue-500");
        assert_eq!(color.focus(600), "focus:ring-blue-600");
    }

    #[test]
    fn test_color_divide() {
        let color = Color::Blue;
        assert_eq!(color.divide(600), "divide-blue-600");
        assert_eq!(color.divide(700), "divide-blue-700");
    }

    #[test]
    fn test_color_placeholder() {
        let color = Color::Blue;
        assert_eq!(color.placeholder(600), "placeholder-blue-600");
        assert_eq!(color.placeholder(700), "placeholder-blue-700");
    }

    #[test]
    fn test_color_accent() {
        let color = Color::Blue;
        assert_eq!(color.accent(600), "accent-blue-600");
        assert_eq!(color.accent(700), "accent-blue-700");
    }

    #[test]
    fn test_color_caret() {
        let color = Color::Blue;
        assert_eq!(color.caret(600), "caret-blue-600");
        assert_eq!(color.caret(700), "caret-blue-700");
    }

    #[test]
    fn test_color_outline() {
        let color = Color::Blue;
        assert_eq!(color.outline(600), "outline-blue-600");
        assert_eq!(color.outline(700), "outline-blue-700");
    }

    #[test]
    fn test_color_decoration() {
        let color = Color::Blue;
        assert_eq!(color.decoration(600), "decoration-blue-600");
        assert_eq!(color.decoration(700), "decoration-blue-700");
    }

    #[test]
    fn test_color_shadow() {
        let color = Color::Blue;
        assert_eq!(color.shadow(600), "shadow-blue-600");
        assert_eq!(color.shadow(700), "shadow-blue-700");
    }

    #[test]
    fn test_color_gradient() {
        let color = Color::Blue;
        assert_eq!(color.from(600), "from-blue-600");
        assert_eq!(color.via(600), "via-blue-600");
        assert_eq!(color.to(600), "to-blue-600");
    }

    #[test]
    fn test_color_display() {
        assert_eq!(format!("{}", Color::Blue), "blue");
        assert_eq!(format!("{}", Color::Red), "red");
    }

    #[test]
    fn test_color_from_str() {
        assert_eq!("blue".parse::<Color>().unwrap(), Color::Blue);
        assert_eq!("red".parse::<Color>().unwrap(), Color::Red);
        assert_eq!("BLUE".parse::<Color>().unwrap(), Color::Blue); // Case insensitive
        assert!("invalid".parse::<Color>().is_err());
    }

    #[test]
    fn test_all_colors() {
        let colors = vec![
            Color::Slate,
            Color::Gray,
            Color::Zinc,
            Color::Neutral,
            Color::Stone,
            Color::Red,
            Color::Orange,
            Color::Amber,
            Color::Yellow,
            Color::Lime,
            Color::Green,
            Color::Emerald,
            Color::Teal,
            Color::Cyan,
            Color::Sky,
            Color::Blue,
            Color::Indigo,
            Color::Violet,
            Color::Purple,
            Color::Fuchsia,
            Color::Pink,
            Color::Rose,
        ];

        for color in colors {
            // Test that all colors generate valid class names
            assert!(!color.background(600).is_empty());
            assert!(!color.text(600).is_empty());
            assert!(!color.border(600).is_empty());
            assert!(!color.ring(600).is_empty());
        }
    }
}
