//! Color utilities for tailwind-rs
//!
//! This module provides utilities for text colors, background colors, border colors,
//! ring colors, and other color-related utilities.

pub mod color_palette;
pub mod color_shade;
pub mod color_struct;
pub mod text_color;
pub mod background_color;
pub mod border_color;
pub mod ring_color;

// Re-export all the main types and traits
pub use color_palette::ColorPalette;
pub use color_shade::ColorShade;
pub use color_struct::Color;
pub use text_color::TextColorUtilities;
pub use background_color::BackgroundColorUtilities;
pub use border_color::BorderColorUtilities;
pub use ring_color::RingColorUtilities;
