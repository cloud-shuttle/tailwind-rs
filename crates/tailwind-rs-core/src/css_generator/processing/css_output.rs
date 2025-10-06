//! Final CSS string generation
//! Handles formatting, minification, and output

use crate::css_generator::types::CssRule;
use std::collections::HashMap;

/// CSS output generator for final formatting
#[derive(Debug)]
pub struct CssOutputGenerator {
    minify: bool,
    include_sourcemaps: bool,
}

impl CssOutputGenerator {
    /// Create a new CSS output generator
    pub fn new() -> Self {
        Self {
            minify: false,
            include_sourcemaps: false,
        }
    }

    /// Generate final CSS string from rules
    pub fn generate_css(&self, base_rules: Vec<CssRule>, responsive_rules: HashMap<String, Vec<CssRule>>) -> String {
        let mut output = String::new();

        // Add base rules
        for rule in base_rules {
            output.push_str(&self.format_css_rule(&rule));
            output.push('\n');
        }

        // Add responsive rules
        let mut sorted_media_queries: Vec<_> = responsive_rules.keys().collect();
        sorted_media_queries.sort_by_key(|mq| self.extract_min_width(mq));

        for media_query in sorted_media_queries {
            if let Some(rules) = responsive_rules.get(media_query) {
                output.push_str(&format!("{} {{\n", media_query));

                for rule in rules {
                    let indented_rule = self.format_css_rule(rule)
                        .lines()
                        .map(|line| format!("  {}", line))
                        .collect::<Vec<_>>()
                        .join("\n");
                    output.push_str(&indented_rule);
                    output.push('\n');
                }

                output.push_str("}\n\n");
            }
        }

        output
    }

    /// Generate minified CSS
    pub fn generate_minified_css(&self, rules: &HashMap<String, CssRule>) -> String {
        let mut output = String::new();

        for rule in rules.values() {
            output.push_str(&self.format_css_rule(rule).replace("\n", "").replace("  ", ""));
            output.push_str(" ");
        }

        output.trim().to_string()
    }

    /// Format single CSS rule
    pub fn format_css_rule(&self, rule: &CssRule) -> String {
        let mut output = format!("{} {{\n", rule.selector);

        for property in &rule.properties {
            output.push_str(&format!("  {}: {};\n", property.name, property.value));
        }

        output.push('}');
        output
    }

    /// Extract min-width value from media query for sorting
    fn extract_min_width(&self, media_query: &str) -> u32 {
        if let Some(width_str) = media_query
            .strip_prefix("@media (min-width: ")
            .and_then(|s| s.strip_suffix("px)"))
        {
            width_str.parse().unwrap_or(0)
        } else {
            0
        }
    }
}
