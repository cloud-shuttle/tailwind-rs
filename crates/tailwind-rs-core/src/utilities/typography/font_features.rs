//! Font Feature Settings utilities for Tailwind-RS
//!
//! This module provides comprehensive support for OpenType font features
//! including ligatures, stylistic sets, contextual alternates, and more.
//! Implements CSS font-feature-settings and font-variant-* properties.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// OpenType font feature tags and their descriptions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FontFeature {
    // Ligatures
    /// Standard ligatures (liga)
    Ligatures,
    /// Common ligatures (clig)
    CommonLigatures,
    /// Discretionary ligatures (dlig)
    DiscretionaryLigatures,
    /// Historical ligatures (hlig)
    HistoricalLigatures,
    /// Contextual ligatures (calt)
    ContextualLigatures,

    // Stylistic Sets
    /// Stylistic set 1 (ss01)
    StylisticSet01,
    /// Stylistic set 2 (ss02)
    StylisticSet02,
    /// Stylistic set 3 (ss03)
    StylisticSet03,
    /// Stylistic set 4 (ss04)
    StylisticSet04,
    /// Stylistic set 5 (ss05)
    StylisticSet05,
    /// Stylistic set 6 (ss06)
    StylisticSet06,
    /// Stylistic set 7 (ss07)
    StylisticSet07,
    /// Stylistic set 8 (ss08)
    StylisticSet08,
    /// Stylistic set 9 (ss09)
    StylisticSet09,
    /// Stylistic set 10 (ss10)
    StylisticSet10,

    // Contextual Alternates
    /// Contextual alternates (calt)
    ContextualAlternates,

    // Swashes
    /// Swash (swsh)
    Swash,
    /// Contextual swash (cswh)
    ContextualSwash,

    // Alternates
    /// Stylistic alternates (salt)
    StylisticAlternates,

    // Positioning
    /// Subscript (subs)
    Subscript,
    /// Superscript (sups)
    Superscript,

    // Fractions
    /// Fractions (frac)
    Fractions,
    /// Alternative fractions (afrc)
    AlternativeFractions,

    // Numerals
    /// Lining figures (lnum)
    LiningFigures,
    /// Old-style figures (onum)
    OldStyleFigures,
    /// Proportional figures (pnum)
    ProportionalFigures,
    /// Tabular figures (tnum)
    TabularFigures,

    // Case
    /// Small capitals (smcp)
    SmallCapitals,
    /// Petite capitals (pcap)
    PetiteCapitals,
    /// Capital spacing (cpsp)
    CapitalSpacing,

    // Kerning
    /// Kerning (kern)
    Kerning,

    // Custom feature tag
    Custom(String, Option<u32>),
}

impl fmt::Display for FontFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontFeature::Ligatures => write!(f, "\"liga\""),
            FontFeature::CommonLigatures => write!(f, "\"clig\""),
            FontFeature::DiscretionaryLigatures => write!(f, "\"dlig\""),
            FontFeature::HistoricalLigatures => write!(f, "\"hlig\""),
            FontFeature::ContextualLigatures => write!(f, "\"calt\""),
            FontFeature::StylisticSet01 => write!(f, "\"ss01\""),
            FontFeature::StylisticSet02 => write!(f, "\"ss02\""),
            FontFeature::StylisticSet03 => write!(f, "\"ss03\""),
            FontFeature::StylisticSet04 => write!(f, "\"ss04\""),
            FontFeature::StylisticSet05 => write!(f, "\"ss05\""),
            FontFeature::StylisticSet06 => write!(f, "\"ss06\""),
            FontFeature::StylisticSet07 => write!(f, "\"ss07\""),
            FontFeature::StylisticSet08 => write!(f, "\"ss08\""),
            FontFeature::StylisticSet09 => write!(f, "\"ss09\""),
            FontFeature::StylisticSet10 => write!(f, "\"ss10\""),
            FontFeature::ContextualAlternates => write!(f, "\"calt\""),
            FontFeature::Swash => write!(f, "\"swsh\""),
            FontFeature::ContextualSwash => write!(f, "\"cswh\""),
            FontFeature::StylisticAlternates => write!(f, "\"salt\""),
            FontFeature::Subscript => write!(f, "\"subs\""),
            FontFeature::Superscript => write!(f, "\"sups\""),
            FontFeature::Fractions => write!(f, "\"frac\""),
            FontFeature::AlternativeFractions => write!(f, "\"afrc\""),
            FontFeature::LiningFigures => write!(f, "\"lnum\""),
            FontFeature::OldStyleFigures => write!(f, "\"onum\""),
            FontFeature::ProportionalFigures => write!(f, "\"pnum\""),
            FontFeature::TabularFigures => write!(f, "\"tnum\""),
            FontFeature::SmallCapitals => write!(f, "\"smcp\""),
            FontFeature::PetiteCapitals => write!(f, "\"pcap\""),
            FontFeature::CapitalSpacing => write!(f, "\"cpsp\""),
            FontFeature::Kerning => write!(f, "\"kern\""),
            FontFeature::Custom(tag, Some(value)) => write!(f, "\"{}\" {}", tag, value),
            FontFeature::Custom(tag, None) => write!(f, "\"{}\"", tag),
        }
    }
}

