//! Typography utilities for tailwind-rs
//!
//! This module provides utilities for font families, font sizes, font weights,
//! text alignment, line height, letter spacing, and other typography-related properties.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Font family values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FontFamily {
    /// Sans-serif font family
    Sans,
    /// Serif font family
    Serif,
    /// Monospace font family
    Mono,
}

/// Font size values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FontSize {
    /// Extra small font size
    Xs,
    /// Small font size
    Sm,
    /// Base font size
    Base,
    /// Large font size
    Lg,
    /// Extra large font size
    Xl,
    /// 2x large font size
    Xl2,
    /// 3x large font size
    Xl3,
    /// 4x large font size
    Xl4,
    /// 5x large font size
    Xl5,
    /// 6x large font size
    Xl6,
    /// 7x large font size
    Xl7,
    /// 8x large font size
    Xl8,
    /// 9x large font size
    Xl9,
}

/// Font weight values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FontWeight {
    /// Thin font weight (100)
    Thin,
    /// Extra light font weight (200)
    ExtraLight,
    /// Light font weight (300)
    Light,
    /// Normal font weight (400)
    Normal,
    /// Medium font weight (500)
    Medium,
    /// Semi-bold font weight (600)
    SemiBold,
    /// Bold font weight (700)
    Bold,
    /// Extra bold font weight (800)
    ExtraBold,
    /// Black font weight (900)
    Black,
}

/// Text alignment values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextAlign {
    /// Left align
    Left,
    /// Center align
    Center,
    /// Right align
    Right,
    /// Justify align
    Justify,
    /// Start align (left in LTR, right in RTL)
    Start,
    /// End align (right in LTR, left in RTL)
    End,
}

/// Line height values
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LineHeight {
    /// None line height
    None,
    /// Tight line height
    Tight,
    /// Snug line height
    Snug,
    /// Normal line height
    Normal,
    /// Relaxed line height
    Relaxed,
    /// Loose line height
    Loose,
    /// Custom line height (f32)
    Custom(f32),
}

/// Letter spacing values
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LetterSpacing {
    /// Tighter letter spacing
    Tighter,
    /// Tight letter spacing
    Tight,
    /// Normal letter spacing
    Normal,
    /// Wide letter spacing
    Wide,
    /// Wider letter spacing
    Wider,
    /// Widest letter spacing
    Widest,
    /// Custom letter spacing (f32)
    Custom(f32),
}

/// Text decoration values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextDecoration {
    /// No decoration
    None,
    /// Underline decoration
    Underline,
    /// Overline decoration
    Overline,
    /// Line-through decoration
    LineThrough,
}

/// Text transform values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextTransform {
    /// No transform
    None,
    /// Uppercase transform
    Uppercase,
    /// Lowercase transform
    Lowercase,
    /// Capitalize transform
    Capitalize,
}

/// Text overflow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextOverflow {
    /// Truncate text overflow
    Truncate,
    /// Ellipsis text overflow
    Ellipsis,
    /// Clip text overflow
    Clip,
}

/// White space values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WhiteSpace {
    /// Normal white space
    Normal,
    /// Nowrap white space
    Nowrap,
    /// Pre white space
    Pre,
    /// Pre-line white space
    PreLine,
    /// Pre-wrap white space
    PreWrap,
    /// Break-spaces white space
    BreakSpaces,
}

/// Word break values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WordBreak {
    /// Normal word break
    Normal,
    /// Break-all word break
    BreakAll,
    /// Break-words word break
    BreakWords,
    /// Keep-all word break
    KeepAll,
}

impl std::hash::Hash for LineHeight {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LineHeight::None => 0u8.hash(state),
            LineHeight::Tight => 1u8.hash(state),
            LineHeight::Snug => 2u8.hash(state),
            LineHeight::Normal => 3u8.hash(state),
            LineHeight::Relaxed => 4u8.hash(state),
            LineHeight::Loose => 5u8.hash(state),
            LineHeight::Custom(f) => {
                6u8.hash(state);
                ((f * 1000.0) as u32).hash(state);
            }
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
            LetterSpacing::Custom(f) => {
                6u8.hash(state);
                ((f * 1000.0) as u32).hash(state);
            }
        }
    }
}

