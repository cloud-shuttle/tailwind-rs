//! Isolation utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Isolation values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Isolation {
    /// Isolate
    Isolate,
    /// Isolate auto
    IsolateAuto,
}

impl fmt::Display for Isolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Isolation::Isolate => write!(f, "isolate"),
            Isolation::IsolateAuto => write!(f, "isolate-auto"),
        }
    }
}

/// Trait for adding isolation utilities to a class builder
pub trait IsolationUtilities {
    fn isolation(self, isolation: Isolation) -> Self;
}

impl IsolationUtilities for ClassBuilder {
    fn isolation(self, isolation: Isolation) -> Self {
        self.class(isolation.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isolation_utilities() {
        let classes = ClassBuilder::new().isolation(Isolation::Isolate).build();

        assert!(classes.to_css_classes().contains("isolate"));
    }
}
