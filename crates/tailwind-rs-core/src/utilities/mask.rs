//! Mask utilities for tailwind-rs
//!
//! This module provides utilities for CSS mask properties.
//! It includes all Tailwind CSS mask variants.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Mask type values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaskType {
    /// No mask
    None,
    /// Alpha mask
    Alpha,
    /// Luminance mask
    Luminance,
}

impl fmt::Display for MaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaskType::None => write!(f, "none"),
            MaskType::Alpha => write!(f, "alpha"),
            MaskType::Luminance => write!(f, "luminance"),
        }
    }
}

/// Mask mode values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaskMode {
    /// Alpha mask mode
    Alpha,
    /// Luminance mask mode
    Luminance,
    /// Match source mask mode
    MatchSource,
}

impl fmt::Display for MaskMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaskMode::Alpha => write!(f, "alpha"),
            MaskMode::Luminance => write!(f, "luminance"),
            MaskMode::MatchSource => write!(f, "match-source"),
        }
    }
}

/// Mask composite values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaskComposite {
    /// Add composite
    Add,
    /// Subtract composite
    Subtract,
    /// Intersect composite
    Intersect,
    /// Exclude composite
    Exclude,
}

impl fmt::Display for MaskComposite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaskComposite::Add => write!(f, "add"),
            MaskComposite::Subtract => write!(f, "subtract"),
            MaskComposite::Intersect => write!(f, "intersect"),
            MaskComposite::Exclude => write!(f, "exclude"),
        }
    }
}

/// Mask repeat values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaskRepeat {
    /// No repeat
    NoRepeat,
    /// Repeat
    Repeat,
    /// Repeat X
    RepeatX,
    /// Repeat Y
    RepeatY,
    /// Round repeat
    Round,
    /// Space repeat
    Space,
}

impl fmt::Display for MaskRepeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaskRepeat::NoRepeat => write!(f, "no-repeat"),
            MaskRepeat::Repeat => write!(f, "repeat"),
            MaskRepeat::RepeatX => write!(f, "repeat-x"),
            MaskRepeat::RepeatY => write!(f, "repeat-y"),
            MaskRepeat::Round => write!(f, "round"),
            MaskRepeat::Space => write!(f, "space"),
        }
    }
}

/// Mask size values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaskSize {
    /// Auto size
    Auto,
    /// Cover size
    Cover,
    /// Contain size
    Contain,
}

impl fmt::Display for MaskSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaskSize::Auto => write!(f, "auto"),
            MaskSize::Cover => write!(f, "cover"),
            MaskSize::Contain => write!(f, "contain"),
        }
    }
}

/// Mask position values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaskPosition {
    /// Center position
    Center,
    /// Top position
    Top,
    /// Bottom position
    Bottom,
    /// Left position
    Left,
    /// Right position
    Right,
    /// Top left position
    TopLeft,
    /// Top right position
    TopRight,
    /// Bottom left position
    BottomLeft,
    /// Bottom right position
    BottomRight,
}

impl fmt::Display for MaskPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaskPosition::Center => write!(f, "center"),
            MaskPosition::Top => write!(f, "top"),
            MaskPosition::Bottom => write!(f, "bottom"),
            MaskPosition::Left => write!(f, "left"),
            MaskPosition::Right => write!(f, "right"),
            MaskPosition::TopLeft => write!(f, "top left"),
            MaskPosition::TopRight => write!(f, "top right"),
            MaskPosition::BottomLeft => write!(f, "bottom left"),
            MaskPosition::BottomRight => write!(f, "bottom right"),
        }
    }
}

/// Mask clip values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaskClip {
    /// Border box clip
    BorderBox,
    /// Padding box clip
    PaddingBox,
    /// Content box clip
    ContentBox,
    /// Text clip
    Text,
}

impl fmt::Display for MaskClip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaskClip::BorderBox => write!(f, "border-box"),
            MaskClip::PaddingBox => write!(f, "padding-box"),
            MaskClip::ContentBox => write!(f, "content-box"),
            MaskClip::Text => write!(f, "text"),
        }
    }
}

/// Mask origin values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaskOrigin {
    /// Border box origin
    BorderBox,
    /// Padding box origin
    PaddingBox,
    /// Content box origin
    ContentBox,
}

impl fmt::Display for MaskOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaskOrigin::BorderBox => write!(f, "border-box"),
            MaskOrigin::PaddingBox => write!(f, "padding-box"),
            MaskOrigin::ContentBox => write!(f, "content-box"),
        }
    }
}

