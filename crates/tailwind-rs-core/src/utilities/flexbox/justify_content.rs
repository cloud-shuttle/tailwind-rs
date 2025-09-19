//! Justify content utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Justify content values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JustifyContent {
    /// Start justify content
    Start,
    /// End justify content
    End,
    /// Center justify content
    Center,
    /// Between justify content
    Between,
    /// Around justify content
    Around,
    /// Evenly justify content
    Evenly,
}

impl fmt::Display for JustifyContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JustifyContent::Start => write!(f, "justify-start"),
            JustifyContent::End => write!(f, "justify-end"),
            JustifyContent::Center => write!(f, "justify-center"),
            JustifyContent::Between => write!(f, "justify-between"),
            JustifyContent::Around => write!(f, "justify-around"),
            JustifyContent::Evenly => write!(f, "justify-evenly"),
        }
    }
}

/// Trait for adding justify content utilities to a class builder
pub trait JustifyContentUtilities {
    fn justify_content(self, justify: JustifyContent) -> Self;
}

impl JustifyContentUtilities for ClassBuilder {
    fn justify_content(self, justify: JustifyContent) -> Self {
        self.class(justify.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_justify_content_utilities() {
        let classes = ClassBuilder::new()
            .justify_content(JustifyContent::Center)
            .build();
        
        assert!(classes.to_css_classes().contains("justify-center"));
    }
}