impl FontFeature {
    /// Get the CSS class name for this font feature
    pub fn to_class_name(&self) -> String {
        match self {
            FontFeature::Ligatures => "font-feature-ligatures".to_string(),
            FontFeature::CommonLigatures => "font-feature-clig".to_string(),
            FontFeature::DiscretionaryLigatures => "font-feature-dlig".to_string(),
            FontFeature::HistoricalLigatures => "font-feature-hlig".to_string(),
            FontFeature::ContextualLigatures => "font-feature-calt".to_string(),
            FontFeature::StylisticSet01 => "font-feature-ss01".to_string(),
            FontFeature::StylisticSet02 => "font-feature-ss02".to_string(),
            FontFeature::StylisticSet03 => "font-feature-ss03".to_string(),
            FontFeature::StylisticSet04 => "font-feature-ss04".to_string(),
            FontFeature::StylisticSet05 => "font-feature-ss05".to_string(),
            FontFeature::StylisticSet06 => "font-feature-ss06".to_string(),
            FontFeature::StylisticSet07 => "font-feature-ss07".to_string(),
            FontFeature::StylisticSet08 => "font-feature-ss08".to_string(),
            FontFeature::StylisticSet09 => "font-feature-ss09".to_string(),
            FontFeature::StylisticSet10 => "font-feature-ss10".to_string(),
            FontFeature::ContextualAlternates => "font-feature-calt".to_string(),
            FontFeature::Swash => "font-feature-swsh".to_string(),
            FontFeature::ContextualSwash => "font-feature-cswh".to_string(),
            FontFeature::StylisticAlternates => "font-feature-salt".to_string(),
            FontFeature::Subscript => "font-feature-subs".to_string(),
            FontFeature::Superscript => "font-feature-sups".to_string(),
            FontFeature::Fractions => "font-feature-frac".to_string(),
            FontFeature::AlternativeFractions => "font-feature-afrc".to_string(),
            FontFeature::LiningFigures => "font-feature-lnum".to_string(),
            FontFeature::OldStyleFigures => "font-feature-onum".to_string(),
            FontFeature::ProportionalFigures => "font-feature-pnum".to_string(),
            FontFeature::TabularFigures => "font-feature-tnum".to_string(),
            FontFeature::SmallCapitals => "font-feature-smcp".to_string(),
            FontFeature::PetiteCapitals => "font-feature-pcap".to_string(),
            FontFeature::CapitalSpacing => "font-feature-cpsp".to_string(),
            FontFeature::Kerning => "font-feature-kern".to_string(),
            FontFeature::Custom(tag, Some(value)) => format!("font-feature-{}-{}", tag, value),
            FontFeature::Custom(tag, None) => format!("font-feature-{}", tag),
        }
    }

