//! Typography utilities for tailwind-rs
//!
//! This module provides comprehensive utilities for typography including:
//! - Font families, sizes, and weights
//! - Text alignment, decoration, and transformation
//! - Line height and letter spacing
//! - Text shadows and font feature settings
//! - Advanced OpenType font features and variants
//!
//! The module is organized into the following sub-modules:
//! - `fonts`: Font families, sizes, and weights
//! - `text_alignment`: Text alignment utilities
//! - `spacing`: Line height and letter spacing utilities
//! - `text_decoration`: Text decoration utilities
//! - `text_transform`: Text transformation utilities
//! - `font_features`: OpenType font features and variants
//! - `text_shadow`: Text shadow effects (re-exported from parent)

pub mod fonts;
pub mod spacing;
pub mod text_alignment;
pub mod text_decoration;
pub mod text_transform;
pub mod font_features;

// Re-export from parent text_shadow module
pub use crate::utilities::text_shadow::{TextShadow, TextShadowUtilities};

// Re-export the main types and traits for convenience
pub use font_features::{FontFeature, FontFeatureUtilities, FontVariant};
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
    + FontFeatureUtilities
    + TextShadowUtilities
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

    /// Apply advanced typography with font features and text shadows
    pub fn typography_advanced(
        &mut self,
        family: Option<FontFamily>,
        size: Option<FontSize>,
        weight: Option<FontWeight>,
        align: Option<TextAlign>,
        line_height: Option<LineHeight>,
        letter_spacing: Option<LetterSpacing>,
        font_features: Vec<FontFeature>,
        text_shadow: Option<TextShadow>,
        font_variant: Option<FontVariant>,
    ) -> &mut Self {
        // Apply basic typography
        self.typography(family, size, weight, align, line_height, letter_spacing);

        // Apply font features
        for feature in font_features {
            self.clone().font_feature_custom(feature);
        }

        // Apply text shadow
        if let Some(shadow) = text_shadow {
            self.clone().text_shadow_custom(shadow);
        }

        // Apply font variant
        if let Some(variant) = font_variant {
            self.clone().font_variant_custom(variant);
        }

        self
    }

    /// Apply typography preset for headings
    pub fn typography_heading(&mut self, level: u8) -> &mut Self {
        match level {
            1 => {
                self.text_size(FontSize::Xl4);
                self.font_weight(FontWeight::Bold);
                self.leading(LineHeight::Tight);
                self.tracking(LetterSpacing::Tighter);
                self
            }
            2 => {
                self.text_size(FontSize::Xl3);
                self.font_weight(FontWeight::Bold);
                self.leading(LineHeight::Tight);
                self.tracking(LetterSpacing::Tighter);
                self
            }
            3 => {
                self.text_size(FontSize::Xl2);
                self.font_weight(FontWeight::SemiBold);
                self.leading(LineHeight::Snug);
                self.tracking(LetterSpacing::Tight);
                self
            }
            4 => {
                self.text_size(FontSize::Xl);
                self.font_weight(FontWeight::SemiBold);
                self.leading(LineHeight::Snug);
                self.tracking(LetterSpacing::Tight);
                self
            }
            5 => {
                self.text_size(FontSize::Lg);
                self.font_weight(FontWeight::SemiBold);
                self.leading(LineHeight::Normal);
                self.tracking(LetterSpacing::Normal);
                self
            }
            6 => {
                self.text_size(FontSize::Base);
                self.font_weight(FontWeight::SemiBold);
                self.leading(LineHeight::Normal);
                self.tracking(LetterSpacing::Normal);
                self
            }
            _ => self,
        }
    }

    /// Apply typography preset for body text
    pub fn typography_body(&mut self) -> &mut Self {
        self.text_size(FontSize::Base);
        self.leading(LineHeight::Relaxed);
        self.tracking(LetterSpacing::Normal);
        self.clone().font_feature_lnum(); // Lining figures for body text
        self.clone().font_feature_kern(); // Enable kerning
        self
    }

    /// Apply typography preset for captions
    pub fn typography_caption(&mut self) -> &mut Self {
        self.text_size(FontSize::Sm);
        self.leading(LineHeight::Normal);
        self.tracking(LetterSpacing::Wide);
        self.clone().font_feature_onum(); // Old-style figures for captions
        self
    }

    /// Apply typography preset for code
    pub fn typography_code(&mut self) -> &mut Self {
        self.font_family(FontFamily::Mono);
        self.text_size(FontSize::Sm);
        self.leading(LineHeight::Normal);
        self.tracking(LetterSpacing::Normal);
        self.clone().font_feature_tnum(); // Tabular figures for code alignment
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
    fn test_typography_advanced() {
        let mut builder = ClassBuilder::new();
        let font_features = vec![
            FontFeature::Ligatures,
            FontFeature::SmallCapitals,
            FontFeature::TabularFigures,
        ];

        builder.typography_advanced(
            Some(FontFamily::Serif),
            Some(FontSize::Xl),
            Some(FontWeight::Medium),
            Some(TextAlign::Left),
            Some(LineHeight::Relaxed),
            Some(LetterSpacing::Normal),
            font_features,
            Some(TextShadow::Sm),
            Some(FontVariant::SmallCaps),
        );

        let classes = builder.build_string();
        assert!(classes.contains("font-serif"));
        assert!(classes.contains("text-xl"));
        assert!(classes.contains("font-medium"));
        assert!(classes.contains("text-left"));
        assert!(classes.contains("leading-relaxed"));
        assert!(classes.contains("tracking-normal"));
        assert!(classes.contains("font-feature-ligatures"));
        assert!(classes.contains("font-feature-smcp"));
        assert!(classes.contains("font-feature-tnum"));
        assert!(classes.contains("text-shadow-sm"));
        assert!(classes.contains("font-variant-small-caps"));
    }

    #[test]
    fn test_typography_presets() {
        // Test heading presets
        let mut builder = ClassBuilder::new();
        builder.typography_heading(1);
        let classes = builder.build_string();
        assert!(classes.contains("text-4xl"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("leading-tight"));
        assert!(classes.contains("tracking-tighter"));

        // Test body preset
        let mut builder = ClassBuilder::new();
        builder.typography_body();
        let classes = builder.build_string();
        assert!(classes.contains("text-base"));
        assert!(classes.contains("leading-relaxed"));
        assert!(classes.contains("font-feature-lnum"));
        assert!(classes.contains("font-feature-kern"));

        // Test code preset
        let mut builder = ClassBuilder::new();
        builder.typography_code();
        let classes = builder.build_string();
        assert!(classes.contains("font-mono"));
        assert!(classes.contains("text-sm"));
        assert!(classes.contains("font-feature-tnum"));
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

    #[test]
    fn test_font_feature_utilities() {
        let mut builder = ClassBuilder::new();
        builder
            .font_feature_ligatures()
            .font_feature_ss01()
            .font_feature_smcp()
            .font_feature_tnum()
            .font_feature_kern();

        let classes = builder.build_string();
        assert!(classes.contains("font-feature-ligatures"));
        assert!(classes.contains("font-feature-ss01"));
        assert!(classes.contains("font-feature-smcp"));
        assert!(classes.contains("font-feature-tnum"));
        assert!(classes.contains("font-feature-kern"));
    }

    #[test]
    fn test_font_variant_utilities() {
        let mut builder = ClassBuilder::new();
        builder
            .font_variant_small_caps()
            .font_variant_lining_nums()
            .font_variant_tabular_nums()
            .font_variant_ordinal();

        let classes = builder.build_string();
        assert!(classes.contains("font-variant-small-caps"));
        assert!(classes.contains("font-variant-lining-nums"));
        assert!(classes.contains("font-variant-tabular-nums"));
        assert!(classes.contains("font-variant-ordinal"));
    }

    #[test]
    fn test_text_shadow_utilities() {
        let mut builder = ClassBuilder::new();
        builder
            .text_shadow_sm()
            .text_shadow_lg()
            .text_shadow_inner();

        let classes = builder.build_string();
        assert!(classes.contains("text-shadow-sm"));
        assert!(classes.contains("text-shadow-lg"));
        assert!(classes.contains("text-shadow-inner"));
    }

    #[test]
    fn test_custom_font_features() {
        let mut builder = ClassBuilder::new();
        let custom_feature = FontFeature::Custom("rlig".to_string(), Some(1));
        builder.font_feature_custom(custom_feature);

        let classes = builder.build_string();
        assert!(classes.contains("font-feature-rlig-1"));
    }

    #[test]
    fn test_combined_typography_features() {
        let mut builder = ClassBuilder::new();

        // Combine multiple typography features
        builder
            .typography_heading(2)
            .font_feature_ss01()
            .text_shadow()
            .font_variant_small_caps();

        let classes = builder.build_string();

        // Check heading preset
        assert!(classes.contains("text-3xl"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("leading-tight"));
        assert!(classes.contains("tracking-tighter"));

        // Check additional features
        assert!(classes.contains("font-feature-ss01"));
        assert!(classes.contains("text-shadow"));
        assert!(classes.contains("font-variant-small-caps"));
    }
}
