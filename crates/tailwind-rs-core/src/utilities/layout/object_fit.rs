//! Object fit utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Object fit values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectFit {
    /// Contain object fit
    Contain,
    /// Cover object fit
    Cover,
    /// Fill object fit
    Fill,
    /// None object fit
    None,
    /// Scale down object fit
    ScaleDown,
}

impl fmt::Display for ObjectFit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectFit::Contain => write!(f, "object-contain"),
            ObjectFit::Cover => write!(f, "object-cover"),
            ObjectFit::Fill => write!(f, "object-fill"),
            ObjectFit::None => write!(f, "object-none"),
            ObjectFit::ScaleDown => write!(f, "object-scale-down"),
        }
    }
}

/// Trait for adding object fit utilities to a class builder
pub trait ObjectFitUtilities {
    fn object_fit(self, object_fit: ObjectFit) -> Self;
}

impl ObjectFitUtilities for ClassBuilder {
    fn object_fit(self, object_fit: ObjectFit) -> Self {
        self.class(object_fit.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_fit_utilities() {
        let classes = ClassBuilder::new().object_fit(ObjectFit::Cover).build();

        assert!(classes.to_css_classes().contains("object-cover"));
    }
}
