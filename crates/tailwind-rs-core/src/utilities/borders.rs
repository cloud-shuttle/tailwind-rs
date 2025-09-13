//! Border utilities for tailwind-rs
//!
//! This module provides utilities for border width, border style, border radius,
//! border color, outline, and divide utilities.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Border width values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderWidth {
    /// No border
    Zero,
    /// Thin border
    Thin,
    /// Default border
    Default,
    /// Medium border
    Medium,
    /// Thick border
    Thick,
}

/// Border style values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderStyle {
    /// Solid border
    Solid,
    /// Dashed border
    Dashed,
    /// Dotted border
    Dotted,
    /// Double border
    Double,
    /// Hidden border
    Hidden,
    /// None border
    None,
}

/// Border radius values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderRadius {
    /// No radius
    None,
    /// Small radius
    Sm,
    /// Default radius
    Default,
    /// Medium radius
    Md,
    /// Large radius
    Lg,
    /// Extra large radius
    Xl,
    /// 2x large radius
    Xl2,
    /// 3x large radius
    Xl3,
    /// Full radius
    Full,
}

/// Outline width values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutlineWidth {
    /// No outline
    Zero,
    /// Thin outline
    Thin,
    /// Default outline
    Default,
    /// Medium outline
    Medium,
    /// Thick outline
    Thick,
}

/// Outline style values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutlineStyle {
    /// Solid outline
    Solid,
    /// Dashed outline
    Dashed,
    /// Dotted outline
    Dotted,
    /// Double outline
    Double,
    /// Hidden outline
    Hidden,
    /// None outline
    None,
}

/// Outline offset values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutlineOffset {
    /// No offset
    Zero,
    /// Small offset
    One,
    /// Medium offset
    Two,
    /// Large offset
    Four,
    /// Extra large offset
    Eight,
}

/// Divide width values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DivideWidth {
    /// No divide
    Zero,
    /// Thin divide
    Thin,
    /// Default divide
    Default,
    /// Medium divide
    Medium,
    /// Thick divide
    Thick,
}

/// Divide style values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DivideStyle {
    /// Solid divide
    Solid,
    /// Dashed divide
    Dashed,
    /// Dotted divide
    Dotted,
    /// Double divide
    Double,
    /// Hidden divide
    Hidden,
    /// None divide
    None,
}

/// Ring width values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RingWidth {
    /// No ring
    Zero,
    /// Thin ring
    Thin,
    /// Default ring
    Default,
    /// Medium ring
    Medium,
    /// Thick ring
    Thick,
    /// Extra thick ring
    ExtraThick,
}

/// Ring offset width values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RingOffsetWidth {
    /// No offset
    Zero,
    /// Small offset
    One,
    /// Medium offset
    Two,
    /// Large offset
    Four,
    /// Extra large offset
    Eight,
}

impl BorderWidth {
    pub fn to_class_name(&self) -> String {
        match self {
            BorderWidth::Zero => "0".to_string(),
            BorderWidth::Thin => "thin".to_string(),
            BorderWidth::Default => "default".to_string(),
            BorderWidth::Medium => "medium".to_string(),
            BorderWidth::Thick => "thick".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            BorderWidth::Zero => "0px".to_string(),
            BorderWidth::Thin => "1px".to_string(),
            BorderWidth::Default => "1px".to_string(),
            BorderWidth::Medium => "2px".to_string(),
            BorderWidth::Thick => "4px".to_string(),
        }
    }
}

impl BorderStyle {
    pub fn to_class_name(&self) -> String {
        match self {
            BorderStyle::Solid => "solid".to_string(),
            BorderStyle::Dashed => "dashed".to_string(),
            BorderStyle::Dotted => "dotted".to_string(),
            BorderStyle::Double => "double".to_string(),
            BorderStyle::Hidden => "hidden".to_string(),
            BorderStyle::None => "none".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            BorderStyle::Solid => "solid".to_string(),
            BorderStyle::Dashed => "dashed".to_string(),
            BorderStyle::Dotted => "dotted".to_string(),
            BorderStyle::Double => "double".to_string(),
            BorderStyle::Hidden => "hidden".to_string(),
            BorderStyle::None => "none".to_string(),
        }
    }
}

