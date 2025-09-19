//! Z-index utilities for tailwind-rs

use crate::classes::ClassBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Z-index values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZIndex {
    /// Auto z-index
    Auto,
    /// Z-index 0
    Z0,
    /// Z-index 10
    Z10,
    /// Z-index 20
    Z20,
    /// Z-index 30
    Z30,
    /// Z-index 40
    Z40,
    /// Z-index 50
    Z50,
}

impl fmt::Display for ZIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZIndex::Auto => write!(f, "z-auto"),
            ZIndex::Z0 => write!(f, "z-0"),
            ZIndex::Z10 => write!(f, "z-10"),
            ZIndex::Z20 => write!(f, "z-20"),
            ZIndex::Z30 => write!(f, "z-30"),
            ZIndex::Z40 => write!(f, "z-40"),
            ZIndex::Z50 => write!(f, "z-50"),
        }
    }
}

/// Trait for adding z-index utilities to a class builder
pub trait ZIndexUtilities {
    fn z_index(self, z_index: ZIndex) -> Self;
}

impl ZIndexUtilities for ClassBuilder {
    fn z_index(self, z_index: ZIndex) -> Self {
        self.class(z_index.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_index_utilities() {
        let classes = ClassBuilder::new()
            .z_index(ZIndex::Z10)
            .build();
        
        assert!(classes.to_css_classes().contains("z-10"));
    }
}
