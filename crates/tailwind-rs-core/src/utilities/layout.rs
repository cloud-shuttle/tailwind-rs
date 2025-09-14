//! Layout utilities for tailwind-rs
//!
//! This module provides utilities for display, position, overflow, z-index, and other layout properties.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Display values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Display {
    /// Block display
    Block,
    /// Inline display
    Inline,
    /// Inline-block display
    InlineBlock,
    /// Flex display
    Flex,
    /// Inline-flex display
    InlineFlex,
    /// Grid display
    Grid,
    /// Inline-grid display
    InlineGrid,
    /// Table display
    Table,
    /// Inline-table display
    InlineTable,
    /// Table-caption display
    TableCaption,
    /// Table-cell display
    TableCell,
    /// Table-column display
    TableColumn,
    /// Table-column-group display
    TableColumnGroup,
    /// Table-footer-group display
    TableFooterGroup,
    /// Table-header-group display
    TableHeaderGroup,
    /// Table-row display
    TableRow,
    /// Table-row-group display
    TableRowGroup,
    /// Flow-root display
    FlowRoot,
    /// Contents display
    Contents,
    /// List-item display
    ListItem,
    /// Hidden display
    Hidden,
}

/// Position values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    /// Static position
    Static,
    /// Fixed position
    Fixed,
    /// Absolute position
    Absolute,
    /// Relative position
    Relative,
    /// Sticky position
    Sticky,
}

/// Overflow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Overflow {
    /// Auto overflow
    Auto,
    /// Hidden overflow
    Hidden,
    /// Clip overflow
    Clip,
    /// Visible overflow
    Visible,
    /// Scroll overflow
    Scroll,
}

/// Z-index values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZIndex {
    /// Auto z-index
    Auto,
    /// Z-index 0
    Zero,
    /// Z-index 10
    Ten,
    /// Z-index 20
    Twenty,
    /// Z-index 30
    Thirty,
    /// Z-index 40
    Forty,
    /// Z-index 50
    Fifty,
}

/// Float values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Float {
    /// Float right
    Right,
    /// Float left
    Left,
    /// Float none
    None,
}

/// Clear values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Clear {
    /// Clear left
    Left,
    /// Clear right
    Right,
    /// Clear both
    Both,
    /// Clear none
    None,
}

/// Isolation values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Isolation {
    /// Isolate
    Isolate,
    /// Isolate auto
    Auto,
}

/// Object fit values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectFit {
    /// Contain object fit
    Contain,
    /// Cover object fit
    Cover,
    /// Fill object fit
    Fill,
    /// None object fit
    None,
    /// Scale-down object fit
    ScaleDown,
}

/// Object position values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectPosition {
    /// Bottom object position
    Bottom,
    /// Center object position
    Center,
    /// Left object position
    Left,
    /// Left bottom object position
    LeftBottom,
    /// Left top object position
    LeftTop,
    /// Right object position
    Right,
    /// Right bottom object position
    RightBottom,
    /// Right top object position
    RightTop,
    /// Top object position
    Top,
}

/// Overscroll behavior values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OverscrollBehavior {
    /// Auto overscroll behavior
    Auto,
    /// Contain overscroll behavior
    Contain,
    /// None overscroll behavior
    None,
}

/// Visibility values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Visibility {
    /// Visible
    Visible,
    /// Hidden
    Hidden,
    /// Collapse
    Collapse,
}