impl BorderRadius {
    pub fn to_class_name(&self) -> String {
        match self {
            BorderRadius::None => "none".to_string(),
            BorderRadius::Sm => "sm".to_string(),
            BorderRadius::Default => "default".to_string(),
            BorderRadius::Md => "md".to_string(),
            BorderRadius::Lg => "lg".to_string(),
            BorderRadius::Xl => "xl".to_string(),
            BorderRadius::Xl2 => "2xl".to_string(),
            BorderRadius::Xl3 => "3xl".to_string(),
            BorderRadius::Full => "full".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            BorderRadius::None => "0px".to_string(),
            BorderRadius::Sm => "0.125rem".to_string(),
            BorderRadius::Default => "0.25rem".to_string(),
            BorderRadius::Md => "0.375rem".to_string(),
            BorderRadius::Lg => "0.5rem".to_string(),
            BorderRadius::Xl => "0.75rem".to_string(),
            BorderRadius::Xl2 => "1rem".to_string(),
            BorderRadius::Xl3 => "1.5rem".to_string(),
            BorderRadius::Full => "9999px".to_string(),
        }
    }
}

impl OutlineWidth {
    pub fn to_class_name(&self) -> String {
        match self {
            OutlineWidth::Zero => "0".to_string(),
            OutlineWidth::Thin => "thin".to_string(),
            OutlineWidth::Default => "default".to_string(),
            OutlineWidth::Medium => "medium".to_string(),
            OutlineWidth::Thick => "thick".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            OutlineWidth::Zero => "0px".to_string(),
            OutlineWidth::Thin => "1px".to_string(),
            OutlineWidth::Default => "2px".to_string(),
            OutlineWidth::Medium => "4px".to_string(),
            OutlineWidth::Thick => "8px".to_string(),
        }
    }
}

impl OutlineStyle {
    pub fn to_class_name(&self) -> String {
        match self {
            OutlineStyle::Solid => "solid".to_string(),
            OutlineStyle::Dashed => "dashed".to_string(),
            OutlineStyle::Dotted => "dotted".to_string(),
            OutlineStyle::Double => "double".to_string(),
            OutlineStyle::Hidden => "hidden".to_string(),
            OutlineStyle::None => "none".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            OutlineStyle::Solid => "solid".to_string(),
            OutlineStyle::Dashed => "dashed".to_string(),
            OutlineStyle::Dotted => "dotted".to_string(),
            OutlineStyle::Double => "double".to_string(),
            OutlineStyle::Hidden => "hidden".to_string(),
            OutlineStyle::None => "none".to_string(),
        }
    }
}

impl OutlineOffset {
    pub fn to_class_name(&self) -> String {
        match self {
            OutlineOffset::Zero => "0".to_string(),
            OutlineOffset::One => "1".to_string(),
            OutlineOffset::Two => "2".to_string(),
            OutlineOffset::Four => "4".to_string(),
            OutlineOffset::Eight => "8".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            OutlineOffset::Zero => "0px".to_string(),
            OutlineOffset::One => "1px".to_string(),
            OutlineOffset::Two => "2px".to_string(),
            OutlineOffset::Four => "4px".to_string(),
            OutlineOffset::Eight => "8px".to_string(),
        }
    }
}

impl DivideWidth {
    pub fn to_class_name(&self) -> String {
        match self {
            DivideWidth::Zero => "0".to_string(),
            DivideWidth::Thin => "thin".to_string(),
            DivideWidth::Default => "default".to_string(),
            DivideWidth::Medium => "medium".to_string(),
            DivideWidth::Thick => "thick".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            DivideWidth::Zero => "0px".to_string(),
            DivideWidth::Thin => "1px".to_string(),
            DivideWidth::Default => "1px".to_string(),
            DivideWidth::Medium => "2px".to_string(),
            DivideWidth::Thick => "4px".to_string(),
        }
    }
}

impl DivideStyle {
    pub fn to_class_name(&self) -> String {
        match self {
            DivideStyle::Solid => "solid".to_string(),
            DivideStyle::Dashed => "dashed".to_string(),
            DivideStyle::Dotted => "dotted".to_string(),
            DivideStyle::Double => "double".to_string(),
            DivideStyle::Hidden => "hidden".to_string(),
            DivideStyle::None => "none".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            DivideStyle::Solid => "solid".to_string(),
            DivideStyle::Dashed => "dashed".to_string(),
            DivideStyle::Dotted => "dotted".to_string(),
            DivideStyle::Double => "double".to_string(),
            DivideStyle::Hidden => "hidden".to_string(),
            DivideStyle::None => "none".to_string(),
        }
    }
}

