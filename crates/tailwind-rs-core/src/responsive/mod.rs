//! # Responsive Design System Module
//!
//! This module provides a comprehensive responsive design system for tailwind-rs.
//! It's organized into focused sub-modules for better maintainability:
//! - `breakpoints` - Breakpoint definitions and utilities
//! - `states` - State definitions for pseudo-classes
//! - `responsive_values` - Responsive value handling
//! - `responsive_config` - Configuration management
//! - `responsive_builder` - Builder pattern for responsive classes
//! - `flexbox` - Flexbox-specific responsive utilities
//! - `grid` - Grid-specific responsive utilities

pub mod breakpoints;
pub mod states;
pub mod responsive_values;
pub mod responsive_config;
pub mod responsive_builder;
pub mod flexbox;
pub mod grid;

// Re-export main types for backward compatibility
pub use breakpoints::Breakpoint;
pub use states::State;
pub use responsive_values::ResponsiveValue;
pub use responsive_config::{ResponsiveConfig, Responsive};
pub use responsive_builder::ResponsiveBuilder;
pub use flexbox::{FlexDirection, FlexWrap, JustifyContent, AlignItems, ResponsiveFlex};
pub use grid::ResponsiveGrid;

/// Create a new responsive configuration with default settings
pub fn create_responsive_config() -> ResponsiveConfig {
    ResponsiveConfig::default()
}

/// Create a new responsive builder
pub fn create_responsive_builder() -> ResponsiveBuilder {
    ResponsiveBuilder::default()
}

/// Create a new responsive flex container
pub fn create_responsive_flex() -> ResponsiveFlex {
    ResponsiveFlex::default()
}

/// Create a new responsive grid container
pub fn create_responsive_grid() -> ResponsiveGrid {
    ResponsiveGrid::default()
}

/// Utility functions for responsive design
pub mod utils {
    use super::*;
    
    /// Check if a breakpoint is active based on screen width
    pub fn is_breakpoint_active(breakpoint: Breakpoint, screen_width: u32) -> bool {
        screen_width >= breakpoint.min_width()
    }
    
    /// Get the appropriate breakpoint for a given screen width
    pub fn get_breakpoint_for_width(screen_width: u32) -> Breakpoint {
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
    
    /// Generate responsive classes for a given breakpoint
    pub fn generate_responsive_classes<T: ToString>(
        base: T,
        sm: Option<T>,
        md: Option<T>,
        lg: Option<T>,
        xl: Option<T>,
        xl2: Option<T>,
    ) -> String {
        let mut classes = Vec::new();
        
        classes.push(base.to_string());
        
        if let Some(sm_val) = sm {
            classes.push(format!("sm:{}", sm_val.to_string()));
        }
        if let Some(md_val) = md {
            classes.push(format!("md:{}", md_val.to_string()));
        }
        if let Some(lg_val) = lg {
            classes.push(format!("lg:{}", lg_val.to_string()));
        }
        if let Some(xl_val) = xl {
            classes.push(format!("xl:{}", xl_val.to_string()));
        }
        if let Some(xl2_val) = xl2 {
            classes.push(format!("2xl:{}", xl2_val.to_string()));
        }
        
        classes.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_responsive_config() {
        let config = create_responsive_config();
        assert_eq!(config.breakpoints.len(), 6); // Base, Sm, Md, Lg, Xl, Xl2
    }

    #[test]
    fn test_create_responsive_builder() {
        let builder = create_responsive_builder();
        assert!(builder.is_empty());
    }

    #[test]
    fn test_create_responsive_flex() {
        let flex = create_responsive_flex();
        assert_eq!(flex.direction.get_breakpoint(Breakpoint::Base), Some(&FlexDirection::Row));
    }

    #[test]
    fn test_create_responsive_grid() {
        let grid = create_responsive_grid();
        assert_eq!(grid.columns.get(&Breakpoint::Base), Some(&1));
    }

    #[test]
    fn test_is_breakpoint_active() {
        assert!(utils::is_breakpoint_active(Breakpoint::Base, 0));
        assert!(utils::is_breakpoint_active(Breakpoint::Sm, 640));
        assert!(utils::is_breakpoint_active(Breakpoint::Md, 768));
        assert!(!utils::is_breakpoint_active(Breakpoint::Sm, 639));
    }

    #[test]
    fn test_get_breakpoint_for_width() {
        assert_eq!(utils::get_breakpoint_for_width(0), Breakpoint::Base);
        assert_eq!(utils::get_breakpoint_for_width(640), Breakpoint::Sm);
        assert_eq!(utils::get_breakpoint_for_width(768), Breakpoint::Md);
        assert_eq!(utils::get_breakpoint_for_width(1024), Breakpoint::Lg);
        assert_eq!(utils::get_breakpoint_for_width(1280), Breakpoint::Xl);
        assert_eq!(utils::get_breakpoint_for_width(1536), Breakpoint::Xl2);
    }

    #[test]
    fn test_generate_responsive_classes() {
        let classes = utils::generate_responsive_classes(
            "text-sm",
            Some("text-base"),
            Some("text-lg"),
            Some("text-xl"),
            None,
            None,
        );
        
        assert!(classes.contains("text-sm"));
        assert!(classes.contains("sm:text-base"));
        assert!(classes.contains("md:text-lg"));
        assert!(classes.contains("lg:text-xl"));
        assert!(!classes.contains("xl:"));
        assert!(!classes.contains("2xl:"));
    }
}
