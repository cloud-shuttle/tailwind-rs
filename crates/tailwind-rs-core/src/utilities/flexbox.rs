//! Flexbox utilities for tailwind-rs
//!
//! This module provides utilities for flexbox layout including flex direction,
//! flex wrap, justify content, align items, align content, and flex properties.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Flex direction values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexDirection {
    /// Row direction
    Row,
    /// Row reverse direction
    RowReverse,
    /// Column direction
    Column,
    /// Column reverse direction
    ColumnReverse,
}

/// Flex wrap values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexWrap {
    /// No wrap
    NoWrap,
    /// Wrap
    Wrap,
    /// Wrap reverse
    WrapReverse,
}

/// Justify content values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JustifyContent {
    /// Start justify
    Start,
    /// End justify
    End,
    /// Center justify
    Center,
    /// Between justify
    Between,
    /// Around justify
    Around,
    /// Evenly justify
    Evenly,
}

/// Align items values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignItems {
    /// Start align
    Start,
    /// End align
    End,
    /// Center align
    Center,
    /// Baseline align
    Baseline,
    /// Stretch align
    Stretch,
}

/// Align content values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignContent {
    /// Start align content
    Start,
    /// End align content
    End,
    /// Center align content
    Center,
    /// Between align content
    Between,
    /// Around align content
    Around,
    /// Evenly align content
    Evenly,
    /// Baseline align content
    Baseline,
    /// Stretch align content
    Stretch,
}

/// Align self values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignSelf {
    /// Auto align self
    Auto,
    /// Start align self
    Start,
    /// End align self
    End,
    /// Center align self
    Center,
    /// Stretch align self
    Stretch,
    /// Baseline align self
    Baseline,
}

/// Flex grow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexGrow {
    /// No grow
    Zero,
    /// Grow
    Grow,
}

/// Flex shrink values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexShrink {
    /// No shrink
    Zero,
    /// Shrink
    Shrink,
}

/// Flex basis values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexBasis {
    /// Auto basis
    Auto,
    /// Full basis
    Full,
    /// Fit basis
    Fit,
    /// Max basis
    Max,
    /// Min basis
    Min,
    /// None basis
    None,
    /// Zero basis
    Zero,
}

/// Order values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Order {
    /// First order
    First,
    /// Last order
    Last,
    /// None order
    None,
    /// Order 1
    One,
    /// Order 2
    Two,
    /// Order 3
    Three,
    /// Order 4
    Four,
    /// Order 5
    Five,
    /// Order 6
    Six,
    /// Order 7
    Seven,
    /// Order 8
    Eight,
    /// Order 9
    Nine,
    /// Order 10
    Ten,
    /// Order 11
    Eleven,
    /// Order 12
    Twelve,
}

impl FlexDirection {
    pub fn to_class_name(&self) -> String {
        match self {
            FlexDirection::Row => "row".to_string(),
            FlexDirection::RowReverse => "row-reverse".to_string(),
            FlexDirection::Column => "col".to_string(),
            FlexDirection::ColumnReverse => "col-reverse".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            FlexDirection::Row => "row".to_string(),
            FlexDirection::RowReverse => "row-reverse".to_string(),
            FlexDirection::Column => "column".to_string(),
            FlexDirection::ColumnReverse => "column-reverse".to_string(),
        }
    }
}

impl FlexWrap {
    pub fn to_class_name(&self) -> String {
        match self {
            FlexWrap::NoWrap => "nowrap".to_string(),
            FlexWrap::Wrap => "wrap".to_string(),
            FlexWrap::WrapReverse => "wrap-reverse".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            FlexWrap::NoWrap => "nowrap".to_string(),
            FlexWrap::Wrap => "wrap".to_string(),
            FlexWrap::WrapReverse => "wrap-reverse".to_string(),
        }
    }
}

