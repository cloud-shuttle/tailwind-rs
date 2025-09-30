//! Enhanced Variants Module
//!
//! A comprehensive system for parsing and managing complex Tailwind CSS variant combinations,
//! supporting responsive variants, state variants, dark mode, motion preferences, and custom variants.
//!
//! ## Architecture
//!
//! ```text
//! enhanced_variants/
//! ├── mod.rs              # Module exports and coordination
//! ├── types.rs            # Core types and data structures
//! ├── definitions.rs      # Built-in variant definitions
//! ├── parser.rs           # Main parsing logic and API
//! ├── utilities.rs        # Optimization and analysis utilities
//! ├── combinations.rs     # Complex variant combination handling
//! └── tests.rs            # Comprehensive test suite
//! ```
//!
//! ## Key Features
//!
//! - **Comprehensive Variant Support**: All standard Tailwind variants
//! - **Complex Combinations**: Multi-variant combinations with proper specificity
//! - **Custom Variants**: User-defined variants with full integration
//! - **Optimization**: Automatic combination optimization and conflict detection
//! - **Caching**: Built-in result caching for performance
//! - **Analysis**: Usage statistics and optimization suggestions
//!
//! ## Basic Usage
//!
//! ```rust
//! use tailwind_rs_core::enhanced_variants::EnhancedVariantParser;
//!
//! let parser = EnhancedVariantParser::new();
//!
//! // Parse a simple variant
//! let result = parser.parse_class("hover:bg-blue-500").unwrap();
//! assert_eq!(result.base_class, "bg-blue-500");
//!
//! // Parse a complex combination
//! let result = parser.parse_class("sm:dark:hover:bg-gradient-to-r").unwrap();
//! assert!(result.success);
//! assert!(result.media_query.is_some());
//! ```
//!
//! ## Advanced Usage
//!
//! ### Custom Variants
//! ```rust
//! use tailwind_rs_core::enhanced_variants::EnhancedVariantParser;
//!
//! let mut parser = EnhancedVariantParser::new();
//!
//! // Add custom variants
//! parser.add_custom_variant("group-hover".to_string(), ".group:hover ".to_string())?;
//! parser.add_custom_variant("peer-focus".to_string(), ".peer:focus ~ ".to_string())?;
//!
//! // Use custom variants
//! let result = parser.parse_class("group-hover:bg-blue-500").unwrap();
//! assert!(result.success);
//! ```
//!
//! ### Combination Analysis
//! ```rust
//! use tailwind_rs_core::enhanced_variants::{VariantAnalyzer, VariantFormatter};
//!
//! let results = vec![/* parsed results */];
//! let stats = VariantAnalyzer::analyze_usage(&results);
//! let suggestions = VariantAnalyzer::find_optimization_opportunities(&results);
//!
//! println!("Usage Report:");
//! println!("{}", VariantFormatter::generate_summary_report(&results));
//! ```
//!
//! ### Complex Combinations
//! ```rust
//! use tailwind_rs_core::enhanced_variants::VariantCombinationBuilder;
//!
//! let combination = VariantCombinationBuilder::new()
//!     .responsive("sm")
//!     .dark_mode()
//!     .state("hover")
//!     .build()?;
//!
//! assert_eq!(combination.variants.len(), 3);
//! assert!(combination.valid);
//! ```
//!
//! ## Variant Types
//!
//! - **State Variants**: `hover`, `focus`, `active`, etc.
//! - **Responsive Variants**: `sm`, `md`, `lg`, `xl`, `2xl`
//! - **Dark Mode**: `dark`
//! - **Focus Variants**: `focus-within`, `focus-visible`
//! - **Motion Variants**: `motion-safe`, `motion-reduce`
//! - **Contrast Variants**: `contrast-more`, `contrast-less`
//! - **Orientation**: `portrait`, `landscape`
//! - **Print/Screen**: `print`, `screen`
//! - **Custom Variants**: User-defined
//!
//! ## Specificity and Ordering
//!
//! Variants are automatically ordered by specificity:
//!
//! 1. Responsive variants (highest specificity)
//! 2. Dark mode variants
//! 3. Focus-within variants
//! 4. State variants
//! 5. Motion variants
//! 6. Contrast variants
//! 7. Orientation variants
//! 8. Print/Screen variants (lowest)
//!
//! ## Conflict Detection
//!
//! The system automatically detects and prevents invalid combinations:
//!
//! - Mutually exclusive variants (`print` + `screen`)
//! - Multiple responsive variants (`sm` + `md`)
//! - Invalid custom variant combinations
//!
//! ## Performance
//!
//! - **Fast Parsing**: Direct string operations, no regex
//! - **Caching**: Built-in result caching with configurable size
//! - **Optimization**: Automatic combination simplification
//! - **Memory Efficient**: Minimal allocations, reuse of structures
//!
//! ## Testing
//!
//! Comprehensive test coverage including:
//!
//! - All standard variant combinations
//! - Custom variant integration
//! - Conflict detection and resolution
//! - Performance regression testing
//! - Edge case handling
//! - Memory usage validation
//! - Complex nested combinations
//!
//! ## Migration Support
//!
//! Utilities for migrating from older variant systems:
//!
//! ```rust
//! use tailwind_rs_core::enhanced_variants::utilities::VariantMigrator;
//!
//! // Migrate old syntax
//! let migrated = VariantMigrator::migrate_class_syntax("mobile:bg-blue-500");
//! assert_eq!(migrated, "sm:bg-blue-500");
//! ```

