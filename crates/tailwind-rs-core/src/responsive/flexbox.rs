//! # Flexbox Responsive Utilities
//!
//! This module provides flexbox-specific responsive utilities.

use super::breakpoints::Breakpoint;
use super::responsive_values::ResponsiveValue;
use serde::{Deserialize, Serialize};

/// Flex direction options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexDirection {
    /// Row direction (default)
    Row,
    /// Row reverse direction
    RowReverse,
    /// Column direction
    Column,
    /// Column reverse direction
    ColumnReverse,
}

impl FlexDirection {
    /// Get the CSS class for this flex direction
    pub fn to_class(&self) -> &'static str {
        match self {
            FlexDirection::Row => "flex-row",
            FlexDirection::RowReverse => "flex-row-reverse",
            FlexDirection::Column => "flex-col",
            FlexDirection::ColumnReverse => "flex-col-reverse",
        }
    }
}

/// Flex wrap options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexWrap {
    /// No wrap
    NoWrap,
    /// Wrap
    Wrap,
    /// Wrap reverse
    WrapReverse,
}

impl FlexWrap {
    /// Get the CSS class for this flex wrap
    pub fn to_class(&self) -> &'static str {
        match self {
            FlexWrap::NoWrap => "flex-nowrap",
            FlexWrap::Wrap => "flex-wrap",
            FlexWrap::WrapReverse => "flex-wrap-reverse",
        }
    }
}

/// Justify content options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JustifyContent {
    /// Start alignment
    Start,
    /// End alignment
    End,
    /// Center alignment
    Center,
    /// Space between
    Between,
    /// Space around
    Around,
    /// Space evenly
    Evenly,
}

impl JustifyContent {
    /// Get the CSS class for this justify content
    pub fn to_class(&self) -> &'static str {
        match self {
            JustifyContent::Start => "justify-start",
            JustifyContent::End => "justify-end",
            JustifyContent::Center => "justify-center",
            JustifyContent::Between => "justify-between",
            JustifyContent::Around => "justify-around",
            JustifyContent::Evenly => "justify-evenly",
        }
    }
}

/// Align items options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignItems {
    /// Start alignment
    Start,
    /// End alignment
    End,
    /// Center alignment
    Center,
    /// Baseline alignment
    Baseline,
    /// Stretch alignment
    Stretch,
}

impl AlignItems {
    /// Get the CSS class for this align items
    pub fn to_class(&self) -> &'static str {
        match self {
            AlignItems::Start => "items-start",
            AlignItems::End => "items-end",
            AlignItems::Center => "items-center",
            AlignItems::Baseline => "items-baseline",
            AlignItems::Stretch => "items-stretch",
        }
    }
}

/// Responsive flex container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveFlex {
    /// Flex direction for each breakpoint
    pub direction: ResponsiveValue<FlexDirection>,
    /// Flex wrap for each breakpoint
    pub wrap: ResponsiveValue<FlexWrap>,
    /// Justify content for each breakpoint
    pub justify: ResponsiveValue<JustifyContent>,
    /// Align items for each breakpoint
    pub align: ResponsiveValue<AlignItems>,
    /// Gap for each breakpoint
    pub gap: ResponsiveValue<u32>,
}

impl ResponsiveFlex {
    /// Create a new responsive flex container
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a responsive flex container with base values
    pub fn with_base(direction: FlexDirection, wrap: FlexWrap, justify: JustifyContent, align: AlignItems, gap: u32) -> Self {
        Self {
            direction: ResponsiveValue::with_base(direction),
            wrap: ResponsiveValue::with_base(wrap),
            justify: ResponsiveValue::with_base(justify),
            align: ResponsiveValue::with_base(align),
            gap: ResponsiveValue::with_base(gap),
        }
    }
    
    /// Set flex direction for a specific breakpoint
    pub fn set_direction(&mut self, breakpoint: Breakpoint, direction: FlexDirection) {
        self.direction.set_breakpoint(breakpoint, direction);
    }
    
    /// Set flex wrap for a specific breakpoint
    pub fn set_wrap(&mut self, breakpoint: Breakpoint, wrap: FlexWrap) {
        self.wrap.set_breakpoint(breakpoint, wrap);
    }
    
    /// Set justify content for a specific breakpoint
    pub fn set_justify(&mut self, breakpoint: Breakpoint, justify: JustifyContent) {
        self.justify.set_breakpoint(breakpoint, justify);
    }
    
    /// Set align items for a specific breakpoint
    pub fn set_align(&mut self, breakpoint: Breakpoint, align: AlignItems) {
        self.align.set_breakpoint(breakpoint, align);
    }
    
    /// Set gap for a specific breakpoint
    pub fn set_gap(&mut self, breakpoint: Breakpoint, gap: u32) {
        self.gap.set_breakpoint(breakpoint, gap);
    }
    