impl JustifyContent {
    pub fn to_class_name(&self) -> String {
        match self {
            JustifyContent::Start => "start".to_string(),
            JustifyContent::End => "end".to_string(),
            JustifyContent::Center => "center".to_string(),
            JustifyContent::Between => "between".to_string(),
            JustifyContent::Around => "around".to_string(),
            JustifyContent::Evenly => "evenly".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            JustifyContent::Start => "flex-start".to_string(),
            JustifyContent::End => "flex-end".to_string(),
            JustifyContent::Center => "center".to_string(),
            JustifyContent::Between => "space-between".to_string(),
            JustifyContent::Around => "space-around".to_string(),
            JustifyContent::Evenly => "space-evenly".to_string(),
        }
    }
}

impl AlignItems {
    pub fn to_class_name(&self) -> String {
        match self {
            AlignItems::Start => "start".to_string(),
            AlignItems::End => "end".to_string(),
            AlignItems::Center => "center".to_string(),
            AlignItems::Baseline => "baseline".to_string(),
            AlignItems::Stretch => "stretch".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            AlignItems::Start => "flex-start".to_string(),
            AlignItems::End => "flex-end".to_string(),
            AlignItems::Center => "center".to_string(),
            AlignItems::Baseline => "baseline".to_string(),
            AlignItems::Stretch => "stretch".to_string(),
        }
    }
}

impl AlignContent {
    pub fn to_class_name(&self) -> String {
        match self {
            AlignContent::Start => "start".to_string(),
            AlignContent::End => "end".to_string(),
            AlignContent::Center => "center".to_string(),
            AlignContent::Between => "between".to_string(),
            AlignContent::Around => "around".to_string(),
            AlignContent::Evenly => "evenly".to_string(),
            AlignContent::Baseline => "baseline".to_string(),
            AlignContent::Stretch => "stretch".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            AlignContent::Start => "flex-start".to_string(),
            AlignContent::End => "flex-end".to_string(),
            AlignContent::Center => "center".to_string(),
            AlignContent::Between => "space-between".to_string(),
            AlignContent::Around => "space-around".to_string(),
            AlignContent::Evenly => "space-evenly".to_string(),
            AlignContent::Baseline => "baseline".to_string(),
            AlignContent::Stretch => "stretch".to_string(),
        }
    }
}

impl AlignSelf {
    pub fn to_class_name(&self) -> String {
        match self {
            AlignSelf::Auto => "auto".to_string(),
            AlignSelf::Start => "start".to_string(),
            AlignSelf::End => "end".to_string(),
            AlignSelf::Center => "center".to_string(),
            AlignSelf::Stretch => "stretch".to_string(),
            AlignSelf::Baseline => "baseline".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            AlignSelf::Auto => "auto".to_string(),
            AlignSelf::Start => "flex-start".to_string(),
            AlignSelf::End => "flex-end".to_string(),
            AlignSelf::Center => "center".to_string(),
            AlignSelf::Stretch => "stretch".to_string(),
            AlignSelf::Baseline => "baseline".to_string(),
        }
    }
}

impl FlexGrow {
    pub fn to_class_name(&self) -> String {
        match self {
            FlexGrow::Zero => "0".to_string(),
            FlexGrow::Grow => "grow".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            FlexGrow::Zero => "0".to_string(),
            FlexGrow::Grow => "1".to_string(),
        }
    }
}

impl FlexShrink {
    pub fn to_class_name(&self) -> String {
        match self {
            FlexShrink::Zero => "0".to_string(),
            FlexShrink::Shrink => "shrink".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            FlexShrink::Zero => "0".to_string(),
            FlexShrink::Shrink => "1".to_string(),
        }
    }
}

impl FlexBasis {
    pub fn to_class_name(&self) -> String {
        match self {
            FlexBasis::Auto => "auto".to_string(),
            FlexBasis::Full => "full".to_string(),
            FlexBasis::Fit => "fit".to_string(),
            FlexBasis::Max => "max".to_string(),
            FlexBasis::Min => "min".to_string(),
            FlexBasis::None => "none".to_string(),
            FlexBasis::Zero => "0".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            FlexBasis::Auto => "auto".to_string(),
            FlexBasis::Full => "100%".to_string(),
            FlexBasis::Fit => "fit-content".to_string(),
            FlexBasis::Max => "max-content".to_string(),
            FlexBasis::Min => "min-content".to_string(),
            FlexBasis::None => "none".to_string(),
            FlexBasis::Zero => "0%".to_string(),
        }
    }
}

