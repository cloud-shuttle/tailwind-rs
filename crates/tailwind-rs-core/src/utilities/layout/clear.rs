//! Clear utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Clear values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Clear {
    /// Left clear
    Left,
    /// Right clear
    Right,
    /// Both clear
    Both,
    /// None clear
    None,
}

impl fmt::Display for Clear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Clear::Left => write!(f, "clear-left"),
            Clear::Right => write!(f, "clear-right"),
            Clear::Both => write!(f, "clear-both"),
            Clear::None => write!(f, "clear-none"),
        }
    }
}

/// Trait for adding clear utilities to a class builder
pub trait ClearUtilities {
    fn clear(self, clear: Clear) -> Self;
}

impl ClearUtilities for ClassBuilder {
    fn clear(self, clear: Clear) -> Self {
        self.class(clear.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_utilities() {
        let classes = ClassBuilder::new()
            .clear(Clear::Both)
            .build();
        
        assert!(classes.to_css_classes().contains("clear-both"));
    }
}
