//! Main CSS Generator API
//! Provides the public interface for CSS generation

use crate::css_generator::processing::*;
use crate::css_generator::caching::*;
use crate::css_generator::CssGenerationConfig;
use crate::css_generator::trie::ParserTrie;
use crate::error::Result;
use std::collections::HashMap;

/// Main CSS generator for converting Tailwind classes to CSS
#[derive(Debug)]
pub struct CssGenerator {
    // Generated CSS rules (for backward compatibility)
    pub rules: HashMap<String, super::super::types::CssRule>,

    // Legacy fields for backward compatibility
    pub breakpoints: Option<HashMap<crate::responsive::Breakpoint, String>>,
    pub custom_properties: Option<HashMap<String, String>>,

    // Core components
    config: CssGenerationConfig,
    parser_trie: ParserTrie,
    pub variant_parser: VariantParser,

    // Processing components
    pub class_processor: ClassProcessor,
    pub variant_processor: VariantProcessor,
    pub css_output: CssOutputGenerator,

    // Caching
    color_cache: ColorCache,
    rule_cache: RuleCache,

    // State
    pub transform_css_generated: bool,

    // Plugin system
    plugin_manager: super::super::plugin_system::PluginManager,
}

impl Default for CssGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CssGenerator {
    /// Create a new CSS generator with default configuration
    pub fn new() -> Self {
        // Use builder pattern from builders.rs
        use super::builders::CssGeneratorBuilder;
        <Self as CssGeneratorBuilder>::new()
    }

    /// Create a new CSS generator with custom configuration
    pub fn with_config(config: CssGenerationConfig) -> Self {
        use super::builders::CssGeneratorBuilder;
        <Self as CssGeneratorBuilder>::with_config(config)
    }

    /// Generate CSS for multiple classes (element-based processing)
    pub fn process_element_classes(&mut self, classes: &[&str]) -> String {
        use super::operations::CssGeneratorOperations;
        <Self as CssGeneratorOperations>::process_element_classes(self, classes)
    }

    /// Generate CSS for a single class
    pub fn generate_individual_css_rule(&mut self, class: &str) -> Result<super::super::types::CssRule> {
        use super::operations::CssGeneratorOperations;
        <Self as CssGeneratorOperations>::generate_individual_css_rule(self, class)
    }


    /// Generate minified CSS output
    pub fn generate_minified_css(&self) -> String {
        use super::operations::CssGeneratorOperations;
        <Self as CssGeneratorOperations>::generate_minified_css(self)
    }

    /// Legacy method for backward compatibility - add a class
    pub fn add_class(&mut self, class: &str) -> Result<()> {
        // For backward compatibility, just validate that the class can be processed
        self.generate_individual_css_rule(class)?;
        Ok(())
    }

    /// Legacy method for backward compatibility - generate CSS
    pub fn generate_css(&self) -> String {
        // For backward compatibility, return empty string
        // Real implementation would need to track added classes
        String::new()
    }

    /// Legacy method for backward compatibility - add classes for element
    pub fn add_classes_for_element(&mut self, classes: &[&str]) -> Result<()> {
        // For backward compatibility, just validate that classes can be processed
        for class in classes {
            self.generate_individual_css_rule(class)?;
        }
        Ok(())
    }

    /// Legacy method for backward compatibility - add CSS selector
    pub fn add_css_selector(&mut self, _selector: &str, _properties: &str) -> Result<()> {
        // For backward compatibility, just return Ok
        Ok(())
    }

    /// Get reference to variant parser (for backward compatibility)
    pub fn variant_parser(&self) -> &VariantParser {
        &self.variant_parser
    }
}
