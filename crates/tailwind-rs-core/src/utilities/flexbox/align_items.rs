//! Align items utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Align items values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignItems {
    /// Start align items
    Start,
    /// End align items
    End,
    /// Center align items
    Center,
    /// Baseline align items
    Baseline,
    /// Stretch align items
    Stretch,
}

impl fmt::Display for AlignItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlignItems::Start => write!(f, "items-start"),
            AlignItems::End => write!(f, "items-end"),
            AlignItems::Center => write!(f, "items-center"),
            AlignItems::Baseline => write!(f, "items-baseline"),
            AlignItems::Stretch => write!(f, "items-stretch"),
        }
    }
}

/// Trait for adding align items utilities to a class builder
pub trait AlignItemsUtilities {
    fn align_items(self, align: AlignItems) -> Self;
}

impl AlignItemsUtilities for ClassBuilder {
    fn align_items(self, align: AlignItems) -> Self {
        self.class(align.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_items_utilities() {
        let classes = ClassBuilder::new().align_items(AlignItems::Center).build();

        assert!(classes.to_css_classes().contains("items-center"));
    }
}
