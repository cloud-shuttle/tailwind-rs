//! Grid placement utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Grid column start values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridColumnStart {
    /// Start at column 1
    Start1,
    /// Start at column 2
    Start2,
    /// Start at column 3
    Start3,
    /// Start at column 4
    Start4,
    /// Start at column 5
    Start5,
    /// Start at column 6
    Start6,
    /// Start at column 7
    Start7,
    /// Start at column 8
    Start8,
    /// Start at column 9
    Start9,
    /// Start at column 10
    Start10,
    /// Start at column 11
    Start11,
    /// Start at column 12
    Start12,
    /// Start at column 13
    Start13,
    /// Auto start
    Auto,
}

/// Grid column end values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridColumnEnd {
    /// End at column 1
    End1,
    /// End at column 2
    End2,
    /// End at column 3
    End3,
    /// End at column 4
    End4,
    /// End at column 5
    End5,
    /// End at column 6
    End6,
    /// End at column 7
    End7,
    /// End at column 8
    End8,
    /// End at column 9
    End9,
    /// End at column 10
    End10,
    /// End at column 11
    End11,
    /// End at column 12
    End12,
    /// End at column 13
    End13,
    /// Auto end
    Auto,
}

/// Grid row start values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridRowStart {
    /// Start at row 1
    Start1,
    /// Start at row 2
    Start2,
    /// Start at row 3
    Start3,
    /// Start at row 4
    Start4,
    /// Start at row 5
    Start5,
    /// Start at row 6
    Start6,
    /// Start at row 7
    Start7,
    /// Auto start
    Auto,
}

/// Grid row end values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridRowEnd {
    /// End at row 1
    End1,
    /// End at row 2
    End2,
    /// End at row 3
    End3,
    /// End at row 4
    End4,
    /// End at row 5
    End5,
    /// End at row 6
    End6,
    /// End at row 7
    End7,
    /// Auto end
    Auto,
}

impl fmt::Display for GridColumnStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridColumnStart::Start1 => write!(f, "col-start-1"),
            GridColumnStart::Start2 => write!(f, "col-start-2"),
            GridColumnStart::Start3 => write!(f, "col-start-3"),
            GridColumnStart::Start4 => write!(f, "col-start-4"),
            GridColumnStart::Start5 => write!(f, "col-start-5"),
            GridColumnStart::Start6 => write!(f, "col-start-6"),
            GridColumnStart::Start7 => write!(f, "col-start-7"),
            GridColumnStart::Start8 => write!(f, "col-start-8"),
            GridColumnStart::Start9 => write!(f, "col-start-9"),
            GridColumnStart::Start10 => write!(f, "col-start-10"),
            GridColumnStart::Start11 => write!(f, "col-start-11"),
            GridColumnStart::Start12 => write!(f, "col-start-12"),
            GridColumnStart::Start13 => write!(f, "col-start-13"),
            GridColumnStart::Auto => write!(f, "col-start-auto"),
        }
    }
}

impl fmt::Display for GridColumnEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridColumnEnd::End1 => write!(f, "col-end-1"),
            GridColumnEnd::End2 => write!(f, "col-end-2"),
            GridColumnEnd::End3 => write!(f, "col-end-3"),
            GridColumnEnd::End4 => write!(f, "col-end-4"),
            GridColumnEnd::End5 => write!(f, "col-end-5"),
            GridColumnEnd::End6 => write!(f, "col-end-6"),
            GridColumnEnd::End7 => write!(f, "col-end-7"),
            GridColumnEnd::End8 => write!(f, "col-end-8"),
            GridColumnEnd::End9 => write!(f, "col-end-9"),
            GridColumnEnd::End10 => write!(f, "col-end-10"),
            GridColumnEnd::End11 => write!(f, "col-end-11"),
            GridColumnEnd::End12 => write!(f, "col-end-12"),
            GridColumnEnd::End13 => write!(f, "col-end-13"),
            GridColumnEnd::Auto => write!(f, "col-end-auto"),
        }
    }
}

impl fmt::Display for GridRowStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridRowStart::Start1 => write!(f, "row-start-1"),
            GridRowStart::Start2 => write!(f, "row-start-2"),
            GridRowStart::Start3 => write!(f, "row-start-3"),
            GridRowStart::Start4 => write!(f, "row-start-4"),
            GridRowStart::Start5 => write!(f, "row-start-5"),
            GridRowStart::Start6 => write!(f, "row-start-6"),
            GridRowStart::Start7 => write!(f, "row-start-7"),
            GridRowStart::Auto => write!(f, "row-start-auto"),
        }
    }
}

impl fmt::Display for GridRowEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridRowEnd::End1 => write!(f, "row-end-1"),
            GridRowEnd::End2 => write!(f, "row-end-2"),
            GridRowEnd::End3 => write!(f, "row-end-3"),
            GridRowEnd::End4 => write!(f, "row-end-4"),
            GridRowEnd::End5 => write!(f, "row-end-5"),
            GridRowEnd::End6 => write!(f, "row-end-6"),
            GridRowEnd::End7 => write!(f, "row-end-7"),
            GridRowEnd::Auto => write!(f, "row-end-auto"),
        }
    }
}

/// Trait for adding grid placement utilities to a class builder
pub trait GridPlacementUtilities {
    fn grid_column_start(self, start: GridColumnStart) -> Self;
    fn grid_column_end(self, end: GridColumnEnd) -> Self;
    fn grid_row_start(self, start: GridRowStart) -> Self;
    fn grid_row_end(self, end: GridRowEnd) -> Self;
}

impl GridPlacementUtilities for ClassBuilder {
    fn grid_column_start(self, start: GridColumnStart) -> Self {
        self.class(start.to_string())
    }
    
    fn grid_column_end(self, end: GridColumnEnd) -> Self {
        self.class(end.to_string())
    }
    
    fn grid_row_start(self, start: GridRowStart) -> Self {
        self.class(start.to_string())
    }
    
    fn grid_row_end(self, end: GridRowEnd) -> Self {
        self.class(end.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_placement_display() {
        assert_eq!(GridColumnStart::Start1.to_string(), "col-start-1");
        assert_eq!(GridColumnEnd::End1.to_string(), "col-end-1");
        assert_eq!(GridRowStart::Start1.to_string(), "row-start-1");
        assert_eq!(GridRowEnd::End1.to_string(), "row-end-1");
    }

    #[test]
    fn test_grid_placement_utilities() {
        let classes = ClassBuilder::new()
            .grid_column_start(GridColumnStart::Start1)
            .grid_column_end(GridColumnEnd::End3)
            .grid_row_start(GridRowStart::Start1)
            .grid_row_end(GridRowEnd::End2)
            .build();
        
        let css_classes = classes.to_css_classes();
        assert!(css_classes.contains("col-start-1"));
        assert!(css_classes.contains("col-end-3"));
        assert!(css_classes.contains("row-start-1"));
        assert!(css_classes.contains("row-end-2"));
    }
}