impl Order {
    pub fn to_class_name(&self) -> String {
        match self {
            Order::First => "first".to_string(),
            Order::Last => "last".to_string(),
            Order::None => "none".to_string(),
            Order::One => "1".to_string(),
            Order::Two => "2".to_string(),
            Order::Three => "3".to_string(),
            Order::Four => "4".to_string(),
            Order::Five => "5".to_string(),
            Order::Six => "6".to_string(),
            Order::Seven => "7".to_string(),
            Order::Eight => "8".to_string(),
            Order::Nine => "9".to_string(),
            Order::Ten => "10".to_string(),
            Order::Eleven => "11".to_string(),
            Order::Twelve => "12".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Order::First => "-9999".to_string(),
            Order::Last => "9999".to_string(),
            Order::None => "0".to_string(),
            Order::One => "1".to_string(),
            Order::Two => "2".to_string(),
            Order::Three => "3".to_string(),
            Order::Four => "4".to_string(),
            Order::Five => "5".to_string(),
            Order::Six => "6".to_string(),
            Order::Seven => "7".to_string(),
            Order::Eight => "8".to_string(),
            Order::Nine => "9".to_string(),
            Order::Ten => "10".to_string(),
            Order::Eleven => "11".to_string(),
            Order::Twelve => "12".to_string(),
        }
    }
}

impl fmt::Display for FlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for FlexWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for JustifyContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for AlignItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for AlignContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for AlignSelf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for FlexGrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for FlexShrink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for FlexBasis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding flex direction utilities to a class builder
pub trait FlexDirectionUtilities {
    fn flex_direction(self, direction: FlexDirection) -> Self;
}

impl FlexDirectionUtilities for ClassBuilder {
    fn flex_direction(self, direction: FlexDirection) -> Self {
        self.class(format!("flex-{}", direction.to_class_name()))
    }
}

/// Trait for adding flex wrap utilities to a class builder
pub trait FlexWrapUtilities {
    fn flex_wrap(self, wrap: FlexWrap) -> Self;
}

impl FlexWrapUtilities for ClassBuilder {
    fn flex_wrap(self, wrap: FlexWrap) -> Self {
        self.class(format!("flex-{}", wrap.to_class_name()))
    }
}

/// Trait for adding justify content utilities to a class builder
pub trait JustifyContentUtilities {
    fn justify_content(self, justify: JustifyContent) -> Self;
}

impl JustifyContentUtilities for ClassBuilder {
    fn justify_content(self, justify: JustifyContent) -> Self {
        self.class(format!("justify-{}", justify.to_class_name()))
    }
}

/// Trait for adding align items utilities to a class builder
pub trait AlignItemsUtilities {
    fn align_items(self, align: AlignItems) -> Self;
}

impl AlignItemsUtilities for ClassBuilder {
    fn align_items(self, align: AlignItems) -> Self {
        self.class(format!("items-{}", align.to_class_name()))
    }
}

/// Trait for adding align content utilities to a class builder
pub trait AlignContentUtilities {
    fn align_content(self, align: AlignContent) -> Self;
}

impl AlignContentUtilities for ClassBuilder {
    fn align_content(self, align: AlignContent) -> Self {
        self.class(format!("content-{}", align.to_class_name()))
    }
}

/// Trait for adding align self utilities to a class builder
pub trait AlignSelfUtilities {
    fn align_self(self, align: AlignSelf) -> Self;
}

impl AlignSelfUtilities for ClassBuilder {
    fn align_self(self, align: AlignSelf) -> Self {
        self.class(format!("self-{}", align.to_class_name()))
    }
}

