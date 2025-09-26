//! Background utilities for tailwind-rs
//!
//! This module provides utilities for background colors, background images,
//! background gradients, background positioning, background sizing, and background repeat.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Background attachment values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundAttachment {
    /// Fixed attachment
    Fixed,
    /// Local attachment
    Local,
    /// Scroll attachment
    Scroll,
}

/// Background clip values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundClip {
    /// Border clip
    Border,
    /// Padding clip
    Padding,
    /// Content clip
    Content,
    /// Text clip
    Text,
}

/// Background origin values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundOrigin {
    /// Border origin
    Border,
    /// Padding origin
    Padding,
    /// Content origin
    Content,
}

/// Background position values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundPosition {
    /// Bottom position
    Bottom,
    /// Center position
    Center,
    /// Left position
    Left,
    /// Left bottom position
    LeftBottom,
    /// Left top position
    LeftTop,
    /// Right position
    Right,
    /// Right bottom position
    RightBottom,
    /// Right top position
    RightTop,
    /// Top position
    Top,
}

/// Background repeat values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundRepeat {
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

/// Background size values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundSize {
    /// Auto size
    Auto,
    /// Cover size
    Cover,
    /// Contain size
    Contain,
}

/// Background image values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackgroundImage {
    /// None image
    None,
    /// Linear gradient
    LinearGradient,
    /// Radial gradient
    RadialGradient,
    /// Conic gradient
    ConicGradient,
}

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

impl BackgroundAttachment {
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundAttachment::Fixed => "fixed".to_string(),
            BackgroundAttachment::Local => "local".to_string(),
            BackgroundAttachment::Scroll => "scroll".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundAttachment::Fixed => "fixed".to_string(),
            BackgroundAttachment::Local => "local".to_string(),
            BackgroundAttachment::Scroll => "scroll".to_string(),
        }
    }
}

impl BackgroundClip {
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundClip::Border => "border".to_string(),
            BackgroundClip::Padding => "padding".to_string(),
            BackgroundClip::Content => "content".to_string(),
            BackgroundClip::Text => "text".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundClip::Border => "border-box".to_string(),
            BackgroundClip::Padding => "padding-box".to_string(),
            BackgroundClip::Content => "content-box".to_string(),
            BackgroundClip::Text => "text".to_string(),
        }
    }
}

impl BackgroundOrigin {
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundOrigin::Border => "border".to_string(),
            BackgroundOrigin::Padding => "padding".to_string(),
            BackgroundOrigin::Content => "content".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundOrigin::Border => "border-box".to_string(),
            BackgroundOrigin::Padding => "padding-box".to_string(),
            BackgroundOrigin::Content => "content-box".to_string(),
        }
    }
}

impl BackgroundPosition {
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundPosition::Bottom => "bottom".to_string(),
            BackgroundPosition::Center => "center".to_string(),
            BackgroundPosition::Left => "left".to_string(),
            BackgroundPosition::LeftBottom => "left-bottom".to_string(),
            BackgroundPosition::LeftTop => "left-top".to_string(),
            BackgroundPosition::Right => "right".to_string(),
            BackgroundPosition::RightBottom => "right-bottom".to_string(),
            BackgroundPosition::RightTop => "right-top".to_string(),
            BackgroundPosition::Top => "top".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundPosition::Bottom => "bottom".to_string(),
            BackgroundPosition::Center => "center".to_string(),
            BackgroundPosition::Left => "left".to_string(),
            BackgroundPosition::LeftBottom => "left bottom".to_string(),
            BackgroundPosition::LeftTop => "left top".to_string(),
            BackgroundPosition::Right => "right".to_string(),
            BackgroundPosition::RightBottom => "right bottom".to_string(),
            BackgroundPosition::RightTop => "right top".to_string(),
            BackgroundPosition::Top => "top".to_string(),
        }
    }
}

impl BackgroundRepeat {
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundRepeat::NoRepeat => "no-repeat".to_string(),
            BackgroundRepeat::Repeat => "repeat".to_string(),
            BackgroundRepeat::RepeatX => "repeat-x".to_string(),
            BackgroundRepeat::RepeatY => "repeat-y".to_string(),
            BackgroundRepeat::Round => "round".to_string(),
            BackgroundRepeat::Space => "space".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundRepeat::NoRepeat => "no-repeat".to_string(),
            BackgroundRepeat::Repeat => "repeat".to_string(),
            BackgroundRepeat::RepeatX => "repeat-x".to_string(),
            BackgroundRepeat::RepeatY => "repeat-y".to_string(),
            BackgroundRepeat::Round => "round".to_string(),
            BackgroundRepeat::Space => "space".to_string(),
        }
    }
}