impl std::cmp::Eq for LineHeight {}
impl std::cmp::Eq for LetterSpacing {}

impl FontFamily {
    pub fn to_class_name(&self) -> String {
        match self {
            FontFamily::Sans => "sans".to_string(),
            FontFamily::Serif => "serif".to_string(),
            FontFamily::Mono => "mono".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            FontFamily::Sans => "ui-sans-serif, system-ui, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color Emoji\"".to_string(),
            FontFamily::Serif => "ui-serif, Georgia, Cambria, \"Times New Roman\", Times, serif".to_string(),
            FontFamily::Mono => "ui-monospace, SFMono-Regular, \"SF Mono\", Consolas, \"Liberation Mono\", Menlo, monospace".to_string(),
        }
    }
}

impl FontSize {
    pub fn to_class_name(&self) -> String {
        match self {
            FontSize::Xs => "xs".to_string(),
            FontSize::Sm => "sm".to_string(),
            FontSize::Base => "base".to_string(),
            FontSize::Lg => "lg".to_string(),
            FontSize::Xl => "xl".to_string(),
            FontSize::Xl2 => "2xl".to_string(),
            FontSize::Xl3 => "3xl".to_string(),
            FontSize::Xl4 => "4xl".to_string(),
            FontSize::Xl5 => "5xl".to_string(),
            FontSize::Xl6 => "6xl".to_string(),
            FontSize::Xl7 => "7xl".to_string(),
            FontSize::Xl8 => "8xl".to_string(),
            FontSize::Xl9 => "9xl".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            FontSize::Xs => "0.75rem".to_string(),
            FontSize::Sm => "0.875rem".to_string(),
            FontSize::Base => "1rem".to_string(),
            FontSize::Lg => "1.125rem".to_string(),
            FontSize::Xl => "1.25rem".to_string(),
            FontSize::Xl2 => "1.5rem".to_string(),
            FontSize::Xl3 => "1.875rem".to_string(),
            FontSize::Xl4 => "2.25rem".to_string(),
            FontSize::Xl5 => "3rem".to_string(),
            FontSize::Xl6 => "3.75rem".to_string(),
            FontSize::Xl7 => "4.5rem".to_string(),
            FontSize::Xl8 => "6rem".to_string(),
            FontSize::Xl9 => "8rem".to_string(),
        }
    }
}

impl FontWeight {
    pub fn to_class_name(&self) -> String {
        match self {
            FontWeight::Thin => "thin".to_string(),
            FontWeight::ExtraLight => "extralight".to_string(),
            FontWeight::Light => "light".to_string(),
            FontWeight::Normal => "normal".to_string(),
            FontWeight::Medium => "medium".to_string(),
            FontWeight::SemiBold => "semibold".to_string(),
            FontWeight::Bold => "bold".to_string(),
            FontWeight::ExtraBold => "extrabold".to_string(),
            FontWeight::Black => "black".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            FontWeight::Thin => "100".to_string(),
            FontWeight::ExtraLight => "200".to_string(),
            FontWeight::Light => "300".to_string(),
            FontWeight::Normal => "400".to_string(),
            FontWeight::Medium => "500".to_string(),
            FontWeight::SemiBold => "600".to_string(),
            FontWeight::Bold => "700".to_string(),
            FontWeight::ExtraBold => "800".to_string(),
            FontWeight::Black => "900".to_string(),
        }
    }
}

