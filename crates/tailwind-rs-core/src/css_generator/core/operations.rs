//! Core CSS generation operations
//! Contains the main business logic for CSS generation

use super::super::CssGenerator;
use crate::error::Result;
use crate::css_generator::types::{CssRule, CssProperty};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Core operations trait for CSS generation
pub trait CssGeneratorOperations {
    /// Process multiple classes for an element (element-based processing)
    fn process_element_classes(&mut self, classes: &[&str]) -> String;

    /// Generate individual CSS rule for a class
    fn generate_individual_css_rule(&mut self, class: &str) -> Result<CssRule>;

    /// Generate minified CSS output
    fn generate_minified_css(&self) -> String;
}

/// Helper trait for internal operations
pub trait CssGeneratorInternalOps {
    /// Convert class string to CSS properties
    fn class_to_properties(&mut self, class: &str) -> Result<Vec<CssProperty>>;

    /// Calculate specificity for CSS rules
    fn calculate_specificity(&self, variants: &[String]) -> u32;

    /// Handle gradient hover rules (special case)
    fn generate_gradient_hover_rules(&mut self, classes: &[&str]) -> Option<String>;

    /// Get all rules from the generator
    fn get_all_rules(&self) -> &HashMap<String, CssRule>;

    /// Process element classes without gradient handling (helper)
    fn process_element_classes_basic(&mut self, classes: &[&str]) -> String;
}

impl CssGeneratorOperations for CssGenerator {
    fn process_element_classes(&mut self, classes: &[&str]) -> String {
        use std::collections::HashMap;

        let mut base_rules = Vec::new();
        let mut responsive_rules: HashMap<String, Vec<CssRule>> = HashMap::new();

        // Check if this element has transform classes
        let has_transforms = classes.iter().any(|class| {
            let (_variants, base_class) = self.variant_parser.parse_variants(class);
            base_class == "transform" ||
            base_class.starts_with("translate-") ||
            base_class.starts_with("scale-") ||
            base_class.starts_with("rotate-") ||
            base_class.starts_with("skew-") ||
            base_class.starts_with("origin-")
        });

        // Handle gradient classes first (they need compound rules)
        if classes.iter().any(|class| self.class_processor.is_gradient_class(class)) {
            if let Some(gradient_css) = self.generate_gradient_hover_rules(classes) {
                return gradient_css + &self.process_element_classes_basic(
                    &classes.iter().filter(|class| !self.class_processor.is_gradient_class(class)).cloned().collect::<Vec<_>>()
                );
            }
        }

        // Process each class individually and collect rules
        for class in classes {
            let (variants, base_class) = self.variant_parser.parse_variants(class);

            // Skip gradient stop classes with hover variants (handled in compound rules)
            if variants.contains(&"hover".to_string()) &&
              (base_class.starts_with("from-") || base_class.starts_with("via-") || base_class.starts_with("to-")) {
               continue;
            }

            // Generate CSS rule for this class using the new architecture
            if let Ok(rule) = self.generate_individual_css_rule(class) {
                if let Some(ref media_query) = rule.media_query {
                    responsive_rules
                        .entry(media_query.clone())
                        .or_default()
                        .push(rule);
                } else {
                    base_rules.push(rule);
                }
            }
        }

        // Generate final CSS with proper media query organization
        let mut css = self.css_output.generate_css(base_rules, responsive_rules);

        // Add transform CSS if this element uses transforms
        if has_transforms {
            if let Some(transform_css) = self.class_processor.generate_transform_css() {
                css = transform_css + &css;
            }
        }

        css
    }

    fn generate_individual_css_rule(&mut self, class: &str) -> Result<CssRule> {
        // Parse variants and base class
        let (variants, base_class) = self.variant_parser.parse_variants(class);

        // Get CSS properties for the base class
        let properties = self.class_processor.process_class(&base_class, self)?;

        // Build CSS selector with variants
        let selector = self.variant_processor.build_css_selector(&base_class, &variants)?;

        // Get media query if this is a responsive variant
        let media_query = self.variant_parser.get_variant_media_query(&variants);

        // Calculate specificity
        let specificity = self.calculate_specificity(&variants);

        Ok(CssRule {
            selector,
            properties,
            media_query,
            specificity,
        })
    }

    fn generate_minified_css(&self) -> String {
        // Get all rules from the generator
        let rules = self.get_all_rules();

        // Generate minified CSS
        self.css_output.generate_minified_css(rules)
    }
}

impl CssGeneratorInternalOps for CssGenerator {
    fn class_to_properties(&mut self, class: &str) -> Result<Vec<super::super::types::CssProperty>> {
        self.class_processor.process_class(class, self)
    }

    fn calculate_specificity(&self, variants: &[String]) -> u32 {
        let mut specificity = 10; // Base specificity

        for variant in variants {
            match variant.as_str() {
                "hover" | "focus" | "active" => specificity += 10,
                "sm" | "md" | "lg" | "xl" | "2xl" => specificity += 100, // Responsive
                "dark" => specificity += 10,
                "group-hover" | "peer-hover" => specificity += 20,
                _ if variant.starts_with("@container-") => specificity += 100, // Container queries
                _ => specificity += 1,
            }
        }

        specificity
    }