impl BackgroundSize {
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundSize::Auto => "auto".to_string(),
            BackgroundSize::Cover => "cover".to_string(),
            BackgroundSize::Contain => "contain".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundSize::Auto => "auto".to_string(),
            BackgroundSize::Cover => "cover".to_string(),
            BackgroundSize::Contain => "contain".to_string(),
        }
    }
}

impl BackgroundImage {
    pub fn to_class_name(&self) -> String {
        match self {
            BackgroundImage::None => "none".to_string(),
            BackgroundImage::LinearGradient => "gradient-to-r".to_string(),
            BackgroundImage::RadialGradient => "radial-gradient".to_string(),
            BackgroundImage::ConicGradient => "conic-gradient".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            BackgroundImage::None => "none".to_string(),
            BackgroundImage::LinearGradient => {
                "linear-gradient(to right, var(--tw-gradient-stops))".to_string()
            }
            BackgroundImage::RadialGradient => {
                "radial-gradient(ellipse at center, var(--tw-gradient-stops))".to_string()
            }
            BackgroundImage::ConicGradient => {
                "conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))".to_string()
            }
        }
    }
}

impl GradientDirection {
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

impl GradientStop {
    pub fn to_class_name(&self) -> String {
        match self {
            GradientStop::From => "from".to_string(),
            GradientStop::Via => "via".to_string(),
            GradientStop::To => "to".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            GradientStop::From => "from".to_string(),
            GradientStop::Via => "via".to_string(),
            GradientStop::To => "to".to_string(),
        }
    }
}

impl fmt::Display for BackgroundAttachment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundClip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundRepeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BackgroundImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GradientDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GradientStop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding background attachment utilities to a class builder
pub trait BackgroundAttachmentUtilities {
    fn background_attachment(self, attachment: BackgroundAttachment) -> Self;
}

impl BackgroundAttachmentUtilities for ClassBuilder {
    fn background_attachment(self, attachment: BackgroundAttachment) -> Self {
        self.class(format!("bg-{}", attachment.to_class_name()))
    }
}

/// Trait for adding background clip utilities to a class builder
pub trait BackgroundClipUtilities {
    fn background_clip(self, clip: BackgroundClip) -> Self;
}

impl BackgroundClipUtilities for ClassBuilder {
    fn background_clip(self, clip: BackgroundClip) -> Self {
        self.class(format!("bg-clip-{}", clip.to_class_name()))
    }
}

/// Trait for adding background origin utilities to a class builder
pub trait BackgroundOriginUtilities {
    fn background_origin(self, origin: BackgroundOrigin) -> Self;
}

impl BackgroundOriginUtilities for ClassBuilder {
    fn background_origin(self, origin: BackgroundOrigin) -> Self {
        self.class(format!("bg-origin-{}", origin.to_class_name()))
    }
}

/// Trait for adding background position utilities to a class builder
pub trait BackgroundPositionUtilities {
    fn background_position(self, position: BackgroundPosition) -> Self;
}

impl BackgroundPositionUtilities for ClassBuilder {
    fn background_position(self, position: BackgroundPosition) -> Self {
        self.class(format!("bg-{}", position.to_class_name()))
    }
}

/// Trait for adding background repeat utilities to a class builder
pub trait BackgroundRepeatUtilities {
    fn background_repeat(self, repeat: BackgroundRepeat) -> Self;
}

impl BackgroundRepeatUtilities for ClassBuilder {
    fn background_repeat(self, repeat: BackgroundRepeat) -> Self {
        self.class(format!("bg-{}", repeat.to_class_name()))
    }
}

/// Trait for adding background size utilities to a class builder
pub trait BackgroundSizeUtilities {
    fn background_size(self, size: BackgroundSize) -> Self;
}

impl BackgroundSizeUtilities for ClassBuilder {
    fn background_size(self, size: BackgroundSize) -> Self {
        self.class(format!("bg-{}", size.to_class_name()))
    }
}

/// Trait for adding background image utilities to a class builder
pub trait BackgroundImageUtilities {
    fn background_image(self, image: BackgroundImage) -> Self;
}

impl BackgroundImageUtilities for ClassBuilder {
    fn background_image(self, image: BackgroundImage) -> Self {
        self.class(format!("bg-{}", image.to_class_name()))
    }
}

/// Trait for adding gradient direction utilities to a class builder
pub trait GradientDirectionUtilities {
    fn gradient_direction(self, direction: GradientDirection) -> Self;
}

impl GradientDirectionUtilities for ClassBuilder {
    fn gradient_direction(self, direction: GradientDirection) -> Self {
        self.class(format!("bg-gradient-{}", direction.to_class_name()))
    }
}

/// Trait for adding gradient stop utilities to a class builder
pub trait GradientStopUtilities {
    fn gradient_from(self, color: crate::utilities::colors::Color) -> Self;
    fn gradient_via(self, color: crate::utilities::colors::Color) -> Self;
    fn gradient_to(self, color: crate::utilities::colors::Color) -> Self;
}

impl GradientStopUtilities for ClassBuilder {
    fn gradient_from(self, color: crate::utilities::colors::Color) -> Self {
        self.class(format!("from-{}", color.to_class_name()))
    }

