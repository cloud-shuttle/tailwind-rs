//! Font-related typography utilities
//!
//! This module provides font families, sizes, and weights for typography utilities.

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

impl fmt::Display for FontFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "font-{}", self.to_class_name())
    }
}

impl fmt::Display for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "text-{}", self.to_class_name())
    }
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "font-{}", self.to_class_name())
    }
}

/// Trait for font family utilities
pub trait FontFamilyUtilities {
    fn font_family(&mut self, family: FontFamily) -> &mut Self;
    fn font_sans(&mut self) -> &mut Self;
    fn font_serif(&mut self) -> &mut Self;
    fn font_mono(&mut self) -> &mut Self;
}

/// Trait for font size utilities
pub trait FontSizeUtilities {
    fn text_size(&mut self, size: FontSize) -> &mut Self;
    fn text_xs(&mut self) -> &mut Self;
    fn text_sm(&mut self) -> &mut Self;
    fn text_base(&mut self) -> &mut Self;
    fn text_lg(&mut self) -> &mut Self;
    fn text_xl(&mut self) -> &mut Self;
    fn text_2xl(&mut self) -> &mut Self;
    fn text_3xl(&mut self) -> &mut Self;
    fn text_4xl(&mut self) -> &mut Self;
    fn text_5xl(&mut self) -> &mut Self;
    fn text_6xl(&mut self) -> &mut Self;
    fn text_7xl(&mut self) -> &mut Self;
    fn text_8xl(&mut self) -> &mut Self;
    fn text_9xl(&mut self) -> &mut Self;
}

/// Trait for font weight utilities
pub trait FontWeightUtilities {
    fn font_weight(&mut self, weight: FontWeight) -> &mut Self;
    fn font_thin(&mut self) -> &mut Self;
    fn font_extralight(&mut self) -> &mut Self;
    fn font_light(&mut self) -> &mut Self;
    fn font_normal(&mut self) -> &mut Self;
    fn font_medium(&mut self) -> &mut Self;
    fn font_semibold(&mut self) -> &mut Self;
    fn font_bold(&mut self) -> &mut Self;
    fn font_extrabold(&mut self) -> &mut Self;
    fn font_black(&mut self) -> &mut Self;
}

impl FontFamilyUtilities for ClassBuilder {
    fn font_family(&mut self, family: FontFamily) -> &mut Self {
        *self = self
            .clone()
            .class(format!("font-{}", family.to_class_name()));
        self
    }

    fn font_sans(&mut self) -> &mut Self {
        self.font_family(FontFamily::Sans)
    }

    fn font_serif(&mut self) -> &mut Self {
        self.font_family(FontFamily::Serif)
    }

    fn font_mono(&mut self) -> &mut Self {
        self.font_family(FontFamily::Mono)
    }
}

impl FontSizeUtilities for ClassBuilder {
    fn text_size(&mut self, size: FontSize) -> &mut Self {
        *self = self
            .clone()
            .class(format!("text-{}", size.to_class_name()));
        self
    }

    fn text_xs(&mut self) -> &mut Self {
        self.text_size(FontSize::Xs)
    }

    fn text_sm(&mut self) -> &mut Self {
        self.text_size(FontSize::Sm)
    }

    fn text_base(&mut self) -> &mut Self {
        self.text_size(FontSize::Base)
    }

    fn text_lg(&mut self) -> &mut Self {
        self.text_size(FontSize::Lg)
    }

    fn text_xl(&mut self) -> &mut Self {
        self.text_size(FontSize::Xl)
    }

    fn text_2xl(&mut self) -> &mut Self {
        self.text_size(FontSize::Xl2)
    }

    fn text_3xl(&mut self) -> &mut Self {
        self.text_size(FontSize::Xl3)
    }

    fn text_4xl(&mut self) -> &mut Self {
        self.text_size(FontSize::Xl4)
    }

    fn text_5xl(&mut self) -> &mut Self {
        self.text_size(FontSize::Xl5)
    }

    fn text_6xl(&mut self) -> &mut Self {
        self.text_size(FontSize::Xl6)
    }

    fn text_7xl(&mut self) -> &mut Self {
        self.text_size(FontSize::Xl7)
    }

    fn text_8xl(&mut self) -> &mut Self {
        self.text_size(FontSize::Xl8)
    }

    fn text_9xl(&mut self) -> &mut Self {
        self.text_size(FontSize::Xl9)
    }
}

impl FontWeightUtilities for ClassBuilder {
    fn font_weight(&mut self, weight: FontWeight) -> &mut Self {
        *self = self
            .clone()
            .class(format!("font-{}", weight.to_class_name()));
        self
    }

    fn font_thin(&mut self) -> &mut Self {
        self.font_weight(FontWeight::Thin)
    }

    fn font_extralight(&mut self) -> &mut Self {
        self.font_weight(FontWeight::ExtraLight)
    }

    fn font_light(&mut self) -> &mut Self {
        self.font_weight(FontWeight::Light)
    }

    fn font_normal(&mut self) -> &mut Self {
        self.font_weight(FontWeight::Normal)
    }

    fn font_medium(&mut self) -> &mut Self {
        self.font_weight(FontWeight::Medium)
    }

    fn font_semibold(&mut self) -> &mut Self {
        self.font_weight(FontWeight::SemiBold)
    }

    fn font_bold(&mut self) -> &mut Self {
        self.font_weight(FontWeight::Bold)
    }

    fn font_extrabold(&mut self) -> &mut Self {
        self.font_weight(FontWeight::ExtraBold)
    }

    fn font_black(&mut self) -> &mut Self {
        self.font_weight(FontWeight::Black)
    }
}
