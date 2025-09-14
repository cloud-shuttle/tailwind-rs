//! Grid utilities for tailwind-rs
//!
//! This module provides utilities for CSS Grid layout including grid template columns,
//! grid template rows, grid column span, grid row span, grid auto flow, and gap utilities.

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Grid template columns values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridTemplateColumns {
    /// None columns
    None,
    /// Subgrid columns
    Subgrid,
    /// Auto columns
    Auto,
    /// 1 column
    One,
    /// 2 columns
    Two,
    /// 3 columns
    Three,
    /// 4 columns
    Four,
    /// 5 columns
    Five,
    /// 6 columns
    Six,
    /// 7 columns
    Seven,
    /// 8 columns
    Eight,
    /// 9 columns
    Nine,
    /// 10 columns
    Ten,
    /// 11 columns
    Eleven,
    /// 12 columns
    Twelve,
}

/// Grid template rows values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridTemplateRows {
    /// None rows
    None,
    /// Subgrid rows
    Subgrid,
    /// Auto rows
    Auto,
    /// 1 row
    One,
    /// 2 rows
    Two,
    /// 3 rows
    Three,
    /// 4 rows
    Four,
    /// 5 rows
    Five,
    /// 6 rows
    Six,
}

/// Grid column span values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridColumnSpan {
    /// Auto span
    Auto,
    /// Span 1
    One,
    /// Span 2
    Two,
    /// Span 3
    Three,
    /// Span 4
    Four,
    /// Span 5
    Five,
    /// Span 6
    Six,
    /// Span 7
    Seven,
    /// Span 8
    Eight,
    /// Span 9
    Nine,
    /// Span 10
    Ten,
    /// Span 11
    Eleven,
    /// Span 12
    Twelve,
    /// Full span
    Full,
}

/// Grid row span values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridRowSpan {
    /// Auto span
    Auto,
    /// Span 1
    One,
    /// Span 2
    Two,
    /// Span 3
    Three,
    /// Span 4
    Four,
    /// Span 5
    Five,
    /// Span 6
    Six,
    /// Full span
    Full,
}

/// Grid column start values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridColumnStart {
    /// Auto start
    Auto,
    /// Start 1
    One,
    /// Start 2
    Two,
    /// Start 3
    Three,
    /// Start 4
    Four,
    /// Start 5
    Five,
    /// Start 6
    Six,
    /// Start 7
    Seven,
    /// Start 8
    Eight,
    /// Start 9
    Nine,
    /// Start 10
    Ten,
    /// Start 11
    Eleven,
    /// Start 12
    Twelve,
    /// Start 13
    Thirteen,
}

/// Grid row start values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridRowStart {
    /// Auto start
    Auto,
    /// Start 1
    One,
    /// Start 2
    Two,
    /// Start 3
    Three,
    /// Start 4
    Four,
    /// Start 5
    Five,
    /// Start 6
    Six,
    /// Start 7
    Seven,
}

/// Grid column end values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridColumnEnd {
    /// Auto end
    Auto,
    /// End 1
    One,
    /// End 2
    Two,
    /// End 3
    Three,
    /// End 4
    Four,
    /// End 5
    Five,
    /// End 6
    Six,
    /// End 7
    Seven,
    /// End 8
    Eight,
    /// End 9
    Nine,
    /// End 10
    Ten,
    /// End 11
    Eleven,
    /// End 12
    Twelve,
    /// End 13
    Thirteen,
}

/// Grid row end values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridRowEnd {
    /// Auto end
    Auto,
    /// End 1
    One,
    /// End 2
    Two,
    /// End 3
    Three,
    /// End 4
    Four,
    /// End 5
    Five,
    /// End 6
    Six,
    /// End 7
    Seven,
}

/// Grid auto flow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridAutoFlow {
    /// Row auto flow
    Row,
    /// Column auto flow
    Column,
    /// Dense row auto flow
    Dense,
    /// Dense column auto flow
    DenseColumn,
}

/// Grid auto columns values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridAutoColumns {
    /// Auto columns
    Auto,
    /// Min columns
    Min,
    /// Max columns
    Max,
    /// Fr columns
    Fr,
}