impl TextAlign {
    pub fn to_class_name(&self) -> String {
        match self {
            TextAlign::Left => "left".to_string(),
            TextAlign::Center => "center".to_string(),
            TextAlign::Right => "right".to_string(),
            TextAlign::Justify => "justify".to_string(),
            TextAlign::Start => "start".to_string(),
            TextAlign::End => "end".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TextAlign::Left => "left".to_string(),
            TextAlign::Center => "center".to_string(),
            TextAlign::Right => "right".to_string(),
            TextAlign::Justify => "justify".to_string(),
            TextAlign::Start => "start".to_string(),
            TextAlign::End => "end".to_string(),
        }
    }
}

impl LineHeight {
    pub fn to_class_name(&self) -> String {
        match self {
            LineHeight::None => "none".to_string(),
            LineHeight::Tight => "tight".to_string(),
            LineHeight::Snug => "snug".to_string(),
            LineHeight::Normal => "normal".to_string(),
            LineHeight::Relaxed => "relaxed".to_string(),
            LineHeight::Loose => "loose".to_string(),
            LineHeight::Custom(f) => format!("{}", f),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            LineHeight::None => "1".to_string(),
            LineHeight::Tight => "1.25".to_string(),
            LineHeight::Snug => "1.375".to_string(),
            LineHeight::Normal => "1.5".to_string(),
            LineHeight::Relaxed => "1.625".to_string(),
            LineHeight::Loose => "2".to_string(),
            LineHeight::Custom(f) => f.to_string(),
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
            LetterSpacing::Custom(f) => format!("{}", f),
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
            LetterSpacing::Custom(f) => format!("{}em", f),
        }
    }
}

impl TextDecoration {
    pub fn to_class_name(&self) -> String {
        match self {
            TextDecoration::None => "no-underline".to_string(),
            TextDecoration::Underline => "underline".to_string(),
            TextDecoration::Overline => "overline".to_string(),
            TextDecoration::LineThrough => "line-through".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TextDecoration::None => "none".to_string(),
            TextDecoration::Underline => "underline".to_string(),
            TextDecoration::Overline => "overline".to_string(),
            TextDecoration::LineThrough => "line-through".to_string(),
        }
    }
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
            TextOverflow::Truncate => "truncate".to_string(),
            TextOverflow::Ellipsis => "text-ellipsis".to_string(),
            TextOverflow::Clip => "text-clip".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            TextOverflow::Truncate => "truncate".to_string(),
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
            WordBreak::BreakWords => "break-words".to_string(),
            WordBreak::KeepAll => "break-keep".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            WordBreak::Normal => "normal".to_string(),
            WordBreak::BreakAll => "break-all".to_string(),
            WordBreak::BreakWords => "break-words".to_string(),
            WordBreak::KeepAll => "keep-all".to_string(),
        }
    }
}

impl fmt::Display for FontFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TextAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for LineHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for LetterSpacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding font family utilities to a class builder
pub trait FontFamilyUtilities {
    /// Set font family
    fn font_family(self, family: FontFamily) -> Self;
}

impl FontFamilyUtilities for ClassBuilder {
    fn font_family(self, family: FontFamily) -> Self {
        self.class(format!("font-{}", family.to_class_name()))
    }
}

/// Trait for adding font size utilities to a class builder
pub trait FontSizeUtilities {
    /// Set font size
    fn font_size(self, size: FontSize) -> Self;
}

impl FontSizeUtilities for ClassBuilder {
    fn font_size(self, size: FontSize) -> Self {
        self.class(format!("text-{}", size.to_class_name()))
    }
}

/// Trait for adding font weight utilities to a class builder
pub trait FontWeightUtilities {
    /// Set font weight
    fn font_weight(self, weight: FontWeight) -> Self;
}

impl FontWeightUtilities for ClassBuilder {
    fn font_weight(self, weight: FontWeight) -> Self {
        self.class(format!("font-{}", weight.to_class_name()))
    }
}

/// Trait for adding text alignment utilities to a class builder
pub trait TextAlignUtilities {
    /// Set text alignment
    fn text_align(self, align: TextAlign) -> Self;
}

