//! Background Utilities Module
//!
//! Comprehensive background utilities with all background-related enums and implementations.
//! This module provides type-safe utilities for background properties including:
//! - Background attachment, clip, origin, position, repeat, size
//! - Background images and gradients
//! - Gradient directions and stops
//! - Display trait implementations

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

// Re-export all background utility types and traits
pub mod attachment;
pub mod clip;
pub mod display;
pub mod gradient;
pub mod image;
pub mod origin;
pub mod position;
pub mod repeat;
pub mod size;

// Re-export all types for easy access
pub use attachment::BackgroundAttachment;
pub use clip::BackgroundClip;
pub use gradient::{GradientDirection, GradientStop};
pub use image::BackgroundImage;
pub use origin::BackgroundOrigin;
pub use position::BackgroundPosition;
pub use repeat::BackgroundRepeat;
pub use size::BackgroundSize;

// Import display implementations
pub use display::*;

/// Background utilities trait for extending ClassBuilder
pub trait BackgroundUtilities {
    /// Add background attachment class
    fn background_attachment(self, attachment: BackgroundAttachment) -> Self;

    /// Add background clip class
    fn background_clip(self, clip: BackgroundClip) -> Self;

    /// Add background origin class
    fn background_origin(self, origin: BackgroundOrigin) -> Self;

    /// Add background position class
    fn background_position(self, position: BackgroundPosition) -> Self;

    /// Add background repeat class
    fn background_repeat(self, repeat: BackgroundRepeat) -> Self;

    /// Add background size class
    fn background_size(self, size: BackgroundSize) -> Self;

    /// Add background image class
    fn background_image(self, image: BackgroundImage) -> Self;

    /// Add gradient direction class
    fn gradient_direction(self, direction: GradientDirection) -> Self;

    /// Add gradient stop class
    fn gradient_stop(self, stop: GradientStop) -> Self;
}

impl BackgroundUtilities for ClassBuilder {
    fn background_attachment(self, attachment: BackgroundAttachment) -> Self {
        self.class(&format!("bg-{}", attachment.to_class_name()))
    }

    fn background_clip(self, clip: BackgroundClip) -> Self {
        self.class(&format!("bg-clip-{}", clip.to_class_name()))
    }

    fn background_origin(self, origin: BackgroundOrigin) -> Self {
        self.class(&format!("bg-origin-{}", origin.to_class_name()))
    }

    fn background_position(self, position: BackgroundPosition) -> Self {
        self.class(&format!("bg-{}", position.to_class_name()))
    }

    fn background_repeat(self, repeat: BackgroundRepeat) -> Self {
        self.class(&format!("bg-{}", repeat.to_class_name()))
    }

    fn background_size(self, size: BackgroundSize) -> Self {
        self.class(&format!("bg-{}", size.to_class_name()))
    }

    fn background_image(self, image: BackgroundImage) -> Self {
        match image {
            BackgroundImage::None => self.class("bg-none"),
            BackgroundImage::LinearGradient => self.class("bg-gradient-to-r"),
            BackgroundImage::RadialGradient => self.class("bg-radial-gradient"),
            BackgroundImage::ConicGradient => self.class("bg-conic-gradient"),
        }
    }

    fn gradient_direction(self, direction: GradientDirection) -> Self {
        self.class(&format!("bg-gradient-{}", direction.to_class_name()))
    }

    fn gradient_stop(self, stop: GradientStop) -> Self {
        self.class(&format!("bg-{}", stop.to_class_name()))
    }
}

/// Utility functions for background utilities
pub struct BackgroundUtils;

impl BackgroundUtils {
    /// Get all background attachment variants
    pub fn attachment_variants() -> &'static [BackgroundAttachment] {
        BackgroundAttachment::variants()
    }

    /// Get all background clip variants
    pub fn clip_variants() -> &'static [BackgroundClip] {
        BackgroundClip::variants()
    }

    /// Get all background position variants
    pub fn position_variants() -> &'static [BackgroundPosition] {
        BackgroundPosition::variants()
    }

    /// Get all background repeat variants
    pub fn repeat_variants() -> &'static [BackgroundRepeat] {
        BackgroundRepeat::variants()
    }

    /// Get all background size variants
    pub fn size_variants() -> &'static [BackgroundSize] {
        BackgroundSize::variants()
    }

    /// Get all background image variants
    pub fn image_variants() -> &'static [BackgroundImage] {
        BackgroundImage::variants()
    }

    /// Get all gradient direction variants
    pub fn gradient_direction_variants() -> &'static [GradientDirection] {
        GradientDirection::variants()
    }

    /// Get all gradient stop variants
    pub fn gradient_stop_variants() -> &'static [GradientStop] {
        GradientStop::variants()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ClassBuilder;

    #[test]
    fn background_utilities_trait() {
        let builder = ClassBuilder::new();

        let result = builder
            .background_attachment(BackgroundAttachment::Fixed)
            .background_clip(BackgroundClip::Border)
            .background_position(BackgroundPosition::Center)
            .background_repeat(BackgroundRepeat::NoRepeat)
            .background_size(BackgroundSize::Cover)
            .background_image(BackgroundImage::None)
            .gradient_direction(GradientDirection::ToRight)
            .gradient_stop(GradientStop::From)
            .build();

        assert!(result.classes.contains("bg-fixed"));
        assert!(result.classes.contains("bg-clip-border"));
        assert!(result.classes.contains("bg-center"));
        assert!(result.classes.contains("bg-no-repeat"));
        assert!(result.classes.contains("bg-cover"));
        assert!(result.classes.contains("bg-none"));
        assert!(result.classes.contains("bg-gradient-to-r"));
        assert!(result.classes.contains("bg-from"));
    }

    #[test]
    fn utility_functions() {
        assert_eq!(BackgroundUtils::attachment_variants().len(), 3);
        assert_eq!(BackgroundUtils::clip_variants().len(), 4);
        assert_eq!(BackgroundUtils::position_variants().len(), 9);
        assert_eq!(BackgroundUtils::repeat_variants().len(), 6);
        assert_eq!(BackgroundUtils::size_variants().len(), 3);
        assert_eq!(BackgroundUtils::image_variants().len(), 4);
        assert_eq!(BackgroundUtils::gradient_direction_variants().len(), 8);
        assert_eq!(BackgroundUtils::gradient_stop_variants().len(), 3);
    }

    #[test]
    fn enum_display_implementations() {
        assert_eq!(format!("{}", BackgroundAttachment::Fixed), "fixed");
        assert_eq!(format!("{}", BackgroundClip::Border), "border");
        assert_eq!(format!("{}", BackgroundPosition::Center), "center");
        assert_eq!(format!("{}", GradientDirection::ToRight), "to-r");
        assert_eq!(format!("{}", GradientStop::From), "from");
    }
}
