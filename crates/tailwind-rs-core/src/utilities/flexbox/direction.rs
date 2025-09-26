//! Flex direction utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Flex direction values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexDirection {
    /// Row flex direction
    Row,
    /// Row reverse flex direction
    RowReverse,
    /// Column flex direction
    Column,
    /// Column reverse flex direction
    ColumnReverse,
}

impl fmt::Display for FlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexDirection::Row => write!(f, "flex-row"),
            FlexDirection::RowReverse => write!(f, "flex-row-reverse"),
            FlexDirection::Column => write!(f, "flex-col"),
            FlexDirection::ColumnReverse => write!(f, "flex-col-reverse"),
        }
    }
}

/// Trait for adding flex direction utilities to a class builder
pub trait FlexDirectionUtilities {
    fn flex_direction(self, direction: FlexDirection) -> Self;
}

impl FlexDirectionUtilities for ClassBuilder {
    fn flex_direction(self, direction: FlexDirection) -> Self {
        self.class(direction.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_direction_utilities() {
        let classes = ClassBuilder::new()
            .flex_direction(FlexDirection::Row)
            .build();

        assert!(classes.to_css_classes().contains("flex-row"));
    }
}
