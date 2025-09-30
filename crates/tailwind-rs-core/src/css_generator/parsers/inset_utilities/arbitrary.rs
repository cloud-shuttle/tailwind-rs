//! Arbitrary Inset Utilities Module
//!
//! Handles parsing of arbitrary inset values.

use crate::css_generator::types::CssProperty;
use super::values::InsetSpacingValues;

/// Arbitrary inset utilities parser
#[derive(Debug, Clone)]
pub struct ArbitraryInsetParser {
    values: InsetSpacingValues,
}

impl ArbitraryInsetParser {
    /// Create a new arbitrary inset parser
    pub fn new(values: InsetSpacingValues) -> Self {
        Self { values }
    }

    /// Parse arbitrary inset class
    pub fn parse_arbitrary_inset_class(&self, _class: &str) -> Option<Vec<CssProperty>> {
        // TODO: Implement arbitrary inset parsing
        // This would handle classes like inset-[10px] or inset-[calc(50%-20px)]
        None
    }

    /// Get supported arbitrary inset patterns
    pub fn supported_patterns(&self) -> Vec<&'static str> {
        // TODO: Return actual arbitrary patterns when implemented
        vec![]
    }
}

impl Default for ArbitraryInsetParser {
    fn default() -> Self {
        Self::new(InsetSpacingValues::new())
    }
}