impl Display {
    pub fn to_class_name(&self) -> String {
        match self {
            Display::Block => "block".to_string(),
            Display::Inline => "inline".to_string(),
            Display::InlineBlock => "inline-block".to_string(),
            Display::Flex => "flex".to_string(),
            Display::InlineFlex => "inline-flex".to_string(),
            Display::Grid => "grid".to_string(),
            Display::InlineGrid => "inline-grid".to_string(),
            Display::Table => "table".to_string(),
            Display::InlineTable => "inline-table".to_string(),
            Display::TableCaption => "table-caption".to_string(),
            Display::TableCell => "table-cell".to_string(),
            Display::TableColumn => "table-column".to_string(),
            Display::TableColumnGroup => "table-column-group".to_string(),
            Display::TableFooterGroup => "table-footer-group".to_string(),
            Display::TableHeaderGroup => "table-header-group".to_string(),
            Display::TableRow => "table-row".to_string(),
            Display::TableRowGroup => "table-row-group".to_string(),
            Display::FlowRoot => "flow-root".to_string(),
            Display::Contents => "contents".to_string(),
            Display::ListItem => "list-item".to_string(),
            Display::Hidden => "hidden".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Display::Block => "block".to_string(),
            Display::Inline => "inline".to_string(),
            Display::InlineBlock => "inline-block".to_string(),
            Display::Flex => "flex".to_string(),
            Display::InlineFlex => "inline-flex".to_string(),
            Display::Grid => "grid".to_string(),
            Display::InlineGrid => "inline-grid".to_string(),
            Display::Table => "table".to_string(),
            Display::InlineTable => "inline-table".to_string(),
            Display::TableCaption => "table-caption".to_string(),
            Display::TableCell => "table-cell".to_string(),
            Display::TableColumn => "table-column".to_string(),
            Display::TableColumnGroup => "table-column-group".to_string(),
            Display::TableFooterGroup => "table-footer-group".to_string(),
            Display::TableHeaderGroup => "table-header-group".to_string(),
            Display::TableRow => "table-row".to_string(),
            Display::TableRowGroup => "table-row-group".to_string(),
            Display::FlowRoot => "flow-root".to_string(),
            Display::Contents => "contents".to_string(),
            Display::ListItem => "list-item".to_string(),
            Display::Hidden => "none".to_string(),
        }
    }
}

impl Position {
    pub fn to_class_name(&self) -> String {
        match self {
            Position::Static => "static".to_string(),
            Position::Fixed => "fixed".to_string(),
            Position::Absolute => "absolute".to_string(),
            Position::Relative => "relative".to_string(),
            Position::Sticky => "sticky".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Position::Static => "static".to_string(),
            Position::Fixed => "fixed".to_string(),
            Position::Absolute => "absolute".to_string(),
            Position::Relative => "relative".to_string(),
            Position::Sticky => "sticky".to_string(),
        }
    }
}

impl Overflow {
    pub fn to_class_name(&self) -> String {
        match self {
            Overflow::Auto => "auto".to_string(),
            Overflow::Hidden => "hidden".to_string(),
            Overflow::Clip => "clip".to_string(),
            Overflow::Visible => "visible".to_string(),
            Overflow::Scroll => "scroll".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Overflow::Auto => "auto".to_string(),
            Overflow::Hidden => "hidden".to_string(),
            Overflow::Clip => "clip".to_string(),
            Overflow::Visible => "visible".to_string(),
            Overflow::Scroll => "scroll".to_string(),
        }
    }
}

impl ZIndex {
    pub fn to_class_name(&self) -> String {
        match self {
            ZIndex::Auto => "auto".to_string(),
            ZIndex::Zero => "0".to_string(),
            ZIndex::Ten => "10".to_string(),
            ZIndex::Twenty => "20".to_string(),
            ZIndex::Thirty => "30".to_string(),
            ZIndex::Forty => "40".to_string(),
            ZIndex::Fifty => "50".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            ZIndex::Auto => "auto".to_string(),
            ZIndex::Zero => "0".to_string(),
            ZIndex::Ten => "10".to_string(),
            ZIndex::Twenty => "20".to_string(),
            ZIndex::Thirty => "30".to_string(),
            ZIndex::Forty => "40".to_string(),
            ZIndex::Fifty => "50".to_string(),
        }
    }
}

impl Float {
    pub fn to_class_name(&self) -> String {
        match self {
            Float::Right => "right".to_string(),
            Float::Left => "left".to_string(),
            Float::None => "none".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Float::Right => "right".to_string(),
            Float::Left => "left".to_string(),
            Float::None => "none".to_string(),
        }
    }
}

impl Clear {
    pub fn to_class_name(&self) -> String {
        match self {
            Clear::Left => "left".to_string(),
            Clear::Right => "right".to_string(),
            Clear::Both => "both".to_string(),
            Clear::None => "none".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Clear::Left => "left".to_string(),
            Clear::Right => "right".to_string(),
            Clear::Both => "both".to_string(),
            Clear::None => "none".to_string(),
        }
    }
}

impl Isolation {
    pub fn to_class_name(&self) -> String {
        match self {
            Isolation::Isolate => "isolate".to_string(),
            Isolation::Auto => "auto".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Isolation::Isolate => "isolate".to_string(),
            Isolation::Auto => "auto".to_string(),
        }
    }
}

