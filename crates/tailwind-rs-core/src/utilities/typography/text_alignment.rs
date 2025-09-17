//! Text alignment utilities
//!
//! This module provides text alignment utilities for typography.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Text alignment values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextAlign {
    /// Left align
    Left,
    /// Center align
    Center,
    /// Right align
    Right,
    /// Justify align
    Justify,
    /// Start align (left in LTR, right in RTL)
    Start,
    /// End align (right in LTR, left in RTL)
    End,
}

impl TextAlign {
    pub fn to_class_name(&self) -> String {
        match self {
            TextAlign::Left => "left".to_string(),
            TextAlign::Center => "center".to_string(),
            TextAlign::Right => "right".to_string(),
            TextAlign::Justify => "justify".to_string(),
            TextAlign::Start => "start".to_string(),
            TextAlign::End => "end".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TextAlign::Left => "left".to_string(),
            TextAlign::Center => "center".to_string(),
            TextAlign::Right => "right".to_string(),
            TextAlign::Justify => "justify".to_string(),
            TextAlign::Start => "start".to_string(),
            TextAlign::End => "end".to_string(),
        }
    }
}

impl fmt::Display for TextAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "text-{}", self.to_class_name())
    }
}

/// Trait for text alignment utilities
pub trait TextAlignUtilities {
    fn text_align(&mut self, align: TextAlign) -> &mut Self;
    fn text_left(&mut self) -> &mut Self;
    fn text_center(&mut self) -> &mut Self;
    fn text_right(&mut self) -> &mut Self;
    fn text_justify(&mut self) -> &mut Self;
    fn text_start(&mut self) -> &mut Self;
    fn text_end(&mut self) -> &mut Self;
}

impl TextAlignUtilities for ClassBuilder {
    fn text_align(&mut self, align: TextAlign) -> &mut Self {
        *self = self.clone().class(&format!("text-{}", align.to_class_name()));
        self
    }

    fn text_left(&mut self) -> &mut Self {
        self.text_align(TextAlign::Left)
    }

    fn text_center(&mut self) -> &mut Self {
        self.text_align(TextAlign::Center)
    }

    fn text_right(&mut self) -> &mut Self {
        self.text_align(TextAlign::Right)
    }

    fn text_justify(&mut self) -> &mut Self {
        self.text_align(TextAlign::Justify)
    }

    fn text_start(&mut self) -> &mut Self {
        self.text_align(TextAlign::Start)
    }

    fn text_end(&mut self) -> &mut Self {
        self.text_align(TextAlign::End)
    }
}