    /// Get flex direction for a specific breakpoint
    pub fn get_direction(&self, breakpoint: Breakpoint) -> Option<FlexDirection> {
        self.direction.get_breakpoint(breakpoint).copied()
    }
    
    /// Get flex wrap for a specific breakpoint
    pub fn get_wrap(&self, breakpoint: Breakpoint) -> Option<FlexWrap> {
        self.wrap.get_breakpoint(breakpoint).copied()
    }
    
    /// Get justify content for a specific breakpoint
    pub fn get_justify(&self, breakpoint: Breakpoint) -> Option<JustifyContent> {
        self.justify.get_breakpoint(breakpoint).copied()
    }
    
    /// Get align items for a specific breakpoint
    pub fn get_align(&self, breakpoint: Breakpoint) -> Option<AlignItems> {
        self.align.get_breakpoint(breakpoint).copied()
    }
    
    /// Get gap for a specific breakpoint
    pub fn get_gap(&self, breakpoint: Breakpoint) -> Option<u32> {
        self.gap.get_breakpoint(breakpoint).copied()
    }
    
    /// Generate CSS classes for all breakpoints
    pub fn to_css_classes(&self) -> String {
        let mut classes = Vec::new();
        
        // Add flex direction classes
        let direction_classes = self.direction.to_css_classes(|d| d.to_class().to_string());
        if !direction_classes.is_empty() {
            classes.push(direction_classes);
        }
        
        // Add flex wrap classes
        let wrap_classes = self.wrap.to_css_classes(|w| w.to_class().to_string());
        if !wrap_classes.is_empty() {
            classes.push(wrap_classes);
        }
        
        // Add justify content classes
        let justify_classes = self.justify.to_css_classes(|j| j.to_class().to_string());
        if !justify_classes.is_empty() {
            classes.push(justify_classes);
        }
        
        // Add align items classes
        let align_classes = self.align.to_css_classes(|a| a.to_class().to_string());
        if !align_classes.is_empty() {
            classes.push(align_classes);
        }
        
        // Add gap classes
        let gap_classes = self.gap.to_css_classes(|g| {
            if *g == 0 {
                "gap-0".to_string()
            } else {
                format!("gap-{}", g)
            }
        });
        if !gap_classes.is_empty() {
            classes.push(gap_classes);
        }
        
        classes.join(" ")
    }
    
    /// Generate CSS classes for a specific screen width
    pub fn to_css_classes_for_width(&self, screen_width: u32) -> String {
        let mut classes = Vec::new();
        
        // Add flex direction classes
        if let Some(direction) = self.direction.get_for_width(screen_width) {
            classes.push(direction.to_class().to_string());
        }
        
        // Add flex wrap classes
        if let Some(wrap) = self.wrap.get_for_width(screen_width) {
            classes.push(wrap.to_class().to_string());
        }
        
        // Add justify content classes
        if let Some(justify) = self.justify.get_for_width(screen_width) {
            classes.push(justify.to_class().to_string());
        }
        
        // Add align items classes
        if let Some(align) = self.align.get_for_width(screen_width) {
            classes.push(align.to_class().to_string());
        }
        
        // Add gap classes
        if let Some(gap) = self.gap.get_for_width(screen_width) {
            if *gap == 0 {
                classes.push("gap-0".to_string());
            } else {
                classes.push(format!("gap-{}", gap));
            }
        }
        
        classes.join(" ")
    }
}

