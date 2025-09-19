//! Overscroll behavior utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Overscroll behavior values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OverscrollBehavior {
    /// Auto overscroll behavior
    Auto,
    /// Contain overscroll behavior
    Contain,
    /// None overscroll behavior
    None,
}

impl fmt::Display for OverscrollBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OverscrollBehavior::Auto => write!(f, "overscroll-auto"),
            OverscrollBehavior::Contain => write!(f, "overscroll-contain"),
            OverscrollBehavior::None => write!(f, "overscroll-none"),
        }
    }
}

/// Trait for adding overscroll behavior utilities to a class builder
pub trait OverscrollBehaviorUtilities {
    fn overscroll_behavior(self, overscroll: OverscrollBehavior) -> Self;
}

impl OverscrollBehaviorUtilities for ClassBuilder {
    fn overscroll_behavior(self, overscroll: OverscrollBehavior) -> Self {
        self.class(overscroll.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overscroll_behavior_utilities() {
        let classes = ClassBuilder::new()
            .overscroll_behavior(OverscrollBehavior::Contain)
            .build();
        
        assert!(classes.to_css_classes().contains("overscroll-contain"));
    }
}
