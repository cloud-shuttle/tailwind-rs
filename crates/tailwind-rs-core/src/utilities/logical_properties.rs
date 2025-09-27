//! CSS Logical Properties utilities for tailwind-rs
//!
//! This module provides utilities for CSS logical properties.
//! It includes inline-start and inline-end properties for better internationalization support.

use crate::classes::ClassBuilder;
use crate::utilities::spacing::SpacingValue;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Logical direction values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogicalDirection {
    /// Inline start (left in LTR, right in RTL)
    InlineStart,
    /// Inline end (right in LTR, left in RTL)
    InlineEnd,
    /// Block start (top in horizontal writing modes)
    BlockStart,
    /// Block end (bottom in horizontal writing modes)
    BlockEnd,
}

impl fmt::Display for LogicalDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicalDirection::InlineStart => write!(f, "inline-start"),
            LogicalDirection::InlineEnd => write!(f, "inline-end"),
            LogicalDirection::BlockStart => write!(f, "block-start"),
            LogicalDirection::BlockEnd => write!(f, "block-end"),
        }
    }
}

/// Trait for adding CSS logical properties to ClassBuilder
pub trait LogicalPropertiesUtilities {
    /// Set margin inline start
    fn margin_inline_start(self, value: SpacingValue) -> Self;
    /// Set margin inline end
    fn margin_inline_end(self, value: SpacingValue) -> Self;
    /// Set margin block start
    fn margin_block_start(self, value: SpacingValue) -> Self;
    /// Set margin block end
    fn margin_block_end(self, value: SpacingValue) -> Self;
    /// Set padding inline start
    fn padding_inline_start(self, value: SpacingValue) -> Self;
    /// Set padding inline end
    fn padding_inline_end(self, value: SpacingValue) -> Self;
    /// Set padding block start
    fn padding_block_start(self, value: SpacingValue) -> Self;
    /// Set padding block end
    fn padding_block_end(self, value: SpacingValue) -> Self;
    /// Set border inline start
    fn border_inline_start(self, value: SpacingValue) -> Self;
    /// Set border inline end
    fn border_inline_end(self, value: SpacingValue) -> Self;
    /// Set border block start
    fn border_block_start(self, value: SpacingValue) -> Self;
    /// Set border block end
    fn border_block_end(self, value: SpacingValue) -> Self;
    /// Set inset inline start
    fn inset_inline_start(self, value: SpacingValue) -> Self;
    /// Set inset inline end
    fn inset_inline_end(self, value: SpacingValue) -> Self;
    /// Set inset block start
    fn inset_block_start(self, value: SpacingValue) -> Self;
    /// Set inset block end
    fn inset_block_end(self, value: SpacingValue) -> Self;
}

impl LogicalPropertiesUtilities for ClassBuilder {
    fn margin_inline_start(self, value: SpacingValue) -> Self {
        let class_name = format!("ms-{}", value);
        self.class(class_name)
    }

    fn margin_inline_end(self, value: SpacingValue) -> Self {
        let class_name = format!("me-{}", value);
        self.class(class_name)
    }

    fn margin_block_start(self, value: SpacingValue) -> Self {
        let class_name = format!("mt-{}", value);
        self.class(class_name)
    }

    fn margin_block_end(self, value: SpacingValue) -> Self {
        let class_name = format!("mb-{}", value);
        self.class(class_name)
    }

    fn padding_inline_start(self, value: SpacingValue) -> Self {
        let class_name = format!("ps-{}", value);
        self.class(class_name)
    }

    fn padding_inline_end(self, value: SpacingValue) -> Self {
        let class_name = format!("pe-{}", value);
        self.class(class_name)
    }

    fn padding_block_start(self, value: SpacingValue) -> Self {
        let class_name = format!("pt-{}", value);
        self.class(class_name)
    }

    fn padding_block_end(self, value: SpacingValue) -> Self {
        let class_name = format!("pb-{}", value);
        self.class(class_name)
    }