/// Trait for adding flex grow utilities to a class builder
pub trait FlexGrowUtilities {
    fn flex_grow(self, grow: FlexGrow) -> Self;
}

impl FlexGrowUtilities for ClassBuilder {
    fn flex_grow(self, grow: FlexGrow) -> Self {
        self.class(format!("flex-grow-{}", grow.to_class_name()))
    }
}

/// Trait for adding flex shrink utilities to a class builder
pub trait FlexShrinkUtilities {
    fn flex_shrink(self, shrink: FlexShrink) -> Self;
}

impl FlexShrinkUtilities for ClassBuilder {
    fn flex_shrink(self, shrink: FlexShrink) -> Self {
        self.class(format!("flex-shrink-{}", shrink.to_class_name()))
    }
}

/// Trait for adding flex basis utilities to a class builder
pub trait FlexBasisUtilities {
    fn flex_basis(self, basis: FlexBasis) -> Self;
}

impl FlexBasisUtilities for ClassBuilder {
    fn flex_basis(self, basis: FlexBasis) -> Self {
        self.class(format!("basis-{}", basis.to_class_name()))
    }
}

/// Trait for adding order utilities to a class builder
pub trait OrderUtilities {
    fn order(self, order: Order) -> Self;
}

impl OrderUtilities for ClassBuilder {
    fn order(self, order: Order) -> Self {
        self.class(format!("order-{}", order.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_flex_direction_utilities() {
        let classes = ClassBuilder::new()
            .flex_direction(FlexDirection::Row)
            .flex_direction(FlexDirection::RowReverse)
            .flex_direction(FlexDirection::Column)
            .flex_direction(FlexDirection::ColumnReverse)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("flex-row"));
        assert!(css_classes.contains("flex-row-reverse"));
        assert!(css_classes.contains("flex-col"));
        assert!(css_classes.contains("flex-col-reverse"));
    }
    
    #[test]
    fn test_flex_wrap_utilities() {
        let classes = ClassBuilder::new()
            .flex_wrap(FlexWrap::NoWrap)
            .flex_wrap(FlexWrap::Wrap)
            .flex_wrap(FlexWrap::WrapReverse)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("flex-nowrap"));
        assert!(css_classes.contains("flex-wrap"));
        assert!(css_classes.contains("flex-wrap-reverse"));
    }
    
    #[test]
    fn test_justify_content_utilities() {
        let classes = ClassBuilder::new()
            .justify_content(JustifyContent::Start)
            .justify_content(JustifyContent::End)
            .justify_content(JustifyContent::Center)
            .justify_content(JustifyContent::Between)
            .justify_content(JustifyContent::Around)
            .justify_content(JustifyContent::Evenly)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("justify-start"));
        assert!(css_classes.contains("justify-end"));
        assert!(css_classes.contains("justify-center"));
        assert!(css_classes.contains("justify-between"));
        assert!(css_classes.contains("justify-around"));
        assert!(css_classes.contains("justify-evenly"));
    }
    