// Core modules
pub mod combinations;
pub mod definitions;
pub mod parser;
pub mod tests;
pub mod types;
pub mod utilities;

// Re-exports for convenience
pub use parser::EnhancedVariantParser;
pub use types::*;
pub use definitions::VariantDefinitions;
pub use utilities::{VariantAnalyzer, VariantCache, VariantFormatter, VariantOptimizer, VariantMigrator};
pub use combinations::{VariantCombinationBuilder, VariantCombinationProcessor, VariantOrdering, VariantConflictDetector, VariantCombinationStrategies};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn module_exports() {
        // Test that all expected types are available
        let _parser: EnhancedVariantParser = EnhancedVariantParser::new();
        let _definitions: std::collections::HashMap<String, VariantDefinition> = VariantDefinitions::standard_definitions();
        let _cache: VariantCache = VariantCache::new(100);
        let _builder: VariantCombinationBuilder = VariantCombinationBuilder::new();
    }

    #[test]
    fn cross_module_integration() {
        // Test that modules work together correctly
        let mut parser = EnhancedVariantParser::new();

        // Add a custom variant
        parser.add_custom_variant("test".to_string(), ":test".to_string()).unwrap();

        // Parse a class using it
        let result = parser.parse_class("test:bg-blue-500").unwrap();
        assert!(result.success);

        // Use the analyzer
        let results = vec![result];
        let stats = VariantAnalyzer::analyze_usage(&results);
        assert_eq!(stats.total_classes, 1);
        assert_eq!(stats.successful_parses, 1);
    }

    #[test]
    fn builder_pattern_integration() {
        let combination = VariantCombinationBuilder::new()
            .responsive("sm")
            .state("hover")
            .build()
            .unwrap();

        assert_eq!(combination.variants.len(), 2);
        assert!(combination.valid);

        // Test with processor
        let variants = vec![
            ParsedVariant::new("hover".to_string(), VariantType::State),
            ParsedVariant::new("sm".to_string(), VariantType::Responsive),
        ];

        let processed = VariantCombinationProcessor::process_combination(&variants).unwrap();
        assert_eq!(processed.specificity, combination.specificity);
    }

    #[test]
    fn caching_integration() {
        let mut cache = VariantCache::new(10);
        let parser = EnhancedVariantParser::new();

        let result1 = parser.parse_class("hover:bg-blue-500").unwrap();
        cache.insert("hover:bg-blue-500".to_string(), result1.clone());

        let cached = cache.get("hover:bg-blue-500").unwrap();
        assert_eq!(cached.original_class, result1.original_class);

        // Test cache stats
        let (size, max_size) = cache.stats();
        assert_eq!(size, 1);
        assert_eq!(max_size, 10);
    }
}
