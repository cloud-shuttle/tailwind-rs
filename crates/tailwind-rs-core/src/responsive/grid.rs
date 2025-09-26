//! # Grid Responsive Utilities
//!
//! This module provides grid-specific responsive utilities.

use super::breakpoints::Breakpoint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Responsive grid container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsiveGrid {
    /// Number of columns for each breakpoint
    pub columns: HashMap<Breakpoint, u32>,
    /// Gap between grid items for each breakpoint
    pub gap: HashMap<Breakpoint, u32>,
    /// Row gap for each breakpoint
    pub row_gap: HashMap<Breakpoint, u32>,
    /// Column gap for each breakpoint
    pub column_gap: HashMap<Breakpoint, u32>,
}

impl ResponsiveGrid {
    /// Create a new responsive grid container
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a responsive grid container with base values
    pub fn with_base(columns: u32, gap: u32) -> Self {
        let mut grid = Self::new();
        grid.set_columns(Breakpoint::Base, columns);
        grid.set_gap(Breakpoint::Base, gap);
        grid
    }

    /// Set number of columns for a specific breakpoint
    pub fn set_columns(&mut self, breakpoint: Breakpoint, columns: u32) {
        self.columns.insert(breakpoint, columns);
    }

    /// Set gap for a specific breakpoint
    pub fn set_gap(&mut self, breakpoint: Breakpoint, gap: u32) {
        self.gap.insert(breakpoint, gap);
    }

    /// Set row gap for a specific breakpoint
    pub fn set_row_gap(&mut self, breakpoint: Breakpoint, gap: u32) {
        self.row_gap.insert(breakpoint, gap);
    }

    /// Set column gap for a specific breakpoint
    pub fn set_column_gap(&mut self, breakpoint: Breakpoint, gap: u32) {
        self.column_gap.insert(breakpoint, gap);
    }

    /// Get number of columns for a specific breakpoint
    pub fn get_columns(&self, breakpoint: Breakpoint) -> Option<u32> {
        self.columns.get(&breakpoint).copied()
    }

    /// Get gap for a specific breakpoint
    pub fn get_gap(&self, breakpoint: Breakpoint) -> Option<u32> {
        self.gap.get(&breakpoint).copied()
    }

    /// Get row gap for a specific breakpoint
    pub fn get_row_gap(&self, breakpoint: Breakpoint) -> Option<u32> {
        self.row_gap.get(&breakpoint).copied()
    }

    /// Get column gap for a specific breakpoint
    pub fn get_column_gap(&self, breakpoint: Breakpoint) -> Option<u32> {
        self.column_gap.get(&breakpoint).copied()
    }

    /// Get number of columns for a specific breakpoint, falling back to base
    pub fn get_columns_or_base(&self, breakpoint: Breakpoint) -> Option<u32> {
        self.columns
            .get(&breakpoint)
            .copied()
            .or_else(|| self.columns.get(&Breakpoint::Base).copied())
    }

    /// Get gap for a specific breakpoint, falling back to base
    pub fn get_gap_or_base(&self, breakpoint: Breakpoint) -> Option<u32> {
        self.gap
            .get(&breakpoint)
            .copied()
            .or_else(|| self.gap.get(&Breakpoint::Base).copied())
    }

    /// Generate CSS classes for all breakpoints
    pub fn to_css_classes(&self) -> String {
        let mut classes = Vec::new();

        // Add grid columns classes
        for (breakpoint, &columns) in &self.columns {
            let class = if columns == 1 {
                "grid-cols-1".to_string()
            } else {
                format!("grid-cols-{}", columns)
            };

            if *breakpoint == Breakpoint::Base {
                classes.push(class);
            } else {
                classes.push(format!("{}{}", breakpoint.prefix(), class));
            }
        }

        // Add gap classes
        for (breakpoint, &gap) in &self.gap {
            let class = if gap == 0 {
                "gap-0".to_string()
            } else {
                format!("gap-{}", gap)
            };

            if *breakpoint == Breakpoint::Base {
                classes.push(class);
            } else {
                classes.push(format!("{}{}", breakpoint.prefix(), class));
            }
        }

        // Add row gap classes
        for (breakpoint, &gap) in &self.row_gap {
            let class = if gap == 0 {
                "gap-y-0".to_string()
            } else {
                format!("gap-y-{}", gap)
            };

            if *breakpoint == Breakpoint::Base {
                classes.push(class);
            } else {
                classes.push(format!("{}{}", breakpoint.prefix(), class));
            }
        }

        // Add column gap classes
        for (breakpoint, &gap) in &self.column_gap {
            let class = if gap == 0 {
                "gap-x-0".to_string()
            } else {
                format!("gap-x-{}", gap)
            };

            if *breakpoint == Breakpoint::Base {
                classes.push(class);
            } else {
                classes.push(format!("{}{}", breakpoint.prefix(), class));
            }
        }

        classes.join(" ")
    }

