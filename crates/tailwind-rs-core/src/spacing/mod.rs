//! Spacing Module
//!
//! Comprehensive spacing utilities for Tailwind CSS parsing and generation.
//!
//! This module provides complete support for:
//! - Padding utilities (p-*, px-*, py-*, pt-*, pr-*, pb-*, pl-*, ps-*, pe-*)
//! - Margin utilities (m-*, mx-*, my-*, mt-*, mr-*, mb-*, ml-*, ms-*, me-*)
//! - Gap utilities (gap-*, gap-x-*, gap-y-*)
//! - Arbitrary values ([10px], [2rem], etc.)
//! - Logical properties (ps-*, pe-*, ms-*, me-*)
//!
//! ## Architecture
//!
//! ```text
//! spacing/
//! ├── mod.rs           # Module exports and coordination
//! ├── parser.rs        # Main SpacingParser implementation
//! ├── utilities.rs     # Core parsing utilities and logic
//! ├── values.rs        # Spacing value mappings and conversions
//! ├── constants.rs     # Patterns, prefixes, and configuration
//! └── tests.rs         # Comprehensive test suite
//! ```
//!
//! ## Key Components
//!
//! ### SpacingParser
//! The main parser that coordinates all spacing-related functionality.
//!
//! ```rust
//! use tailwind_rs_core::spacing::SpacingParser;
//!
//! let parser = SpacingParser::new();
//! let properties = parser.parse_spacing_class("p-4");
//! assert_eq!(properties.unwrap()[0].value, "1rem");
//! ```
//!
//! ### SpacingUtilities
//! Core utility functions for parsing different spacing types.
//!
//! ### SpacingValues
//! Value mapping from Tailwind tokens to CSS values.
//!
//! ## Usage Examples
//!
//! ### Basic Parsing
//! ```rust
//! use tailwind_rs_core::spacing::SpacingParser;
//!
//! let parser = SpacingParser::new();
//!
//! // Parse padding
//! let padding = parser.parse_spacing_class("p-4");
//!
//! // Parse margin
//! let margin = parser.parse_spacing_class("m-2");
//!
//! // Parse gap
//! let gap = parser.parse_spacing_class("gap-3");
//! ```
//!
//! ### Arbitrary Values
//! ```rust
//! let parser = SpacingParser::new();
//!
//! // Parse arbitrary padding
//! let padding = parser.parse_spacing_class("p-[10px]");
//!
//! // Parse arbitrary margin
//! let margin = parser.parse_spacing_class("m-[2rem]");
//! ```
//!
//! ### Logical Properties
//! ```rust
//! let parser = SpacingParser::new();
//!
//! // Parse logical padding
//! let start_padding = parser.parse_spacing_class("ps-4");
//! let end_padding = parser.parse_spacing_class("pe-2");
//! ```
//!
//! ## Configuration
//!
//! The parser can be configured to enable/disable features:
//!
//! ```rust
//! use tailwind_rs_core::spacing::{SpacingParserBuilder, SpacingParserConfig};
//!
//! let parser = SpacingParserBuilder::new()
//!     .arbitrary_values(true)
//!     .logical_properties(true)
//!     .axis_properties(true)
//!     .max_spacing_scale(96.0)
//!     .build();
//! ```
//!
//! ## Performance
//!
//! - **Fast parsing**: Direct string matching with no regex
//! - **Memory efficient**: Minimal allocations
//! - **Zero-copy**: Where possible, uses string references
//! - **Configurable**: Disable unused features for better performance
//!
//! ## Testing
//!
//! Comprehensive test suite covering:
//! - All standard spacing values (0-96)
//! - Arbitrary value parsing
//! - Logical properties
//! - Axis properties
//! - Edge cases and error handling
//! - Performance benchmarks
//! - Memory safety tests

// Core modules
pub mod constants;
pub mod parser;
pub mod tests;
pub mod utilities;
pub mod values;

// Re-exports for convenience
pub use parser::{SpacingParser, SpacingParserBuilder};
pub use utilities::SpacingUtilities;
pub use values::{SpacingValues, SpacingScale, SpacingValueUtils};
pub use constants::{
    SpacingDirections, SpacingAxes, SpacingPrefixes,
    SpacingValidation, SpacingParserConfig, SPACING_PATTERNS,
    SpacingPriority,
};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn module_exports() {
        // Test that all expected types are available
        let _parser: SpacingParser = SpacingParser::new();
        let _utilities: SpacingUtilities = SpacingUtilities::new();
        let _values: SpacingValues = SpacingValues::new();
        let _config: SpacingParserConfig = SpacingParserConfig::default();
    }

    #[test]
    fn cross_module_integration() {
        // Test that modules work together correctly
        let parser = SpacingParser::new();
        let utilities = parser.utilities();
        let values = utilities.get_spacing_value("4");

        assert_eq!(values, Some("1rem".to_string()));

        let result = parser.parse_spacing_class("p-4");
        assert!(result.is_some());
    }

    #[test]
    fn builder_pattern() {
        let parser = SpacingParserBuilder::new()
            .arbitrary_values(false)
            .build();

        // Should not parse arbitrary values
        assert!(parser.parse_spacing_class("p-[10px]").is_none());

        // Should still parse regular values
        assert!(parser.parse_spacing_class("p-4").is_some());
    }
}
