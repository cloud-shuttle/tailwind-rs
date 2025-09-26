//! Typography spacing utilities
//!
//! This module provides line height and letter spacing utilities.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Line height values
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LineHeight {
    /// None line height
    None,
    /// leading-3 (0.75rem / 12px)
    Three,
    /// leading-4 (1rem / 16px)
    Four,
    /// leading-5 (1.25rem / 20px)
    Five,
    /// leading-6 (1.5rem / 24px)
    Six,
    /// leading-7 (1.75rem / 28px)
    Seven,
    /// leading-8 (2rem / 32px)
    Eight,
    /// leading-9 (2.25rem / 36px)
    Nine,
    /// leading-10 (2.5rem / 40px)
    Ten,
    /// leading-tight (1.25)
    Tight,
    /// leading-snug (1.375)
    Snug,
    /// leading-normal (1.5)
    Normal,
    /// leading-relaxed (1.625)
    Relaxed,
    /// leading-loose (2)
    Loose,
}

/// Letter spacing values
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LetterSpacing {
    /// tracking-tighter (-0.05em)
    Tighter,
    /// tracking-tight (-0.025em)
    Tight,
    /// tracking-normal (0em)
    Normal,
    /// tracking-wide (0.025em)
    Wide,
    /// tracking-wider (0.05em)
    Wider,
    /// tracking-widest (0.1em)
    Widest,
}

impl std::hash::Hash for LineHeight {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LineHeight::None => 0u8.hash(state),
            LineHeight::Three => 1u8.hash(state),
            LineHeight::Four => 2u8.hash(state),
            LineHeight::Five => 3u8.hash(state),
            LineHeight::Six => 4u8.hash(state),
            LineHeight::Seven => 5u8.hash(state),
            LineHeight::Eight => 6u8.hash(state),
            LineHeight::Nine => 7u8.hash(state),
            LineHeight::Ten => 8u8.hash(state),
            LineHeight::Tight => 9u8.hash(state),
            LineHeight::Snug => 10u8.hash(state),
            LineHeight::Normal => 11u8.hash(state),
            LineHeight::Relaxed => 12u8.hash(state),
            LineHeight::Loose => 13u8.hash(state),
        }
    }
}

impl std::hash::Hash for LetterSpacing {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LetterSpacing::Tighter => 0u8.hash(state),
            LetterSpacing::Tight => 1u8.hash(state),
            LetterSpacing::Normal => 2u8.hash(state),
            LetterSpacing::Wide => 3u8.hash(state),
            LetterSpacing::Wider => 4u8.hash(state),
            LetterSpacing::Widest => 5u8.hash(state),
        }
    }
}

impl std::cmp::Eq for LineHeight {}
impl std::cmp::Eq for LetterSpacing {}

impl LineHeight {
    pub fn to_class_name(&self) -> String {
        match self {
            LineHeight::None => "none".to_string(),
            LineHeight::Three => "3".to_string(),
            LineHeight::Four => "4".to_string(),
            LineHeight::Five => "5".to_string(),
            LineHeight::Six => "6".to_string(),
            LineHeight::Seven => "7".to_string(),
            LineHeight::Eight => "8".to_string(),
            LineHeight::Nine => "9".to_string(),
            LineHeight::Ten => "10".to_string(),
            LineHeight::Tight => "tight".to_string(),
            LineHeight::Snug => "snug".to_string(),
            LineHeight::Normal => "normal".to_string(),
            LineHeight::Relaxed => "relaxed".to_string(),
            LineHeight::Loose => "loose".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            LineHeight::None => "1".to_string(),
            LineHeight::Three => "0.75rem".to_string(),
            LineHeight::Four => "1rem".to_string(),
            LineHeight::Five => "1.25rem".to_string(),
            LineHeight::Six => "1.5rem".to_string(),
            LineHeight::Seven => "1.75rem".to_string(),
            LineHeight::Eight => "2rem".to_string(),
            LineHeight::Nine => "2.25rem".to_string(),
            LineHeight::Ten => "2.5rem".to_string(),
            LineHeight::Tight => "1.25".to_string(),
            LineHeight::Snug => "1.375".to_string(),
            LineHeight::Normal => "1.5".to_string(),
            LineHeight::Relaxed => "1.625".to_string(),
            LineHeight::Loose => "2".to_string(),
        }
    }
}

impl LetterSpacing {
    pub fn to_class_name(&self) -> String {
        match self {
            LetterSpacing::Tighter => "tighter".to_string(),
            LetterSpacing::Tight => "tight".to_string(),
            LetterSpacing::Normal => "normal".to_string(),
            LetterSpacing::Wide => "wide".to_string(),
            LetterSpacing::Wider => "wider".to_string(),
            LetterSpacing::Widest => "widest".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            LetterSpacing::Tighter => "-0.05em".to_string(),
            LetterSpacing::Tight => "-0.025em".to_string(),
            LetterSpacing::Normal => "0em".to_string(),
            LetterSpacing::Wide => "0.025em".to_string(),
            LetterSpacing::Wider => "0.05em".to_string(),
            LetterSpacing::Widest => "0.1em".to_string(),
        }
    }
}

