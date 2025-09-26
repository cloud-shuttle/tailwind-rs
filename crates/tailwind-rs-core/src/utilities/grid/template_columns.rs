//! Grid template columns utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Grid template columns values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridTemplateColumns {
    /// 1 column
    Cols1,
    /// 2 columns
    Cols2,
    /// 3 columns
    Cols3,
    /// 4 columns
    Cols4,
    /// 5 columns
    Cols5,
    /// 6 columns
    Cols6,
    /// 7 columns
    Cols7,
    /// 8 columns
    Cols8,
    /// 9 columns
    Cols9,
    /// 10 columns
    Cols10,
    /// 11 columns
    Cols11,
    /// 12 columns
    Cols12,
    /// None
    None,
    /// Subgrid
    Subgrid,
}

impl fmt::Display for GridTemplateColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridTemplateColumns::Cols1 => write!(f, "grid-cols-1"),
            GridTemplateColumns::Cols2 => write!(f, "grid-cols-2"),
            GridTemplateColumns::Cols3 => write!(f, "grid-cols-3"),
            GridTemplateColumns::Cols4 => write!(f, "grid-cols-4"),
            GridTemplateColumns::Cols5 => write!(f, "grid-cols-5"),
            GridTemplateColumns::Cols6 => write!(f, "grid-cols-6"),
            GridTemplateColumns::Cols7 => write!(f, "grid-cols-7"),
            GridTemplateColumns::Cols8 => write!(f, "grid-cols-8"),
            GridTemplateColumns::Cols9 => write!(f, "grid-cols-9"),
            GridTemplateColumns::Cols10 => write!(f, "grid-cols-10"),
            GridTemplateColumns::Cols11 => write!(f, "grid-cols-11"),
            GridTemplateColumns::Cols12 => write!(f, "grid-cols-12"),
            GridTemplateColumns::None => write!(f, "grid-cols-none"),
            GridTemplateColumns::Subgrid => write!(f, "grid-cols-subgrid"),
        }
    }
}

/// Trait for adding grid template columns utilities to a class builder
pub trait GridTemplateColumnsUtilities {
    fn grid_template_columns(self, columns: GridTemplateColumns) -> Self;
}

impl GridTemplateColumnsUtilities for ClassBuilder {
    fn grid_template_columns(self, columns: GridTemplateColumns) -> Self {
        self.class(columns.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_template_columns_display() {
        assert_eq!(GridTemplateColumns::Cols1.to_string(), "grid-cols-1");
        assert_eq!(GridTemplateColumns::Cols2.to_string(), "grid-cols-2");
        assert_eq!(GridTemplateColumns::Cols3.to_string(), "grid-cols-3");
        assert_eq!(GridTemplateColumns::Cols4.to_string(), "grid-cols-4");
        assert_eq!(GridTemplateColumns::Cols5.to_string(), "grid-cols-5");
        assert_eq!(GridTemplateColumns::Cols6.to_string(), "grid-cols-6");
        assert_eq!(GridTemplateColumns::Cols7.to_string(), "grid-cols-7");
        assert_eq!(GridTemplateColumns::Cols8.to_string(), "grid-cols-8");
        assert_eq!(GridTemplateColumns::Cols9.to_string(), "grid-cols-9");
        assert_eq!(GridTemplateColumns::Cols10.to_string(), "grid-cols-10");
        assert_eq!(GridTemplateColumns::Cols11.to_string(), "grid-cols-11");
        assert_eq!(GridTemplateColumns::Cols12.to_string(), "grid-cols-12");
        assert_eq!(GridTemplateColumns::None.to_string(), "grid-cols-none");
        assert_eq!(
            GridTemplateColumns::Subgrid.to_string(),
            "grid-cols-subgrid"
        );
    }

    #[test]
    fn test_grid_template_columns_utilities() {
        let classes = ClassBuilder::new()
            .grid_template_columns(GridTemplateColumns::Cols3)
            .build();

        assert!(classes.to_css_classes().contains("grid-cols-3"));
    }

    #[test]
    fn test_grid_template_columns_serialization() {
        let columns = GridTemplateColumns::Cols4;
        let serialized = serde_json::to_string(&columns).unwrap();
        let deserialized: GridTemplateColumns = serde_json::from_str(&serialized).unwrap();
        assert_eq!(columns, deserialized);
    }
}
