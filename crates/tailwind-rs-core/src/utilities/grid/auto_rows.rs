//! Grid auto rows utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Grid auto rows values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridAutoRows {
    /// Auto rows
    Auto,
    /// Min content rows
    Min,
    /// Max content rows
    Max,
    /// Fr rows
    Fr,
}

impl fmt::Display for GridAutoRows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridAutoRows::Auto => write!(f, "auto-rows-auto"),
            GridAutoRows::Min => write!(f, "auto-rows-min"),
            GridAutoRows::Max => write!(f, "auto-rows-max"),
            GridAutoRows::Fr => write!(f, "auto-rows-fr"),
        }
    }
}

/// Trait for adding grid auto rows utilities to a class builder
pub trait GridAutoRowsUtilities {
    fn grid_auto_rows(self, rows: GridAutoRows) -> Self;
}

impl GridAutoRowsUtilities for ClassBuilder {
    fn grid_auto_rows(self, rows: GridAutoRows) -> Self {
        self.class(rows.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_auto_rows_display() {
        assert_eq!(GridAutoRows::Auto.to_string(), "auto-rows-auto");
        assert_eq!(GridAutoRows::Min.to_string(), "auto-rows-min");
        assert_eq!(GridAutoRows::Max.to_string(), "auto-rows-max");
    }

    #[test]
    fn test_grid_auto_rows_utilities() {
        let classes = ClassBuilder::new()
            .grid_auto_rows(GridAutoRows::Auto)
            .build();

        assert!(classes.to_css_classes().contains("auto-rows-auto"));
    }
}