    /// Generate CSS classes for a specific screen width
    pub fn to_css_classes_for_width(&self, screen_width: u32) -> String {
        let mut classes = Vec::new();

        // Find the appropriate breakpoint for this screen width
        let target_breakpoint = self.get_breakpoint_for_width(screen_width);

        // Add grid columns class
        if let Some(columns) = self.get_columns_or_base(target_breakpoint) {
            let class = if columns == 1 {
                "grid-cols-1".to_string()
            } else {
                format!("grid-cols-{}", columns)
            };
            classes.push(class);
        }

        // Add gap class
        if let Some(gap) = self.get_gap_or_base(target_breakpoint) {
            let class = if gap == 0 {
                "gap-0".to_string()
            } else {
                format!("gap-{}", gap)
            };
            classes.push(class);
        }

        // Add row gap class
        if let Some(gap) = self.get_row_gap(target_breakpoint) {
            let class = if gap == 0 {
                "gap-y-0".to_string()
            } else {
                format!("gap-y-{}", gap)
            };
            classes.push(class);
        }

        // Add column gap class
        if let Some(gap) = self.get_column_gap(target_breakpoint) {
            let class = if gap == 0 {
                "gap-x-0".to_string()
            } else {
                format!("gap-x-{}", gap)
            };
            classes.push(class);
        }

        classes.join(" ")
    }

    /// Get the appropriate breakpoint for a given screen width
    fn get_breakpoint_for_width(&self, screen_width: u32) -> Breakpoint {
        if screen_width >= Breakpoint::Xl2.min_width() {
            Breakpoint::Xl2
        } else if screen_width >= Breakpoint::Xl.min_width() {
            Breakpoint::Xl
        } else if screen_width >= Breakpoint::Lg.min_width() {
            Breakpoint::Lg
        } else if screen_width >= Breakpoint::Md.min_width() {
            Breakpoint::Md
        } else if screen_width >= Breakpoint::Sm.min_width() {
            Breakpoint::Sm
        } else {
            Breakpoint::Base
        }
    }

    /// Check if the grid is empty
    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
            && self.gap.is_empty()
            && self.row_gap.is_empty()
            && self.column_gap.is_empty()
    }

    /// Get the number of breakpoints with configurations
    pub fn len(&self) -> usize {
        let mut count = 0;
        count += self.columns.len();
        count += self.gap.len();
        count += self.row_gap.len();
        count += self.column_gap.len();
        count
    }

    /// Clear all configurations
    pub fn clear(&mut self) {
        self.columns.clear();
        self.gap.clear();
        self.row_gap.clear();
        self.column_gap.clear();
    }
}