impl ObjectFit {
    pub fn to_class_name(&self) -> String {
        match self {
            ObjectFit::Contain => "contain".to_string(),
            ObjectFit::Cover => "cover".to_string(),
            ObjectFit::Fill => "fill".to_string(),
            ObjectFit::None => "none".to_string(),
            ObjectFit::ScaleDown => "scale-down".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            ObjectFit::Contain => "contain".to_string(),
            ObjectFit::Cover => "cover".to_string(),
            ObjectFit::Fill => "fill".to_string(),
            ObjectFit::None => "none".to_string(),
            ObjectFit::ScaleDown => "scale-down".to_string(),
        }
    }
}

impl ObjectPosition {
    pub fn to_class_name(&self) -> String {
        match self {
            ObjectPosition::Bottom => "bottom".to_string(),
            ObjectPosition::Center => "center".to_string(),
            ObjectPosition::Left => "left".to_string(),
            ObjectPosition::LeftBottom => "left-bottom".to_string(),
            ObjectPosition::LeftTop => "left-top".to_string(),
            ObjectPosition::Right => "right".to_string(),
            ObjectPosition::RightBottom => "right-bottom".to_string(),
            ObjectPosition::RightTop => "right-top".to_string(),
            ObjectPosition::Top => "top".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            ObjectPosition::Bottom => "bottom".to_string(),
            ObjectPosition::Center => "center".to_string(),
            ObjectPosition::Left => "left".to_string(),
            ObjectPosition::LeftBottom => "left bottom".to_string(),
            ObjectPosition::LeftTop => "left top".to_string(),
            ObjectPosition::Right => "right".to_string(),
            ObjectPosition::RightBottom => "right bottom".to_string(),
            ObjectPosition::RightTop => "right top".to_string(),
            ObjectPosition::Top => "top".to_string(),
        }
    }
}

impl OverscrollBehavior {
    pub fn to_class_name(&self) -> String {
        match self {
            OverscrollBehavior::Auto => "auto".to_string(),
            OverscrollBehavior::Contain => "contain".to_string(),
            OverscrollBehavior::None => "none".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            OverscrollBehavior::Auto => "auto".to_string(),
            OverscrollBehavior::Contain => "contain".to_string(),
            OverscrollBehavior::None => "none".to_string(),
        }
    }
}