    fn gradient_via(self, color: crate::utilities::colors::Color) -> Self {
        self.class(format!("via-{}", color.to_class_name()))
    }

    fn gradient_to(self, color: crate::utilities::colors::Color) -> Self {
        self.class(format!("to-{}", color.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::colors::{Color, ColorPalette, ColorShade};

    #[test]
    fn test_background_attachment_utilities() {
        let classes = ClassBuilder::new()
            .background_attachment(BackgroundAttachment::Fixed)
            .background_attachment(BackgroundAttachment::Local)
            .background_attachment(BackgroundAttachment::Scroll)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-fixed"));
        assert!(css_classes.contains("bg-local"));
        assert!(css_classes.contains("bg-scroll"));
    }

    #[test]
    fn test_background_clip_utilities() {
        let classes = ClassBuilder::new()
            .background_clip(BackgroundClip::Border)
            .background_clip(BackgroundClip::Padding)
            .background_clip(BackgroundClip::Content)
            .background_clip(BackgroundClip::Text)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-clip-border"));
        assert!(css_classes.contains("bg-clip-padding"));
        assert!(css_classes.contains("bg-clip-content"));
        assert!(css_classes.contains("bg-clip-text"));
    }

    #[test]
    fn test_background_origin_utilities() {
        let classes = ClassBuilder::new()
            .background_origin(BackgroundOrigin::Border)
            .background_origin(BackgroundOrigin::Padding)
            .background_origin(BackgroundOrigin::Content)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-origin-border"));
        assert!(css_classes.contains("bg-origin-padding"));
        assert!(css_classes.contains("bg-origin-content"));
    }

    #[test]
    fn test_background_position_utilities() {
        let classes = ClassBuilder::new()
            .background_position(BackgroundPosition::Bottom)
            .background_position(BackgroundPosition::Center)
            .background_position(BackgroundPosition::Left)
            .background_position(BackgroundPosition::Right)
            .background_position(BackgroundPosition::Top)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-bottom"));
        assert!(css_classes.contains("bg-center"));
        assert!(css_classes.contains("bg-left"));
        assert!(css_classes.contains("bg-right"));
        assert!(css_classes.contains("bg-top"));
    }

    #[test]
    fn test_background_repeat_utilities() {
        let classes = ClassBuilder::new()
            .background_repeat(BackgroundRepeat::NoRepeat)
            .background_repeat(BackgroundRepeat::Repeat)
            .background_repeat(BackgroundRepeat::RepeatX)
            .background_repeat(BackgroundRepeat::RepeatY)
            .background_repeat(BackgroundRepeat::Round)
            .background_repeat(BackgroundRepeat::Space)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-no-repeat"));
        assert!(css_classes.contains("bg-repeat"));
        assert!(css_classes.contains("bg-repeat-x"));
        assert!(css_classes.contains("bg-repeat-y"));
        assert!(css_classes.contains("bg-round"));
        assert!(css_classes.contains("bg-space"));
    }

    #[test]
    fn test_background_size_utilities() {
        let classes = ClassBuilder::new()
            .background_size(BackgroundSize::Auto)
            .background_size(BackgroundSize::Cover)
            .background_size(BackgroundSize::Contain)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-auto"));
        assert!(css_classes.contains("bg-cover"));
        assert!(css_classes.contains("bg-contain"));
    }

    #[test]
    fn test_background_image_utilities() {
        let classes = ClassBuilder::new()
            .background_image(BackgroundImage::None)
            .background_image(BackgroundImage::LinearGradient)
            .background_image(BackgroundImage::RadialGradient)
            .background_image(BackgroundImage::ConicGradient)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-none"));
        assert!(css_classes.contains("bg-gradient-to-r"));
        assert!(css_classes.contains("bg-radial-gradient"));
        assert!(css_classes.contains("bg-conic-gradient"));
    }

    #[test]
    fn test_gradient_direction_utilities() {
        let classes = ClassBuilder::new()
            .gradient_direction(GradientDirection::ToRight)
            .gradient_direction(GradientDirection::ToLeft)
            .gradient_direction(GradientDirection::ToTop)
            .gradient_direction(GradientDirection::ToBottom)
            .gradient_direction(GradientDirection::ToTopRight)
            .gradient_direction(GradientDirection::ToTopLeft)
            .gradient_direction(GradientDirection::ToBottomRight)
            .gradient_direction(GradientDirection::ToBottomLeft)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-gradient-to-r"));
        assert!(css_classes.contains("bg-gradient-to-l"));
        assert!(css_classes.contains("bg-gradient-to-t"));
        assert!(css_classes.contains("bg-gradient-to-b"));
        assert!(css_classes.contains("bg-gradient-to-tr"));
        assert!(css_classes.contains("bg-gradient-to-tl"));
        assert!(css_classes.contains("bg-gradient-to-br"));
        assert!(css_classes.contains("bg-gradient-to-bl"));
    }

    #[test]
    fn test_gradient_stop_utilities() {
        let classes = ClassBuilder::new()
            .gradient_from(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .gradient_via(Color::new(ColorPalette::Purple, ColorShade::Shade500))
            .gradient_to(Color::new(ColorPalette::Pink, ColorShade::Shade500))
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("from-blue-500"));
        assert!(css_classes.contains("via-purple-500"));
        assert!(css_classes.contains("to-pink-500"));
    }

    #[test]
    fn test_complex_background_combination() {
        let classes = ClassBuilder::new()
            .background_attachment(BackgroundAttachment::Fixed)
            .background_clip(BackgroundClip::Padding)
            .background_origin(BackgroundOrigin::Border)
            .background_position(BackgroundPosition::Center)
            .background_repeat(BackgroundRepeat::NoRepeat)
            .background_size(BackgroundSize::Cover)
            .background_image(BackgroundImage::LinearGradient)
            .gradient_direction(GradientDirection::ToRight)
            .gradient_from(Color::new(ColorPalette::Blue, ColorShade::Shade500))
            .gradient_to(Color::new(ColorPalette::Red, ColorShade::Shade500))
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("bg-fixed"));
        assert!(css_classes.contains("bg-clip-padding"));
        assert!(css_classes.contains("bg-origin-border"));
        assert!(css_classes.contains("bg-center"));
        assert!(css_classes.contains("bg-no-repeat"));
        assert!(css_classes.contains("bg-cover"));
        assert!(css_classes.contains("bg-gradient-to-r"));
        assert!(css_classes.contains("from-blue-500"));
        assert!(css_classes.contains("to-red-500"));
    }

    /// Test that all Week 8 background utilities are implemented
    #[test]
    fn test_week8_background_utilities() {
        // Test all Week 8 background utilities
        let classes = ClassBuilder::new()
            // Background Properties
            .background_attachment(BackgroundAttachment::Fixed)
            .background_attachment(BackgroundAttachment::Local)
            .background_attachment(BackgroundAttachment::Scroll)
            .background_clip(BackgroundClip::Border)
            .background_clip(BackgroundClip::Padding)
            .background_clip(BackgroundClip::Content)
            .background_clip(BackgroundClip::Text)
            .background_position(BackgroundPosition::Bottom)
            .background_position(BackgroundPosition::Center)
            .background_position(BackgroundPosition::Left)
            .background_position(BackgroundPosition::Right)
            .background_position(BackgroundPosition::Top)
            .background_repeat(BackgroundRepeat::Repeat)
            .background_repeat(BackgroundRepeat::NoRepeat)
            .background_repeat(BackgroundRepeat::RepeatX)
            .background_repeat(BackgroundRepeat::RepeatY)
            .background_repeat(BackgroundRepeat::Round)
            .background_repeat(BackgroundRepeat::Space)
            .background_size(BackgroundSize::Auto)
            .background_size(BackgroundSize::Cover)
            .background_size(BackgroundSize::Contain)
            .build();

        let css_classes = classes.to_css_classes();

        // Background Properties
        assert!(css_classes.contains("bg-fixed"));
        assert!(css_classes.contains("bg-local"));
        assert!(css_classes.contains("bg-scroll"));
        assert!(css_classes.contains("bg-clip-border"));
        assert!(css_classes.contains("bg-clip-padding"));
        assert!(css_classes.contains("bg-clip-content"));
        assert!(css_classes.contains("bg-clip-text"));
        assert!(css_classes.contains("bg-bottom"));
        assert!(css_classes.contains("bg-center"));
        assert!(css_classes.contains("bg-left"));
        assert!(css_classes.contains("bg-right"));
        assert!(css_classes.contains("bg-top"));
        assert!(css_classes.contains("bg-repeat"));
        assert!(css_classes.contains("bg-no-repeat"));
        assert!(css_classes.contains("bg-repeat-x"));
        assert!(css_classes.contains("bg-repeat-y"));
        assert!(css_classes.contains("bg-round"));
        assert!(css_classes.contains("bg-space"));
        assert!(css_classes.contains("bg-auto"));
        assert!(css_classes.contains("bg-cover"));
        assert!(css_classes.contains("bg-contain"));
    }
}
