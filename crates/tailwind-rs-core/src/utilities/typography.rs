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

/// Text decoration style values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextDecorationStyle {
    /// Solid decoration style
    Solid,
    /// Double decoration style
    Double,
    /// Dotted decoration style
    Dotted,
    /// Dashed decoration style
    Dashed,
    /// Wavy decoration style
    Wavy,
}

/// Text decoration thickness values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextDecorationThickness {
    /// Auto thickness
    Auto,
    /// From font thickness
    FromFont,
    /// 0px thickness
    Zero,
    /// 1px thickness
    One,
    /// 2px thickness
    Two,
    /// 4px thickness
    Four,
    /// 8px thickness
    Eight,
}

/// Text underline offset values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextUnderlineOffset {
    /// Auto offset
    Auto,
    /// 0px offset
    Zero,
    /// 1px offset
    One,
    /// 2px offset
    Two,
    /// 4px offset
    Four,
    /// 8px offset
    Eight,
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

/// Overflow wrap values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OverflowWrap {
    /// Normal overflow wrap
    Normal,
    /// Break-word overflow wrap
    BreakWord,
    /// Anywhere overflow wrap
    Anywhere,
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
            LineHeight::Custom(f) => {
                14u8.hash(state);
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
            LineHeight::Custom(f) => format!("{}", f),
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
            LineHeight::Custom(f) => f.to_string(),
        }
    }

    /// Get all available line height values
    pub fn all_values() -> Vec<LineHeight> {
        vec![
            LineHeight::None,
            LineHeight::Three,
            LineHeight::Four,
            LineHeight::Five,
            LineHeight::Six,
            LineHeight::Seven,
            LineHeight::Eight,
            LineHeight::Nine,
            LineHeight::Ten,
            LineHeight::Tight,
            LineHeight::Snug,
            LineHeight::Normal,
            LineHeight::Relaxed,
            LineHeight::Loose,
        ]
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

impl TextDecorationStyle {
    pub fn to_class_name(&self) -> String {
        match self {
            TextDecorationStyle::Solid => "decoration-solid".to_string(),
            TextDecorationStyle::Double => "decoration-double".to_string(),
            TextDecorationStyle::Dotted => "decoration-dotted".to_string(),
            TextDecorationStyle::Dashed => "decoration-dashed".to_string(),
            TextDecorationStyle::Wavy => "decoration-wavy".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            TextDecorationStyle::Solid => "solid".to_string(),
            TextDecorationStyle::Double => "double".to_string(),
            TextDecorationStyle::Dotted => "dotted".to_string(),
            TextDecorationStyle::Dashed => "dashed".to_string(),
            TextDecorationStyle::Wavy => "wavy".to_string(),
        }
    }
}

impl TextDecorationThickness {
    pub fn to_class_name(&self) -> String {
        match self {
            TextDecorationThickness::Auto => "decoration-auto".to_string(),
            TextDecorationThickness::FromFont => "decoration-from-font".to_string(),
            TextDecorationThickness::Zero => "decoration-0".to_string(),
            TextDecorationThickness::One => "decoration-1".to_string(),
            TextDecorationThickness::Two => "decoration-2".to_string(),
            TextDecorationThickness::Four => "decoration-4".to_string(),
            TextDecorationThickness::Eight => "decoration-8".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            TextDecorationThickness::Auto => "auto".to_string(),
            TextDecorationThickness::FromFont => "from-font".to_string(),
            TextDecorationThickness::Zero => "0px".to_string(),
            TextDecorationThickness::One => "1px".to_string(),
            TextDecorationThickness::Two => "2px".to_string(),
            TextDecorationThickness::Four => "4px".to_string(),
            TextDecorationThickness::Eight => "8px".to_string(),
        }
    }
}

impl TextUnderlineOffset {
    pub fn to_class_name(&self) -> String {
        match self {
            TextUnderlineOffset::Auto => "underline-offset-auto".to_string(),
            TextUnderlineOffset::Zero => "underline-offset-0".to_string(),
            TextUnderlineOffset::One => "underline-offset-1".to_string(),
            TextUnderlineOffset::Two => "underline-offset-2".to_string(),
            TextUnderlineOffset::Four => "underline-offset-4".to_string(),
            TextUnderlineOffset::Eight => "underline-offset-8".to_string(),
        }
    }

    pub fn to_css_value(&self) -> String {
        match self {
            TextUnderlineOffset::Auto => "auto".to_string(),
            TextUnderlineOffset::Zero => "0px".to_string(),
            TextUnderlineOffset::One => "1px".to_string(),
            TextUnderlineOffset::Two => "2px".to_string(),
            TextUnderlineOffset::Four => "4px".to_string(),
            TextUnderlineOffset::Eight => "8px".to_string(),
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

impl OverflowWrap {
    pub fn to_class_name(&self) -> String {
        match self {
            OverflowWrap::Normal => "overflow-wrap-normal".to_string(),
            OverflowWrap::BreakWord => "overflow-wrap-break".to_string(),
            OverflowWrap::Anywhere => "overflow-wrap-anywhere".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            OverflowWrap::Normal => "normal".to_string(),
            OverflowWrap::BreakWord => "break-word".to_string(),
            OverflowWrap::Anywhere => "anywhere".to_string(),
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

impl fmt::Display for TextDecoration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TextDecorationStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TextDecorationThickness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TextUnderlineOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for TextTransform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for OverflowWrap {
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

/// Trait for adding overflow wrap utilities to a class builder
pub trait OverflowWrapUtilities {
    /// Set overflow wrap
    fn overflow_wrap(self, wrap_type: OverflowWrap) -> Self;
}

impl OverflowWrapUtilities for ClassBuilder {
    fn overflow_wrap(self, wrap_type: OverflowWrap) -> Self {
        self.class(wrap_type.to_class_name())
    }
}

/// Trait for adding text decoration style utilities to a class builder
pub trait TextDecorationStyleUtilities {
    /// Set text decoration style
    fn text_decoration_style(self, style: TextDecorationStyle) -> Self;
}

impl TextDecorationStyleUtilities for ClassBuilder {
    fn text_decoration_style(self, style: TextDecorationStyle) -> Self {
        self.class(style.to_class_name())
    }
}

/// Trait for adding text decoration thickness utilities to a class builder
pub trait TextDecorationThicknessUtilities {
    /// Set text decoration thickness
    fn text_decoration_thickness(self, thickness: TextDecorationThickness) -> Self;
}

impl TextDecorationThicknessUtilities for ClassBuilder {
    fn text_decoration_thickness(self, thickness: TextDecorationThickness) -> Self {
        self.class(thickness.to_class_name())
    }
}

/// Trait for adding text underline offset utilities to a class builder
pub trait TextUnderlineOffsetUtilities {
    /// Set text underline offset
    fn text_underline_offset(self, offset: TextUnderlineOffset) -> Self;
}

impl TextUnderlineOffsetUtilities for ClassBuilder {
    fn text_underline_offset(self, offset: TextUnderlineOffset) -> Self {
        self.class(offset.to_class_name())
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

    /// Test extended numeric line height utilities (leading-3 through leading-10)
    #[test]
    fn test_extended_line_height_utilities() {
        let classes = ClassBuilder::new()
            .line_height(LineHeight::Three)
            .line_height(LineHeight::Four)
            .line_height(LineHeight::Five)
            .line_height(LineHeight::Six)
            .line_height(LineHeight::Seven)
            .line_height(LineHeight::Eight)
            .line_height(LineHeight::Nine)
            .line_height(LineHeight::Ten)
            .build();

        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("leading-3"));
        assert!(css_classes.contains("leading-4"));
        assert!(css_classes.contains("leading-5"));
        assert!(css_classes.contains("leading-6"));
        assert!(css_classes.contains("leading-7"));
        assert!(css_classes.contains("leading-8"));
        assert!(css_classes.contains("leading-9"));
        assert!(css_classes.contains("leading-10"));
    }

    /// Test extended text decoration utilities
    #[test]
    fn test_extended_text_decoration_utilities() {
        let classes = ClassBuilder::new()
            .text_decoration_style(TextDecorationStyle::Solid)
            .text_decoration_style(TextDecorationStyle::Double)
            .text_decoration_style(TextDecorationStyle::Dotted)
            .text_decoration_style(TextDecorationStyle::Dashed)
            .text_decoration_style(TextDecorationStyle::Wavy)
            .text_decoration_thickness(TextDecorationThickness::Auto)
            .text_decoration_thickness(TextDecorationThickness::FromFont)
            .text_decoration_thickness(TextDecorationThickness::One)
            .text_decoration_thickness(TextDecorationThickness::Two)
            .text_decoration_thickness(TextDecorationThickness::Four)
            .text_underline_offset(TextUnderlineOffset::Auto)
            .text_underline_offset(TextUnderlineOffset::Zero)
            .text_underline_offset(TextUnderlineOffset::One)
            .text_underline_offset(TextUnderlineOffset::Two)
            .text_underline_offset(TextUnderlineOffset::Four)
            .build();

        let css_classes = classes.to_css_classes();

        // Text decoration styles
        assert!(css_classes.contains("decoration-solid"));
        assert!(css_classes.contains("decoration-double"));
        assert!(css_classes.contains("decoration-dotted"));
        assert!(css_classes.contains("decoration-dashed"));
        assert!(css_classes.contains("decoration-wavy"));

        // Text decoration thickness
        assert!(css_classes.contains("decoration-auto"));
        assert!(css_classes.contains("decoration-from-font"));
        assert!(css_classes.contains("decoration-1"));
        assert!(css_classes.contains("decoration-2"));
        assert!(css_classes.contains("decoration-4"));

        // Text underline offset
        assert!(css_classes.contains("underline-offset-auto"));
        assert!(css_classes.contains("underline-offset-0"));
        assert!(css_classes.contains("underline-offset-1"));
        assert!(css_classes.contains("underline-offset-2"));
        assert!(css_classes.contains("underline-offset-4"));
    }

    /// Test comprehensive extended typography features
    #[test]
    fn test_comprehensive_extended_typography() {
        let classes = ClassBuilder::new()
            // Extended line heights
            .line_height(LineHeight::Three)
            .line_height(LineHeight::Ten)
            // Extended text decorations
            .text_decoration(TextDecoration::Underline)
            .text_decoration_style(TextDecorationStyle::Wavy)
            .text_decoration_thickness(TextDecorationThickness::Two)
            .text_underline_offset(TextUnderlineOffset::Four)
            // Existing features
            .font_size(FontSize::Xl2)
            .font_weight(FontWeight::Bold)
            .text_transform(TextTransform::Uppercase)
            .letter_spacing(LetterSpacing::Wide)
            .build();

        let css_classes = classes.to_css_classes();

        // Extended line heights
        assert!(css_classes.contains("leading-3"));
        assert!(css_classes.contains("leading-10"));

        // Extended text decorations
        assert!(css_classes.contains("underline"));
        assert!(css_classes.contains("decoration-wavy"));
        assert!(css_classes.contains("decoration-2"));
        assert!(css_classes.contains("underline-offset-4"));

        // Existing features still work
        assert!(css_classes.contains("text-2xl"));
        assert!(css_classes.contains("font-bold"));
        assert!(css_classes.contains("uppercase"));
        assert!(css_classes.contains("tracking-wide"));
    }

    /// Test LineHeight::all_values utility function
    #[test]
    fn test_line_height_all_values() {
        let all_values = LineHeight::all_values();
        assert_eq!(all_values.len(), 14); // 8 numeric + 6 named values

        // Test that all values are present
        assert!(all_values.contains(&LineHeight::None));
        assert!(all_values.contains(&LineHeight::Three));
        assert!(all_values.contains(&LineHeight::Four));
        assert!(all_values.contains(&LineHeight::Five));
        assert!(all_values.contains(&LineHeight::Six));
        assert!(all_values.contains(&LineHeight::Seven));
        assert!(all_values.contains(&LineHeight::Eight));
        assert!(all_values.contains(&LineHeight::Nine));
        assert!(all_values.contains(&LineHeight::Ten));
        assert!(all_values.contains(&LineHeight::Tight));
        assert!(all_values.contains(&LineHeight::Snug));
        assert!(all_values.contains(&LineHeight::Normal));
        assert!(all_values.contains(&LineHeight::Relaxed));
        assert!(all_values.contains(&LineHeight::Loose));
    }

    #[test]
    fn test_font_family_display() {
        // Test that FontFamily displays correctly
        assert_eq!(format!("{}", FontFamily::Sans), "sans");
        assert_eq!(format!("{}", FontFamily::Serif), "serif");
        assert_eq!(format!("{}", FontFamily::Mono), "mono");
    }

    #[test]
    fn test_font_size_display() {
        // Test that FontSize displays correctly
        assert_eq!(format!("{}", FontSize::Xs), "xs");
        assert_eq!(format!("{}", FontSize::Sm), "sm");
        assert_eq!(format!("{}", FontSize::Base), "base");
        assert_eq!(format!("{}", FontSize::Lg), "lg");
        assert_eq!(format!("{}", FontSize::Xl), "xl");
        assert_eq!(format!("{}", FontSize::Xl2), "2xl");
        assert_eq!(format!("{}", FontSize::Xl3), "3xl");
        assert_eq!(format!("{}", FontSize::Xl4), "4xl");
        assert_eq!(format!("{}", FontSize::Xl5), "5xl");
        assert_eq!(format!("{}", FontSize::Xl6), "6xl");
        assert_eq!(format!("{}", FontSize::Xl7), "7xl");
        assert_eq!(format!("{}", FontSize::Xl8), "8xl");
        assert_eq!(format!("{}", FontSize::Xl9), "9xl");
    }

    #[test]
    fn test_font_weight_display() {
        // Test that FontWeight displays correctly
        assert_eq!(format!("{}", FontWeight::Thin), "thin");
        assert_eq!(format!("{}", FontWeight::ExtraLight), "extralight");
        assert_eq!(format!("{}", FontWeight::Light), "light");
        assert_eq!(format!("{}", FontWeight::Normal), "normal");
        assert_eq!(format!("{}", FontWeight::Medium), "medium");
        assert_eq!(format!("{}", FontWeight::SemiBold), "semibold");
        assert_eq!(format!("{}", FontWeight::Bold), "bold");
        assert_eq!(format!("{}", FontWeight::ExtraBold), "extrabold");
        assert_eq!(format!("{}", FontWeight::Black), "black");
    }

    #[test]
    fn test_text_align_display() {
        // Test that TextAlign displays correctly
        assert_eq!(format!("{}", TextAlign::Left), "left");
        assert_eq!(format!("{}", TextAlign::Center), "center");
        assert_eq!(format!("{}", TextAlign::Right), "right");
        assert_eq!(format!("{}", TextAlign::Justify), "justify");
        assert_eq!(format!("{}", TextAlign::Start), "start");
        assert_eq!(format!("{}", TextAlign::End), "end");
    }

    #[test]
    fn test_line_height_display() {
        // Test that LineHeight displays correctly
        assert_eq!(format!("{}", LineHeight::None), "none");
        assert_eq!(format!("{}", LineHeight::Three), "3");
        assert_eq!(format!("{}", LineHeight::Four), "4");
        assert_eq!(format!("{}", LineHeight::Five), "5");
        assert_eq!(format!("{}", LineHeight::Six), "6");
        assert_eq!(format!("{}", LineHeight::Seven), "7");
        assert_eq!(format!("{}", LineHeight::Eight), "8");
        assert_eq!(format!("{}", LineHeight::Nine), "9");
        assert_eq!(format!("{}", LineHeight::Ten), "10");
        assert_eq!(format!("{}", LineHeight::Tight), "tight");
        assert_eq!(format!("{}", LineHeight::Snug), "snug");
        assert_eq!(format!("{}", LineHeight::Normal), "normal");
        assert_eq!(format!("{}", LineHeight::Relaxed), "relaxed");
        assert_eq!(format!("{}", LineHeight::Loose), "loose");
        assert_eq!(format!("{}", LineHeight::Custom(1.5)), "1.5");
    }

    #[test]
    fn test_letter_spacing_display() {
        // Test that LetterSpacing displays correctly
        assert_eq!(format!("{}", LetterSpacing::Tighter), "tighter");
        assert_eq!(format!("{}", LetterSpacing::Tight), "tight");
        assert_eq!(format!("{}", LetterSpacing::Normal), "normal");
        assert_eq!(format!("{}", LetterSpacing::Wide), "wide");
        assert_eq!(format!("{}", LetterSpacing::Wider), "wider");
        assert_eq!(format!("{}", LetterSpacing::Widest), "widest");
        assert_eq!(format!("{}", LetterSpacing::Custom(0.1)), "0.1");
    }

    #[test]
    fn test_text_decoration_display() {
        // Test that TextDecoration displays correctly
        assert_eq!(format!("{}", TextDecoration::None), "no-underline");
        assert_eq!(format!("{}", TextDecoration::Underline), "underline");
        assert_eq!(format!("{}", TextDecoration::Overline), "overline");
        assert_eq!(format!("{}", TextDecoration::LineThrough), "line-through");
    }

    #[test]
    fn test_text_transform_display() {
        // Test that TextTransform displays correctly
        assert_eq!(format!("{}", TextTransform::None), "normal-case");
        assert_eq!(format!("{}", TextTransform::Uppercase), "uppercase");
        assert_eq!(format!("{}", TextTransform::Lowercase), "lowercase");
        assert_eq!(format!("{}", TextTransform::Capitalize), "capitalize");
    }

    #[test]
    fn test_text_overflow_class_names() {
        // Test that TextOverflow generates correct class names
        assert_eq!(TextOverflow::Truncate.to_class_name(), "truncate");
        assert_eq!(TextOverflow::Ellipsis.to_class_name(), "text-ellipsis");
        assert_eq!(TextOverflow::Clip.to_class_name(), "text-clip");
    }

    #[test]
    fn test_white_space_class_names() {
        // Test that WhiteSpace generates correct class names
        assert_eq!(WhiteSpace::Normal.to_class_name(), "whitespace-normal");
        assert_eq!(WhiteSpace::Nowrap.to_class_name(), "whitespace-nowrap");
        assert_eq!(WhiteSpace::Pre.to_class_name(), "whitespace-pre");
        assert_eq!(WhiteSpace::PreLine.to_class_name(), "whitespace-pre-line");
        assert_eq!(WhiteSpace::PreWrap.to_class_name(), "whitespace-pre-wrap");
        assert_eq!(WhiteSpace::BreakSpaces.to_class_name(), "whitespace-break-spaces");
    }

    #[test]
    fn test_word_break_class_names() {
        // Test that WordBreak generates correct class names
        assert_eq!(WordBreak::Normal.to_class_name(), "break-normal");
        assert_eq!(WordBreak::BreakAll.to_class_name(), "break-all");
        assert_eq!(WordBreak::BreakWords.to_class_name(), "break-words");
        assert_eq!(WordBreak::KeepAll.to_class_name(), "break-keep");
    }

    #[test]
    fn test_font_family_css_values() {
        // Test that FontFamily generates correct CSS values
        assert_eq!(FontFamily::Sans.to_css_value(), "ui-sans-serif, system-ui, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color Emoji\"");
        assert_eq!(FontFamily::Serif.to_css_value(), "ui-serif, Georgia, Cambria, \"Times New Roman\", Times, serif");
        assert_eq!(FontFamily::Mono.to_css_value(), "ui-monospace, SFMono-Regular, \"SF Mono\", Consolas, \"Liberation Mono\", Menlo, monospace");
    }

    #[test]
    fn test_font_size_css_values() {
        // Test that FontSize generates correct CSS values
        assert_eq!(FontSize::Xs.to_css_value(), "0.75rem");
        assert_eq!(FontSize::Sm.to_css_value(), "0.875rem");
        assert_eq!(FontSize::Base.to_css_value(), "1rem");
        assert_eq!(FontSize::Lg.to_css_value(), "1.125rem");
        assert_eq!(FontSize::Xl.to_css_value(), "1.25rem");
        assert_eq!(FontSize::Xl2.to_css_value(), "1.5rem");
        assert_eq!(FontSize::Xl3.to_css_value(), "1.875rem");
        assert_eq!(FontSize::Xl4.to_css_value(), "2.25rem");
        assert_eq!(FontSize::Xl5.to_css_value(), "3rem");
        assert_eq!(FontSize::Xl6.to_css_value(), "3.75rem");
        assert_eq!(FontSize::Xl7.to_css_value(), "4.5rem");
        assert_eq!(FontSize::Xl8.to_css_value(), "6rem");
        assert_eq!(FontSize::Xl9.to_css_value(), "8rem");
    }

    #[test]
    fn test_font_weight_css_values() {
        // Test that FontWeight generates correct CSS values
        assert_eq!(FontWeight::Thin.to_css_value(), "100");
        assert_eq!(FontWeight::ExtraLight.to_css_value(), "200");
        assert_eq!(FontWeight::Light.to_css_value(), "300");
        assert_eq!(FontWeight::Normal.to_css_value(), "400");
        assert_eq!(FontWeight::Medium.to_css_value(), "500");
        assert_eq!(FontWeight::SemiBold.to_css_value(), "600");
        assert_eq!(FontWeight::Bold.to_css_value(), "700");
        assert_eq!(FontWeight::ExtraBold.to_css_value(), "800");
        assert_eq!(FontWeight::Black.to_css_value(), "900");
    }

    #[test]
    fn test_text_align_css_values() {
        // Test that TextAlign generates correct CSS values
        assert_eq!(TextAlign::Left.to_css_value(), "left");
        assert_eq!(TextAlign::Center.to_css_value(), "center");
        assert_eq!(TextAlign::Right.to_css_value(), "right");
        assert_eq!(TextAlign::Justify.to_css_value(), "justify");
        assert_eq!(TextAlign::Start.to_css_value(), "start");
        assert_eq!(TextAlign::End.to_css_value(), "end");
    }

    #[test]
    fn test_line_height_css_values() {
        // Test that LineHeight generates correct CSS values
        assert_eq!(LineHeight::None.to_css_value(), "1");
        assert_eq!(LineHeight::Three.to_css_value(), "0.75rem");
        assert_eq!(LineHeight::Four.to_css_value(), "1rem");
        assert_eq!(LineHeight::Five.to_css_value(), "1.25rem");
        assert_eq!(LineHeight::Six.to_css_value(), "1.5rem");
        assert_eq!(LineHeight::Seven.to_css_value(), "1.75rem");
        assert_eq!(LineHeight::Eight.to_css_value(), "2rem");
        assert_eq!(LineHeight::Nine.to_css_value(), "2.25rem");
        assert_eq!(LineHeight::Ten.to_css_value(), "2.5rem");
        assert_eq!(LineHeight::Tight.to_css_value(), "1.25");
        assert_eq!(LineHeight::Snug.to_css_value(), "1.375");
        assert_eq!(LineHeight::Normal.to_css_value(), "1.5");
        assert_eq!(LineHeight::Relaxed.to_css_value(), "1.625");
        assert_eq!(LineHeight::Loose.to_css_value(), "2");
        assert_eq!(LineHeight::Custom(1.5).to_css_value(), "1.5");
    }

    #[test]
    fn test_letter_spacing_css_values() {
        // Test that LetterSpacing generates correct CSS values
        assert_eq!(LetterSpacing::Tighter.to_css_value(), "-0.05em");
        assert_eq!(LetterSpacing::Tight.to_css_value(), "-0.025em");
        assert_eq!(LetterSpacing::Normal.to_css_value(), "0em");
        assert_eq!(LetterSpacing::Wide.to_css_value(), "0.025em");
        assert_eq!(LetterSpacing::Wider.to_css_value(), "0.05em");
        assert_eq!(LetterSpacing::Widest.to_css_value(), "0.1em");
        assert_eq!(LetterSpacing::Custom(0.1).to_css_value(), "0.1em");
    }

    #[test]
    fn test_typography_serialization() {
        // Test that typography enums can be serialized and deserialized
        let font_family = FontFamily::Sans;
        let serialized = serde_json::to_string(&font_family).unwrap();
        let deserialized: FontFamily = serde_json::from_str(&serialized).unwrap();
        assert_eq!(font_family, deserialized);

        let font_size = FontSize::Lg;
        let serialized = serde_json::to_string(&font_size).unwrap();
        let deserialized: FontSize = serde_json::from_str(&serialized).unwrap();
        assert_eq!(font_size, deserialized);

        let font_weight = FontWeight::Bold;
        let serialized = serde_json::to_string(&font_weight).unwrap();
        let deserialized: FontWeight = serde_json::from_str(&serialized).unwrap();
        assert_eq!(font_weight, deserialized);

        let text_align = TextAlign::Center;
        let serialized = serde_json::to_string(&text_align).unwrap();
        let deserialized: TextAlign = serde_json::from_str(&serialized).unwrap();
        assert_eq!(text_align, deserialized);

        let line_height = LineHeight::Relaxed;
        let serialized = serde_json::to_string(&line_height).unwrap();
        let deserialized: LineHeight = serde_json::from_str(&serialized).unwrap();
        assert_eq!(line_height, deserialized);

        let letter_spacing = LetterSpacing::Wide;
        let serialized = serde_json::to_string(&letter_spacing).unwrap();
        let deserialized: LetterSpacing = serde_json::from_str(&serialized).unwrap();
        assert_eq!(letter_spacing, deserialized);
    }

    #[test]
    fn test_typography_equality_and_hash() {
        // Test that typography enums can be compared for equality and hashed
        let font_family1 = FontFamily::Sans;
        let font_family2 = FontFamily::Sans;
        let font_family3 = FontFamily::Serif;
        
        assert_eq!(font_family1, font_family2);
        assert_ne!(font_family1, font_family3);
        
        // Test that equal enums have the same hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        font_family1.hash(&mut hasher1);
        font_family2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_comprehensive_typography_utilities() {
        // Test comprehensive usage of all typography utility methods
        let classes = ClassBuilder::new()
            // Font families
            .font_family(FontFamily::Sans)
            .font_family(FontFamily::Serif)
            .font_family(FontFamily::Mono)
            
            // Font sizes
            .font_size(FontSize::Xs)
            .font_size(FontSize::Sm)
            .font_size(FontSize::Base)
            .font_size(FontSize::Lg)
            .font_size(FontSize::Xl)
            .font_size(FontSize::Xl2)
            .font_size(FontSize::Xl3)
            .font_size(FontSize::Xl4)
            .font_size(FontSize::Xl5)
            .font_size(FontSize::Xl6)
            .font_size(FontSize::Xl7)
            .font_size(FontSize::Xl8)
            .font_size(FontSize::Xl9)
            
            // Font weights
            .font_weight(FontWeight::Thin)
            .font_weight(FontWeight::ExtraLight)
            .font_weight(FontWeight::Light)
            .font_weight(FontWeight::Normal)
            .font_weight(FontWeight::Medium)
            .font_weight(FontWeight::SemiBold)
            .font_weight(FontWeight::Bold)
            .font_weight(FontWeight::ExtraBold)
            .font_weight(FontWeight::Black)
            
            // Text alignment
            .text_align(TextAlign::Left)
            .text_align(TextAlign::Center)
            .text_align(TextAlign::Right)
            .text_align(TextAlign::Justify)
            .text_align(TextAlign::Start)
            .text_align(TextAlign::End)
            
            // Line heights
            .line_height(LineHeight::None)
            .line_height(LineHeight::Three)
            .line_height(LineHeight::Four)
            .line_height(LineHeight::Five)
            .line_height(LineHeight::Six)
            .line_height(LineHeight::Seven)
            .line_height(LineHeight::Eight)
            .line_height(LineHeight::Nine)
            .line_height(LineHeight::Ten)
            .line_height(LineHeight::Tight)
            .line_height(LineHeight::Snug)
            .line_height(LineHeight::Normal)
            .line_height(LineHeight::Relaxed)
            .line_height(LineHeight::Loose)
            .line_height(LineHeight::Custom(1.5))
            
            // Letter spacing
            .letter_spacing(LetterSpacing::Tighter)
            .letter_spacing(LetterSpacing::Tight)
            .letter_spacing(LetterSpacing::Normal)
            .letter_spacing(LetterSpacing::Wide)
            .letter_spacing(LetterSpacing::Wider)
            .letter_spacing(LetterSpacing::Widest)
            .letter_spacing(LetterSpacing::Custom(0.1))
            
            // Text decoration
            .text_decoration(TextDecoration::None)
            .text_decoration(TextDecoration::Underline)
            .text_decoration(TextDecoration::Overline)
            .text_decoration(TextDecoration::LineThrough)
            
            // Text transform
            .text_transform(TextTransform::None)
            .text_transform(TextTransform::Uppercase)
            .text_transform(TextTransform::Lowercase)
            .text_transform(TextTransform::Capitalize)
            
            // Text overflow
            .text_overflow(TextOverflow::Truncate)
            .text_overflow(TextOverflow::Ellipsis)
            .text_overflow(TextOverflow::Clip)
            
            // White space
            .white_space(WhiteSpace::Normal)
            .white_space(WhiteSpace::Nowrap)
            .white_space(WhiteSpace::Pre)
            .white_space(WhiteSpace::PreLine)
            .white_space(WhiteSpace::PreWrap)
            .white_space(WhiteSpace::BreakSpaces)
            
            // Word break
            .word_break(WordBreak::Normal)
            .word_break(WordBreak::BreakAll)
            .word_break(WordBreak::BreakWords)
            .word_break(WordBreak::KeepAll)
            
            // Text decoration style
            .text_decoration_style(TextDecorationStyle::Solid)
            .text_decoration_style(TextDecorationStyle::Double)
            .text_decoration_style(TextDecorationStyle::Dotted)
            .text_decoration_style(TextDecorationStyle::Dashed)
            .text_decoration_style(TextDecorationStyle::Wavy)
            
            // Text decoration thickness
            .text_decoration_thickness(TextDecorationThickness::Auto)
            .text_decoration_thickness(TextDecorationThickness::FromFont)
            .text_decoration_thickness(TextDecorationThickness::Zero)
            .text_decoration_thickness(TextDecorationThickness::One)
            .text_decoration_thickness(TextDecorationThickness::Two)
            .text_decoration_thickness(TextDecorationThickness::Four)
            .text_decoration_thickness(TextDecorationThickness::Eight)
            
            // Text underline offset
            .text_underline_offset(TextUnderlineOffset::Auto)
            .text_underline_offset(TextUnderlineOffset::Zero)
            .text_underline_offset(TextUnderlineOffset::One)
            .text_underline_offset(TextUnderlineOffset::Two)
            .text_underline_offset(TextUnderlineOffset::Four)
            .text_underline_offset(TextUnderlineOffset::Eight)
            .build();
        
        let css_classes = classes.to_css_classes();
        
        // Verify font families
        assert!(css_classes.contains("font-sans"));
        assert!(css_classes.contains("font-serif"));
        assert!(css_classes.contains("font-mono"));
        
        // Verify font sizes
        assert!(css_classes.contains("text-xs"));
        assert!(css_classes.contains("text-sm"));
        assert!(css_classes.contains("text-base"));
        assert!(css_classes.contains("text-lg"));
        assert!(css_classes.contains("text-xl"));
        assert!(css_classes.contains("text-2xl"));
        assert!(css_classes.contains("text-3xl"));
        assert!(css_classes.contains("text-4xl"));
        assert!(css_classes.contains("text-5xl"));
        assert!(css_classes.contains("text-6xl"));
        assert!(css_classes.contains("text-7xl"));
        assert!(css_classes.contains("text-8xl"));
        assert!(css_classes.contains("text-9xl"));
        
        // Verify font weights
        assert!(css_classes.contains("font-thin"));
        assert!(css_classes.contains("font-extralight"));
        assert!(css_classes.contains("font-light"));
        assert!(css_classes.contains("font-normal"));
        assert!(css_classes.contains("font-medium"));
        assert!(css_classes.contains("font-semibold"));
        assert!(css_classes.contains("font-bold"));
        assert!(css_classes.contains("font-extrabold"));
        assert!(css_classes.contains("font-black"));
        
        // Verify text alignment
        assert!(css_classes.contains("text-left"));
        assert!(css_classes.contains("text-center"));
        assert!(css_classes.contains("text-right"));
        assert!(css_classes.contains("text-justify"));
        assert!(css_classes.contains("text-start"));
        assert!(css_classes.contains("text-end"));
        
        // Verify line heights
        assert!(css_classes.contains("leading-none"));
        assert!(css_classes.contains("leading-3"));
        assert!(css_classes.contains("leading-4"));
        assert!(css_classes.contains("leading-5"));
        assert!(css_classes.contains("leading-6"));
        assert!(css_classes.contains("leading-7"));
        assert!(css_classes.contains("leading-8"));
        assert!(css_classes.contains("leading-9"));
        assert!(css_classes.contains("leading-10"));
        assert!(css_classes.contains("leading-tight"));
        assert!(css_classes.contains("leading-snug"));
        assert!(css_classes.contains("leading-normal"));
        assert!(css_classes.contains("leading-relaxed"));
        assert!(css_classes.contains("leading-loose"));
        assert!(css_classes.contains("leading-1.5"));
        
        // Verify letter spacing
        assert!(css_classes.contains("tracking-tighter"));
        assert!(css_classes.contains("tracking-tight"));
        assert!(css_classes.contains("tracking-normal"));
        assert!(css_classes.contains("tracking-wide"));
        assert!(css_classes.contains("tracking-wider"));
        assert!(css_classes.contains("tracking-widest"));
        assert!(css_classes.contains("tracking-0.1"));
        
        // Verify text decoration
        assert!(css_classes.contains("no-underline"));
        assert!(css_classes.contains("underline"));
        assert!(css_classes.contains("overline"));
        assert!(css_classes.contains("line-through"));
        
        // Verify text transform
        assert!(css_classes.contains("normal-case"));
        assert!(css_classes.contains("uppercase"));
        assert!(css_classes.contains("lowercase"));
        assert!(css_classes.contains("capitalize"));
        
        // Verify text overflow
        assert!(css_classes.contains("truncate"));
        assert!(css_classes.contains("text-ellipsis"));
        assert!(css_classes.contains("text-clip"));
        
        // Verify white space
        assert!(css_classes.contains("whitespace-normal"));
        assert!(css_classes.contains("whitespace-nowrap"));
        assert!(css_classes.contains("whitespace-pre"));
        assert!(css_classes.contains("whitespace-pre-line"));
        assert!(css_classes.contains("whitespace-pre-wrap"));
        assert!(css_classes.contains("whitespace-break-spaces"));
        
        // Verify word break
        assert!(css_classes.contains("break-normal"));
        assert!(css_classes.contains("break-all"));
        assert!(css_classes.contains("break-words"));
        assert!(css_classes.contains("break-keep"));
        
        // Verify text decoration style
        assert!(css_classes.contains("decoration-solid"));
        assert!(css_classes.contains("decoration-double"));
        assert!(css_classes.contains("decoration-dotted"));
        assert!(css_classes.contains("decoration-dashed"));
        assert!(css_classes.contains("decoration-wavy"));
        
        // Verify text decoration thickness
        assert!(css_classes.contains("decoration-auto"));
        assert!(css_classes.contains("decoration-from-font"));
        assert!(css_classes.contains("decoration-0"));
        assert!(css_classes.contains("decoration-1"));
        assert!(css_classes.contains("decoration-2"));
        assert!(css_classes.contains("decoration-4"));
        assert!(css_classes.contains("decoration-8"));
        
        // Verify text underline offset
        assert!(css_classes.contains("underline-offset-auto"));
        assert!(css_classes.contains("underline-offset-0"));
        assert!(css_classes.contains("underline-offset-1"));
        assert!(css_classes.contains("underline-offset-2"));
        assert!(css_classes.contains("underline-offset-4"));
        assert!(css_classes.contains("underline-offset-8"));
    }
}
