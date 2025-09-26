//! Align content utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Align content values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignContent {
    /// Start align content
    Start,
    /// End align content
    End,
    /// Center align content
    Center,
    /// Between align content
    Between,
    /// Around align content
    Around,
    /// Evenly align content
    Evenly,
    /// Stretch align content
    Stretch,
}

impl fmt::Display for AlignContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlignContent::Start => write!(f, "content-start"),
            AlignContent::End => write!(f, "content-end"),
            AlignContent::Center => write!(f, "content-center"),
            AlignContent::Between => write!(f, "content-between"),
            AlignContent::Around => write!(f, "content-around"),
            AlignContent::Evenly => write!(f, "content-evenly"),
            AlignContent::Stretch => write!(f, "content-stretch"),
        }
    }
}

/// Trait for adding align content utilities to a class builder
pub trait AlignContentUtilities {
    fn align_content(self, align: AlignContent) -> Self;
}

impl AlignContentUtilities for ClassBuilder {
    fn align_content(self, align: AlignContent) -> Self {
        self.class(align.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_content_utilities() {
        let classes = ClassBuilder::new()
            .align_content(AlignContent::Center)
            .build();

        assert!(classes.to_css_classes().contains("content-center"));
    }
}