    fn border_inline_start(self, value: SpacingValue) -> Self {
        let class_name = format!("border-s-{}", value);
        self.class(class_name)
    }

    fn border_inline_end(self, value: SpacingValue) -> Self {
        let class_name = format!("border-e-{}", value);
        self.class(class_name)
    }

    fn border_block_start(self, value: SpacingValue) -> Self {
        let class_name = format!("border-t-{}", value);
        self.class(class_name)
    }

    fn border_block_end(self, value: SpacingValue) -> Self {
        let class_name = format!("border-b-{}", value);
        self.class(class_name)
    }

    fn inset_inline_start(self, value: SpacingValue) -> Self {
        let class_name = format!("start-{}", value);
        self.class(class_name)
    }

    fn inset_inline_end(self, value: SpacingValue) -> Self {
        let class_name = format!("end-{}", value);
        self.class(class_name)
    }

    fn inset_block_start(self, value: SpacingValue) -> Self {
        let class_name = format!("top-{}", value);
        self.class(class_name)
    }

    fn inset_block_end(self, value: SpacingValue) -> Self {
        let class_name = format!("bottom-{}", value);
        self.class(class_name)
    }
}

/// Convenience methods for common spacing values
pub trait LogicalPropertiesConvenience {
    /// Set margin inline start to 4
    fn margin_inline_start_4(self) -> Self;
    /// Set margin inline end to 4
    fn margin_inline_end_4(self) -> Self;
    /// Set padding inline start to 2
    fn padding_inline_start_2(self) -> Self;
    /// Set padding inline end to 2
    fn padding_inline_end_2(self) -> Self;
    /// Set padding inline start to 4
    fn padding_inline_start_4(self) -> Self;
    /// Set padding inline end to 4
    fn padding_inline_end_4(self) -> Self;
    /// Set border inline start to 1
    fn border_inline_start_1(self) -> Self;
    /// Set border inline end to 1
    fn border_inline_end_1(self) -> Self;
    /// Set border inline start to 2
    fn border_inline_start_2(self) -> Self;
    /// Set border inline end to 2
    fn border_inline_end_2(self) -> Self;
}

impl LogicalPropertiesConvenience for ClassBuilder {
    fn margin_inline_start_4(self) -> Self {
        self.margin_inline_start(SpacingValue::Integer(4))
    }

    fn margin_inline_end_4(self) -> Self {
        self.margin_inline_end(SpacingValue::Integer(4))
    }

    fn padding_inline_start_2(self) -> Self {
        self.padding_inline_start(SpacingValue::Integer(2))
    }

    fn padding_inline_end_2(self) -> Self {
        self.padding_inline_end(SpacingValue::Integer(2))
    }

    fn border_inline_start_1(self) -> Self {
        self.border_inline_start(SpacingValue::Integer(1))
    }

    fn border_inline_end_1(self) -> Self {
        self.border_inline_end(SpacingValue::Integer(1))
    }

    fn padding_inline_start_4(self) -> Self {
        self.padding_inline_start(SpacingValue::Integer(4))
    }

    fn padding_inline_end_4(self) -> Self {
        self.padding_inline_end(SpacingValue::Integer(4))
    }

    fn border_inline_start_2(self) -> Self {
        self.border_inline_start(SpacingValue::Integer(2))
    }

