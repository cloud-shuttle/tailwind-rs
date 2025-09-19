//! Visibility utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Visibility values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Visibility {
    /// Visible
    Visible,
    /// Invisible
    Invisible,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Visibility::Visible => write!(f, "visible"),
            Visibility::Invisible => write!(f, "invisible"),
        }
    }
}

/// Trait for adding visibility utilities to a class builder
pub trait VisibilityUtilities {
    fn visibility(self, visibility: Visibility) -> Self;
}

impl VisibilityUtilities for ClassBuilder {
    fn visibility(self, visibility: Visibility) -> Self {
        self.class(visibility.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_utilities() {
        let classes = ClassBuilder::new()
            .visibility(Visibility::Visible)
            .build();
        
        assert!(classes.to_css_classes().contains("visible"));
    }
}
