//! Grid column span utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Grid column span values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridColumnSpan {
    /// Span 1 column
    Span1,
    /// Span 2 columns
    Span2,
    /// Span 3 columns
    Span3,
    /// Span 4 columns
    Span4,
    /// Span 5 columns
    Span5,
    /// Span 6 columns
    Span6,
    /// Span 7 columns
    Span7,
    /// Span 8 columns
    Span8,
    /// Span 9 columns
    Span9,
    /// Span 10 columns
    Span10,
    /// Span 11 columns
    Span11,
    /// Span 12 columns
    Span12,
    /// Span all columns
    SpanFull,
}

impl fmt::Display for GridColumnSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridColumnSpan::Span1 => write!(f, "col-span-1"),
            GridColumnSpan::Span2 => write!(f, "col-span-2"),
            GridColumnSpan::Span3 => write!(f, "col-span-3"),
            GridColumnSpan::Span4 => write!(f, "col-span-4"),
            GridColumnSpan::Span5 => write!(f, "col-span-5"),
            GridColumnSpan::Span6 => write!(f, "col-span-6"),
            GridColumnSpan::Span7 => write!(f, "col-span-7"),
            GridColumnSpan::Span8 => write!(f, "col-span-8"),
            GridColumnSpan::Span9 => write!(f, "col-span-9"),
            GridColumnSpan::Span10 => write!(f, "col-span-10"),
            GridColumnSpan::Span11 => write!(f, "col-span-11"),
            GridColumnSpan::Span12 => write!(f, "col-span-12"),
            GridColumnSpan::SpanFull => write!(f, "col-span-full"),
        }
    }
}

/// Trait for adding grid column span utilities to a class builder
pub trait GridColumnSpanUtilities {
    fn grid_column_span(self, span: GridColumnSpan) -> Self;
}

impl GridColumnSpanUtilities for ClassBuilder {
    fn grid_column_span(self, span: GridColumnSpan) -> Self {
        self.class(span.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_column_span_display() {
        assert_eq!(GridColumnSpan::Span1.to_string(), "col-span-1");
        assert_eq!(GridColumnSpan::Span2.to_string(), "col-span-2");
        assert_eq!(GridColumnSpan::SpanFull.to_string(), "col-span-full");
    }

    #[test]
    fn test_grid_column_span_utilities() {
        let classes = ClassBuilder::new()
            .grid_column_span(GridColumnSpan::Span3)
            .build();

        assert!(classes.to_css_classes().contains("col-span-3"));
    }
}