    fn border_inline_end_2(self) -> Self {
        self.border_inline_end(SpacingValue::Integer(2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classes::ClassBuilder;
    use crate::utilities::spacing::SpacingValue;

    #[test]
    fn test_logical_direction_enum_values() {
        assert_eq!(LogicalDirection::InlineStart.to_string(), "inline-start");
        assert_eq!(LogicalDirection::InlineEnd.to_string(), "inline-end");
        assert_eq!(LogicalDirection::BlockStart.to_string(), "block-start");
        assert_eq!(LogicalDirection::BlockEnd.to_string(), "block-end");
    }

    #[test]
    fn test_logical_properties_utilities() {
        let classes = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(4))
            .margin_inline_end(SpacingValue::Integer(4))
            .padding_inline_start(SpacingValue::Integer(2))
            .padding_inline_end(SpacingValue::Integer(2))
            .border_inline_start(SpacingValue::Integer(1))
            .border_inline_end(SpacingValue::Integer(1));

        let result = classes.build();
        assert!(result.classes.contains("ms-4"));
        assert!(result.classes.contains("me-4"));
        assert!(result.classes.contains("ps-2"));
        assert!(result.classes.contains("pe-2"));
        assert!(result.classes.contains("border-s-1"));
        assert!(result.classes.contains("border-e-1"));
    }

    #[test]
    fn test_logical_properties_convenience() {
        let classes = ClassBuilder::new()
            .margin_inline_start_4()
            .margin_inline_end_4()
            .padding_inline_start_4()
            .padding_inline_end_4()
            .border_inline_start_2()
            .border_inline_end_2();

        let result = classes.build();
        assert!(result.classes.contains("ms-4"));
        assert!(result.classes.contains("me-4"));
        assert!(result.classes.contains("ps-4"));
        assert!(result.classes.contains("pe-4"));
        assert!(result.classes.contains("border-s-2"));
        assert!(result.classes.contains("border-e-2"));
    }

    #[test]
    fn test_logical_properties_serialization() {
        let direction = LogicalDirection::InlineStart;
        let serialized = serde_json::to_string(&direction).unwrap();
        let deserialized: LogicalDirection = serde_json::from_str(&serialized).unwrap();
        assert_eq!(direction, deserialized);
    }

    #[test]
    fn test_logical_properties_comprehensive_usage() {
        let classes = ClassBuilder::new()
            .margin_inline_start_4()
            .margin_inline_end_4()
            .padding_inline_start_2()
            .padding_inline_end_2()
            .border_inline_start_1()
            .border_inline_end_1();

        let result = classes.build();
        assert!(result.classes.contains("ms-4"));
        assert!(result.classes.contains("me-4"));
        assert!(result.classes.contains("ps-2"));
        assert!(result.classes.contains("pe-2"));
        assert!(result.classes.contains("border-s-1"));
        assert!(result.classes.contains("border-e-1"));
    }

    #[test]
    fn test_logical_properties_with_different_spacing_values() {
        let classes = ClassBuilder::new()
            .margin_inline_start(SpacingValue::Integer(8))
            .margin_inline_end(SpacingValue::Integer(12))
            .padding_inline_start(SpacingValue::Integer(6))
            .padding_inline_end(SpacingValue::Integer(10));

        let result = classes.build();
        assert!(result.classes.contains("ms-8"));
        assert!(result.classes.contains("me-12"));
        assert!(result.classes.contains("ps-6"));
        assert!(result.classes.contains("pe-10"));
    }

    #[test]
    fn test_logical_properties_block_directions() {
        let classes = ClassBuilder::new()
            .margin_block_start(SpacingValue::Integer(4))
            .margin_block_end(SpacingValue::Integer(4))
            .padding_block_start(SpacingValue::Integer(2))
            .padding_block_end(SpacingValue::Integer(2));

        let result = classes.build();
        assert!(result.classes.contains("mt-4"));
        assert!(result.classes.contains("mb-4"));
        assert!(result.classes.contains("pt-2"));
        assert!(result.classes.contains("pb-2"));
    }

    #[test]
    fn test_logical_properties_inset() {
        let classes = ClassBuilder::new()
            .inset_inline_start(SpacingValue::Integer(4))
            .inset_inline_end(SpacingValue::Integer(4))
            .inset_block_start(SpacingValue::Integer(2))
            .inset_block_end(SpacingValue::Integer(2));

        let result = classes.build();
        assert!(result.classes.contains("start-4"));
        assert!(result.classes.contains("end-4"));
        assert!(result.classes.contains("top-2"));
        assert!(result.classes.contains("bottom-2"));
    }
}
