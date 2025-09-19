//! Grid gap utilities for tailwind-rs

use crate::classes::ClassBuilder;
use crate::utilities::spacing::SpacingValue;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Grid gap values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GridGap {
    /// Gap 0
    Gap0,
    /// Gap 1
    Gap1,
    /// Gap 2
    Gap2,
    /// Gap 3
    Gap3,
    /// Gap 4
    Gap4,
    /// Gap 5
    Gap5,
    /// Gap 6
    Gap6,
    /// Gap 8
    Gap8,
    /// Gap 10
    Gap10,
    /// Gap 12
    Gap12,
    /// Gap 16
    Gap16,
    /// Gap 20
    Gap20,
    /// Gap 24
    Gap24,
    /// Gap 32
    Gap32,
    /// Gap 40
    Gap40,
    /// Gap 48
    Gap48,
    /// Gap 56
    Gap56,
    /// Gap 64
    Gap64,
}

impl fmt::Display for GridGap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridGap::Gap0 => write!(f, "gap-0"),
            GridGap::Gap1 => write!(f, "gap-1"),
            GridGap::Gap2 => write!(f, "gap-2"),
            GridGap::Gap3 => write!(f, "gap-3"),
            GridGap::Gap4 => write!(f, "gap-4"),
            GridGap::Gap5 => write!(f, "gap-5"),
            GridGap::Gap6 => write!(f, "gap-6"),
            GridGap::Gap8 => write!(f, "gap-8"),
            GridGap::Gap10 => write!(f, "gap-10"),
            GridGap::Gap12 => write!(f, "gap-12"),
            GridGap::Gap16 => write!(f, "gap-16"),
            GridGap::Gap20 => write!(f, "gap-20"),
            GridGap::Gap24 => write!(f, "gap-24"),
            GridGap::Gap32 => write!(f, "gap-32"),
            GridGap::Gap40 => write!(f, "gap-40"),
            GridGap::Gap48 => write!(f, "gap-48"),
            GridGap::Gap56 => write!(f, "gap-56"),
            GridGap::Gap64 => write!(f, "gap-64"),
        }
    }
}

/// Trait for adding grid gap utilities to a class builder
pub trait GridGapUtilities {
    fn grid_gap(self, gap: GridGap) -> Self;
    fn gap(self, gap: SpacingValue) -> Self;
}

impl GridGapUtilities for ClassBuilder {
    fn grid_gap(self, gap: GridGap) -> Self {
        self.class(gap.to_string())
    }
    
    fn gap(self, gap: SpacingValue) -> Self {
        self.class(format!("gap-{}", gap.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_gap_display() {
        assert_eq!(GridGap::Gap0.to_string(), "gap-0");
        assert_eq!(GridGap::Gap1.to_string(), "gap-1");
        assert_eq!(GridGap::Gap2.to_string(), "gap-2");
        assert_eq!(GridGap::Gap4.to_string(), "gap-4");
    }

    #[test]
    fn test_grid_gap_utilities() {
        let classes = ClassBuilder::new()
            .grid_gap(GridGap::Gap4)
            .build();
        
        assert!(classes.to_css_classes().contains("gap-4"));
    }
}