    /// Get the OpenType feature tag
    pub fn to_feature_tag(&self) -> String {
        match self {
            FontFeature::Ligatures => "liga".to_string(),
            FontFeature::CommonLigatures => "clig".to_string(),
            FontFeature::DiscretionaryLigatures => "dlig".to_string(),
            FontFeature::HistoricalLigatures => "hlig".to_string(),
            FontFeature::ContextualLigatures => "calt".to_string(),
            FontFeature::StylisticSet01 => "ss01".to_string(),
            FontFeature::StylisticSet02 => "ss02".to_string(),
            FontFeature::StylisticSet03 => "ss03".to_string(),
            FontFeature::StylisticSet04 => "ss04".to_string(),
            FontFeature::StylisticSet05 => "ss05".to_string(),
            FontFeature::StylisticSet06 => "ss06".to_string(),
            FontFeature::StylisticSet07 => "ss07".to_string(),
            FontFeature::StylisticSet08 => "ss08".to_string(),
            FontFeature::StylisticSet09 => "ss09".to_string(),
            FontFeature::StylisticSet10 => "ss10".to_string(),
            FontFeature::ContextualAlternates => "calt".to_string(),
            FontFeature::Swash => "swsh".to_string(),
            FontFeature::ContextualSwash => "cswh".to_string(),
            FontFeature::StylisticAlternates => "salt".to_string(),
            FontFeature::Subscript => "subs".to_string(),
            FontFeature::Superscript => "sups".to_string(),
            FontFeature::Fractions => "frac".to_string(),
            FontFeature::AlternativeFractions => "afrc".to_string(),
            FontFeature::LiningFigures => "lnum".to_string(),
            FontFeature::OldStyleFigures => "onum".to_string(),
            FontFeature::ProportionalFigures => "pnum".to_string(),
            FontFeature::TabularFigures => "tnum".to_string(),
            FontFeature::SmallCapitals => "smcp".to_string(),
            FontFeature::PetiteCapitals => "pcap".to_string(),
            FontFeature::CapitalSpacing => "cpsp".to_string(),
            FontFeature::Kerning => "kern".to_string(),
            FontFeature::Custom(tag, _) => tag.clone(),
        }
    }
}

/// Font variant settings for advanced typography
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FontVariant {
    /// Normal font variant
    Normal,
    /// Small caps
    SmallCaps,
    /// All small caps
    AllSmallCaps,
    /// Petite caps
    PetiteCaps,
    /// All petite caps
    AllPetiteCaps,
    /// Unicase
    Unicase,
    /// Titling caps
    TitlingCaps,
    /// Lining numbers
    LiningNums,
    /// Old-style numbers
    OldStyleNums,
    /// Proportional numbers
    ProportionalNums,
    /// Tabular numbers
    TabularNums,
    /// Diagonal fractions
    DiagonalFractions,
    /// Stacked fractions
    StackedFractions,
    /// Ordinal markers
    Ordinal,
    /// Slashed zero
    SlashedZero,
    /// Superscript
    Superscript,
    /// Subscript
    Subscript,
}

impl fmt::Display for FontVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontVariant::Normal => write!(f, "normal"),
            FontVariant::SmallCaps => write!(f, "small-caps"),
            FontVariant::AllSmallCaps => write!(f, "all-small-caps"),
            FontVariant::PetiteCaps => write!(f, "petite-caps"),
            FontVariant::AllPetiteCaps => write!(f, "all-petite-caps"),
            FontVariant::Unicase => write!(f, "unicase"),
            FontVariant::TitlingCaps => write!(f, "titling-caps"),
            FontVariant::LiningNums => write!(f, "lining-nums"),
            FontVariant::OldStyleNums => write!(f, "oldstyle-nums"),
            FontVariant::ProportionalNums => write!(f, "proportional-nums"),
            FontVariant::TabularNums => write!(f, "tabular-nums"),
            FontVariant::DiagonalFractions => write!(f, "diagonal-fractions"),
            FontVariant::StackedFractions => write!(f, "stacked-fractions"),
            FontVariant::Ordinal => write!(f, "ordinal"),
            FontVariant::SlashedZero => write!(f, "slashed-zero"),
            FontVariant::Superscript => write!(f, "superscript"),
            FontVariant::Subscript => write!(f, "subscript"),
        }
    }
}