/// Trait for adding mask utilities to ClassBuilder
pub trait MaskUtilities {
    /// Set mask to none
    fn mask_none(self) -> Self;
    /// Set mask to alpha
    fn mask_alpha(self) -> Self;
    /// Set mask to luminance
    fn mask_luminance(self) -> Self;
    /// Set mask repeat to none
    fn mask_repeat_none(self) -> Self;
    /// Set mask repeat to default
    fn mask_repeat(self) -> Self;
    /// Set mask repeat to X
    fn mask_repeat_x(self) -> Self;
    /// Set mask repeat to Y
    fn mask_repeat_y(self) -> Self;
    /// Set mask repeat to round
    fn mask_repeat_round(self) -> Self;
    /// Set mask repeat to space
    fn mask_repeat_space(self) -> Self;
    /// Set mask size to auto
    fn mask_size_auto(self) -> Self;
    /// Set mask size to cover
    fn mask_size_cover(self) -> Self;
    /// Set mask size to contain
    fn mask_size_contain(self) -> Self;
    /// Set mask position to center
    fn mask_center(self) -> Self;
    /// Set mask position to top
    fn mask_top(self) -> Self;
    /// Set mask position to bottom
    fn mask_bottom(self) -> Self;
    /// Set mask position to left
    fn mask_left(self) -> Self;
    /// Set mask position to right
    fn mask_right(self) -> Self;
    /// Set mask position to top left
    fn mask_top_left(self) -> Self;
    /// Set mask position to top right
    fn mask_top_right(self) -> Self;
    /// Set mask position to bottom left
    fn mask_bottom_left(self) -> Self;
    /// Set mask position to bottom right
    fn mask_bottom_right(self) -> Self;
    /// Set mask clip to border
    fn mask_clip_border(self) -> Self;
    /// Set mask clip to padding
    fn mask_clip_padding(self) -> Self;
    /// Set mask clip to content
    fn mask_clip_content(self) -> Self;
    /// Set mask clip to text
    fn mask_clip_text(self) -> Self;
    /// Set mask origin to border
    fn mask_origin_border(self) -> Self;
    /// Set mask origin to padding
    fn mask_origin_padding(self) -> Self;
    /// Set mask origin to content
    fn mask_origin_content(self) -> Self;
}

impl MaskUtilities for ClassBuilder {
    fn mask_none(self) -> Self {
        self.class("mask-none")
    }

    fn mask_alpha(self) -> Self {
        self.class("mask-alpha")
    }

    fn mask_luminance(self) -> Self {
        self.class("mask-luminance")
    }

    fn mask_repeat_none(self) -> Self {
        self.class("mask-repeat-none")
    }

    fn mask_repeat(self) -> Self {
        self.class("mask-repeat")
    }

    fn mask_repeat_x(self) -> Self {
        self.class("mask-repeat-x")
    }

    fn mask_repeat_y(self) -> Self {
        self.class("mask-repeat-y")
    }

    fn mask_repeat_round(self) -> Self {
        self.class("mask-repeat-round")
    }

    fn mask_repeat_space(self) -> Self {
        self.class("mask-repeat-space")
    }

    fn mask_size_auto(self) -> Self {
        self.class("mask-size-auto")
    }

    fn mask_size_cover(self) -> Self {
        self.class("mask-size-cover")
    }

    fn mask_size_contain(self) -> Self {
        self.class("mask-size-contain")
    }

    fn mask_center(self) -> Self {
        self.class("mask-center")
    }

    fn mask_top(self) -> Self {
        self.class("mask-top")
    }

    fn mask_bottom(self) -> Self {
        self.class("mask-bottom")
    }

    fn mask_left(self) -> Self {
        self.class("mask-left")
    }

    fn mask_right(self) -> Self {
        self.class("mask-right")
    }

    fn mask_top_left(self) -> Self {
        self.class("mask-top-left")
    }

    fn mask_top_right(self) -> Self {
        self.class("mask-top-right")
    }

    fn mask_bottom_left(self) -> Self {
        self.class("mask-bottom-left")
    }

    fn mask_bottom_right(self) -> Self {
        self.class("mask-bottom-right")
    }

    fn mask_clip_border(self) -> Self {
        self.class("mask-clip-border")
    }

    fn mask_clip_padding(self) -> Self {
        self.class("mask-clip-padding")
    }

    fn mask_clip_content(self) -> Self {
        self.class("mask-clip-content")
    }

    fn mask_clip_text(self) -> Self {
        self.class("mask-clip-text")
    }

    fn mask_origin_border(self) -> Self {
        self.class("mask-origin-border")
    }

    fn mask_origin_padding(self) -> Self {
        self.class("mask-origin-padding")
    }

