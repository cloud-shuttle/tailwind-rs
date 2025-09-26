//! Align self utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Align self values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignSelf {
    /// Auto align self
    Auto,
    /// Start align self
    Start,
    /// End align self
    End,
    /// Center align self
    Center,
    /// Stretch align self
    Stretch,
    /// Baseline align self
    Baseline,
}

impl fmt::Display for AlignSelf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlignSelf::Auto => write!(f, "self-auto"),
            AlignSelf::Start => write!(f, "self-start"),
            AlignSelf::End => write!(f, "self-end"),
            AlignSelf::Center => write!(f, "self-center"),
            AlignSelf::Stretch => write!(f, "self-stretch"),
            AlignSelf::Baseline => write!(f, "self-baseline"),
        }
    }
}

/// Trait for adding align self utilities to a class builder
pub trait AlignSelfUtilities {
    fn align_self(self, align: AlignSelf) -> Self;
}

impl AlignSelfUtilities for ClassBuilder {
    fn align_self(self, align: AlignSelf) -> Self {
        self.class(align.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_self_utilities() {
        let classes = ClassBuilder::new().align_self(AlignSelf::Center).build();

        assert!(classes.to_css_classes().contains("self-center"));
    }
}