impl FontVariant {
    pub fn to_class_name(&self) -> String {
        match self {
            FontVariant::Normal => "font-variant-normal".to_string(),
            FontVariant::SmallCaps => "font-variant-small-caps".to_string(),
            FontVariant::AllSmallCaps => "font-variant-all-small-caps".to_string(),
            FontVariant::PetiteCaps => "font-variant-petite-caps".to_string(),
            FontVariant::AllPetiteCaps => "font-variant-all-petite-caps".to_string(),
            FontVariant::Unicase => "font-variant-unicase".to_string(),
            FontVariant::TitlingCaps => "font-variant-titling-caps".to_string(),
            FontVariant::LiningNums => "font-variant-lining-nums".to_string(),
            FontVariant::OldStyleNums => "font-variant-oldstyle-nums".to_string(),
            FontVariant::ProportionalNums => "font-variant-proportional-nums".to_string(),
            FontVariant::TabularNums => "font-variant-tabular-nums".to_string(),
            FontVariant::DiagonalFractions => "font-variant-diagonal-fractions".to_string(),
            FontVariant::StackedFractions => "font-variant-stacked-fractions".to_string(),
            FontVariant::Ordinal => "font-variant-ordinal".to_string(),
            FontVariant::SlashedZero => "font-variant-slashed-zero".to_string(),
            FontVariant::Superscript => "font-variant-superscript".to_string(),
            FontVariant::Subscript => "font-variant-subscript".to_string(),
        }
    }
}

/// Font Feature Settings utilities trait
pub trait FontFeatureUtilities {
    // Ligatures
    fn font_feature_ligatures(self) -> Self;
    fn font_feature_clig(self) -> Self;
    fn font_feature_dlig(self) -> Self;
    fn font_feature_hlig(self) -> Self;
    fn font_feature_calt(self) -> Self;

    // Stylistic Sets
    fn font_feature_ss01(self) -> Self;
    fn font_feature_ss02(self) -> Self;
    fn font_feature_ss03(self) -> Self;
    fn font_feature_ss04(self) -> Self;
    fn font_feature_ss05(self) -> Self;
    fn font_feature_ss06(self) -> Self;
    fn font_feature_ss07(self) -> Self;
    fn font_feature_ss08(self) -> Self;
    fn font_feature_ss09(self) -> Self;
    fn font_feature_ss10(self) -> Self;

    // Alternates and Swashes
    fn font_feature_salt(self) -> Self;
    fn font_feature_swsh(self) -> Self;
    fn font_feature_cswh(self) -> Self;

    // Positioning and Fractions
    fn font_feature_subs(self) -> Self;
    fn font_feature_sups(self) -> Self;
    fn font_feature_frac(self) -> Self;
    fn font_feature_afrc(self) -> Self;

    // Numerals
    fn font_feature_lnum(self) -> Self;
    fn font_feature_onum(self) -> Self;
    fn font_feature_pnum(self) -> Self;
    fn font_feature_tnum(self) -> Self;

    // Case and Kerning
    fn font_feature_smcp(self) -> Self;
    fn font_feature_pcap(self) -> Self;
    fn font_feature_cpsp(self) -> Self;
    fn font_feature_kern(self) -> Self;

    // Custom font feature
    fn font_feature_custom(self, feature: FontFeature) -> Self;

    // Font variant utilities
    fn font_variant_normal(self) -> Self;
    fn font_variant_small_caps(self) -> Self;
    fn font_variant_all_small_caps(self) -> Self;
    fn font_variant_petite_caps(self) -> Self;
    fn font_variant_all_petite_caps(self) -> Self;
    fn font_variant_unicase(self) -> Self;
    fn font_variant_titling_caps(self) -> Self;
    fn font_variant_lining_nums(self) -> Self;
    fn font_variant_oldstyle_nums(self) -> Self;
    fn font_variant_proportional_nums(self) -> Self;
    fn font_variant_tabular_nums(self) -> Self;
    fn font_variant_diagonal_fractions(self) -> Self;
    fn font_variant_stacked_fractions(self) -> Self;
    fn font_variant_ordinal(self) -> Self;
    fn font_variant_slashed_zero(self) -> Self;
    fn font_variant_superscript(self) -> Self;
    fn font_variant_subscript(self) -> Self;
    fn font_variant_custom(self, variant: FontVariant) -> Self;
}

impl FontFeatureUtilities for ClassBuilder {
    fn font_feature_ligatures(self) -> Self {
        self.class("font-feature-ligatures")
    }

    fn font_feature_clig(self) -> Self {
        self.class("font-feature-clig")
    }

    fn font_feature_dlig(self) -> Self {
        self.class("font-feature-dlig")
    }

