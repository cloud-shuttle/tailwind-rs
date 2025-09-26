//! Color function utilities for tailwind-rs
//!
//! This module provides support for modern CSS color functions including
//! rgb(), hsl(), oklch(), and CSS variables.

use serde::{Deserialize, Serialize};
use std::fmt;

/// CSS color function types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColorFunction {
    /// RGB color function: rgb(255, 0, 0)
    Rgb {
        r: u8,
        g: u8,
        b: u8,
        alpha: Option<f32>,
    },
    /// HSL color function: hsl(0, 100%, 50%)
    Hsl {
        h: u16,
        s: u8,
        l: u8,
        alpha: Option<f32>,
    },
    /// OKLCH color function: oklch(0.7 0.15 0)
    Oklch {
        l: f32,
        c: f32,
        h: f32,
        alpha: Option<f32>,
    },
    /// LAB color function: lab(70% 0 0)
    Lab {
        l: f32,
        a: f32,
        b: f32,
        alpha: Option<f32>,
    },
    /// LCH color function: lch(70% 0 0)
    Lch {
        l: f32,
        c: f32,
        h: f32,
        alpha: Option<f32>,
    },
}

/// CSS variable reference
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CssVariable {
    /// Variable name without -- prefix
    pub name: String,
    /// Fallback value if variable is not defined
    pub fallback: Option<String>,
}

impl ColorFunction {
    /// Create a new RGB color function
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb {
            r,
            g,
            b,
            alpha: None,
        }
    }

    /// Create a new RGB color function with alpha
    pub fn rgba(r: u8, g: u8, b: u8, alpha: f32) -> Self {
        Self::Rgb {
            r,
            g,
            b,
            alpha: Some(alpha),
        }
    }

    /// Create a new HSL color function
    pub fn hsl(h: u16, s: u8, l: u8) -> Self {
        Self::Hsl {
            h,
            s,
            l,
            alpha: None,
        }
    }

    /// Create a new HSL color function with alpha
    pub fn hsla(h: u16, s: u8, l: u8, alpha: f32) -> Self {
        Self::Hsl {
            h,
            s,
            l,
            alpha: Some(alpha),
        }
    }

    /// Create a new OKLCH color function
    pub fn oklch(l: f32, c: f32, h: f32) -> Self {
        Self::Oklch {
            l,
            c,
            h,
            alpha: None,
        }
    }

    /// Create a new OKLCH color function with alpha
    pub fn oklcha(l: f32, c: f32, h: f32, alpha: f32) -> Self {
        Self::Oklch {
            l,
            c,
            h,
            alpha: Some(alpha),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        match self {
            ColorFunction::Rgb { r, g, b, alpha } => {
                if let Some(a) = alpha {
                    format!("rgba({}, {}, {}, {})", r, g, b, a)
                } else {
                    format!("rgb({}, {}, {})", r, g, b)
                }
            }
            ColorFunction::Hsl { h, s, l, alpha } => {
                if let Some(a) = alpha {
                    format!("hsla({}, {}%, {}%, {})", h, s, l, a)
                } else {
                    format!("hsl({}, {}%, {}%)", h, s, l)
                }
            }
            ColorFunction::Oklch { l, c, h, alpha } => {
                if let Some(a) = alpha {
                    format!("oklch({} {} {} / {})", l, c, h, a)
                } else {
                    format!("oklch({} {} {})", l, c, h)
                }
            }
            ColorFunction::Lab { l, a, b, alpha } => {
                if let Some(a) = alpha {
                    format!("lab({}% {} {} / {})", l, a, b, a)
                } else {
                    format!("lab({}% {} {})", l, a, b)
                }
            }
            ColorFunction::Lch { l, c, h, alpha } => {
                if let Some(a) = alpha {
                    format!("lch({}% {} {} / {})", l, c, h, a)
                } else {
                    format!("lch({}% {} {})", l, c, h)
                }
            }
        }
    }

    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        match self {
            ColorFunction::Rgb { r, g, b, alpha } => {
                if let Some(a) = alpha {
                    format!("bg-[rgba({},{},{},{})]", r, g, b, a)
                } else {
                    format!("bg-[rgb({},{},{})]", r, g, b)
                }
            }
            ColorFunction::Hsl { h, s, l, alpha } => {
                if let Some(a) = alpha {
                    format!("bg-[hsla({},{}%,{}%,{})]", h, s, l, a)
                } else {
                    format!("bg-[hsl({},{}%,{}%)]", h, s, l)
                }
            }
            ColorFunction::Oklch { l, c, h, alpha } => {
                if let Some(a) = alpha {
                    format!("bg-[oklch({} {} {} / {})]", l, c, h, a)
                } else {
                    format!("bg-[oklch({} {} {})]", l, c, h)
                }
            }
            ColorFunction::Lab { l, a, b, alpha } => {
                if let Some(a) = alpha {
                    format!("bg-[lab({}% {} {} / {})]", l, a, b, a)
                } else {
                    format!("bg-[lab({}% {} {})]", l, a, b)
                }
            }
            ColorFunction::Lch { l, c, h, alpha } => {
                if let Some(a) = alpha {
                    format!("bg-[lch({}% {} {} / {})]", l, c, h, a)
                } else {
                    format!("bg-[lch({}% {} {})]", l, c, h)
                }
            }
        }
    }
}