impl Visibility {
    pub fn to_class_name(&self) -> String {
        match self {
            Visibility::Visible => "visible".to_string(),
            Visibility::Hidden => "invisible".to_string(),
            Visibility::Collapse => "collapse".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            Visibility::Visible => "visible".to_string(),
            Visibility::Hidden => "hidden".to_string(),
            Visibility::Collapse => "collapse".to_string(),
        }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for ZIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding display utilities to a class builder
pub trait DisplayUtilities {
    fn display(self, display: Display) -> Self;
}

impl DisplayUtilities for ClassBuilder {
    fn display(self, display: Display) -> Self {
        self.class(display.to_class_name())
    }
}

/// Trait for adding position utilities to a class builder
pub trait PositionUtilities {
    fn position(self, position: Position) -> Self;
}

impl PositionUtilities for ClassBuilder {
    fn position(self, position: Position) -> Self {
        self.class(position.to_class_name())
    }
}

/// Trait for adding overflow utilities to a class builder
pub trait OverflowUtilities {
    fn overflow(self, overflow: Overflow) -> Self;
    fn overflow_x(self, overflow: Overflow) -> Self;
    fn overflow_y(self, overflow: Overflow) -> Self;
}

impl OverflowUtilities for ClassBuilder {
    fn overflow(self, overflow: Overflow) -> Self {
        self.class(format!("overflow-{}", overflow.to_class_name()))
    }
    
    fn overflow_x(self, overflow: Overflow) -> Self {
        self.class(format!("overflow-x-{}", overflow.to_class_name()))
    }
    
    fn overflow_y(self, overflow: Overflow) -> Self {
        self.class(format!("overflow-y-{}", overflow.to_class_name()))
    }
}

/// Trait for adding z-index utilities to a class builder
pub trait ZIndexUtilities {
    fn z_index(self, z_index: ZIndex) -> Self;
}

impl ZIndexUtilities for ClassBuilder {
    fn z_index(self, z_index: ZIndex) -> Self {
        self.class(format!("z-{}", z_index.to_class_name()))
    }
}

/// Trait for adding float utilities to a class builder
pub trait FloatUtilities {
    fn float(self, float: Float) -> Self;
}

impl FloatUtilities for ClassBuilder {
    fn float(self, float: Float) -> Self {
        self.class(format!("float-{}", float.to_class_name()))
    }
}

/// Trait for adding clear utilities to a class builder
pub trait ClearUtilities {
    fn clear(self, clear: Clear) -> Self;
}

impl ClearUtilities for ClassBuilder {
    fn clear(self, clear: Clear) -> Self {
        self.class(format!("clear-{}", clear.to_class_name()))
    }
}

/// Trait for adding isolation utilities to a class builder
pub trait IsolationUtilities {
    fn isolation(self, isolation: Isolation) -> Self;
}

impl IsolationUtilities for ClassBuilder {
    fn isolation(self, isolation: Isolation) -> Self {
        self.class(format!("isolation-{}", isolation.to_class_name()))
    }
}

/// Trait for adding object fit utilities to a class builder
pub trait ObjectFitUtilities {
    fn object_fit(self, object_fit: ObjectFit) -> Self;
}

impl ObjectFitUtilities for ClassBuilder {
    fn object_fit(self, object_fit: ObjectFit) -> Self {
        self.class(format!("object-{}", object_fit.to_class_name()))
    }
}

/// Trait for adding object position utilities to a class builder
pub trait ObjectPositionUtilities {
    fn object_position(self, object_position: ObjectPosition) -> Self;
}

impl ObjectPositionUtilities for ClassBuilder {
    fn object_position(self, object_position: ObjectPosition) -> Self {
        self.class(format!("object-{}", object_position.to_class_name()))
    }
}

/// Trait for adding overscroll behavior utilities to a class builder
pub trait OverscrollBehaviorUtilities {
    fn overscroll_behavior(self, behavior: OverscrollBehavior) -> Self;
    fn overscroll_behavior_x(self, behavior: OverscrollBehavior) -> Self;
    fn overscroll_behavior_y(self, behavior: OverscrollBehavior) -> Self;
}

impl OverscrollBehaviorUtilities for ClassBuilder {
    fn overscroll_behavior(self, behavior: OverscrollBehavior) -> Self {
        self.class(format!("overscroll-{}", behavior.to_class_name()))
    }
    
    fn overscroll_behavior_x(self, behavior: OverscrollBehavior) -> Self {
        self.class(format!("overscroll-x-{}", behavior.to_class_name()))
    }
    
    fn overscroll_behavior_y(self, behavior: OverscrollBehavior) -> Self {
        self.class(format!("overscroll-y-{}", behavior.to_class_name()))
    }
}

/// Trait for adding visibility utilities to a class builder
pub trait VisibilityUtilities {
    fn visibility(self, visibility: Visibility) -> Self;
}

impl VisibilityUtilities for ClassBuilder {
    fn visibility(self, visibility: Visibility) -> Self {
        self.class(visibility.to_class_name())
    }
}

/// Trait for adding positioning utilities to a class builder
pub trait PositioningUtilities {
    /// Add top positioning
    fn top(self, value: crate::utilities::spacing::SpacingValue) -> Self;
    
    /// Add right positioning
    fn right(self, value: crate::utilities::spacing::SpacingValue) -> Self;
    
    /// Add bottom positioning
    fn bottom(self, value: crate::utilities::spacing::SpacingValue) -> Self;
    
    /// Add left positioning
    fn left(self, value: crate::utilities::spacing::SpacingValue) -> Self;
    
    /// Add inset positioning (all sides)
    fn inset(self, value: crate::utilities::spacing::SpacingValue) -> Self;
    
    /// Add horizontal inset positioning
    fn inset_x(self, value: crate::utilities::spacing::SpacingValue) -> Self;
    
    /// Add vertical inset positioning
    fn inset_y(self, value: crate::utilities::spacing::SpacingValue) -> Self;
}

impl PositioningUtilities for ClassBuilder {
    fn top(self, value: crate::utilities::spacing::SpacingValue) -> Self {
        self.class(format!("top-{}", value.to_class_name()))
    }
    
    fn right(self, value: crate::utilities::spacing::SpacingValue) -> Self {
        self.class(format!("right-{}", value.to_class_name()))
    }
    
    fn bottom(self, value: crate::utilities::spacing::SpacingValue) -> Self {
        self.class(format!("bottom-{}", value.to_class_name()))
    }
    
    fn left(self, value: crate::utilities::spacing::SpacingValue) -> Self {
        self.class(format!("left-{}", value.to_class_name()))
    }
    
    fn inset(self, value: crate::utilities::spacing::SpacingValue) -> Self {
        self.class(format!("inset-{}", value.to_class_name()))
    }
    
    fn inset_x(self, value: crate::utilities::spacing::SpacingValue) -> Self {
        self.class(format!("inset-x-{}", value.to_class_name()))
    }
    
    fn inset_y(self, value: crate::utilities::spacing::SpacingValue) -> Self {
        self.class(format!("inset-y-{}", value.to_class_name()))
    }
}

/// Convenience methods for positioning utilities
impl ClassBuilder {
    /// Add top positioning with value 4
    pub fn top_4(self) -> Self {
        self.top(crate::utilities::spacing::SpacingValue::Integer(4))
    }
    
    /// Add right positioning with value 2
    pub fn right_2(self) -> Self {
        self.right(crate::utilities::spacing::SpacingValue::Integer(2))
    }
    
    /// Add bottom positioning with value 6
    pub fn bottom_6(self) -> Self {
        self.bottom(crate::utilities::spacing::SpacingValue::Integer(6))
    }
    
    /// Add left positioning with value 8
    pub fn left_8(self) -> Self {
        self.left(crate::utilities::spacing::SpacingValue::Integer(8))
    }
    
    /// Add inset positioning with value 0
    pub fn inset_0(self) -> Self {
        self.inset(crate::utilities::spacing::SpacingValue::Zero)
    }
    
    /// Add horizontal inset positioning with value 4
    pub fn inset_x_4(self) -> Self {
        self.inset_x(crate::utilities::spacing::SpacingValue::Integer(4))
    }
    
    /// Add vertical inset positioning with value 2
    pub fn inset_y_2(self) -> Self {
        self.inset_y(crate::utilities::spacing::SpacingValue::Integer(2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_display_utilities() {
        let classes = ClassBuilder::new()
            .display(Display::Block)
            .display(Display::Flex)
            .display(Display::Grid)
            .display(Display::Hidden)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("block"));
        assert!(css_classes.contains("flex"));
        assert!(css_classes.contains("grid"));
        assert!(css_classes.contains("hidden"));
    }
    
    #[test]
    fn test_position_utilities() {
        let classes = ClassBuilder::new()
            .position(Position::Static)
            .position(Position::Relative)
            .position(Position::Absolute)
            .position(Position::Fixed)
            .position(Position::Sticky)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("static"));
        assert!(css_classes.contains("relative"));
        assert!(css_classes.contains("absolute"));
        assert!(css_classes.contains("fixed"));
        assert!(css_classes.contains("sticky"));
    }
    
    #[test]
    fn test_overflow_utilities() {
        let classes = ClassBuilder::new()
            .overflow(Overflow::Auto)
            .overflow(Overflow::Hidden)
            .overflow(Overflow::Visible)
            .overflow(Overflow::Scroll)
            .overflow_x(Overflow::Hidden)
            .overflow_y(Overflow::Scroll)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("overflow-auto"));
        assert!(css_classes.contains("overflow-hidden"));
        assert!(css_classes.contains("overflow-visible"));
        assert!(css_classes.contains("overflow-scroll"));
        assert!(css_classes.contains("overflow-x-hidden"));
        assert!(css_classes.contains("overflow-y-scroll"));
    }
    
    #[test]
    fn test_z_index_utilities() {
        let classes = ClassBuilder::new()
            .z_index(ZIndex::Auto)
            .z_index(ZIndex::Zero)
            .z_index(ZIndex::Ten)
            .z_index(ZIndex::Fifty)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("z-auto"));
        assert!(css_classes.contains("z-0"));
        assert!(css_classes.contains("z-10"));
        assert!(css_classes.contains("z-50"));
    }
    
    #[test]
    fn test_float_utilities() {
        let classes = ClassBuilder::new()
            .float(Float::Left)
            .float(Float::Right)
            .float(Float::None)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("float-left"));
        assert!(css_classes.contains("float-right"));
        assert!(css_classes.contains("float-none"));
    }
    