    fn mask_origin_content(self) -> Self {
        self.class("mask-origin-content")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ClassBuilder;

    #[test]
    fn test_mask_type_enum_values() {
        assert_eq!(MaskType::None.to_string(), "none");
        assert_eq!(MaskType::Alpha.to_string(), "alpha");
        assert_eq!(MaskType::Luminance.to_string(), "luminance");
    }

    #[test]
    fn test_mask_mode_enum_values() {
        assert_eq!(MaskMode::Alpha.to_string(), "alpha");
        assert_eq!(MaskMode::Luminance.to_string(), "luminance");
        assert_eq!(MaskMode::MatchSource.to_string(), "match-source");
    }

    #[test]
    fn test_mask_composite_enum_values() {
        assert_eq!(MaskComposite::Add.to_string(), "add");
        assert_eq!(MaskComposite::Subtract.to_string(), "subtract");
        assert_eq!(MaskComposite::Intersect.to_string(), "intersect");
        assert_eq!(MaskComposite::Exclude.to_string(), "exclude");
    }

    #[test]
    fn test_mask_repeat_enum_values() {
        assert_eq!(MaskRepeat::NoRepeat.to_string(), "no-repeat");
        assert_eq!(MaskRepeat::Repeat.to_string(), "repeat");
        assert_eq!(MaskRepeat::RepeatX.to_string(), "repeat-x");
        assert_eq!(MaskRepeat::RepeatY.to_string(), "repeat-y");
        assert_eq!(MaskRepeat::Round.to_string(), "round");
        assert_eq!(MaskRepeat::Space.to_string(), "space");
    }

    #[test]
    fn test_mask_size_enum_values() {
        assert_eq!(MaskSize::Auto.to_string(), "auto");
        assert_eq!(MaskSize::Cover.to_string(), "cover");
        assert_eq!(MaskSize::Contain.to_string(), "contain");
    }

    #[test]
    fn test_mask_position_enum_values() {
        assert_eq!(MaskPosition::Center.to_string(), "center");
        assert_eq!(MaskPosition::Top.to_string(), "top");
        assert_eq!(MaskPosition::Bottom.to_string(), "bottom");
        assert_eq!(MaskPosition::Left.to_string(), "left");
        assert_eq!(MaskPosition::Right.to_string(), "right");
        assert_eq!(MaskPosition::TopLeft.to_string(), "top left");
        assert_eq!(MaskPosition::TopRight.to_string(), "top right");
        assert_eq!(MaskPosition::BottomLeft.to_string(), "bottom left");
        assert_eq!(MaskPosition::BottomRight.to_string(), "bottom right");
    }

    #[test]
    fn test_mask_clip_enum_values() {
        assert_eq!(MaskClip::BorderBox.to_string(), "border-box");
        assert_eq!(MaskClip::PaddingBox.to_string(), "padding-box");
        assert_eq!(MaskClip::ContentBox.to_string(), "content-box");
        assert_eq!(MaskClip::Text.to_string(), "text");
    }

    #[test]
    fn test_mask_origin_enum_values() {
        assert_eq!(MaskOrigin::BorderBox.to_string(), "border-box");
        assert_eq!(MaskOrigin::PaddingBox.to_string(), "padding-box");
        assert_eq!(MaskOrigin::ContentBox.to_string(), "content-box");
    }

    #[test]
    fn test_mask_utilities() {
        let classes = ClassBuilder::new()
            .mask_alpha()
            .mask_repeat_round()
            .mask_size_cover()
            .mask_center()
            .mask_clip_border()
            .mask_origin_padding();

        let result = classes.build();
        assert!(result.contains("mask-alpha"));
        assert!(result.contains("mask-repeat-round"));
        assert!(result.contains("mask-size-cover"));
        assert!(result.contains("mask-center"));
        assert!(result.contains("mask-clip-border"));
        assert!(result.contains("mask-origin-padding"));
    }

    #[test]
    fn test_mask_serialization() {
        let mask_type = MaskType::Alpha;
        let serialized = serde_json::to_string(&mask_type).unwrap();
        let deserialized: MaskType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(mask_type, deserialized);
    }

    #[test]
    fn test_mask_comprehensive_usage() {
        let classes = ClassBuilder::new()
            .mask_luminance()
            .mask_repeat_round()
            .mask_size_contain()
            .mask_top_left();

        let result = classes.build();
        assert!(result.contains("mask-luminance"));
        assert!(result.contains("mask-repeat-round"));
        assert!(result.contains("mask-size-contain"));
        assert!(result.contains("mask-top-left"));
    }
}
