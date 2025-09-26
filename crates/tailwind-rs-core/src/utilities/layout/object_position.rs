//! Object position utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Object position values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectPosition {
    /// Bottom object position
    Bottom,
    /// Center object position
    Center,
    /// Left object position
    Left,
    /// Left bottom object position
    LeftBottom,
    /// Left top object position
    LeftTop,
    /// Right object position
    Right,
    /// Right bottom object position
    RightBottom,
    /// Right top object position
    RightTop,
    /// Top object position
    Top,
}

impl fmt::Display for ObjectPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectPosition::Bottom => write!(f, "object-bottom"),
            ObjectPosition::Center => write!(f, "object-center"),
            ObjectPosition::Left => write!(f, "object-left"),
            ObjectPosition::LeftBottom => write!(f, "object-left-bottom"),
            ObjectPosition::LeftTop => write!(f, "object-left-top"),
            ObjectPosition::Right => write!(f, "object-right"),
            ObjectPosition::RightBottom => write!(f, "object-right-bottom"),
            ObjectPosition::RightTop => write!(f, "object-right-top"),
            ObjectPosition::Top => write!(f, "object-top"),
        }
    }
}

/// Trait for adding object position utilities to a class builder
pub trait ObjectPositionUtilities {
    fn object_position(self, object_position: ObjectPosition) -> Self;
}

impl ObjectPositionUtilities for ClassBuilder {
    fn object_position(self, object_position: ObjectPosition) -> Self {
        self.class(object_position.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_position_utilities() {
        let classes = ClassBuilder::new()
            .object_position(ObjectPosition::Center)
            .build();

        assert!(classes.to_css_classes().contains("object-center"));
    }
}
