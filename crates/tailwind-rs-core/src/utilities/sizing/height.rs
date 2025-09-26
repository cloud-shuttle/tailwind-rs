//! Height utilities for tailwind-rs

use crate::classes::ClassBuilder;
use crate::utilities::sizing::SizingValue;

/// Trait for adding height utilities to a class builder
pub trait HeightUtilities {
    fn height(self, height: SizingValue) -> Self;
}

impl HeightUtilities for ClassBuilder {
    fn height(self, height: SizingValue) -> Self {
        self.class(format!("h-{}", height.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_utilities() {
        let classes = ClassBuilder::new().height(SizingValue::Full).build();

        assert!(classes.to_css_classes().contains("h-full"));
    }
}
