//! Grid template rows utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for GridTemplateRows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridTemplateRows::None => write!(f, "grid-rows-none"),
            GridTemplateRows::Subgrid => write!(f, "grid-rows-subgrid"),
            GridTemplateRows::Auto => write!(f, "grid-rows-auto"),
            GridTemplateRows::One => write!(f, "grid-rows-1"),
            GridTemplateRows::Two => write!(f, "grid-rows-2"),
            GridTemplateRows::Three => write!(f, "grid-rows-3"),
            GridTemplateRows::Four => write!(f, "grid-rows-4"),
            GridTemplateRows::Five => write!(f, "grid-rows-5"),
            GridTemplateRows::Six => write!(f, "grid-rows-6"),
        }
    }
}

/// Trait for adding grid template rows utilities to a class builder
pub trait GridTemplateRowsUtilities {
    fn grid_template_rows(self, rows: GridTemplateRows) -> Self;
}

impl GridTemplateRowsUtilities for ClassBuilder {
    fn grid_template_rows(self, rows: GridTemplateRows) -> Self {
        self.class(rows.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_template_rows_display() {
        assert_eq!(GridTemplateRows::One.to_string(), "grid-rows-1");
        assert_eq!(GridTemplateRows::Two.to_string(), "grid-rows-2");
        assert_eq!(GridTemplateRows::Three.to_string(), "grid-rows-3");
    }

    #[test]
    fn test_grid_template_rows_utilities() {
        let classes = ClassBuilder::new()
            .grid_template_rows(GridTemplateRows::Three)
            .build();

        assert!(classes.to_css_classes().contains("grid-rows-3"));
    }
}
