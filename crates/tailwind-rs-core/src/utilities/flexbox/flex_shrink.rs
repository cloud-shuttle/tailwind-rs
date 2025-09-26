//! Flex shrink utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Flex shrink values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexShrink {
    /// Shrink 0
    Shrink0,
    /// Shrink 1
    Shrink,
}

impl fmt::Display for FlexShrink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexShrink::Shrink0 => write!(f, "shrink-0"),
            FlexShrink::Shrink => write!(f, "shrink"),
        }
    }
}

/// Trait for adding flex shrink utilities to a class builder
pub trait FlexShrinkUtilities {
    fn flex_shrink(self, shrink: FlexShrink) -> Self;
}

impl FlexShrinkUtilities for ClassBuilder {
    fn flex_shrink(self, shrink: FlexShrink) -> Self {
        self.class(shrink.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_shrink_utilities() {
        let classes = ClassBuilder::new().flex_shrink(FlexShrink::Shrink).build();

        assert!(classes.to_css_classes().contains("shrink"));
    }
}