    #[test]
    fn test_align_items_utilities() {
        let classes = ClassBuilder::new()
            .align_items(AlignItems::Start)
            .align_items(AlignItems::End)
            .align_items(AlignItems::Center)
            .align_items(AlignItems::Baseline)
            .align_items(AlignItems::Stretch)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("items-start"));
        assert!(css_classes.contains("items-end"));
        assert!(css_classes.contains("items-center"));
        assert!(css_classes.contains("items-baseline"));
        assert!(css_classes.contains("items-stretch"));
    }
    
    #[test]
    fn test_align_content_utilities() {
        let classes = ClassBuilder::new()
            .align_content(AlignContent::Start)
            .align_content(AlignContent::End)
            .align_content(AlignContent::Center)
            .align_content(AlignContent::Between)
            .align_content(AlignContent::Around)
            .align_content(AlignContent::Evenly)
            .align_content(AlignContent::Baseline)
            .align_content(AlignContent::Stretch)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("content-start"));
        assert!(css_classes.contains("content-end"));
        assert!(css_classes.contains("content-center"));
        assert!(css_classes.contains("content-between"));
        assert!(css_classes.contains("content-around"));
        assert!(css_classes.contains("content-evenly"));
        assert!(css_classes.contains("content-baseline"));
        assert!(css_classes.contains("content-stretch"));
    }
    
    #[test]
    fn test_align_self_utilities() {
        let classes = ClassBuilder::new()
            .align_self(AlignSelf::Auto)
            .align_self(AlignSelf::Start)
            .align_self(AlignSelf::End)
            .align_self(AlignSelf::Center)
            .align_self(AlignSelf::Stretch)
            .align_self(AlignSelf::Baseline)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("self-auto"));
        assert!(css_classes.contains("self-start"));
        assert!(css_classes.contains("self-end"));
        assert!(css_classes.contains("self-center"));
        assert!(css_classes.contains("self-stretch"));
        assert!(css_classes.contains("self-baseline"));
    }
    
    #[test]
    fn test_flex_grow_utilities() {
        let classes = ClassBuilder::new()
            .flex_grow(FlexGrow::Zero)
            .flex_grow(FlexGrow::Grow)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("flex-grow-0"));
        assert!(css_classes.contains("flex-grow-grow"));
    }
    
    #[test]
    fn test_flex_shrink_utilities() {
        let classes = ClassBuilder::new()
            .flex_shrink(FlexShrink::Zero)
            .flex_shrink(FlexShrink::Shrink)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("flex-shrink-0"));
        assert!(css_classes.contains("flex-shrink-shrink"));
    }
    
    #[test]
    fn test_flex_basis_utilities() {
        let classes = ClassBuilder::new()
            .flex_basis(FlexBasis::Auto)
            .flex_basis(FlexBasis::Full)
            .flex_basis(FlexBasis::Fit)
            .flex_basis(FlexBasis::Max)
            .flex_basis(FlexBasis::Min)
            .flex_basis(FlexBasis::None)
            .flex_basis(FlexBasis::Zero)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("basis-auto"));
        assert!(css_classes.contains("basis-full"));
        assert!(css_classes.contains("basis-fit"));
        assert!(css_classes.contains("basis-max"));
        assert!(css_classes.contains("basis-min"));
        assert!(css_classes.contains("basis-none"));
        assert!(css_classes.contains("basis-0"));
    }
    
    #[test]
    fn test_order_utilities() {
        let classes = ClassBuilder::new()
            .order(Order::First)
            .order(Order::Last)
            .order(Order::None)
            .order(Order::One)
            .order(Order::Two)
            .order(Order::Three)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("order-first"));
        assert!(css_classes.contains("order-last"));
        assert!(css_classes.contains("order-none"));
        assert!(css_classes.contains("order-1"));
        assert!(css_classes.contains("order-2"));
        assert!(css_classes.contains("order-3"));
    }
    
    #[test]
    fn test_complex_flexbox_combination() {
        let classes = ClassBuilder::new()
            .flex_direction(FlexDirection::Row)
            .flex_wrap(FlexWrap::Wrap)
            .justify_content(JustifyContent::Between)
            .align_items(AlignItems::Center)
            .align_content(AlignContent::Stretch)
            .align_self(AlignSelf::Start)
            .flex_grow(FlexGrow::Grow)
            .flex_shrink(FlexShrink::Shrink)
            .flex_basis(FlexBasis::Auto)
            .order(Order::One)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("flex-row"));
        assert!(css_classes.contains("flex-wrap"));
        assert!(css_classes.contains("justify-between"));
        assert!(css_classes.contains("items-center"));
        assert!(css_classes.contains("content-stretch"));
        assert!(css_classes.contains("self-start"));
        assert!(css_classes.contains("flex-grow-grow"));
        assert!(css_classes.contains("flex-shrink-shrink"));
        assert!(css_classes.contains("basis-auto"));
        assert!(css_classes.contains("order-1"));
    }
}
