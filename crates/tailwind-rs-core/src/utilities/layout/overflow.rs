//! Overflow utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Overflow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Overflow {
    /// Auto overflow
    Auto,
    /// Hidden overflow
    Hidden,
    /// Clip overflow
    Clip,
    /// Visible overflow
    Visible,
    /// Scroll overflow
    Scroll,
}

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Overflow::Auto => write!(f, "overflow-auto"),
            Overflow::Hidden => write!(f, "overflow-hidden"),
            Overflow::Clip => write!(f, "overflow-clip"),
            Overflow::Visible => write!(f, "overflow-visible"),
            Overflow::Scroll => write!(f, "overflow-scroll"),
        }
    }
}

/// Trait for adding overflow utilities to a class builder
pub trait OverflowUtilities {
    fn overflow(self, overflow: Overflow) -> Self;
}

impl OverflowUtilities for ClassBuilder {
    fn overflow(self, overflow: Overflow) -> Self {
        self.class(overflow.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_utilities() {
        let classes = ClassBuilder::new()
            .overflow(Overflow::Hidden)
            .build();
        
        assert!(classes.to_css_classes().contains("overflow-hidden"));
    }
}