impl RingWidth {
    pub fn to_class_name(&self) -> String {
        match self {
            RingWidth::Zero => "0".to_string(),
            RingWidth::Thin => "thin".to_string(),
            RingWidth::Default => "default".to_string(),
            RingWidth::Medium => "medium".to_string(),
            RingWidth::Thick => "thick".to_string(),
            RingWidth::ExtraThick => "extra-thick".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            RingWidth::Zero => "0px".to_string(),
            RingWidth::Thin => "1px".to_string(),
            RingWidth::Default => "3px".to_string(),
            RingWidth::Medium => "4px".to_string(),
            RingWidth::Thick => "8px".to_string(),
            RingWidth::ExtraThick => "12px".to_string(),
        }
    }
}

impl RingOffsetWidth {
    pub fn to_class_name(&self) -> String {
        match self {
            RingOffsetWidth::Zero => "0".to_string(),
            RingOffsetWidth::One => "1".to_string(),
            RingOffsetWidth::Two => "2".to_string(),
            RingOffsetWidth::Four => "4".to_string(),
            RingOffsetWidth::Eight => "8".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            RingOffsetWidth::Zero => "0px".to_string(),
            RingOffsetWidth::One => "1px".to_string(),
            RingOffsetWidth::Two => "2px".to_string(),
            RingOffsetWidth::Four => "4px".to_string(),
            RingOffsetWidth::Eight => "8px".to_string(),
        }
    }
}

impl fmt::Display for BorderWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for BorderRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for OutlineWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for OutlineStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for OutlineOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for DivideWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for DivideStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for RingWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for RingOffsetWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding border width utilities to a class builder
pub trait BorderWidthUtilities {
    fn border_width(self, width: BorderWidth) -> Self;
    fn border_width_x(self, width: BorderWidth) -> Self;
    fn border_width_y(self, width: BorderWidth) -> Self;
    fn border_width_t(self, width: BorderWidth) -> Self;
    fn border_width_r(self, width: BorderWidth) -> Self;
    fn border_width_b(self, width: BorderWidth) -> Self;
    fn border_width_l(self, width: BorderWidth) -> Self;
}

impl BorderWidthUtilities for ClassBuilder {
    fn border_width(self, width: BorderWidth) -> Self {
        self.class(format!("border-{}", width.to_class_name()))
    }
    
    fn border_width_x(self, width: BorderWidth) -> Self {
        self.class(format!("border-x-{}", width.to_class_name()))
    }
    
    fn border_width_y(self, width: BorderWidth) -> Self {
        self.class(format!("border-y-{}", width.to_class_name()))
    }
    
    fn border_width_t(self, width: BorderWidth) -> Self {
        self.class(format!("border-t-{}", width.to_class_name()))
    }
    
    fn border_width_r(self, width: BorderWidth) -> Self {
        self.class(format!("border-r-{}", width.to_class_name()))
    }
    
    fn border_width_b(self, width: BorderWidth) -> Self {
        self.class(format!("border-b-{}", width.to_class_name()))
    }
    
    fn border_width_l(self, width: BorderWidth) -> Self {
        self.class(format!("border-l-{}", width.to_class_name()))
    }
}

/// Trait for adding border style utilities to a class builder
pub trait BorderStyleUtilities {
    fn border_style(self, style: BorderStyle) -> Self;
}

impl BorderStyleUtilities for ClassBuilder {
    fn border_style(self, style: BorderStyle) -> Self {
        self.class(format!("border-{}", style.to_class_name()))
    }
}

/// Trait for adding border radius utilities to a class builder
pub trait BorderRadiusUtilities {
    fn border_radius(self, radius: BorderRadius) -> Self;
    fn border_radius_t(self, radius: BorderRadius) -> Self;
    fn border_radius_r(self, radius: BorderRadius) -> Self;
    fn border_radius_b(self, radius: BorderRadius) -> Self;
    fn border_radius_l(self, radius: BorderRadius) -> Self;
    fn border_radius_tl(self, radius: BorderRadius) -> Self;
    fn border_radius_tr(self, radius: BorderRadius) -> Self;
    fn border_radius_br(self, radius: BorderRadius) -> Self;
    fn border_radius_bl(self, radius: BorderRadius) -> Self;
}