    #[test]
    fn test_clear_utilities() {
        let classes = ClassBuilder::new()
            .clear(Clear::Left)
            .clear(Clear::Right)
            .clear(Clear::Both)
            .clear(Clear::None)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("clear-left"));
        assert!(css_classes.contains("clear-right"));
        assert!(css_classes.contains("clear-both"));
        assert!(css_classes.contains("clear-none"));
    }
    
    #[test]
    fn test_isolation_utilities() {
        let classes = ClassBuilder::new()
            .isolation(Isolation::Isolate)
            .isolation(Isolation::Auto)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("isolation-isolate"));
        assert!(css_classes.contains("isolation-auto"));
    }
    
    #[test]
    fn test_object_fit_utilities() {
        let classes = ClassBuilder::new()
            .object_fit(ObjectFit::Contain)
            .object_fit(ObjectFit::Cover)
            .object_fit(ObjectFit::Fill)
            .object_fit(ObjectFit::None)
            .object_fit(ObjectFit::ScaleDown)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("object-contain"));
        assert!(css_classes.contains("object-cover"));
        assert!(css_classes.contains("object-fill"));
        assert!(css_classes.contains("object-none"));
        assert!(css_classes.contains("object-scale-down"));
    }
    
    #[test]
    fn test_object_position_utilities() {
        let classes = ClassBuilder::new()
            .object_position(ObjectPosition::Center)
            .object_position(ObjectPosition::Top)
            .object_position(ObjectPosition::Bottom)
            .object_position(ObjectPosition::Left)
            .object_position(ObjectPosition::Right)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("object-center"));
        assert!(css_classes.contains("object-top"));
        assert!(css_classes.contains("object-bottom"));
        assert!(css_classes.contains("object-left"));
        assert!(css_classes.contains("object-right"));
    }
    
    #[test]
    fn test_overscroll_behavior_utilities() {
        let classes = ClassBuilder::new()
            .overscroll_behavior(OverscrollBehavior::Auto)
            .overscroll_behavior(OverscrollBehavior::Contain)
            .overscroll_behavior(OverscrollBehavior::None)
            .overscroll_behavior_x(OverscrollBehavior::Contain)
            .overscroll_behavior_y(OverscrollBehavior::None)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("overscroll-auto"));
        assert!(css_classes.contains("overscroll-contain"));
        assert!(css_classes.contains("overscroll-none"));
        assert!(css_classes.contains("overscroll-x-contain"));
        assert!(css_classes.contains("overscroll-y-none"));
    }
    
    #[test]
    fn test_visibility_utilities() {
        let classes = ClassBuilder::new()
            .visibility(Visibility::Visible)
            .visibility(Visibility::Hidden)
            .visibility(Visibility::Collapse)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("visible"));
        assert!(css_classes.contains("invisible"));
        assert!(css_classes.contains("collapse"));
    }
    
    #[test]
    fn test_complex_layout_combination() {
        let classes = ClassBuilder::new()
            .display(Display::Flex)
            .position(Position::Relative)
            .overflow(Overflow::Hidden)
            .z_index(ZIndex::Ten)
            .float(Float::None)
            .clear(Clear::Both)
            .isolation(Isolation::Isolate)
            .object_fit(ObjectFit::Cover)
            .object_position(ObjectPosition::Center)
            .overscroll_behavior(OverscrollBehavior::Contain)
            .visibility(Visibility::Visible)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("flex"));
        assert!(css_classes.contains("relative"));
        assert!(css_classes.contains("overflow-hidden"));
        assert!(css_classes.contains("z-10"));
        assert!(css_classes.contains("float-none"));
        assert!(css_classes.contains("clear-both"));
        assert!(css_classes.contains("isolation-isolate"));
        assert!(css_classes.contains("object-cover"));
        assert!(css_classes.contains("object-center"));
        assert!(css_classes.contains("overscroll-contain"));
        assert!(css_classes.contains("visible"));
    }

    /// Test that positioning utilities (top/right/bottom/left) are implemented
    #[test]
    fn test_positioning_utilities() {
        // This test will fail until we implement positioning utilities
        let classes = ClassBuilder::new()
            .top_4()  // top-4
            .right_2()  // right-2
            .bottom_6()  // bottom-6
            .left_8()  // left-8
            .inset_0()  // inset-0
            .inset_x_4()  // inset-x-4
            .inset_y_2()  // inset-y-2
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("top-4"));
        assert!(css_classes.contains("right-2"));
        assert!(css_classes.contains("bottom-6"));
        assert!(css_classes.contains("left-8"));
        assert!(css_classes.contains("inset-0"));
        assert!(css_classes.contains("inset-x-4"));
        assert!(css_classes.contains("inset-y-2"));
    }
}
