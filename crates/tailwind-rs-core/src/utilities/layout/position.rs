//! Position utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Position values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    /// Static position
    Static,
    /// Fixed position
    Fixed,
    /// Absolute position
    Absolute,
    /// Relative position
    Relative,
    /// Sticky position
    Sticky,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Position::Static => write!(f, "static"),
            Position::Fixed => write!(f, "fixed"),
            Position::Absolute => write!(f, "absolute"),
            Position::Relative => write!(f, "relative"),
            Position::Sticky => write!(f, "sticky"),
        }
    }
}

/// Trait for adding position utilities to a class builder
pub trait PositionUtilities {
    fn position(self, position: Position) -> Self;
}

impl PositionUtilities for ClassBuilder {
    fn position(self, position: Position) -> Self {
        self.class(position.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_utilities() {
        let classes = ClassBuilder::new()
            .position(Position::Relative)
            .build();
        
        assert!(classes.to_css_classes().contains("relative"));
    }
}