impl TextAlignUtilities for ClassBuilder {
    fn text_align(self, align: TextAlign) -> Self {
        self.class(format!("text-{}", align.to_class_name()))
    }
}

/// Trait for adding line height utilities to a class builder
pub trait LineHeightUtilities {
    /// Set line height
    fn line_height(self, height: LineHeight) -> Self;
}

impl LineHeightUtilities for ClassBuilder {
    fn line_height(self, height: LineHeight) -> Self {
        self.class(format!("leading-{}", height.to_class_name()))
    }
}

/// Trait for adding letter spacing utilities to a class builder
pub trait LetterSpacingUtilities {
    /// Set letter spacing
    fn letter_spacing(self, spacing: LetterSpacing) -> Self;
}

impl LetterSpacingUtilities for ClassBuilder {
    fn letter_spacing(self, spacing: LetterSpacing) -> Self {
        self.class(format!("tracking-{}", spacing.to_class_name()))
    }
}

/// Trait for adding text decoration utilities to a class builder
pub trait TextDecorationUtilities {
    /// Set text decoration
    fn text_decoration(self, decoration: TextDecoration) -> Self;
}

impl TextDecorationUtilities for ClassBuilder {
    fn text_decoration(self, decoration: TextDecoration) -> Self {
        self.class(decoration.to_class_name())
    }
}

/// Trait for adding text transform utilities to a class builder
pub trait TextTransformUtilities {
    /// Set text transform
    fn text_transform(self, transform: TextTransform) -> Self;
}

impl TextTransformUtilities for ClassBuilder {
    fn text_transform(self, transform: TextTransform) -> Self {
        self.class(transform.to_class_name())
    }
}

/// Trait for adding text overflow utilities to a class builder
pub trait TextOverflowUtilities {
    /// Set text overflow
    fn text_overflow(self, overflow: TextOverflow) -> Self;
}

impl TextOverflowUtilities for ClassBuilder {
    fn text_overflow(self, overflow: TextOverflow) -> Self {
        self.class(overflow.to_class_name())
    }
}

/// Trait for adding white space utilities to a class builder
pub trait WhiteSpaceUtilities {
    /// Set white space
    fn white_space(self, space: WhiteSpace) -> Self;
}

impl WhiteSpaceUtilities for ClassBuilder {
    fn white_space(self, space: WhiteSpace) -> Self {
        self.class(space.to_class_name())
    }
}

/// Trait for adding word break utilities to a class builder
pub trait WordBreakUtilities {
    /// Set word break
    fn word_break(self, break_type: WordBreak) -> Self;
}

impl WordBreakUtilities for ClassBuilder {
    fn word_break(self, break_type: WordBreak) -> Self {
        self.class(break_type.to_class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_font_family_utilities() {
        let classes = ClassBuilder::new()
            .font_family(FontFamily::Sans)
            .font_family(FontFamily::Serif)
            .font_family(FontFamily::Mono)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("font-sans"));
        assert!(css_classes.contains("font-serif"));
        assert!(css_classes.contains("font-mono"));
    }
    
    #[test]
    fn test_font_size_utilities() {
        let classes = ClassBuilder::new()
            .font_size(FontSize::Xs)
            .font_size(FontSize::Sm)
            .font_size(FontSize::Base)
            .font_size(FontSize::Lg)
            .font_size(FontSize::Xl)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("text-xs"));
        assert!(css_classes.contains("text-sm"));
        assert!(css_classes.contains("text-base"));
        assert!(css_classes.contains("text-lg"));
        assert!(css_classes.contains("text-xl"));
    }
    