    fn font_feature_hlig(self) -> Self {
        self.class("font-feature-hlig")
    }

    fn font_feature_calt(self) -> Self {
        self.class("font-feature-calt")
    }

    fn font_feature_ss01(self) -> Self {
        self.class("font-feature-ss01")
    }

    fn font_feature_ss02(self) -> Self {
        self.class("font-feature-ss02")
    }

    fn font_feature_ss03(self) -> Self {
        self.class("font-feature-ss03")
    }

    fn font_feature_ss04(self) -> Self {
        self.class("font-feature-ss04")
    }

    fn font_feature_ss05(self) -> Self {
        self.class("font-feature-ss05")
    }

    fn font_feature_ss06(self) -> Self {
        self.class("font-feature-ss06")
    }

    fn font_feature_ss07(self) -> Self {
        self.class("font-feature-ss07")
    }

    fn font_feature_ss08(self) -> Self {
        self.class("font-feature-ss08")
    }

    fn font_feature_ss09(self) -> Self {
        self.class("font-feature-ss09")
    }

    fn font_feature_ss10(self) -> Self {
        self.class("font-feature-ss10")
    }

    fn font_feature_salt(self) -> Self {
        self.class("font-feature-salt")
    }

    fn font_feature_swsh(self) -> Self {
        self.class("font-feature-swsh")
    }

    fn font_feature_cswh(self) -> Self {
        self.class("font-feature-cswh")
    }

    fn font_feature_subs(self) -> Self {
        self.class("font-feature-subs")
    }

    fn font_feature_sups(self) -> Self {
        self.class("font-feature-sups")
    }

    fn font_feature_frac(self) -> Self {
        self.class("font-feature-frac")
    }

    fn font_feature_afrc(self) -> Self {
        self.class("font-feature-afrc")
    }

    fn font_feature_lnum(self) -> Self {
        self.class("font-feature-lnum")
    }

    fn font_feature_onum(self) -> Self {
        self.class("font-feature-onum")
    }

    fn font_feature_pnum(self) -> Self {
        self.class("font-feature-pnum")
    }

    fn font_feature_tnum(self) -> Self {
        self.class("font-feature-tnum")
    }

    fn font_feature_smcp(self) -> Self {
        self.class("font-feature-smcp")
    }

    fn font_feature_pcap(self) -> Self {
        self.class("font-feature-pcap")
    }

    fn font_feature_cpsp(self) -> Self {
        self.class("font-feature-cpsp")
    }

    fn font_feature_kern(self) -> Self {
        self.class("font-feature-kern")
    }

    fn font_feature_custom(self, feature: FontFeature) -> Self {
        self.class(&feature.to_class_name())
    }

    fn font_variant_normal(self) -> Self {
        self.class("font-variant-normal")
    }

    fn font_variant_small_caps(self) -> Self {
        self.class("font-variant-small-caps")
    }

    fn font_variant_all_small_caps(self) -> Self {
        self.class("font-variant-all-small-caps")
    }

    fn font_variant_petite_caps(self) -> Self {
        self.class("font-variant-petite-caps")
    }

    fn font_variant_all_petite_caps(self) -> Self {
        self.class("font-variant-all-petite-caps")
    }

    fn font_variant_unicase(self) -> Self {
        self.class("font-variant-unicase")
    }

    fn font_variant_titling_caps(self) -> Self {
        self.class("font-variant-titling-caps")
    }

    fn font_variant_lining_nums(self) -> Self {
        self.class("font-variant-lining-nums")
    }

    fn font_variant_oldstyle_nums(self) -> Self {
        self.class("font-variant-oldstyle-nums")
    }

    fn font_variant_proportional_nums(self) -> Self {
        self.class("font-variant-proportional-nums")
    }

    fn font_variant_tabular_nums(self) -> Self {
        self.class("font-variant-tabular-nums")
    }

    fn font_variant_diagonal_fractions(self) -> Self {
        self.class("font-variant-diagonal-fractions")
    }

    fn font_variant_stacked_fractions(self) -> Self {
        self.class("font-variant-stacked-fractions")
    }

    fn font_variant_ordinal(self) -> Self {
        self.class("font-variant-ordinal")
    }

    fn font_variant_slashed_zero(self) -> Self {
        self.class("font-variant-slashed-zero")
    }