impl BorderRadiusUtilities for ClassBuilder {
    fn border_radius(self, radius: BorderRadius) -> Self {
        self.class(format!("rounded-{}", radius.to_class_name()))
    }
    
    fn border_radius_t(self, radius: BorderRadius) -> Self {
        self.class(format!("rounded-t-{}", radius.to_class_name()))
    }
    
    fn border_radius_r(self, radius: BorderRadius) -> Self {
        self.class(format!("rounded-r-{}", radius.to_class_name()))
    }
    
    fn border_radius_b(self, radius: BorderRadius) -> Self {
        self.class(format!("rounded-b-{}", radius.to_class_name()))
    }
    
    fn border_radius_l(self, radius: BorderRadius) -> Self {
        self.class(format!("rounded-l-{}", radius.to_class_name()))
    }
    
    fn border_radius_tl(self, radius: BorderRadius) -> Self {
        self.class(format!("rounded-tl-{}", radius.to_class_name()))
    }
    
    fn border_radius_tr(self, radius: BorderRadius) -> Self {
        self.class(format!("rounded-tr-{}", radius.to_class_name()))
    }
    
    fn border_radius_br(self, radius: BorderRadius) -> Self {
        self.class(format!("rounded-br-{}", radius.to_class_name()))
    }
    
    fn border_radius_bl(self, radius: BorderRadius) -> Self {
        self.class(format!("rounded-bl-{}", radius.to_class_name()))
    }
}

/// Trait for adding outline utilities to a class builder
pub trait OutlineUtilities {
    fn outline_width(self, width: OutlineWidth) -> Self;
    fn outline_style(self, style: OutlineStyle) -> Self;
    fn outline_offset(self, offset: OutlineOffset) -> Self;
}

impl OutlineUtilities for ClassBuilder {
    fn outline_width(self, width: OutlineWidth) -> Self {
        self.class(format!("outline-{}", width.to_class_name()))
    }
    
    fn outline_style(self, style: OutlineStyle) -> Self {
        self.class(format!("outline-{}", style.to_class_name()))
    }
    
    fn outline_offset(self, offset: OutlineOffset) -> Self {
        self.class(format!("outline-offset-{}", offset.to_class_name()))
    }
}

/// Trait for adding divide utilities to a class builder
pub trait DivideUtilities {
    fn divide_width(self, width: DivideWidth) -> Self;
    fn divide_width_x(self, width: DivideWidth) -> Self;
    fn divide_width_y(self, width: DivideWidth) -> Self;
    fn divide_style(self, style: DivideStyle) -> Self;
}

impl DivideUtilities for ClassBuilder {
    fn divide_width(self, width: DivideWidth) -> Self {
        self.class(format!("divide-{}", width.to_class_name()))
    }
    
    fn divide_width_x(self, width: DivideWidth) -> Self {
        self.class(format!("divide-x-{}", width.to_class_name()))
    }
    
    fn divide_width_y(self, width: DivideWidth) -> Self {
        self.class(format!("divide-y-{}", width.to_class_name()))
    }
    
    fn divide_style(self, style: DivideStyle) -> Self {
        self.class(format!("divide-{}", style.to_class_name()))
    }
}

/// Trait for adding ring utilities to a class builder
pub trait RingUtilities {
    fn ring_width(self, width: RingWidth) -> Self;
    fn ring_offset_width(self, width: RingOffsetWidth) -> Self;
}

impl RingUtilities for ClassBuilder {
    fn ring_width(self, width: RingWidth) -> Self {
        self.class(format!("ring-{}", width.to_class_name()))
    }
    