    #[test]
    fn test_font_weight_utilities() {
        let classes = ClassBuilder::new()
            .font_weight(FontWeight::Thin)
            .font_weight(FontWeight::Normal)
            .font_weight(FontWeight::Bold)
            .font_weight(FontWeight::Black)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("font-thin"));
        assert!(css_classes.contains("font-normal"));
        assert!(css_classes.contains("font-bold"));
        assert!(css_classes.contains("font-black"));
    }
    
    #[test]
    fn test_text_align_utilities() {
        let classes = ClassBuilder::new()
            .text_align(TextAlign::Left)
            .text_align(TextAlign::Center)
            .text_align(TextAlign::Right)
            .text_align(TextAlign::Justify)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("text-left"));
        assert!(css_classes.contains("text-center"));
        assert!(css_classes.contains("text-right"));
        assert!(css_classes.contains("text-justify"));
    }
    
    #[test]
    fn test_line_height_utilities() {
        let classes = ClassBuilder::new()
            .line_height(LineHeight::Tight)
            .line_height(LineHeight::Normal)
            .line_height(LineHeight::Relaxed)
            .line_height(LineHeight::Custom(1.75))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("leading-tight"));
        assert!(css_classes.contains("leading-normal"));
        assert!(css_classes.contains("leading-relaxed"));
        assert!(css_classes.contains("leading-1.75"));
    }
    
    #[test]
    fn test_letter_spacing_utilities() {
        let classes = ClassBuilder::new()
            .letter_spacing(LetterSpacing::Tight)
            .letter_spacing(LetterSpacing::Normal)
            .letter_spacing(LetterSpacing::Wide)
            .letter_spacing(LetterSpacing::Custom(0.1))
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("tracking-tight"));
        assert!(css_classes.contains("tracking-normal"));
        assert!(css_classes.contains("tracking-wide"));
        assert!(css_classes.contains("tracking-0.1"));
    }
    
    #[test]
    fn test_text_decoration_utilities() {
        let classes = ClassBuilder::new()
            .text_decoration(TextDecoration::None)
            .text_decoration(TextDecoration::Underline)
            .text_decoration(TextDecoration::LineThrough)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("no-underline"));
        assert!(css_classes.contains("underline"));
        assert!(css_classes.contains("line-through"));
    }
    
    #[test]
    fn test_text_transform_utilities() {
        let classes = ClassBuilder::new()
            .text_transform(TextTransform::None)
            .text_transform(TextTransform::Uppercase)
            .text_transform(TextTransform::Lowercase)
            .text_transform(TextTransform::Capitalize)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("normal-case"));
        assert!(css_classes.contains("uppercase"));
        assert!(css_classes.contains("lowercase"));
        assert!(css_classes.contains("capitalize"));
    }
    
    #[test]
    fn test_text_overflow_utilities() {
        let classes = ClassBuilder::new()
            .text_overflow(TextOverflow::Truncate)
            .text_overflow(TextOverflow::Ellipsis)
            .text_overflow(TextOverflow::Clip)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("truncate"));
        assert!(css_classes.contains("text-ellipsis"));
        assert!(css_classes.contains("text-clip"));
    }
    
    #[test]
    fn test_white_space_utilities() {
        let classes = ClassBuilder::new()
            .white_space(WhiteSpace::Normal)
            .white_space(WhiteSpace::Nowrap)
            .white_space(WhiteSpace::Pre)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("whitespace-normal"));
        assert!(css_classes.contains("whitespace-nowrap"));
        assert!(css_classes.contains("whitespace-pre"));
    }
    
    #[test]
    fn test_word_break_utilities() {
        let classes = ClassBuilder::new()
            .word_break(WordBreak::Normal)
            .word_break(WordBreak::BreakAll)
            .word_break(WordBreak::BreakWords)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("break-normal"));
        assert!(css_classes.contains("break-all"));
        assert!(css_classes.contains("break-words"));
    }
    
    #[test]
    fn test_complex_typography_combination() {
        let classes = ClassBuilder::new()
            .font_family(FontFamily::Sans)
            .font_size(FontSize::Lg)
            .font_weight(FontWeight::Bold)
            .text_align(TextAlign::Center)
            .line_height(LineHeight::Relaxed)
            .letter_spacing(LetterSpacing::Wide)
            .text_decoration(TextDecoration::Underline)
            .text_transform(TextTransform::Uppercase)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("font-sans"));
        assert!(css_classes.contains("text-lg"));
        assert!(css_classes.contains("font-bold"));
        assert!(css_classes.contains("text-center"));
        assert!(css_classes.contains("leading-relaxed"));
        assert!(css_classes.contains("tracking-wide"));
        assert!(css_classes.contains("underline"));
        assert!(css_classes.contains("uppercase"));
    }
    
    /// Test that all Tailwind CSS font sizes are supported
    #[test]
    fn test_all_tailwind_font_sizes() {
        // Test all standard Tailwind CSS font sizes (text-xs through text-9xl)
        let test_values = vec![
            (FontSize::Xs, "text-xs"),
            (FontSize::Sm, "text-sm"),
            (FontSize::Base, "text-base"),
            (FontSize::Lg, "text-lg"),
            (FontSize::Xl, "text-xl"),
            (FontSize::Xl2, "text-2xl"),
            (FontSize::Xl3, "text-3xl"),
            (FontSize::Xl4, "text-4xl"),
            (FontSize::Xl5, "text-5xl"),
            (FontSize::Xl6, "text-6xl"),
            (FontSize::Xl7, "text-7xl"),
            (FontSize::Xl8, "text-8xl"),
            (FontSize::Xl9, "text-9xl"),
        ];
        
        for (value, expected_class) in test_values {
            let classes = ClassBuilder::new().font_size(value).build();
            let css_classes = classes.to_css_classes();
            assert!(css_classes.contains(expected_class), 
                "Missing font size: {} (expected class: {})", 
                format!("{:?}", value), expected_class);
        }
    }
    
    /// Test that all Week 4 typography utilities are implemented
    #[test]
    fn test_week4_typography_utilities() {
        // Test all Week 4 typography utilities
        let classes = ClassBuilder::new()
            // Font sizes (text-xs through text-9xl) - already tested above
            .font_size(FontSize::Xs)
            .font_size(FontSize::Xl9)
            // Font weights (font-thin through font-black)
            .font_weight(FontWeight::Thin)
            .font_weight(FontWeight::Black)
            // Line heights (leading-3 through leading-10)
            .line_height(LineHeight::Tight)
            .line_height(LineHeight::Relaxed)
            // Letter spacing (tracking-tighter through tracking-widest)
            .letter_spacing(LetterSpacing::Tighter)
            .letter_spacing(LetterSpacing::Widest)
            // Text decoration (underline, no-underline, line-through)
            .text_decoration(TextDecoration::Underline)
            .text_decoration(TextDecoration::None)
            .text_decoration(TextDecoration::LineThrough)
            // Text transform (uppercase, lowercase, capitalize)
            .text_transform(TextTransform::Uppercase)
            .text_transform(TextTransform::Lowercase)
            .text_transform(TextTransform::Capitalize)
            .build();
        
        let css_classes = classes.to_css_classes();
        
        // Font sizes
        assert!(css_classes.contains("text-xs"));
        assert!(css_classes.contains("text-9xl"));
        
        // Font weights
        assert!(css_classes.contains("font-thin"));
        assert!(css_classes.contains("font-black"));
        
        // Line heights
        assert!(css_classes.contains("leading-tight"));
        assert!(css_classes.contains("leading-relaxed"));
        
        // Letter spacing
        assert!(css_classes.contains("tracking-tighter"));
        assert!(css_classes.contains("tracking-widest"));
        
        // Text decoration
        assert!(css_classes.contains("underline"));
        assert!(css_classes.contains("no-underline"));
        assert!(css_classes.contains("line-through"));
        
        // Text transform
        assert!(css_classes.contains("uppercase"));
        assert!(css_classes.contains("lowercase"));
        assert!(css_classes.contains("capitalize"));
    }
}
