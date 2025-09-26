//! Width utilities for tailwind-rs

use crate::classes::ClassBuilder;
use crate::utilities::sizing::SizingValue;

/// Trait for adding width utilities to a class builder
pub trait WidthUtilities {
    fn width(self, width: SizingValue) -> Self;
}

impl WidthUtilities for ClassBuilder {
    fn width(self, width: SizingValue) -> Self {
        self.class(format!("w-{}", width.to_class_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_width_utilities() {
        let classes = ClassBuilder::new().width(SizingValue::Full).build();

        assert!(classes.to_css_classes().contains("w-full"));
    }
}