impl Default for ResponsiveFlex {
    fn default() -> Self {
        Self {
            direction: ResponsiveValue::with_base(FlexDirection::Row),
            wrap: ResponsiveValue::with_base(FlexWrap::NoWrap),
            justify: ResponsiveValue::with_base(JustifyContent::Start),
            align: ResponsiveValue::with_base(AlignItems::Stretch),
            gap: ResponsiveValue::with_base(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_direction_to_class() {
        assert_eq!(FlexDirection::Row.to_class(), "flex-row");
        assert_eq!(FlexDirection::RowReverse.to_class(), "flex-row-reverse");
        assert_eq!(FlexDirection::Column.to_class(), "flex-col");
        assert_eq!(FlexDirection::ColumnReverse.to_class(), "flex-col-reverse");
    }

    #[test]
    fn test_flex_wrap_to_class() {
        assert_eq!(FlexWrap::NoWrap.to_class(), "flex-nowrap");
        assert_eq!(FlexWrap::Wrap.to_class(), "flex-wrap");
        assert_eq!(FlexWrap::WrapReverse.to_class(), "flex-wrap-reverse");
    }

    #[test]
    fn test_justify_content_to_class() {
        assert_eq!(JustifyContent::Start.to_class(), "justify-start");
        assert_eq!(JustifyContent::End.to_class(), "justify-end");
        assert_eq!(JustifyContent::Center.to_class(), "justify-center");
        assert_eq!(JustifyContent::Between.to_class(), "justify-between");
        assert_eq!(JustifyContent::Around.to_class(), "justify-around");
        assert_eq!(JustifyContent::Evenly.to_class(), "justify-evenly");
    }

    #[test]
    fn test_align_items_to_class() {
        assert_eq!(AlignItems::Start.to_class(), "items-start");
        assert_eq!(AlignItems::End.to_class(), "items-end");
        assert_eq!(AlignItems::Center.to_class(), "items-center");
        assert_eq!(AlignItems::Baseline.to_class(), "items-baseline");
        assert_eq!(AlignItems::Stretch.to_class(), "items-stretch");
    }

    #[test]
    fn test_responsive_flex_new() {
        let flex = ResponsiveFlex::new();
        assert_eq!(flex.get_direction(Breakpoint::Base), Some(FlexDirection::Row));
        assert_eq!(flex.get_wrap(Breakpoint::Base), Some(FlexWrap::NoWrap));
        assert_eq!(flex.get_justify(Breakpoint::Base), Some(JustifyContent::Start));
        assert_eq!(flex.get_align(Breakpoint::Base), Some(AlignItems::Stretch));
        assert_eq!(flex.get_gap(Breakpoint::Base), Some(0));
    }

    #[test]
    fn test_responsive_flex_with_base() {
        let flex = ResponsiveFlex::with_base(
            FlexDirection::Column,
            FlexWrap::Wrap,
            JustifyContent::Center,
            AlignItems::Center,
            4,
        );
        
        assert_eq!(flex.get_direction(Breakpoint::Base), Some(FlexDirection::Column));
        assert_eq!(flex.get_wrap(Breakpoint::Base), Some(FlexWrap::Wrap));
        assert_eq!(flex.get_justify(Breakpoint::Base), Some(JustifyContent::Center));
        assert_eq!(flex.get_align(Breakpoint::Base), Some(AlignItems::Center));
        assert_eq!(flex.get_gap(Breakpoint::Base), Some(4));
    }

    #[test]
    fn test_responsive_flex_set_get() {
        let mut flex = ResponsiveFlex::new();
        
        flex.set_direction(Breakpoint::Sm, FlexDirection::Column);
        flex.set_wrap(Breakpoint::Md, FlexWrap::Wrap);
        flex.set_justify(Breakpoint::Lg, JustifyContent::Between);
        flex.set_align(Breakpoint::Xl, AlignItems::Center);
        flex.set_gap(Breakpoint::Xl2, 8);
        
        assert_eq!(flex.get_direction(Breakpoint::Sm), Some(FlexDirection::Column));
        assert_eq!(flex.get_wrap(Breakpoint::Md), Some(FlexWrap::Wrap));
        assert_eq!(flex.get_justify(Breakpoint::Lg), Some(JustifyContent::Between));
        assert_eq!(flex.get_align(Breakpoint::Xl), Some(AlignItems::Center));
        assert_eq!(flex.get_gap(Breakpoint::Xl2), Some(8));
    }

    #[test]
    fn test_responsive_flex_to_css_classes() {
        let mut flex = ResponsiveFlex::new();
        flex.set_direction(Breakpoint::Sm, FlexDirection::Column);
        flex.set_justify(Breakpoint::Md, JustifyContent::Center);
        flex.set_gap(Breakpoint::Lg, 4);
        
        let classes = flex.to_css_classes();
        assert!(classes.contains("flex-row"));
        assert!(classes.contains("sm:flex-col"));
        assert!(classes.contains("md:justify-center"));
        assert!(classes.contains("lg:gap-4"));
    }

    #[test]
    fn test_responsive_flex_to_css_classes_for_width() {
        let mut flex = ResponsiveFlex::new();
        flex.set_direction(Breakpoint::Sm, FlexDirection::Column);
        flex.set_justify(Breakpoint::Md, JustifyContent::Center);
        flex.set_gap(Breakpoint::Lg, 4);
        
        // Test width 0 (base only)
        let classes_0 = flex.to_css_classes_for_width(0);
        assert!(classes_0.contains("flex-row"));
        assert!(!classes_0.contains("flex-col"));
        assert!(!classes_0.contains("justify-center"));
        assert!(!classes_0.contains("gap-4"));
        
        // Test width 640 (sm active)
        let classes_640 = flex.to_css_classes_for_width(640);
        assert!(classes_640.contains("flex-col"));
        assert!(!classes_640.contains("justify-center"));
        assert!(!classes_640.contains("gap-4"));
        
        // Test width 768 (md active)
        let classes_768 = flex.to_css_classes_for_width(768);
        assert!(classes_768.contains("flex-col"));
        assert!(classes_768.contains("justify-center"));
        assert!(!classes_768.contains("gap-4"));
        
        // Test width 1024 (lg active)
        let classes_1024 = flex.to_css_classes_for_width(1024);
        assert!(classes_1024.contains("flex-col"));
        assert!(classes_1024.contains("justify-center"));
        assert!(classes_1024.contains("gap-4"));
    }
}