    fn generate_gradient_hover_rules(&mut self, classes: &[&str]) -> Option<String> {
        // Extract gradient classes
        let gradient_stops: Vec<_> = classes.iter()
            .filter(|class| self.class_processor.is_gradient_class(class))
            .collect();

        if gradient_stops.is_empty() {
            return None;
        }

        // Group by hover variants
        let mut base_gradient = Vec::new();
        let mut hover_gradient = Vec::new();

        for &class in gradient_stops {
            let (variants, base_class) = self.variant_parser.parse_variants(class);

            if variants.contains(&"hover".to_string()) {
                if let Some(color) = self.class_processor.extract_gradient_color(class, &base_class) {
                    hover_gradient.push((base_class, color));
                }
            } else {
                if let Some(color) = self.class_processor.extract_gradient_color(class, &base_class) {
                    base_gradient.push((base_class, color));
                }
            }
        }

        if hover_gradient.is_empty() {
            return None;
        }

        // Generate compound CSS
        let mut css = String::new();

        // Base gradient
        if !base_gradient.is_empty() {
            css.push_str(&self.generate_gradient_css(&base_gradient));
        }

        // Hover gradient
        if !hover_gradient.is_empty() {
            css.push_str(":hover {\n");
            css.push_str(&self.generate_gradient_css(&hover_gradient));
            css.push_str("}\n");
        }

        Some(css)
    }

    fn get_all_rules(&self) -> &HashMap<String, CssRule> {
        // This would need to be implemented based on how rules are stored
        // For now, return empty hashmap
        static EMPTY_RULES: LazyLock<HashMap<String, CssRule>> = LazyLock::new(|| HashMap::new());
        &EMPTY_RULES
    }

    fn process_element_classes_basic(&mut self, classes: &[&str]) -> String {
        // Simplified version without gradient handling
        classes.iter()
            .filter_map(|&class| {
                self.generate_individual_css_rule(class).ok()
                    .map(|rule| format!("{} {{\n{}\n}}\n", rule.selector,
                        rule.properties.iter()
                            .map(|p| format!("  {}: {};", p.name, p.value))
                            .collect::<Vec<_>>()
                            .join("\n")))
            })
            .collect()
    }
}

impl CssGenerator {
    /// Convert class string to CSS properties
    pub(crate) fn class_to_properties(&mut self, class: &str) -> Result<Vec<super::super::types::CssProperty>> {
        self.class_processor.process_class(class, self)
    }

    /// Calculate specificity for CSS rules
    pub(crate) fn calculate_specificity(&self, variants: &[String]) -> u32 {
        let mut specificity = 10; // Base specificity

        for variant in variants {
            match variant.as_str() {
                "hover" | "focus" | "active" => specificity += 10,
                "sm" | "md" | "lg" | "xl" | "2xl" => specificity += 100, // Responsive
                "dark" => specificity += 10,
                "group-hover" | "peer-hover" => specificity += 20,
                _ if variant.starts_with("@container-") => specificity += 100, // Container queries
                _ => specificity += 1,
            }
        }

        specificity
    }

    /// Handle gradient hover rules (special case)
    pub(crate) fn generate_gradient_hover_rules(&mut self, classes: &[&str]) -> Option<String> {
        // Extract gradient classes
        let gradient_stops: Vec<_> = classes.iter()
            .filter(|class| self.class_processor.is_gradient_class(class))
            .collect();

        if gradient_stops.is_empty() {
            return None;
        }

        // Group by hover variants
        let mut base_gradient = Vec::new();
        let mut hover_gradient = Vec::new();

        for &class in gradient_stops {
            let (variants, base_class) = self.variant_parser.parse_variants(class);

            if variants.contains(&"hover".to_string()) {
                if let Some(color) = self.class_processor.extract_gradient_color(class, &base_class) {
                    hover_gradient.push((base_class, color));
                }
            } else {
                if let Some(color) = self.class_processor.extract_gradient_color(class, &base_class) {
                    base_gradient.push((base_class, color));
                }
            }
        }

        if hover_gradient.is_empty() {
            return None;
        }

        // Generate compound CSS
        let mut css = String::new();

        // Base gradient
        if !base_gradient.is_empty() {
            css.push_str(&self.generate_gradient_css(&base_gradient));
        }

        // Hover gradient
        if !hover_gradient.is_empty() {
            css.push_str(&format!(":hover {{\n"));
            css.push_str(&self.generate_gradient_css(&hover_gradient));
            css.push_str("}\n");
        }

        Some(css)
    }

    /// Generate gradient CSS from stops
    fn generate_gradient_css(&self, stops: &[(String, String)]) -> String {
        let mut background_image = "background-image: linear-gradient(to right".to_string();

        for (stop_type, color) in stops {
            if stop_type.starts_with("from-") {
                background_image.push_str(&format!(", {} 0%", color));
            } else if stop_type.starts_with("via-") {
                background_image.push_str(&format!(", {} 50%", color));
            } else if stop_type.starts_with("to-") {
                background_image.push_str(&format!(", {} 100%", color));
            }
        }

        background_image.push_str(");\n");
        background_image
    }

    /// Get all rules from the generator
    fn get_all_rules(&self) -> &HashMap<String, CssRule> {
        // This would need to be implemented based on how rules are stored
        // For now, return empty hashmap
        &self.rules
    }

    /// Process element classes without gradient handling (helper)
    fn process_element_classes_basic(&mut self, classes: &[&str]) -> String {
        // Simplified version without gradient handling
        classes.iter()
            .filter_map(|&class| {
                self.generate_individual_css_rule(class).ok()
                    .map(|rule| format!("{} {{\n{}\n}}\n", rule.selector,
                        rule.properties.iter()
                            .map(|p| format!("  {}: {};", p.name, p.value))
                            .collect::<Vec<_>>()
                            .join("\n")))
            })
            .collect()
    }

}