    fn ring_offset_width(self, width: RingOffsetWidth) -> Self {
        self.class(format!("ring-offset-{}", width.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_border_width_utilities() {
        let classes = ClassBuilder::new()
            .border_width(BorderWidth::Zero)
            .border_width(BorderWidth::Thin)
            .border_width(BorderWidth::Default)
            .border_width(BorderWidth::Medium)
            .border_width(BorderWidth::Thick)
            .border_width_x(BorderWidth::Thin)
            .border_width_y(BorderWidth::Medium)
            .border_width_t(BorderWidth::Thick)
            .border_width_r(BorderWidth::Default)
            .border_width_b(BorderWidth::Thin)
            .border_width_l(BorderWidth::Zero)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("border-0"));
        assert!(css_classes.contains("border-thin"));
        assert!(css_classes.contains("border-default"));
        assert!(css_classes.contains("border-medium"));
        assert!(css_classes.contains("border-thick"));
        assert!(css_classes.contains("border-x-thin"));
        assert!(css_classes.contains("border-y-medium"));
        assert!(css_classes.contains("border-t-thick"));
        assert!(css_classes.contains("border-r-default"));
        assert!(css_classes.contains("border-b-thin"));
        assert!(css_classes.contains("border-l-0"));
    }
    
    #[test]
    fn test_border_style_utilities() {
        let classes = ClassBuilder::new()
            .border_style(BorderStyle::Solid)
            .border_style(BorderStyle::Dashed)
            .border_style(BorderStyle::Dotted)
            .border_style(BorderStyle::Double)
            .border_style(BorderStyle::Hidden)
            .border_style(BorderStyle::None)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("border-solid"));
        assert!(css_classes.contains("border-dashed"));
        assert!(css_classes.contains("border-dotted"));
        assert!(css_classes.contains("border-double"));
        assert!(css_classes.contains("border-hidden"));
        assert!(css_classes.contains("border-none"));
    }
    
    #[test]
    fn test_border_radius_utilities() {
        let classes = ClassBuilder::new()
            .border_radius(BorderRadius::None)
            .border_radius(BorderRadius::Sm)
            .border_radius(BorderRadius::Default)
            .border_radius(BorderRadius::Md)
            .border_radius(BorderRadius::Lg)
            .border_radius(BorderRadius::Xl)
            .border_radius(BorderRadius::Xl2)
            .border_radius(BorderRadius::Xl3)
            .border_radius(BorderRadius::Full)
            .border_radius_t(BorderRadius::Lg)
            .border_radius_r(BorderRadius::Md)
            .border_radius_b(BorderRadius::Sm)
            .border_radius_l(BorderRadius::Default)
            .border_radius_tl(BorderRadius::Xl)
            .border_radius_tr(BorderRadius::Xl2)
            .border_radius_br(BorderRadius::Xl3)
            .border_radius_bl(BorderRadius::Full)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("rounded-none"));
        assert!(css_classes.contains("rounded-sm"));
        assert!(css_classes.contains("rounded-default"));
        assert!(css_classes.contains("rounded-md"));
        assert!(css_classes.contains("rounded-lg"));
        assert!(css_classes.contains("rounded-xl"));
        assert!(css_classes.contains("rounded-2xl"));
        assert!(css_classes.contains("rounded-3xl"));
        assert!(css_classes.contains("rounded-full"));
        assert!(css_classes.contains("rounded-t-lg"));
        assert!(css_classes.contains("rounded-r-md"));
        assert!(css_classes.contains("rounded-b-sm"));
        assert!(css_classes.contains("rounded-l-default"));
        assert!(css_classes.contains("rounded-tl-xl"));
        assert!(css_classes.contains("rounded-tr-2xl"));
        assert!(css_classes.contains("rounded-br-3xl"));
        assert!(css_classes.contains("rounded-bl-full"));
    }
    
