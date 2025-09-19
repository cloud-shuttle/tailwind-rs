//! Typography utilities for tailwind-rs
//!
//! This module provides utilities for font families, font sizes, font weights,
//! text alignment, line height, letter spacing, and other typography-related properties.
//!
//! The module is organized into the following sub-modules:
//! - `fonts`: Font families, sizes, and weights
//! - `text_alignment`: Text alignment utilities
//! - `spacing`: Line height and letter spacing utilities
//! - `text_decoration`: Text decoration utilities
//! - `text_transform`: Text transformation utilities

pub mod fonts;
pub mod spacing;
pub mod text_alignment;
pub mod text_decoration;
pub mod text_transform;

// Re-export the main types and traits for convenience
pub use fonts::{
    FontFamily, FontFamilyUtilities, FontSize, FontSizeUtilities, FontWeight, FontWeightUtilities,
};
pub use spacing::{LetterSpacing, LetterSpacingUtilities, LineHeight, LineHeightUtilities};
pub use text_alignment::{TextAlign, TextAlignUtilities};
pub use text_decoration::{
    TextDecoration, TextDecorationStyle, TextDecorationThickness, TextDecorationUtilities,
    TextUnderlineOffset,
};
pub use text_transform::{
    OverflowWrap, TextOverflow, TextTransform, TextTransformUtilities, WhiteSpace, WordBreak,
};

use crate::classes::ClassBuilder;

/// Combined trait for all typography utilities
pub trait TypographyUtilities:
    FontFamilyUtilities
    + FontSizeUtilities
    + FontWeightUtilities
    + TextAlignUtilities
    + LineHeightUtilities
    + LetterSpacingUtilities
    + TextDecorationUtilities
    + TextTransformUtilities
{
}

// Blanket implementation for ClassBuilder
impl TypographyUtilities for ClassBuilder {}

// Add convenient methods to ClassBuilder for comprehensive typography utility functions
impl ClassBuilder {
    /// Apply comprehensive typography styling
    pub fn typography(
        &mut self,
        family: Option<FontFamily>,
        size: Option<FontSize>,
        weight: Option<FontWeight>,
        align: Option<TextAlign>,
        line_height: Option<LineHeight>,
        letter_spacing: Option<LetterSpacing>,
    ) -> &mut Self {
        if let Some(family) = family {
            self.font_family(family);
        }
        if let Some(size) = size {
            self.text_size(size);
        }
        if let Some(weight) = weight {
            self.font_weight(weight);
        }
        if let Some(align) = align {
            self.text_align(align);
        }
        if let Some(line_height) = line_height {
            self.leading(line_height);
        }
        if let Some(letter_spacing) = letter_spacing {
            self.tracking(letter_spacing);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ClassBuilder;

    #[test]
    fn test_typography_comprehensive() {
        let mut builder = ClassBuilder::new();
        builder.typography(
            Some(FontFamily::Sans),
            Some(FontSize::Lg),
            Some(FontWeight::Bold),
            Some(TextAlign::Center),
            Some(LineHeight::Normal),
            Some(LetterSpacing::Wide),
        );

        let classes = builder.build_string();
        assert!(classes.contains("font-sans"));
        assert!(classes.contains("text-lg"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("text-center"));
        assert!(classes.contains("leading-normal"));
        assert!(classes.contains("tracking-wide"));
    }

    #[test]
    fn test_font_utilities() {
        let mut builder = ClassBuilder::new();
        builder.font_sans().text_xl().font_bold();

        let classes = builder.build_string();
        assert!(classes.contains("font-sans"));
        assert!(classes.contains("text-xl"));
        assert!(classes.contains("font-bold"));
    }

    #[test]
    fn test_text_alignment_utilities() {
        let mut builder = ClassBuilder::new();
        builder.text_center();

        let classes = builder.build_string();
        assert!(classes.contains("text-center"));
    }

    #[test]
    fn test_spacing_utilities() {
        let mut builder = ClassBuilder::new();
        builder.leading_relaxed().tracking_wide();

        let classes = builder.build_string();
        assert!(classes.contains("leading-relaxed"));
        assert!(classes.contains("tracking-wide"));
    }
}
