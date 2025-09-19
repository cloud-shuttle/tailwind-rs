//! Grid row span utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Grid row span values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridRowSpan {
    /// Span 1 row
    Span1,
    /// Span 2 rows
    Span2,
    /// Span 3 rows
    Span3,
    /// Span 4 rows
    Span4,
    /// Span 5 rows
    Span5,
    /// Span 6 rows
    Span6,
    /// Span all rows
    SpanFull,
}

impl fmt::Display for GridRowSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridRowSpan::Span1 => write!(f, "row-span-1"),
            GridRowSpan::Span2 => write!(f, "row-span-2"),
            GridRowSpan::Span3 => write!(f, "row-span-3"),
            GridRowSpan::Span4 => write!(f, "row-span-4"),
            GridRowSpan::Span5 => write!(f, "row-span-5"),
            GridRowSpan::Span6 => write!(f, "row-span-6"),
            GridRowSpan::SpanFull => write!(f, "row-span-full"),
        }
    }
}

/// Trait for adding grid row span utilities to a class builder
pub trait GridRowSpanUtilities {
    fn grid_row_span(self, span: GridRowSpan) -> Self;
}

impl GridRowSpanUtilities for ClassBuilder {
    fn grid_row_span(self, span: GridRowSpan) -> Self {
        self.class(span.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_row_span_display() {
        assert_eq!(GridRowSpan::Span1.to_string(), "row-span-1");
        assert_eq!(GridRowSpan::Span2.to_string(), "row-span-2");
        assert_eq!(GridRowSpan::SpanFull.to_string(), "row-span-full");
    }

    #[test]
    fn test_grid_row_span_utilities() {
        let classes = ClassBuilder::new()
            .grid_row_span(GridRowSpan::Span3)
            .build();
        
        assert!(classes.to_css_classes().contains("row-span-3"));
    }
}
