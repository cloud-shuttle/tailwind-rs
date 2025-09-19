//! Flex grow utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Flex grow values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexGrow {
    /// Grow 0
    Grow0,
    /// Grow 1
    Grow,
}

impl fmt::Display for FlexGrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexGrow::Grow0 => write!(f, "grow-0"),
            FlexGrow::Grow => write!(f, "grow"),
        }
    }
}

/// Trait for adding flex grow utilities to a class builder
pub trait FlexGrowUtilities {
    fn flex_grow(self, grow: FlexGrow) -> Self;
}

impl FlexGrowUtilities for ClassBuilder {
    fn flex_grow(self, grow: FlexGrow) -> Self {
        self.class(grow.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_grow_utilities() {
        let classes = ClassBuilder::new()
            .flex_grow(FlexGrow::Grow)
            .build();
        
        assert!(classes.to_css_classes().contains("grow"));
    }
}
