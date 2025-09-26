//! Float utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Float values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Float {
    /// Right float
    Right,
    /// Left float
    Left,
    /// None float
    None,
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Float::Right => write!(f, "float-right"),
            Float::Left => write!(f, "float-left"),
            Float::None => write!(f, "float-none"),
        }
    }
}

/// Trait for adding float utilities to a class builder
pub trait FloatUtilities {
    fn float(self, float: Float) -> Self;
}

impl FloatUtilities for ClassBuilder {
    fn float(self, float: Float) -> Self {
        self.class(float.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_utilities() {
        let classes = ClassBuilder::new().float(Float::Left).build();

        assert!(classes.to_css_classes().contains("float-left"));
    }
}