impl fmt::Display for LineHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "leading-{}", self.to_class_name())
    }
}

impl fmt::Display for LetterSpacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tracking-{}", self.to_class_name())
    }
}

/// Trait for line height utilities
pub trait LineHeightUtilities {
    fn leading(&mut self, height: LineHeight) -> &mut Self;
    fn leading_none(&mut self) -> &mut Self;
    fn leading_tight(&mut self) -> &mut Self;
    fn leading_snug(&mut self) -> &mut Self;
    fn leading_normal(&mut self) -> &mut Self;
    fn leading_relaxed(&mut self) -> &mut Self;
    fn leading_loose(&mut self) -> &mut Self;
    fn leading_3(&mut self) -> &mut Self;
    fn leading_4(&mut self) -> &mut Self;
    fn leading_5(&mut self) -> &mut Self;
    fn leading_6(&mut self) -> &mut Self;
    fn leading_7(&mut self) -> &mut Self;
    fn leading_8(&mut self) -> &mut Self;
    fn leading_9(&mut self) -> &mut Self;
    fn leading_10(&mut self) -> &mut Self;
}

/// Trait for letter spacing utilities
pub trait LetterSpacingUtilities {
    fn tracking(&mut self, spacing: LetterSpacing) -> &mut Self;
    fn tracking_tighter(&mut self) -> &mut Self;
    fn tracking_tight(&mut self) -> &mut Self;
    fn tracking_normal(&mut self) -> &mut Self;
    fn tracking_wide(&mut self) -> &mut Self;
    fn tracking_wider(&mut self) -> &mut Self;
    fn tracking_widest(&mut self) -> &mut Self;
}

impl LineHeightUtilities for ClassBuilder {
    fn leading(&mut self, height: LineHeight) -> &mut Self {
        *self = self
            .clone()
            .class(&format!("leading-{}", height.to_class_name()));
        self
    }

    fn leading_none(&mut self) -> &mut Self {
        self.leading(LineHeight::None)
    }

    fn leading_tight(&mut self) -> &mut Self {
        self.leading(LineHeight::Tight)
    }

    fn leading_snug(&mut self) -> &mut Self {
        self.leading(LineHeight::Snug)
    }

    fn leading_normal(&mut self) -> &mut Self {
        self.leading(LineHeight::Normal)
    }

    fn leading_relaxed(&mut self) -> &mut Self {
        self.leading(LineHeight::Relaxed)
    }

    fn leading_loose(&mut self) -> &mut Self {
        self.leading(LineHeight::Loose)
    }

    fn leading_3(&mut self) -> &mut Self {
        self.leading(LineHeight::Three)
    }

    fn leading_4(&mut self) -> &mut Self {
        self.leading(LineHeight::Four)
    }

    fn leading_5(&mut self) -> &mut Self {
        self.leading(LineHeight::Five)
    }

    fn leading_6(&mut self) -> &mut Self {
        self.leading(LineHeight::Six)
    }

    fn leading_7(&mut self) -> &mut Self {
        self.leading(LineHeight::Seven)
    }

    fn leading_8(&mut self) -> &mut Self {
        self.leading(LineHeight::Eight)
    }

    fn leading_9(&mut self) -> &mut Self {
        self.leading(LineHeight::Nine)
    }

    fn leading_10(&mut self) -> &mut Self {
        self.leading(LineHeight::Ten)
    }
}

impl LetterSpacingUtilities for ClassBuilder {
    fn tracking(&mut self, spacing: LetterSpacing) -> &mut Self {
        *self = self
            .clone()
            .class(&format!("tracking-{}", spacing.to_class_name()));
        self
    }

    fn tracking_tighter(&mut self) -> &mut Self {
        self.tracking(LetterSpacing::Tighter)
    }

    fn tracking_tight(&mut self) -> &mut Self {
        self.tracking(LetterSpacing::Tight)
    }

    fn tracking_normal(&mut self) -> &mut Self {
        self.tracking(LetterSpacing::Normal)
    }

    fn tracking_wide(&mut self) -> &mut Self {
        self.tracking(LetterSpacing::Wide)
    }

    fn tracking_wider(&mut self) -> &mut Self {
        self.tracking(LetterSpacing::Wider)
    }

    fn tracking_widest(&mut self) -> &mut Self {
        self.tracking(LetterSpacing::Widest)
    }
}