/// Grid auto rows values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridAutoRows {
    /// Auto rows
    Auto,
    /// Min rows
    Min,
    /// Max rows
    Max,
    /// Fr rows
    Fr,
}

impl GridTemplateColumns {
    pub fn to_class_name(&self) -> String {
        match self {
            GridTemplateColumns::None => "none".to_string(),
            GridTemplateColumns::Subgrid => "subgrid".to_string(),
            GridTemplateColumns::Auto => "auto".to_string(),
            GridTemplateColumns::One => "1".to_string(),
            GridTemplateColumns::Two => "2".to_string(),
            GridTemplateColumns::Three => "3".to_string(),
            GridTemplateColumns::Four => "4".to_string(),
            GridTemplateColumns::Five => "5".to_string(),
            GridTemplateColumns::Six => "6".to_string(),
            GridTemplateColumns::Seven => "7".to_string(),
            GridTemplateColumns::Eight => "8".to_string(),
            GridTemplateColumns::Nine => "9".to_string(),
            GridTemplateColumns::Ten => "10".to_string(),
            GridTemplateColumns::Eleven => "11".to_string(),
            GridTemplateColumns::Twelve => "12".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            GridTemplateColumns::None => "none".to_string(),
            GridTemplateColumns::Subgrid => "subgrid".to_string(),
            GridTemplateColumns::Auto => "repeat(auto-fit, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::One => "repeat(1, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::Two => "repeat(2, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::Three => "repeat(3, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::Four => "repeat(4, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::Five => "repeat(5, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::Six => "repeat(6, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::Seven => "repeat(7, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::Eight => "repeat(8, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::Nine => "repeat(9, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::Ten => "repeat(10, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::Eleven => "repeat(11, minmax(0, 1fr))".to_string(),
            GridTemplateColumns::Twelve => "repeat(12, minmax(0, 1fr))".to_string(),
        }
    }
}

impl GridTemplateRows {
    pub fn to_class_name(&self) -> String {
        match self {
            GridTemplateRows::None => "none".to_string(),
            GridTemplateRows::Subgrid => "subgrid".to_string(),
            GridTemplateRows::Auto => "auto".to_string(),
            GridTemplateRows::One => "1".to_string(),
            GridTemplateRows::Two => "2".to_string(),
            GridTemplateRows::Three => "3".to_string(),
            GridTemplateRows::Four => "4".to_string(),
            GridTemplateRows::Five => "5".to_string(),
            GridTemplateRows::Six => "6".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            GridTemplateRows::None => "none".to_string(),
            GridTemplateRows::Subgrid => "subgrid".to_string(),
            GridTemplateRows::Auto => "repeat(auto-fit, minmax(0, 1fr))".to_string(),
            GridTemplateRows::One => "repeat(1, minmax(0, 1fr))".to_string(),
            GridTemplateRows::Two => "repeat(2, minmax(0, 1fr))".to_string(),
            GridTemplateRows::Three => "repeat(3, minmax(0, 1fr))".to_string(),
            GridTemplateRows::Four => "repeat(4, minmax(0, 1fr))".to_string(),
            GridTemplateRows::Five => "repeat(5, minmax(0, 1fr))".to_string(),
            GridTemplateRows::Six => "repeat(6, minmax(0, 1fr))".to_string(),
        }
    }
}

impl GridColumnSpan {
    pub fn to_class_name(&self) -> String {
        match self {
            GridColumnSpan::Auto => "auto".to_string(),
            GridColumnSpan::One => "1".to_string(),
            GridColumnSpan::Two => "2".to_string(),
            GridColumnSpan::Three => "3".to_string(),
            GridColumnSpan::Four => "4".to_string(),
            GridColumnSpan::Five => "5".to_string(),
            GridColumnSpan::Six => "6".to_string(),
            GridColumnSpan::Seven => "7".to_string(),
            GridColumnSpan::Eight => "8".to_string(),
            GridColumnSpan::Nine => "9".to_string(),
            GridColumnSpan::Ten => "10".to_string(),
            GridColumnSpan::Eleven => "11".to_string(),
            GridColumnSpan::Twelve => "12".to_string(),
            GridColumnSpan::Full => "full".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            GridColumnSpan::Auto => "auto".to_string(),
            GridColumnSpan::One => "span 1 / span 1".to_string(),
            GridColumnSpan::Two => "span 2 / span 2".to_string(),
            GridColumnSpan::Three => "span 3 / span 3".to_string(),
            GridColumnSpan::Four => "span 4 / span 4".to_string(),
            GridColumnSpan::Five => "span 5 / span 5".to_string(),
            GridColumnSpan::Six => "span 6 / span 6".to_string(),
            GridColumnSpan::Seven => "span 7 / span 7".to_string(),
            GridColumnSpan::Eight => "span 8 / span 8".to_string(),
            GridColumnSpan::Nine => "span 9 / span 9".to_string(),
            GridColumnSpan::Ten => "span 10 / span 10".to_string(),
            GridColumnSpan::Eleven => "span 11 / span 11".to_string(),
            GridColumnSpan::Twelve => "span 12 / span 12".to_string(),
            GridColumnSpan::Full => "1 / -1".to_string(),
        }
    }
}

impl GridRowSpan {
    pub fn to_class_name(&self) -> String {
        match self {
            GridRowSpan::Auto => "auto".to_string(),
            GridRowSpan::One => "1".to_string(),
            GridRowSpan::Two => "2".to_string(),
            GridRowSpan::Three => "3".to_string(),
            GridRowSpan::Four => "4".to_string(),
            GridRowSpan::Five => "5".to_string(),
            GridRowSpan::Six => "6".to_string(),
            GridRowSpan::Full => "full".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            GridRowSpan::Auto => "auto".to_string(),
            GridRowSpan::One => "span 1 / span 1".to_string(),
            GridRowSpan::Two => "span 2 / span 2".to_string(),
            GridRowSpan::Three => "span 3 / span 3".to_string(),
            GridRowSpan::Four => "span 4 / span 4".to_string(),
            GridRowSpan::Five => "span 5 / span 5".to_string(),
            GridRowSpan::Six => "span 6 / span 6".to_string(),
            GridRowSpan::Full => "1 / -1".to_string(),
        }
    }
}

impl GridColumnStart {
    pub fn to_class_name(&self) -> String {
        match self {
            GridColumnStart::Auto => "auto".to_string(),
            GridColumnStart::One => "1".to_string(),
            GridColumnStart::Two => "2".to_string(),
            GridColumnStart::Three => "3".to_string(),
            GridColumnStart::Four => "4".to_string(),
            GridColumnStart::Five => "5".to_string(),
            GridColumnStart::Six => "6".to_string(),
            GridColumnStart::Seven => "7".to_string(),
            GridColumnStart::Eight => "8".to_string(),
            GridColumnStart::Nine => "9".to_string(),
            GridColumnStart::Ten => "10".to_string(),
            GridColumnStart::Eleven => "11".to_string(),
            GridColumnStart::Twelve => "12".to_string(),
            GridColumnStart::Thirteen => "13".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            GridColumnStart::Auto => "auto".to_string(),
            GridColumnStart::One => "1".to_string(),
            GridColumnStart::Two => "2".to_string(),
            GridColumnStart::Three => "3".to_string(),
            GridColumnStart::Four => "4".to_string(),
            GridColumnStart::Five => "5".to_string(),
            GridColumnStart::Six => "6".to_string(),
            GridColumnStart::Seven => "7".to_string(),
            GridColumnStart::Eight => "8".to_string(),
            GridColumnStart::Nine => "9".to_string(),
            GridColumnStart::Ten => "10".to_string(),
            GridColumnStart::Eleven => "11".to_string(),
            GridColumnStart::Twelve => "12".to_string(),
            GridColumnStart::Thirteen => "13".to_string(),
        }
    }
}

impl GridRowStart {
    pub fn to_class_name(&self) -> String {
        match self {
            GridRowStart::Auto => "auto".to_string(),
            GridRowStart::One => "1".to_string(),
            GridRowStart::Two => "2".to_string(),
            GridRowStart::Three => "3".to_string(),
            GridRowStart::Four => "4".to_string(),
            GridRowStart::Five => "5".to_string(),
            GridRowStart::Six => "6".to_string(),
            GridRowStart::Seven => "7".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            GridRowStart::Auto => "auto".to_string(),
            GridRowStart::One => "1".to_string(),
            GridRowStart::Two => "2".to_string(),
            GridRowStart::Three => "3".to_string(),
            GridRowStart::Four => "4".to_string(),
            GridRowStart::Five => "5".to_string(),
            GridRowStart::Six => "6".to_string(),
            GridRowStart::Seven => "7".to_string(),
        }
    }
}

impl GridColumnEnd {
    pub fn to_class_name(&self) -> String {
        match self {
            GridColumnEnd::Auto => "auto".to_string(),
            GridColumnEnd::One => "1".to_string(),
            GridColumnEnd::Two => "2".to_string(),
            GridColumnEnd::Three => "3".to_string(),
            GridColumnEnd::Four => "4".to_string(),
            GridColumnEnd::Five => "5".to_string(),
            GridColumnEnd::Six => "6".to_string(),
            GridColumnEnd::Seven => "7".to_string(),
            GridColumnEnd::Eight => "8".to_string(),
            GridColumnEnd::Nine => "9".to_string(),
            GridColumnEnd::Ten => "10".to_string(),
            GridColumnEnd::Eleven => "11".to_string(),
            GridColumnEnd::Twelve => "12".to_string(),
            GridColumnEnd::Thirteen => "13".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            GridColumnEnd::Auto => "auto".to_string(),
            GridColumnEnd::One => "1".to_string(),
            GridColumnEnd::Two => "2".to_string(),
            GridColumnEnd::Three => "3".to_string(),
            GridColumnEnd::Four => "4".to_string(),
            GridColumnEnd::Five => "5".to_string(),
            GridColumnEnd::Six => "6".to_string(),
            GridColumnEnd::Seven => "7".to_string(),
            GridColumnEnd::Eight => "8".to_string(),
            GridColumnEnd::Nine => "9".to_string(),
            GridColumnEnd::Ten => "10".to_string(),
            GridColumnEnd::Eleven => "11".to_string(),
            GridColumnEnd::Twelve => "12".to_string(),
            GridColumnEnd::Thirteen => "13".to_string(),
        }
    }
}

impl GridRowEnd {
    pub fn to_class_name(&self) -> String {
        match self {
            GridRowEnd::Auto => "auto".to_string(),
            GridRowEnd::One => "1".to_string(),
            GridRowEnd::Two => "2".to_string(),
            GridRowEnd::Three => "3".to_string(),
            GridRowEnd::Four => "4".to_string(),
            GridRowEnd::Five => "5".to_string(),
            GridRowEnd::Six => "6".to_string(),
            GridRowEnd::Seven => "7".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            GridRowEnd::Auto => "auto".to_string(),
            GridRowEnd::One => "1".to_string(),
            GridRowEnd::Two => "2".to_string(),
            GridRowEnd::Three => "3".to_string(),
            GridRowEnd::Four => "4".to_string(),
            GridRowEnd::Five => "5".to_string(),
            GridRowEnd::Six => "6".to_string(),
            GridRowEnd::Seven => "7".to_string(),
        }
    }
}

impl GridAutoFlow {
    pub fn to_class_name(&self) -> String {
        match self {
            GridAutoFlow::Row => "row".to_string(),
            GridAutoFlow::Column => "col".to_string(),
            GridAutoFlow::Dense => "dense".to_string(),
            GridAutoFlow::DenseColumn => "col-dense".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            GridAutoFlow::Row => "row".to_string(),
            GridAutoFlow::Column => "column".to_string(),
            GridAutoFlow::Dense => "row dense".to_string(),
            GridAutoFlow::DenseColumn => "column dense".to_string(),
        }
    }
}

impl GridAutoColumns {
    pub fn to_class_name(&self) -> String {
        match self {
            GridAutoColumns::Auto => "auto".to_string(),
            GridAutoColumns::Min => "min".to_string(),
            GridAutoColumns::Max => "max".to_string(),
            GridAutoColumns::Fr => "fr".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            GridAutoColumns::Auto => "auto".to_string(),
            GridAutoColumns::Min => "min-content".to_string(),
            GridAutoColumns::Max => "max-content".to_string(),
            GridAutoColumns::Fr => "minmax(0, 1fr)".to_string(),
        }
    }
}

impl GridAutoRows {
    pub fn to_class_name(&self) -> String {
        match self {
            GridAutoRows::Auto => "auto".to_string(),
            GridAutoRows::Min => "min".to_string(),
            GridAutoRows::Max => "max".to_string(),
            GridAutoRows::Fr => "fr".to_string(),
        }
    }
    
    pub fn to_css_value(&self) -> String {
        match self {
            GridAutoRows::Auto => "auto".to_string(),
            GridAutoRows::Min => "min-content".to_string(),
            GridAutoRows::Max => "max-content".to_string(),
            GridAutoRows::Fr => "minmax(0, 1fr)".to_string(),
        }
    }
}

impl fmt::Display for GridTemplateColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GridTemplateRows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GridColumnSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GridRowSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GridColumnStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GridRowStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GridColumnEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GridRowEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GridAutoFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GridAutoColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

impl fmt::Display for GridAutoRows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_class_name())
    }
}

/// Trait for adding grid template columns utilities to a class builder
pub trait GridTemplateColumnsUtilities {
    fn grid_template_columns(self, columns: GridTemplateColumns) -> Self;
}

impl GridTemplateColumnsUtilities for ClassBuilder {
    fn grid_template_columns(self, columns: GridTemplateColumns) -> Self {
        self.class(format!("grid-cols-{}", columns.to_class_name()))
    }
}

/// Trait for adding grid template rows utilities to a class builder
pub trait GridTemplateRowsUtilities {
    fn grid_template_rows(self, rows: GridTemplateRows) -> Self;
}

impl GridTemplateRowsUtilities for ClassBuilder {
    fn grid_template_rows(self, rows: GridTemplateRows) -> Self {
        self.class(format!("grid-rows-{}", rows.to_class_name()))
    }
}

/// Trait for adding grid column span utilities to a class builder
pub trait GridColumnSpanUtilities {
    fn grid_column_span(self, span: GridColumnSpan) -> Self;
}

impl GridColumnSpanUtilities for ClassBuilder {
    fn grid_column_span(self, span: GridColumnSpan) -> Self {
        self.class(format!("col-span-{}", span.to_class_name()))
    }
}

/// Trait for adding grid row span utilities to a class builder
pub trait GridRowSpanUtilities {
    fn grid_row_span(self, span: GridRowSpan) -> Self;
}

impl GridRowSpanUtilities for ClassBuilder {
    fn grid_row_span(self, span: GridRowSpan) -> Self {
        self.class(format!("row-span-{}", span.to_class_name()))
    }
}

/// Trait for adding grid column start utilities to a class builder
pub trait GridColumnStartUtilities {
    fn grid_column_start(self, start: GridColumnStart) -> Self;
}

impl GridColumnStartUtilities for ClassBuilder {
    fn grid_column_start(self, start: GridColumnStart) -> Self {
        self.class(format!("col-start-{}", start.to_class_name()))
    }
}

/// Trait for adding grid row start utilities to a class builder
pub trait GridRowStartUtilities {
    fn grid_row_start(self, start: GridRowStart) -> Self;
}

impl GridRowStartUtilities for ClassBuilder {
    fn grid_row_start(self, start: GridRowStart) -> Self {
        self.class(format!("row-start-{}", start.to_class_name()))
    }
}

/// Trait for adding grid column end utilities to a class builder
pub trait GridColumnEndUtilities {
    fn grid_column_end(self, end: GridColumnEnd) -> Self;
}

impl GridColumnEndUtilities for ClassBuilder {
    fn grid_column_end(self, end: GridColumnEnd) -> Self {
        self.class(format!("col-end-{}", end.to_class_name()))
    }
}

/// Trait for adding grid row end utilities to a class builder
pub trait GridRowEndUtilities {
    fn grid_row_end(self, end: GridRowEnd) -> Self;
}

impl GridRowEndUtilities for ClassBuilder {
    fn grid_row_end(self, end: GridRowEnd) -> Self {
        self.class(format!("row-end-{}", end.to_class_name()))
    }
}

/// Trait for adding grid auto flow utilities to a class builder
pub trait GridAutoFlowUtilities {
    fn grid_auto_flow(self, flow: GridAutoFlow) -> Self;
}

impl GridAutoFlowUtilities for ClassBuilder {
    fn grid_auto_flow(self, flow: GridAutoFlow) -> Self {
        self.class(format!("grid-flow-{}", flow.to_class_name()))
    }
}

/// Trait for adding grid auto columns utilities to a class builder
pub trait GridAutoColumnsUtilities {
    fn grid_auto_columns(self, columns: GridAutoColumns) -> Self;
}

impl GridAutoColumnsUtilities for ClassBuilder {
    fn grid_auto_columns(self, columns: GridAutoColumns) -> Self {
        self.class(format!("auto-cols-{}", columns.to_class_name()))
    }
}

/// Trait for adding grid auto rows utilities to a class builder
pub trait GridAutoRowsUtilities {
    fn grid_auto_rows(self, rows: GridAutoRows) -> Self;
}

impl GridAutoRowsUtilities for ClassBuilder {
    fn grid_auto_rows(self, rows: GridAutoRows) -> Self {
        self.class(format!("auto-rows-{}", rows.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::spacing::GapUtilities;
    
    #[test]
    fn test_grid_template_columns_utilities() {
        let classes = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::None)
            .grid_template_columns(GridTemplateColumns::Auto)
            .grid_template_columns(GridTemplateColumns::One)
            .grid_template_columns(GridTemplateColumns::Two)
            .grid_template_columns(GridTemplateColumns::Three)
            .grid_template_columns(GridTemplateColumns::Twelve)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("grid-cols-none"));
        assert!(css_classes.contains("grid-cols-auto"));
        assert!(css_classes.contains("grid-cols-1"));
        assert!(css_classes.contains("grid-cols-2"));
        assert!(css_classes.contains("grid-cols-3"));
        assert!(css_classes.contains("grid-cols-12"));
    }
    
    #[test]
    fn test_grid_template_rows_utilities() {
        let classes = ClassBuilder::new()
            .grid_template_rows(GridTemplateRows::None)
            .grid_template_rows(GridTemplateRows::Auto)
            .grid_template_rows(GridTemplateRows::One)
            .grid_template_rows(GridTemplateRows::Two)
            .grid_template_rows(GridTemplateRows::Three)
            .grid_template_rows(GridTemplateRows::Six)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("grid-rows-none"));
        assert!(css_classes.contains("grid-rows-auto"));
        assert!(css_classes.contains("grid-rows-1"));
        assert!(css_classes.contains("grid-rows-2"));
        assert!(css_classes.contains("grid-rows-3"));
        assert!(css_classes.contains("grid-rows-6"));
    }
    
    #[test]
    fn test_grid_column_span_utilities() {
        let classes = ClassBuilder::new()
            .grid_column_span(GridColumnSpan::Auto)
            .grid_column_span(GridColumnSpan::One)
            .grid_column_span(GridColumnSpan::Two)
            .grid_column_span(GridColumnSpan::Three)
            .grid_column_span(GridColumnSpan::Twelve)
            .grid_column_span(GridColumnSpan::Full)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("col-span-auto"));
        assert!(css_classes.contains("col-span-1"));
        assert!(css_classes.contains("col-span-2"));
        assert!(css_classes.contains("col-span-3"));
        assert!(css_classes.contains("col-span-12"));
        assert!(css_classes.contains("col-span-full"));
    }
    
    #[test]
    fn test_grid_row_span_utilities() {
        let classes = ClassBuilder::new()
            .grid_row_span(GridRowSpan::Auto)
            .grid_row_span(GridRowSpan::One)
            .grid_row_span(GridRowSpan::Two)
            .grid_row_span(GridRowSpan::Three)
            .grid_row_span(GridRowSpan::Six)
            .grid_row_span(GridRowSpan::Full)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("row-span-auto"));
        assert!(css_classes.contains("row-span-1"));
        assert!(css_classes.contains("row-span-2"));
        assert!(css_classes.contains("row-span-3"));
        assert!(css_classes.contains("row-span-6"));
        assert!(css_classes.contains("row-span-full"));
    }
    
