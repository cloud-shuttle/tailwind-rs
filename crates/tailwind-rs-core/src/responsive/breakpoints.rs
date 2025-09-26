//! # Breakpoint Definitions and Utilities
//!
//! This module provides breakpoint definitions and utilities for responsive design.

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Breakpoint definitions for responsive design
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Breakpoint {
    /// Base breakpoint (no prefix)
    Base,
    /// Small screens (640px and up)
    Sm,
    /// Medium screens (768px and up)
    Md,
    /// Large screens (1024px and up)
    Lg,
    /// Extra large screens (1280px and up)
    Xl,
    /// 2X large screens (1536px and up)
    Xl2,
}

impl Breakpoint {
    /// Get the minimum width for this breakpoint
    pub fn min_width(&self) -> u32 {
        match self {
            Breakpoint::Base => 0,
            Breakpoint::Sm => 640,
            Breakpoint::Md => 768,
            Breakpoint::Lg => 1024,
            Breakpoint::Xl => 1280,
            Breakpoint::Xl2 => 1536,
        }
    }

    /// Get the CSS media query for this breakpoint
    pub fn media_query(&self) -> &'static str {
        match self {
            Breakpoint::Base => "",
            Breakpoint::Sm => "(min-width: 640px)",
            Breakpoint::Md => "(min-width: 768px)",
            Breakpoint::Lg => "(min-width: 1024px)",
            Breakpoint::Xl => "(min-width: 1280px)",
            Breakpoint::Xl2 => "(min-width: 1536px)",
        }
    }

    /// Get the Tailwind CSS prefix for this breakpoint
    pub fn prefix(&self) -> &'static str {
        match self {
            Breakpoint::Base => "",
            Breakpoint::Sm => "sm:",
            Breakpoint::Md => "md:",
            Breakpoint::Lg => "lg:",
            Breakpoint::Xl => "xl:",
            Breakpoint::Xl2 => "2xl:",
        }
    }

    /// Get all breakpoints in order
    pub fn all() -> Vec<Breakpoint> {
        vec![
            Breakpoint::Base,
            Breakpoint::Sm,
            Breakpoint::Md,
            Breakpoint::Lg,
            Breakpoint::Xl,
            Breakpoint::Xl2,
        ]
    }

    /// Get the next breakpoint in the sequence
    pub fn next(&self) -> Option<Breakpoint> {
        match self {
            Breakpoint::Base => Some(Breakpoint::Sm),
            Breakpoint::Sm => Some(Breakpoint::Md),
            Breakpoint::Md => Some(Breakpoint::Lg),
            Breakpoint::Lg => Some(Breakpoint::Xl),
            Breakpoint::Xl => Some(Breakpoint::Xl2),
            Breakpoint::Xl2 => None,
        }
    }

    /// Get the previous breakpoint in the sequence
    pub fn previous(&self) -> Option<Breakpoint> {
        match self {
            Breakpoint::Base => None,
            Breakpoint::Sm => Some(Breakpoint::Base),
            Breakpoint::Md => Some(Breakpoint::Sm),
            Breakpoint::Lg => Some(Breakpoint::Md),
            Breakpoint::Xl => Some(Breakpoint::Lg),
            Breakpoint::Xl2 => Some(Breakpoint::Xl),
        }
    }
}

impl FromStr for Breakpoint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "" | "base" => Ok(Breakpoint::Base),
            "sm" => Ok(Breakpoint::Sm),
            "md" => Ok(Breakpoint::Md),
            "lg" => Ok(Breakpoint::Lg),
            "xl" => Ok(Breakpoint::Xl),
            "2xl" | "xl2" => Ok(Breakpoint::Xl2),
            _ => Err(format!("Invalid breakpoint: {}", s)),
        }
    }
}

