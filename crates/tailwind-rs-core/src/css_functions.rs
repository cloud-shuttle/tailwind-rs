//! CSS Functions Module
//!
//! Handles CSS directives like @apply, @layer, and @import

use crate::error::Result;
use std::collections::HashMap;

/// CSS Functions processor for handling directives
#[derive(Debug)]
pub struct CssFunctionsProcessor {
    layers: HashMap<String, Vec<String>>,
    imports: Vec<String>,
}

impl CssFunctionsProcessor {
    /// Create a new CSS functions processor
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
            imports: Vec::new(),
        }
    }

    /// Process @apply directive
    pub fn process_apply(&mut self, apply_value: &str, available_classes: &HashMap<String, String>) -> Result<String> {
        let classes: Vec<&str> = apply_value.split_whitespace().collect();
        let mut applied_css = String::new();

        for class in classes {
            if let Some(css) = available_classes.get(class) {
                applied_css.push_str(css);
                applied_css.push('\n');
            } else {
                return Err(crate::error::TailwindError::Validation { message: format!("Unknown class '{}' in @apply directive", class) });
            }
        }

        Ok(applied_css)
    }

    /// Process @layer directive
    pub fn process_layer(&mut self, layer_name: &str, css_content: &str) -> Result<String> {
        let layer_css = self.layers.entry(layer_name.to_string()).or_insert_with(Vec::new);
        layer_css.push(css_content.to_string());

        Ok(format!("/* @layer {} */\n{}", layer_name, css_content))
    }

    /// Process @import directive
    pub fn process_import(&mut self, import_value: &str) -> Result<String> {
        // Basic validation - in a real implementation, this would handle URL validation
        if import_value.trim().is_empty() {
            return Err(crate::error::TailwindError::Validation { message: "Empty import value".to_string() });
        }

        self.imports.push(import_value.to_string());
        Ok(format!("@import {};\n", import_value))
    }

    /// Generate final CSS with proper layer ordering
    pub fn generate_final_css(&self) -> String {
        let mut result = String::new();

        // Add imports first
        for import in &self.imports {
            result.push_str(&format!("@import {};\n", import));
        }

        // Add layers in correct order: base, components, utilities
        let layer_order = ["base", "components", "utilities"];

        for layer_name in &layer_order {
            if let Some(layer_css) = self.layers.get(*layer_name) {
                result.push_str(&format!("\n@layer {} {{\n", layer_name));
                for css in layer_css {
                    result.push_str(css);
                }
                result.push_str("}\n");
            }
        }

        // Add any custom layers
        for (layer_name, layer_css) in &self.layers {
            if !layer_order.contains(&layer_name.as_str()) {
                result.push_str(&format!("\n@layer {} {{\n", layer_name));
                for css in layer_css {
                    result.push_str(css);
                }
                result.push_str("}\n");
            }
        }

        result
    }

    /// Get available classes for @apply (would be populated from CSS generation)
    pub fn get_available_classes(&self) -> HashMap<String, String> {
        // This would be populated with actual generated CSS classes
        // For now, return empty - would be filled by the CSS generator
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_import() {
        let mut processor = CssFunctionsProcessor::new();
        let result = processor.process_import("url('https://fonts.googleapis.com/css2?family=Inter:wght@400;700&display=swap')");
        assert!(result.is_ok());
        assert_eq!(processor.imports.len(), 1);
    }

    #[test]
    fn test_process_layer() {
        let mut processor = CssFunctionsProcessor::new();
        let result = processor.process_layer("components", ".btn { color: blue; }");
        assert!(result.is_ok());
        assert_eq!(processor.layers.len(), 1);
    }

    #[test]
    fn test_process_apply_unknown_class() {
        let mut processor = CssFunctionsProcessor::new();
        let available_classes = HashMap::new();
        let result = processor.process_apply("unknown-class", &available_classes);
        assert!(result.is_err());
    }
}
