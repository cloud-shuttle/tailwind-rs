//! Text decoration utilities
//!
//! This module provides text decoration utilities for underlines, overlines, and strikethrough.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Text decoration values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextDecoration {
    /// No text decoration
    None,
    /// Underline
    Underline,
    /// Overline
    Overline,
    /// Line through (strikethrough)
    LineThrough,
}

/// Text decoration style values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextDecorationStyle {
    /// Solid line
    Solid,
    /// Double line
    Double,
    /// Dotted line
    Dotted,
    /// Dashed line
    Dashed,
    /// Wavy line
    Wavy,
}

/// Text decoration thickness values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextDecorationThickness {
    /// Auto thickness
    Auto,
    /// From font
    FromFont,
    /// 0px
    Zero,
    /// 1px
    One,
    /// 2px
    Two,
    /// 4px
    Four,
    /// 8px
    Eight,
}

/// Text underline offset values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextUnderlineOffset {
    /// Auto offset
    Auto,
    /// 0px
    Zero,
    /// 1px
    One,
    /// 2px
    Two,
    /// 4px
    Four,
    /// 8px
    Eight,
}

impl TextDecoration {
    pub fn to_class_name(&self) -> String {
        match self {
            TextDecoration::None => "no-underline".to_string(),
            TextDecoration::Underline => "underline".to_string(),
            TextDecoration::Overline => "overline".to_string(),
            TextDecoration::LineThrough => "line-through".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TextDecoration::None => "none".to_string(),
            TextDecoration::Underline => "underline".to_string(),
            TextDecoration::Overline => "overline".to_string(),
            TextDecoration::LineThrough => "line-through".to_string(),
        }
    }
}

impl TextDecorationStyle {
    pub fn to_class_name(&self) -> String {
        match self {
            TextDecorationStyle::Solid => "solid".to_string(),
            TextDecorationStyle::Double => "double".to_string(),
            TextDecorationStyle::Dotted => "dotted".to_string(),
            TextDecorationStyle::Dashed => "dashed".to_string(),
            TextDecorationStyle::Wavy => "wavy".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TextDecorationStyle::Solid => "solid".to_string(),
            TextDecorationStyle::Double => "double".to_string(),
            TextDecorationStyle::Dotted => "dotted".to_string(),
            TextDecorationStyle::Dashed => "dashed".to_string(),
            TextDecorationStyle::Wavy => "wavy".to_string(),
        }
    }
}

impl TextDecorationThickness {
    pub fn to_class_name(&self) -> String {
        match self {
            TextDecorationThickness::Auto => "auto".to_string(),
            TextDecorationThickness::FromFont => "from-font".to_string(),
            TextDecorationThickness::Zero => "0".to_string(),
            TextDecorationThickness::One => "1".to_string(),
            TextDecorationThickness::Two => "2".to_string(),
            TextDecorationThickness::Four => "4".to_string(),
            TextDecorationThickness::Eight => "8".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TextDecorationThickness::Auto => "auto".to_string(),
            TextDecorationThickness::FromFont => "from-font".to_string(),
            TextDecorationThickness::Zero => "0px".to_string(),
            TextDecorationThickness::One => "1px".to_string(),
            TextDecorationThickness::Two => "2px".to_string(),
            TextDecorationThickness::Four => "4px".to_string(),
            TextDecorationThickness::Eight => "8px".to_string(),
        }
    }
}

impl TextUnderlineOffset {
    pub fn to_class_name(&self) -> String {
        match self {
            TextUnderlineOffset::Auto => "auto".to_string(),
            TextUnderlineOffset::Zero => "0".to_string(),
            TextUnderlineOffset::One => "1".to_string(),
            TextUnderlineOffset::Two => "2".to_string(),
            TextUnderlineOffset::Four => "4".to_string(),
            TextUnderlineOffset::Eight => "8".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TextUnderlineOffset::Auto => "auto".to_string(),
            TextUnderlineOffset::Zero => "0px".to_string(),
            TextUnderlineOffset::One => "1px".to_string(),
            TextUnderlineOffset::Two => "2px".to_string(),
            TextUnderlineOffset::Four => "4px".to_string(),
            TextUnderlineOffset::Eight => "8px".to_string(),
        }
    }
}

impl fmt::Display for TextDecoration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TextDecorationStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "decoration-{}", self.to_class_name())
    }
}

impl fmt::Display for TextDecorationThickness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "decoration-{}", self.to_class_name())
    }
}

impl fmt::Display for TextUnderlineOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "underline-offset-{}", self.to_class_name())
    }
}

/// Trait for text decoration utilities
pub trait TextDecorationUtilities {
    fn text_decoration(&mut self, decoration: TextDecoration) -> &mut Self;
    fn underline(&mut self) -> &mut Self;
    fn overline(&mut self) -> &mut Self;
    fn line_through(&mut self) -> &mut Self;
    fn no_underline(&mut self) -> &mut Self;
    