impl std::fmt::Display for Breakpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Breakpoint::Base => write!(f, "base"),
            Breakpoint::Sm => write!(f, "sm"),
            Breakpoint::Md => write!(f, "md"),
            Breakpoint::Lg => write!(f, "lg"),
            Breakpoint::Xl => write!(f, "xl"),
            Breakpoint::Xl2 => write!(f, "2xl"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_min_width() {
        assert_eq!(Breakpoint::Base.min_width(), 0);
        assert_eq!(Breakpoint::Sm.min_width(), 640);
        assert_eq!(Breakpoint::Md.min_width(), 768);
        assert_eq!(Breakpoint::Lg.min_width(), 1024);
        assert_eq!(Breakpoint::Xl.min_width(), 1280);
        assert_eq!(Breakpoint::Xl2.min_width(), 1536);
    }

    #[test]
    fn test_breakpoint_media_query() {
        assert_eq!(Breakpoint::Base.media_query(), "");
        assert_eq!(Breakpoint::Sm.media_query(), "(min-width: 640px)");
        assert_eq!(Breakpoint::Md.media_query(), "(min-width: 768px)");
        assert_eq!(Breakpoint::Lg.media_query(), "(min-width: 1024px)");
        assert_eq!(Breakpoint::Xl.media_query(), "(min-width: 1280px)");
        assert_eq!(Breakpoint::Xl2.media_query(), "(min-width: 1536px)");
    }

    #[test]
    fn test_breakpoint_prefix() {
        assert_eq!(Breakpoint::Base.prefix(), "");
        assert_eq!(Breakpoint::Sm.prefix(), "sm:");
        assert_eq!(Breakpoint::Md.prefix(), "md:");
        assert_eq!(Breakpoint::Lg.prefix(), "lg:");
        assert_eq!(Breakpoint::Xl.prefix(), "xl:");
        assert_eq!(Breakpoint::Xl2.prefix(), "2xl:");
    }

    #[test]
    fn test_breakpoint_from_str() {
        assert_eq!(Breakpoint::from_str("").unwrap(), Breakpoint::Base);
        assert_eq!(Breakpoint::from_str("base").unwrap(), Breakpoint::Base);
        assert_eq!(Breakpoint::from_str("sm").unwrap(), Breakpoint::Sm);
        assert_eq!(Breakpoint::from_str("md").unwrap(), Breakpoint::Md);
        assert_eq!(Breakpoint::from_str("lg").unwrap(), Breakpoint::Lg);
        assert_eq!(Breakpoint::from_str("xl").unwrap(), Breakpoint::Xl);
        assert_eq!(Breakpoint::from_str("2xl").unwrap(), Breakpoint::Xl2);
        assert_eq!(Breakpoint::from_str("xl2").unwrap(), Breakpoint::Xl2);
    }

    #[test]
    fn test_breakpoint_display() {
        assert_eq!(format!("{}", Breakpoint::Base), "base");
        assert_eq!(format!("{}", Breakpoint::Sm), "sm");
        assert_eq!(format!("{}", Breakpoint::Md), "md");
        assert_eq!(format!("{}", Breakpoint::Lg), "lg");
        assert_eq!(format!("{}", Breakpoint::Xl), "xl");
        assert_eq!(format!("{}", Breakpoint::Xl2), "2xl");
    }

    #[test]
    fn test_breakpoint_all() {
        let all = Breakpoint::all();
        assert_eq!(all.len(), 6);
        assert_eq!(all[0], Breakpoint::Base);
        assert_eq!(all[1], Breakpoint::Sm);
        assert_eq!(all[2], Breakpoint::Md);
        assert_eq!(all[3], Breakpoint::Lg);
        assert_eq!(all[4], Breakpoint::Xl);
        assert_eq!(all[5], Breakpoint::Xl2);
    }

    #[test]
    fn test_breakpoint_next() {
        assert_eq!(Breakpoint::Base.next(), Some(Breakpoint::Sm));
        assert_eq!(Breakpoint::Sm.next(), Some(Breakpoint::Md));
        assert_eq!(Breakpoint::Md.next(), Some(Breakpoint::Lg));
        assert_eq!(Breakpoint::Lg.next(), Some(Breakpoint::Xl));
        assert_eq!(Breakpoint::Xl.next(), Some(Breakpoint::Xl2));
        assert_eq!(Breakpoint::Xl2.next(), None);
    }

    #[test]
    fn test_breakpoint_previous() {
        assert_eq!(Breakpoint::Base.previous(), None);
        assert_eq!(Breakpoint::Sm.previous(), Some(Breakpoint::Base));
        assert_eq!(Breakpoint::Md.previous(), Some(Breakpoint::Sm));
        assert_eq!(Breakpoint::Lg.previous(), Some(Breakpoint::Md));
        assert_eq!(Breakpoint::Xl.previous(), Some(Breakpoint::Lg));
        assert_eq!(Breakpoint::Xl2.previous(), Some(Breakpoint::Xl));
    }
}
