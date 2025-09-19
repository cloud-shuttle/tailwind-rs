//! Grid auto columns utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Grid auto columns values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridAutoColumns {
    /// Auto columns
    Auto,
    /// Min content columns
    Min,
    /// Max content columns
    Max,
    /// Fr columns
    Fr,
}

impl fmt::Display for GridAutoColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridAutoColumns::Auto => write!(f, "auto-cols-auto"),
            GridAutoColumns::Min => write!(f, "auto-cols-min"),
            GridAutoColumns::Max => write!(f, "auto-cols-max"),
            GridAutoColumns::Fr => write!(f, "auto-cols-fr"),
        }
    }
}

/// Trait for adding grid auto columns utilities to a class builder
pub trait GridAutoColumnsUtilities {
    fn grid_auto_columns(self, columns: GridAutoColumns) -> Self;
}

impl GridAutoColumnsUtilities for ClassBuilder {
    fn grid_auto_columns(self, columns: GridAutoColumns) -> Self {
        self.class(columns.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_auto_columns_display() {
        assert_eq!(GridAutoColumns::Auto.to_string(), "auto-cols-auto");
        assert_eq!(GridAutoColumns::Min.to_string(), "auto-cols-min");
        assert_eq!(GridAutoColumns::Max.to_string(), "auto-cols-max");
    }

    #[test]
    fn test_grid_auto_columns_utilities() {
        let classes = ClassBuilder::new()
            .grid_auto_columns(GridAutoColumns::Auto)
            .build();
        
        assert!(classes.to_css_classes().contains("auto-cols-auto"));
    }
}