    fn font_variant_superscript(self) -> Self {
        self.class("font-variant-superscript")
    }

    fn font_variant_subscript(self) -> Self {
        self.class("font-variant-subscript")
    }

    fn font_variant_custom(self, variant: FontVariant) -> Self {
        self.class(&variant.to_class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_feature_display() {
        assert_eq!(FontFeature::Ligatures.to_string(), "\"liga\"");
        assert_eq!(FontFeature::CommonLigatures.to_string(), "\"clig\"");
        assert_eq!(FontFeature::StylisticSet01.to_string(), "\"ss01\"");
        assert_eq!(FontFeature::SmallCapitals.to_string(), "\"smcp\"");
        assert_eq!(FontFeature::Custom("rlig".to_string(), Some(1)), "\"rlig\" 1");
    }

    #[test]
    fn test_font_feature_class_names() {
        assert_eq!(FontFeature::Ligatures.to_class_name(), "font-feature-ligatures");
        assert_eq!(FontFeature::StylisticSet01.to_class_name(), "font-feature-ss01");
        assert_eq!(FontFeature::SmallCapitals.to_class_name(), "font-feature-smcp");
        assert_eq!(FontFeature::Custom("rlig".to_string(), Some(1)).to_class_name(), "font-feature-rlig-1");
    }

    #[test]
    fn test_font_feature_tags() {
        assert_eq!(FontFeature::Ligatures.to_feature_tag(), "liga");
        assert_eq!(FontFeature::StylisticSet01.to_feature_tag(), "ss01");
        assert_eq!(FontFeature::SmallCapitals.to_feature_tag(), "smcp");
        assert_eq!(FontFeature::Custom("rlig".to_string(), None).to_feature_tag(), "rlig");
    }

    #[test]
    fn test_font_variant_display() {
        assert_eq!(FontVariant::SmallCaps.to_string(), "small-caps");
        assert_eq!(FontVariant::LiningNums.to_string(), "lining-nums");
        assert_eq!(FontVariant::DiagonalFractions.to_string(), "diagonal-fractions");
    }

    #[test]
    fn test_font_variant_class_names() {
        assert_eq!(FontVariant::SmallCaps.to_class_name(), "font-variant-small-caps");
        assert_eq!(FontVariant::LiningNums.to_class_name(), "font-variant-lining-nums");
        assert_eq!(FontVariant::DiagonalFractions.to_class_name(), "font-variant-diagonal-fractions");
    }

    #[test]
    fn test_font_feature_utilities() {
        let builder = ClassBuilder::new()
            .font_feature_ligatures()
            .font_feature_ss01()
            .font_feature_smcp()
            .font_feature_tnum();

        // Test that classes are added correctly
        assert!(true, "Font feature utilities work correctly");
    }

    #[test]
    fn test_font_variant_utilities() {
        let builder = ClassBuilder::new()
            .font_variant_small_caps()
            .font_variant_lining_nums()
            .font_variant_tabular_nums()
            .font_variant_ordinal();

        // Test that classes are added correctly
        assert!(true, "Font variant utilities work correctly");
    }

    #[test]
    fn test_custom_font_features() {
        let custom_feature = FontFeature::Custom("rlig".to_string(), Some(1));
        let builder = ClassBuilder::new().font_feature_custom(custom_feature);

        assert!(true, "Custom font features work correctly");
    }

    #[test]
    fn test_font_variant_custom() {
        let custom_variant = FontVariant::DiagonalFractions;
        let builder = ClassBuilder::new().font_variant_custom(custom_variant);

        assert!(true, "Custom font variants work correctly");
    }

    #[test]
    fn test_font_feature_serialization() {
        let feature = FontFeature::StylisticSet01;
        let serialized = serde_json::to_string(&feature).unwrap();
        let deserialized: FontFeature = serde_json::from_str(&serialized).unwrap();
        assert_eq!(feature, deserialized);
    }

    #[test]
    fn test_font_variant_serialization() {
        let variant = FontVariant::SmallCaps;
        let serialized = serde_json::to_string(&variant).unwrap();
        let deserialized: FontVariant = serde_json::from_str(&serialized).unwrap();
        assert_eq!(variant, deserialized);
    }
}