    #[test]
    fn test_outline_utilities() {
        let classes = ClassBuilder::new()
            .outline_width(OutlineWidth::Zero)
            .outline_width(OutlineWidth::Thin)
            .outline_width(OutlineWidth::Default)
            .outline_width(OutlineWidth::Medium)
            .outline_width(OutlineWidth::Thick)
            .outline_style(OutlineStyle::Solid)
            .outline_style(OutlineStyle::Dashed)
            .outline_style(OutlineStyle::Dotted)
            .outline_style(OutlineStyle::Double)
            .outline_style(OutlineStyle::Hidden)
            .outline_style(OutlineStyle::None)
            .outline_offset(OutlineOffset::Zero)
            .outline_offset(OutlineOffset::One)
            .outline_offset(OutlineOffset::Two)
            .outline_offset(OutlineOffset::Four)
            .outline_offset(OutlineOffset::Eight)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("outline-0"));
        assert!(css_classes.contains("outline-thin"));
        assert!(css_classes.contains("outline-default"));
        assert!(css_classes.contains("outline-medium"));
        assert!(css_classes.contains("outline-thick"));
        assert!(css_classes.contains("outline-solid"));
        assert!(css_classes.contains("outline-dashed"));
        assert!(css_classes.contains("outline-dotted"));
        assert!(css_classes.contains("outline-double"));
        assert!(css_classes.contains("outline-hidden"));
        assert!(css_classes.contains("outline-none"));
        assert!(css_classes.contains("outline-offset-0"));
        assert!(css_classes.contains("outline-offset-1"));
        assert!(css_classes.contains("outline-offset-2"));
        assert!(css_classes.contains("outline-offset-4"));
        assert!(css_classes.contains("outline-offset-8"));
    }
    
    #[test]
    fn test_divide_utilities() {
        let classes = ClassBuilder::new()
            .divide_width(DivideWidth::Zero)
            .divide_width(DivideWidth::Thin)
            .divide_width(DivideWidth::Default)
            .divide_width(DivideWidth::Medium)
            .divide_width(DivideWidth::Thick)
            .divide_width_x(DivideWidth::Thin)
            .divide_width_y(DivideWidth::Medium)
            .divide_style(DivideStyle::Solid)
            .divide_style(DivideStyle::Dashed)
            .divide_style(DivideStyle::Dotted)
            .divide_style(DivideStyle::Double)
            .divide_style(DivideStyle::Hidden)
            .divide_style(DivideStyle::None)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("divide-0"));
        assert!(css_classes.contains("divide-thin"));
        assert!(css_classes.contains("divide-default"));
        assert!(css_classes.contains("divide-medium"));
        assert!(css_classes.contains("divide-thick"));
        assert!(css_classes.contains("divide-x-thin"));
        assert!(css_classes.contains("divide-y-medium"));
        assert!(css_classes.contains("divide-solid"));
        assert!(css_classes.contains("divide-dashed"));
        assert!(css_classes.contains("divide-dotted"));
        assert!(css_classes.contains("divide-double"));
        assert!(css_classes.contains("divide-hidden"));
        assert!(css_classes.contains("divide-none"));
    }
    
    #[test]
    fn test_ring_utilities() {
        let classes = ClassBuilder::new()
            .ring_width(RingWidth::Zero)
            .ring_width(RingWidth::Thin)
            .ring_width(RingWidth::Default)
            .ring_width(RingWidth::Medium)
            .ring_width(RingWidth::Thick)
            .ring_width(RingWidth::ExtraThick)
            .ring_offset_width(RingOffsetWidth::Zero)
            .ring_offset_width(RingOffsetWidth::One)
            .ring_offset_width(RingOffsetWidth::Two)
            .ring_offset_width(RingOffsetWidth::Four)
            .ring_offset_width(RingOffsetWidth::Eight)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("ring-0"));
        assert!(css_classes.contains("ring-thin"));
        assert!(css_classes.contains("ring-default"));
        assert!(css_classes.contains("ring-medium"));
        assert!(css_classes.contains("ring-thick"));
        assert!(css_classes.contains("ring-extra-thick"));
        assert!(css_classes.contains("ring-offset-0"));
        assert!(css_classes.contains("ring-offset-1"));
        assert!(css_classes.contains("ring-offset-2"));
        assert!(css_classes.contains("ring-offset-4"));
        assert!(css_classes.contains("ring-offset-8"));
    }
    
    #[test]
    fn test_complex_border_combination() {
        let classes = ClassBuilder::new()
            .border_width(BorderWidth::Medium)
            .border_style(BorderStyle::Solid)
            .border_radius(BorderRadius::Lg)
            .outline_width(OutlineWidth::Thin)
            .outline_style(OutlineStyle::Dashed)
            .outline_offset(OutlineOffset::Two)
            .divide_width(DivideWidth::Thin)
            .divide_style(DivideStyle::Solid)
            .ring_width(RingWidth::Default)
            .ring_offset_width(RingOffsetWidth::One)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("border-medium"));
        assert!(css_classes.contains("border-solid"));
        assert!(css_classes.contains("rounded-lg"));
        assert!(css_classes.contains("outline-thin"));
        assert!(css_classes.contains("outline-dashed"));
        assert!(css_classes.contains("outline-offset-2"));
        assert!(css_classes.contains("divide-thin"));
        assert!(css_classes.contains("divide-solid"));
        assert!(css_classes.contains("ring-default"));
        assert!(css_classes.contains("ring-offset-1"));
    }
}
