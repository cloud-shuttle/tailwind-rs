//! Flex basis utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Flex basis values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexBasis {
    /// Auto flex basis
    Auto,
    /// Full flex basis
    Full,
    /// Min content flex basis
    Min,
    /// Max content flex basis
    Max,
    /// Fit content flex basis
    Fit,
}

impl fmt::Display for FlexBasis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexBasis::Auto => write!(f, "basis-auto"),
            FlexBasis::Full => write!(f, "basis-full"),
            FlexBasis::Min => write!(f, "basis-min"),
            FlexBasis::Max => write!(f, "basis-max"),
            FlexBasis::Fit => write!(f, "basis-fit"),
        }
    }
}

/// Trait for adding flex basis utilities to a class builder
pub trait FlexBasisUtilities {
    fn flex_basis(self, basis: FlexBasis) -> Self;
}

impl FlexBasisUtilities for ClassBuilder {
    fn flex_basis(self, basis: FlexBasis) -> Self {
        self.class(basis.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_basis_utilities() {
        let classes = ClassBuilder::new().flex_basis(FlexBasis::Auto).build();

        assert!(classes.to_css_classes().contains("basis-auto"));
    }
}
