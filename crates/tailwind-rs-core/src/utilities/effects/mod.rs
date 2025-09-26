//! Effects utilities for tailwind-rs
//!
//! This module provides utilities for box shadow, drop shadow, opacity, mix blend mode,
//! background blend mode, and other visual effects.

pub mod backdrop_filters;
pub mod blend_modes;
pub mod box_shadow;
pub mod drop_shadow;
pub mod opacity;

// Re-export all the main types and traits
pub use backdrop_filters::{
    BackdropBlur, BackdropBrightness, BackdropContrast, BackdropFilterUtilities, BackdropGrayscale,
    BackdropHueRotate, BackdropInvert, BackdropOpacity, BackdropSaturate, BackdropSepia,
};
pub use blend_modes::{
    BackgroundBlendMode, BackgroundBlendModeUtilities, MixBlendMode, MixBlendModeUtilities,
};
pub use box_shadow::{BoxShadow, BoxShadowUtilities};
pub use drop_shadow::{DropShadow, DropShadowUtilities};
pub use opacity::{Opacity, OpacityUtilities};
