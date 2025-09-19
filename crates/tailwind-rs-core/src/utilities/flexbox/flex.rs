//! Flex utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Flex values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Flex {
    /// Flex 1
    Flex1,
    /// Flex auto
    FlexAuto,
    /// Flex initial
    FlexInitial,
    /// Flex none
    FlexNone,
}

impl fmt::Display for Flex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Flex::Flex1 => write!(f, "flex-1"),
            Flex::FlexAuto => write!(f, "flex-auto"),
            Flex::FlexInitial => write!(f, "flex-initial"),
            Flex::FlexNone => write!(f, "flex-none"),
        }
    }
}

/// Trait for adding flex utilities to a class builder
pub trait FlexUtilities {
    fn flex(self, flex: Flex) -> Self;
}

impl FlexUtilities for ClassBuilder {
    fn flex(self, flex: Flex) -> Self {
        self.class(flex.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_utilities() {
        let classes = ClassBuilder::new()
            .flex(Flex::Flex1)
            .build();
        
        assert!(classes.to_css_classes().contains("flex-1"));
    }
}