    fn decoration_style(&mut self, style: TextDecorationStyle) -> &mut Self;
    fn decoration_solid(&mut self) -> &mut Self;
    fn decoration_double(&mut self) -> &mut Self;
    fn decoration_dotted(&mut self) -> &mut Self;
    fn decoration_dashed(&mut self) -> &mut Self;
    fn decoration_wavy(&mut self) -> &mut Self;
    
    fn decoration_thickness(&mut self, thickness: TextDecorationThickness) -> &mut Self;
    fn decoration_auto(&mut self) -> &mut Self;
    fn decoration_from_font(&mut self) -> &mut Self;
    fn decoration_0(&mut self) -> &mut Self;
    fn decoration_1(&mut self) -> &mut Self;
    fn decoration_2(&mut self) -> &mut Self;
    fn decoration_4(&mut self) -> &mut Self;
    fn decoration_8(&mut self) -> &mut Self;
    
    fn underline_offset(&mut self, offset: TextUnderlineOffset) -> &mut Self;
    fn underline_offset_auto(&mut self) -> &mut Self;
    fn underline_offset_0(&mut self) -> &mut Self;
    fn underline_offset_1(&mut self) -> &mut Self;
    fn underline_offset_2(&mut self) -> &mut Self;
    fn underline_offset_4(&mut self) -> &mut Self;
    fn underline_offset_8(&mut self) -> &mut Self;
}

impl TextDecorationUtilities for ClassBuilder {
    fn text_decoration(&mut self, decoration: TextDecoration) -> &mut Self {
        *self = self.clone().class(&decoration.to_class_name());
        self
    }

    fn underline(&mut self) -> &mut Self {
        self.text_decoration(TextDecoration::Underline)
    }

    fn overline(&mut self) -> &mut Self {
        self.text_decoration(TextDecoration::Overline)
    }

    fn line_through(&mut self) -> &mut Self {
        self.text_decoration(TextDecoration::LineThrough)
    }

    fn no_underline(&mut self) -> &mut Self {
        self.text_decoration(TextDecoration::None)
    }

    fn decoration_style(&mut self, style: TextDecorationStyle) -> &mut Self {
        *self = self.clone().class(&format!("decoration-{}", style.to_class_name()));
        self
    }

    fn decoration_solid(&mut self) -> &mut Self {
        self.decoration_style(TextDecorationStyle::Solid)
    }

    fn decoration_double(&mut self) -> &mut Self {
        self.decoration_style(TextDecorationStyle::Double)
    }

    fn decoration_dotted(&mut self) -> &mut Self {
        self.decoration_style(TextDecorationStyle::Dotted)
    }

    fn decoration_dashed(&mut self) -> &mut Self {
        self.decoration_style(TextDecorationStyle::Dashed)
    }

    fn decoration_wavy(&mut self) -> &mut Self {
        self.decoration_style(TextDecorationStyle::Wavy)
    }

    fn decoration_thickness(&mut self, thickness: TextDecorationThickness) -> &mut Self {
        *self = self.clone().class(&format!("decoration-{}", thickness.to_class_name()));
        self
    }

    fn decoration_auto(&mut self) -> &mut Self {
        self.decoration_thickness(TextDecorationThickness::Auto)
    }

    fn decoration_from_font(&mut self) -> &mut Self {
        self.decoration_thickness(TextDecorationThickness::FromFont)
    }

    fn decoration_0(&mut self) -> &mut Self {
        self.decoration_thickness(TextDecorationThickness::Zero)
    }

    fn decoration_1(&mut self) -> &mut Self {
        self.decoration_thickness(TextDecorationThickness::One)
    }

    fn decoration_2(&mut self) -> &mut Self {
        self.decoration_thickness(TextDecorationThickness::Two)
    }

    fn decoration_4(&mut self) -> &mut Self {
        self.decoration_thickness(TextDecorationThickness::Four)
    }

    fn decoration_8(&mut self) -> &mut Self {
        self.decoration_thickness(TextDecorationThickness::Eight)
    }

    fn underline_offset(&mut self, offset: TextUnderlineOffset) -> &mut Self {
        *self = self.clone().class(&format!("underline-offset-{}", offset.to_class_name()));
        self
    }

    fn underline_offset_auto(&mut self) -> &mut Self {
        self.underline_offset(TextUnderlineOffset::Auto)
    }

    fn underline_offset_0(&mut self) -> &mut Self {
        self.underline_offset(TextUnderlineOffset::Zero)
    }

    fn underline_offset_1(&mut self) -> &mut Self {
        self.underline_offset(TextUnderlineOffset::One)
    }

    fn underline_offset_2(&mut self) -> &mut Self {
        self.underline_offset(TextUnderlineOffset::Two)
    }

    fn underline_offset_4(&mut self) -> &mut Self {
        self.underline_offset(TextUnderlineOffset::Four)
    }

    fn underline_offset_8(&mut self) -> &mut Self {
        self.underline_offset(TextUnderlineOffset::Eight)
    }
}