impl CssVariable {
    /// Create a new CSS variable reference
    pub fn new(name: String) -> Self {
        Self {
            name,
            fallback: None,
        }
    }

    /// Create a new CSS variable reference with fallback
    pub fn with_fallback(name: String, fallback: String) -> Self {
        Self {
            name,
            fallback: Some(fallback),
        }
    }

    /// Convert to CSS value
    pub fn to_css_value(&self) -> String {
        if let Some(fallback) = &self.fallback {
            format!("var(--{}, {})", self.name, fallback)
        } else {
            format!("var(--{})", self.name)
        }
    }

    /// Convert to class name
    pub fn to_class_name(&self) -> String {
        if let Some(fallback) = &self.fallback {
            format!("bg-[var(--{}, {})]", self.name, fallback)
        } else {
            format!("bg-[var(--{})]", self.name)
        }
    }
}

impl fmt::Display for ColorFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css_value())
    }
}

impl fmt::Display for CssVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_css_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_color_function() {
        let color = ColorFunction::rgb(255, 0, 0);
        assert_eq!(color.to_css_value(), "rgb(255, 0, 0)");
        assert_eq!(color.to_class_name(), "bg-[rgb(255,0,0)]");
    }

    #[test]
    fn test_rgba_color_function() {
        let color = ColorFunction::rgba(255, 0, 0, 0.5);
        assert_eq!(color.to_css_value(), "rgba(255, 0, 0, 0.5)");
        assert_eq!(color.to_class_name(), "bg-[rgba(255,0,0,0.5)]");
    }

    #[test]
    fn test_hsl_color_function() {
        let color = ColorFunction::hsl(0, 100, 50);
        assert_eq!(color.to_css_value(), "hsl(0, 100%, 50%)");
        assert_eq!(color.to_class_name(), "bg-[hsl(0,100%,50%)]");
    }

    #[test]
    fn test_hsla_color_function() {
        let color = ColorFunction::hsla(0, 100, 50, 0.8);
        assert_eq!(color.to_css_value(), "hsla(0, 100%, 50%, 0.8)");
        assert_eq!(color.to_class_name(), "bg-[hsla(0,100%,50%,0.8)]");
    }

    #[test]
    fn test_oklch_color_function() {
        let color = ColorFunction::oklch(0.7, 0.15, 0.0);
        assert_eq!(color.to_css_value(), "oklch(0.7 0.15 0)");
        assert_eq!(color.to_class_name(), "bg-[oklch(0.7 0.15 0)]");
    }

    #[test]
    fn test_oklcha_color_function() {
        let color = ColorFunction::oklcha(0.7, 0.15, 0.0, 0.9);
        assert_eq!(color.to_css_value(), "oklch(0.7 0.15 0 / 0.9)");
        assert_eq!(color.to_class_name(), "bg-[oklch(0.7 0.15 0 / 0.9)]");
    }

    #[test]
    fn test_lab_color_function() {
        let color = ColorFunction::Lab {
            l: 70.0,
            a: 0.0,
            b: 0.0,
            alpha: None,
        };
        assert_eq!(color.to_css_value(), "lab(70% 0 0)");
        assert_eq!(color.to_class_name(), "bg-[lab(70% 0 0)]");
    }

    #[test]
    fn test_lch_color_function() {
        let color = ColorFunction::Lch {
            l: 70.0,
            c: 0.0,
            h: 0.0,
            alpha: None,
        };
        assert_eq!(color.to_css_value(), "lch(70% 0 0)");
        assert_eq!(color.to_class_name(), "bg-[lch(70% 0 0)]");
    }

    #[test]
    fn test_css_variable() {
        let var = CssVariable::new("primary-color".to_string());
        assert_eq!(var.to_css_value(), "var(--primary-color)");
        assert_eq!(var.to_class_name(), "bg-[var(--primary-color)]");
    }

    #[test]
    fn test_css_variable_with_fallback() {
        let var = CssVariable::with_fallback("primary-color".to_string(), "#ff0000".to_string());
        assert_eq!(var.to_css_value(), "var(--primary-color, #ff0000)");
        assert_eq!(var.to_class_name(), "bg-[var(--primary-color, #ff0000)]");
    }

    #[test]
    fn test_color_function_display() {
        let color = ColorFunction::rgb(255, 0, 0);
        assert_eq!(format!("{}", color), "rgb(255, 0, 0)");
    }

    #[test]
    fn test_css_variable_display() {
        let var = CssVariable::new("primary-color".to_string());
        assert_eq!(format!("{}", var), "var(--primary-color)");
    }

    #[test]
    fn test_color_function_serialization() {
        let color = ColorFunction::rgb(255, 0, 0);
        let serialized = serde_json::to_string(&color).unwrap();
        let deserialized: ColorFunction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(color, deserialized);
    }

    #[test]
    fn test_css_variable_serialization() {
        let var = CssVariable::new("primary-color".to_string());
        let serialized = serde_json::to_string(&var).unwrap();
        let deserialized: CssVariable = serde_json::from_str(&serialized).unwrap();
        assert_eq!(var, deserialized);
    }
}
