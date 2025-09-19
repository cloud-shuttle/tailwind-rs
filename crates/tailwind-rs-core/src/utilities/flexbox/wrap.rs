//! Flex wrap utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Flex wrap values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexWrap {
    /// Wrap flex
    Wrap,
    /// Wrap reverse flex
    WrapReverse,
    /// No wrap flex
    Nowrap,
}

impl fmt::Display for FlexWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlexWrap::Wrap => write!(f, "flex-wrap"),
            FlexWrap::WrapReverse => write!(f, "flex-wrap-reverse"),
            FlexWrap::Nowrap => write!(f, "flex-nowrap"),
        }
    }
}

/// Trait for adding flex wrap utilities to a class builder
pub trait FlexWrapUtilities {
    fn flex_wrap(self, wrap: FlexWrap) -> Self;
}

impl FlexWrapUtilities for ClassBuilder {
    fn flex_wrap(self, wrap: FlexWrap) -> Self {
        self.class(wrap.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_wrap_utilities() {
        let classes = ClassBuilder::new()
            .flex_wrap(FlexWrap::Wrap)
            .build();
        
        assert!(classes.to_css_classes().contains("flex-wrap"));
    }
}