    #[test]
    fn test_grid_column_start_utilities() {
        let classes = ClassBuilder::new()
            .grid_column_start(GridColumnStart::Auto)
            .grid_column_start(GridColumnStart::One)
            .grid_column_start(GridColumnStart::Two)
            .grid_column_start(GridColumnStart::Three)
            .grid_column_start(GridColumnStart::Twelve)
            .grid_column_start(GridColumnStart::Thirteen)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("col-start-auto"));
        assert!(css_classes.contains("col-start-1"));
        assert!(css_classes.contains("col-start-2"));
        assert!(css_classes.contains("col-start-3"));
        assert!(css_classes.contains("col-start-12"));
        assert!(css_classes.contains("col-start-13"));
    }
    
    #[test]
    fn test_grid_row_start_utilities() {
        let classes = ClassBuilder::new()
            .grid_row_start(GridRowStart::Auto)
            .grid_row_start(GridRowStart::One)
            .grid_row_start(GridRowStart::Two)
            .grid_row_start(GridRowStart::Three)
            .grid_row_start(GridRowStart::Six)
            .grid_row_start(GridRowStart::Seven)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("row-start-auto"));
        assert!(css_classes.contains("row-start-1"));
        assert!(css_classes.contains("row-start-2"));
        assert!(css_classes.contains("row-start-3"));
        assert!(css_classes.contains("row-start-6"));
        assert!(css_classes.contains("row-start-7"));
    }
    
    #[test]
    fn test_grid_column_end_utilities() {
        let classes = ClassBuilder::new()
            .grid_column_end(GridColumnEnd::Auto)
            .grid_column_end(GridColumnEnd::One)
            .grid_column_end(GridColumnEnd::Two)
            .grid_column_end(GridColumnEnd::Three)
            .grid_column_end(GridColumnEnd::Twelve)
            .grid_column_end(GridColumnEnd::Thirteen)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("col-end-auto"));
        assert!(css_classes.contains("col-end-1"));
        assert!(css_classes.contains("col-end-2"));
        assert!(css_classes.contains("col-end-3"));
        assert!(css_classes.contains("col-end-12"));
        assert!(css_classes.contains("col-end-13"));
    }
    
    #[test]
    fn test_grid_row_end_utilities() {
        let classes = ClassBuilder::new()
            .grid_row_end(GridRowEnd::Auto)
            .grid_row_end(GridRowEnd::One)
            .grid_row_end(GridRowEnd::Two)
            .grid_row_end(GridRowEnd::Three)
            .grid_row_end(GridRowEnd::Six)
            .grid_row_end(GridRowEnd::Seven)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("row-end-auto"));
        assert!(css_classes.contains("row-end-1"));
        assert!(css_classes.contains("row-end-2"));
        assert!(css_classes.contains("row-end-3"));
        assert!(css_classes.contains("row-end-6"));
        assert!(css_classes.contains("row-end-7"));
    }
    
    #[test]
    fn test_grid_auto_flow_utilities() {
        let classes = ClassBuilder::new()
            .grid_auto_flow(GridAutoFlow::Row)
            .grid_auto_flow(GridAutoFlow::Column)
            .grid_auto_flow(GridAutoFlow::Dense)
            .grid_auto_flow(GridAutoFlow::DenseColumn)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("grid-flow-row"));
        assert!(css_classes.contains("grid-flow-col"));
        assert!(css_classes.contains("grid-flow-dense"));
        assert!(css_classes.contains("grid-flow-col-dense"));
    }
    
    #[test]
    fn test_grid_auto_columns_utilities() {
        let classes = ClassBuilder::new()
            .grid_auto_columns(GridAutoColumns::Auto)
            .grid_auto_columns(GridAutoColumns::Min)
            .grid_auto_columns(GridAutoColumns::Max)
            .grid_auto_columns(GridAutoColumns::Fr)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("auto-cols-auto"));
        assert!(css_classes.contains("auto-cols-min"));
        assert!(css_classes.contains("auto-cols-max"));
        assert!(css_classes.contains("auto-cols-fr"));
    }
    
    #[test]
    fn test_grid_auto_rows_utilities() {
        let classes = ClassBuilder::new()
            .grid_auto_rows(GridAutoRows::Auto)
            .grid_auto_rows(GridAutoRows::Min)
            .grid_auto_rows(GridAutoRows::Max)
            .grid_auto_rows(GridAutoRows::Fr)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("auto-rows-auto"));
        assert!(css_classes.contains("auto-rows-min"));
        assert!(css_classes.contains("auto-rows-max"));
        assert!(css_classes.contains("auto-rows-fr"));
    }
    
    #[test]
    fn test_complex_grid_combination() {
        let classes = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Three)
            .grid_template_rows(GridTemplateRows::Two)
            .grid_column_span(GridColumnSpan::Two)
            .grid_row_span(GridRowSpan::One)
            .grid_column_start(GridColumnStart::One)
            .grid_row_start(GridRowStart::One)
            .grid_column_end(GridColumnEnd::Three)
            .grid_row_end(GridRowEnd::Two)
            .grid_auto_flow(GridAutoFlow::Row)
            .grid_auto_columns(GridAutoColumns::Auto)
            .grid_auto_rows(GridAutoRows::Auto)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("grid-cols-3"));
        assert!(css_classes.contains("grid-rows-2"));
        assert!(css_classes.contains("col-span-2"));
        assert!(css_classes.contains("row-span-1"));
        assert!(css_classes.contains("col-start-1"));
        assert!(css_classes.contains("row-start-1"));
        assert!(css_classes.contains("col-end-3"));
        assert!(css_classes.contains("row-end-2"));
        assert!(css_classes.contains("grid-flow-row"));
        assert!(css_classes.contains("auto-cols-auto"));
        assert!(css_classes.contains("auto-rows-auto"));
    }
    
    /// Test that all Week 6 grid utilities are implemented
    #[test]
    fn test_week6_grid_utilities() {
        // Test all Week 6 grid utilities
        let classes = ClassBuilder::new()
            // Grid Template
            .grid_template_columns(GridTemplateColumns::One)
            .grid_template_columns(GridTemplateColumns::Twelve)
            .grid_template_columns(GridTemplateColumns::None)
            .grid_template_rows(GridTemplateRows::One)
            .grid_template_rows(GridTemplateRows::Six)
            .grid_template_rows(GridTemplateRows::None)
            .grid_column_span(GridColumnSpan::Auto)
            .grid_column_span(GridColumnSpan::One)
            .grid_column_span(GridColumnSpan::Twelve)
            .grid_column_span(GridColumnSpan::Full)
            .grid_row_span(GridRowSpan::Auto)
            .grid_row_span(GridRowSpan::One)
            .grid_row_span(GridRowSpan::Six)
            .grid_row_span(GridRowSpan::Full)
            // Grid Gap & Alignment
            .gap(crate::utilities::spacing::SpacingValue::Zero)
            .gap(crate::utilities::spacing::SpacingValue::Integer(4))
            .gap_x(crate::utilities::spacing::SpacingValue::Integer(2))
            .gap_y(crate::utilities::spacing::SpacingValue::Integer(6))
            .build();
        
        let css_classes = classes.to_css_classes();
        
        // Grid Template
        assert!(css_classes.contains("grid-cols-1"));
        assert!(css_classes.contains("grid-cols-12"));
        assert!(css_classes.contains("grid-cols-none"));
        assert!(css_classes.contains("grid-rows-1"));
        assert!(css_classes.contains("grid-rows-6"));
        assert!(css_classes.contains("grid-rows-none"));
        assert!(css_classes.contains("col-span-auto"));
        assert!(css_classes.contains("col-span-1"));
        assert!(css_classes.contains("col-span-12"));
        assert!(css_classes.contains("col-span-full"));
        assert!(css_classes.contains("row-span-auto"));
        assert!(css_classes.contains("row-span-1"));
        assert!(css_classes.contains("row-span-6"));
        assert!(css_classes.contains("row-span-full"));
        
        // Grid Gap & Alignment
        assert!(css_classes.contains("gap-0"));
        assert!(css_classes.contains("gap-4"));
        assert!(css_classes.contains("gap-x-2"));
        assert!(css_classes.contains("gap-y-6"));
    }
}