impl Default for ResponsiveGrid {
    fn default() -> Self {
        let mut grid = Self {
            columns: HashMap::new(),
            gap: HashMap::new(),
            row_gap: HashMap::new(),
            column_gap: HashMap::new(),
        };

        // Set default values
        grid.set_columns(Breakpoint::Base, 1);
        grid.set_gap(Breakpoint::Base, 0);

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_grid_new() {
        let grid = ResponsiveGrid::new();
        assert_eq!(grid.get_columns(Breakpoint::Base), Some(1));
        assert_eq!(grid.get_gap(Breakpoint::Base), Some(0));
    }

    #[test]
    fn test_responsive_grid_with_base() {
        let grid = ResponsiveGrid::with_base(3, 4);
        assert_eq!(grid.get_columns(Breakpoint::Base), Some(3));
        assert_eq!(grid.get_gap(Breakpoint::Base), Some(4));
    }

    #[test]
    fn test_responsive_grid_set_get() {
        let mut grid = ResponsiveGrid::new();

        grid.set_columns(Breakpoint::Sm, 2);
        grid.set_columns(Breakpoint::Md, 3);
        grid.set_columns(Breakpoint::Lg, 4);
        grid.set_gap(Breakpoint::Sm, 2);
        grid.set_gap(Breakpoint::Md, 4);
        grid.set_row_gap(Breakpoint::Lg, 6);
        grid.set_column_gap(Breakpoint::Xl, 8);

        assert_eq!(grid.get_columns(Breakpoint::Sm), Some(2));
        assert_eq!(grid.get_columns(Breakpoint::Md), Some(3));
        assert_eq!(grid.get_columns(Breakpoint::Lg), Some(4));
        assert_eq!(grid.get_gap(Breakpoint::Sm), Some(2));
        assert_eq!(grid.get_gap(Breakpoint::Md), Some(4));
        assert_eq!(grid.get_row_gap(Breakpoint::Lg), Some(6));
        assert_eq!(grid.get_column_gap(Breakpoint::Xl), Some(8));
    }

    #[test]
    fn test_responsive_grid_get_or_base() {
        let mut grid = ResponsiveGrid::new();
        grid.set_columns(Breakpoint::Base, 1);
        grid.set_columns(Breakpoint::Sm, 2);
        grid.set_gap(Breakpoint::Base, 0);
        grid.set_gap(Breakpoint::Md, 4);

        assert_eq!(grid.get_columns_or_base(Breakpoint::Base), Some(1));
        assert_eq!(grid.get_columns_or_base(Breakpoint::Sm), Some(2));
        assert_eq!(grid.get_columns_or_base(Breakpoint::Md), Some(1)); // Falls back to base
        assert_eq!(grid.get_gap_or_base(Breakpoint::Base), Some(0));
        assert_eq!(grid.get_gap_or_base(Breakpoint::Sm), Some(0)); // Falls back to base
        assert_eq!(grid.get_gap_or_base(Breakpoint::Md), Some(4));
    }

    #[test]
    fn test_responsive_grid_to_css_classes() {
        let mut grid = ResponsiveGrid::new();
        grid.set_columns(Breakpoint::Base, 1);
        grid.set_columns(Breakpoint::Sm, 2);
        grid.set_columns(Breakpoint::Md, 3);
        grid.set_gap(Breakpoint::Base, 0);
        grid.set_gap(Breakpoint::Sm, 2);
        grid.set_gap(Breakpoint::Md, 4);

        let classes = grid.to_css_classes();
        assert!(classes.contains("grid-cols-1"));
        assert!(classes.contains("sm:grid-cols-2"));
        assert!(classes.contains("md:grid-cols-3"));
        assert!(classes.contains("gap-0"));
        assert!(classes.contains("sm:gap-2"));
        assert!(classes.contains("md:gap-4"));
    }

    #[test]
    fn test_responsive_grid_to_css_classes_for_width() {
        let mut grid = ResponsiveGrid::new();
        grid.set_columns(Breakpoint::Base, 1);
        grid.set_columns(Breakpoint::Sm, 2);
        grid.set_columns(Breakpoint::Md, 3);
        grid.set_gap(Breakpoint::Base, 0);
        grid.set_gap(Breakpoint::Sm, 2);
        grid.set_gap(Breakpoint::Md, 4);

        // Test width 0 (base only)
        let classes_0 = grid.to_css_classes_for_width(0);
        assert!(classes_0.contains("grid-cols-1"));
        assert!(classes_0.contains("gap-0"));
        assert!(!classes_0.contains("grid-cols-2"));
        assert!(!classes_0.contains("gap-2"));

        // Test width 640 (sm active)
        let classes_640 = grid.to_css_classes_for_width(640);
        assert!(classes_640.contains("grid-cols-2"));
        assert!(classes_640.contains("gap-2"));
        assert!(!classes_640.contains("grid-cols-3"));
        assert!(!classes_640.contains("gap-4"));

        // Test width 768 (md active)
        let classes_768 = grid.to_css_classes_for_width(768);
        assert!(classes_768.contains("grid-cols-3"));
        assert!(classes_768.contains("gap-4"));
    }

    #[test]
    fn test_responsive_grid_is_empty() {
        let grid = ResponsiveGrid::new();
        assert!(!grid.is_empty()); // Has default values

        let empty_grid = ResponsiveGrid {
            columns: HashMap::new(),
            gap: HashMap::new(),
            row_gap: HashMap::new(),
            column_gap: HashMap::new(),
        };
        assert!(empty_grid.is_empty());
    }

    #[test]
    fn test_responsive_grid_clear() {
        let mut grid = ResponsiveGrid::new();
        grid.set_columns(Breakpoint::Sm, 2);
        grid.set_gap(Breakpoint::Md, 4);

        assert!(!grid.is_empty());
        grid.clear();
        assert!(grid.is_empty());
    }
}
