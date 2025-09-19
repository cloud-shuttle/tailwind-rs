//! Text transformation utilities
//!
//! This module provides text transformation utilities for case changes,
//! text overflow, white space handling, and word breaking.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Text transformation values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextTransform {
    /// No transformation
    None,
    /// Uppercase
    Uppercase,
    /// Lowercase
    Lowercase,
    /// Capitalize (first letter of each word)
    Capitalize,
}

/// Text overflow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextOverflow {
    /// Truncate with ellipsis
    Ellipsis,
    /// Clip text
    Clip,
}

/// White space values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WhiteSpace {
    /// Normal white space handling
    Normal,
    /// No wrap
    Nowrap,
    /// Pre (preserve whitespace)
    Pre,
    /// Pre line
    PreLine,
    /// Pre wrap
    PreWrap,
    /// Break spaces
    BreakSpaces,
}

/// Word break values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WordBreak {
    /// Normal word breaking
    Normal,
    /// Break all characters
    BreakAll,
    /// Keep all words together
    KeepAll,
    /// Break word
    BreakWord,
}

/// Overflow wrap values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OverflowWrap {
    /// Normal wrapping
    Normal,
    /// Break word
    BreakWord,
    /// Anywhere
    Anywhere,
}

impl TextTransform {
    pub fn to_class_name(&self) -> String {
        match self {
            TextTransform::None => "normal-case".to_string(),
            TextTransform::Uppercase => "uppercase".to_string(),
            TextTransform::Lowercase => "lowercase".to_string(),
            TextTransform::Capitalize => "capitalize".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TextTransform::None => "none".to_string(),
            TextTransform::Uppercase => "uppercase".to_string(),
            TextTransform::Lowercase => "lowercase".to_string(),
            TextTransform::Capitalize => "capitalize".to_string(),
        }
    }
}

impl TextOverflow {
    pub fn to_class_name(&self) -> String {
        match self {
            TextOverflow::Ellipsis => "truncate".to_string(),
            TextOverflow::Clip => "text-clip".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TextOverflow::Ellipsis => "ellipsis".to_string(),
            TextOverflow::Clip => "clip".to_string(),
        }
    }
}

impl WhiteSpace {
    pub fn to_class_name(&self) -> String {
        match self {
            WhiteSpace::Normal => "whitespace-normal".to_string(),
            WhiteSpace::Nowrap => "whitespace-nowrap".to_string(),
            WhiteSpace::Pre => "whitespace-pre".to_string(),
            WhiteSpace::PreLine => "whitespace-pre-line".to_string(),
            WhiteSpace::PreWrap => "whitespace-pre-wrap".to_string(),
            WhiteSpace::BreakSpaces => "whitespace-break-spaces".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            WhiteSpace::Normal => "normal".to_string(),
            WhiteSpace::Nowrap => "nowrap".to_string(),
            WhiteSpace::Pre => "pre".to_string(),
            WhiteSpace::PreLine => "pre-line".to_string(),
            WhiteSpace::PreWrap => "pre-wrap".to_string(),
            WhiteSpace::BreakSpaces => "break-spaces".to_string(),
        }
    }
}

impl WordBreak {
    pub fn to_class_name(&self) -> String {
        match self {
            WordBreak::Normal => "break-normal".to_string(),
            WordBreak::BreakAll => "break-all".to_string(),
            WordBreak::KeepAll => "break-keep".to_string(),
            WordBreak::BreakWord => "break-words".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            WordBreak::Normal => "normal".to_string(),
            WordBreak::BreakAll => "break-all".to_string(),
            WordBreak::KeepAll => "keep-all".to_string(),
            WordBreak::BreakWord => "break-word".to_string(),
        }
    }
}

impl OverflowWrap {
    pub fn to_class_name(&self) -> String {
        match self {
            OverflowWrap::Normal => "overflow-wrap-normal".to_string(),
            OverflowWrap::BreakWord => "overflow-wrap-break-word".to_string(),
            OverflowWrap::Anywhere => "overflow-wrap-anywhere".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            OverflowWrap::Normal => "normal".to_string(),
            OverflowWrap::BreakWord => "break-word".to_string(),
            OverflowWrap::Anywhere => "anywhere".to_string(),
        }
    }
}

impl fmt::Display for TextTransform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TextOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for WhiteSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for WordBreak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for OverflowWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for text transformation utilities
pub trait TextTransformUtilities {
    fn text_transform(&mut self, transform: TextTransform) -> &mut Self;
    fn uppercase(&mut self) -> &mut Self;
    fn lowercase(&mut self) -> &mut Self;
    fn capitalize(&mut self) -> &mut Self;
    fn normal_case(&mut self) -> &mut Self;
    
    fn text_overflow(&mut self, overflow: TextOverflow) -> &mut Self;
    fn truncate(&mut self) -> &mut Self;
    fn text_clip(&mut self) -> &mut Self;
    
    fn white_space(&mut self, whitespace: WhiteSpace) -> &mut Self;
    fn whitespace_normal(&mut self) -> &mut Self;
    fn whitespace_nowrap(&mut self) -> &mut Self;
    fn whitespace_pre(&mut self) -> &mut Self;
    fn whitespace_pre_line(&mut self) -> &mut Self;
    fn whitespace_pre_wrap(&mut self) -> &mut Self;
    fn whitespace_break_spaces(&mut self) -> &mut Self;
    
    fn word_break(&mut self, word_break: WordBreak) -> &mut Self;
    fn break_normal(&mut self) -> &mut Self;
    fn break_all(&mut self) -> &mut Self;
    fn break_keep(&mut self) -> &mut Self;
    fn break_words(&mut self) -> &mut Self;
    
    fn overflow_wrap(&mut self, wrap: OverflowWrap) -> &mut Self;
    fn overflow_wrap_normal(&mut self) -> &mut Self;
    fn overflow_wrap_break_word(&mut self) -> &mut Self;
    fn overflow_wrap_anywhere(&mut self) -> &mut Self;
}

impl TextTransformUtilities for ClassBuilder {
    fn text_transform(&mut self, transform: TextTransform) -> &mut Self {
        *self = self.clone().class(&transform.to_class_name());
        self
    }

    fn uppercase(&mut self) -> &mut Self {
        self.text_transform(TextTransform::Uppercase)
    }

    fn lowercase(&mut self) -> &mut Self {
        self.text_transform(TextTransform::Lowercase)
    }

    fn capitalize(&mut self) -> &mut Self {
        self.text_transform(TextTransform::Capitalize)
    }

    fn normal_case(&mut self) -> &mut Self {
        self.text_transform(TextTransform::None)
    }

    fn text_overflow(&mut self, overflow: TextOverflow) -> &mut Self {
        *self = self.clone().class(&overflow.to_class_name());
        self
    }

    fn truncate(&mut self) -> &mut Self {
        self.text_overflow(TextOverflow::Ellipsis)
    }

    fn text_clip(&mut self) -> &mut Self {
        self.text_overflow(TextOverflow::Clip)
    }

    fn white_space(&mut self, whitespace: WhiteSpace) -> &mut Self {
        *self = self.clone().class(&whitespace.to_class_name());
        self
    }

    fn whitespace_normal(&mut self) -> &mut Self {
        self.white_space(WhiteSpace::Normal)
    }

    fn whitespace_nowrap(&mut self) -> &mut Self {
        self.white_space(WhiteSpace::Nowrap)
    }

    fn whitespace_pre(&mut self) -> &mut Self {
        self.white_space(WhiteSpace::Pre)
    }

    fn whitespace_pre_line(&mut self) -> &mut Self {
        self.white_space(WhiteSpace::PreLine)
    }

    fn whitespace_pre_wrap(&mut self) -> &mut Self {
        self.white_space(WhiteSpace::PreWrap)
    }

    fn whitespace_break_spaces(&mut self) -> &mut Self {
        self.white_space(WhiteSpace::BreakSpaces)
    }

    fn word_break(&mut self, word_break: WordBreak) -> &mut Self {
        *self = self.clone().class(&word_break.to_class_name());
        self
    }

    fn break_normal(&mut self) -> &mut Self {
        self.word_break(WordBreak::Normal)
    }

    fn break_all(&mut self) -> &mut Self {
        self.word_break(WordBreak::BreakAll)
    }

    fn break_keep(&mut self) -> &mut Self {
        self.word_break(WordBreak::KeepAll)
    }

    fn break_words(&mut self) -> &mut Self {
        self.word_break(WordBreak::BreakWord)
    }

    fn overflow_wrap(&mut self, wrap: OverflowWrap) -> &mut Self {
        *self = self.clone().class(&wrap.to_class_name());
        self
    }

    fn overflow_wrap_normal(&mut self) -> &mut Self {
        self.overflow_wrap(OverflowWrap::Normal)
    }

    fn overflow_wrap_break_word(&mut self) -> &mut Self {
        self.overflow_wrap(OverflowWrap::BreakWord)
    }

    fn overflow_wrap_anywhere(&mut self) -> &mut Self {
        self.overflow_wrap(OverflowWrap::Anywhere)
    }
}
