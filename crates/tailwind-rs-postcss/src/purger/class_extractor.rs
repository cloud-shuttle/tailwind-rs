//! Class extractor for CSS classes

use std::collections::HashSet;
use regex::Regex;
use super::types::*;

/// Class extractor for CSS classes
pub struct ClassExtractor {
    _class_patterns: Vec<String>,
    css_rule_pattern: Regex,
}

impl ClassExtractor {
    /// Create new class extractor
    pub fn new() -> Self {
        Self {
            _class_patterns: Self::get_default_patterns(),
            css_rule_pattern: Regex::new(r"([^{]+)\s*\{([^}]+)\}").unwrap(),
        }
    }
    
    /// Extract classes from CSS
    pub fn extract_classes(&self, css: &str) -> Result<HashSet<String>, PurgeError> {
        let mut classes = HashSet::new();
        
        // Extract classes from CSS selectors
        for cap in self.css_rule_pattern.captures_iter(css) {
            let selector = cap[1].trim();
            let selector_classes = self.extract_classes_from_selector(selector);
            classes.extend(selector_classes);
        }
        
        Ok(classes)
    }
    
    /// Extract classes from a CSS selector
    fn extract_classes_from_selector(&self, selector: &str) -> HashSet<String> {
        let mut classes = HashSet::new();
        
        // Split selector by combinators
        let parts: Vec<&str> = selector.split(&[' ', '>', '+', '~', ','][..]).collect();
        
        for part in parts {
            let part = part.trim();
            if part.starts_with('.') {
                // Extract class name
                let class_name = part.strip_prefix('.').unwrap_or(part);
                classes.insert(class_name.to_string());
            }
        }
        
        classes
    }
    
    /// Get default class patterns
    fn get_default_patterns() -> Vec<String> {
        vec![
            r"\.([a-zA-Z0-9_-]+)".to_string(),
            "class\\s*=\\s*[\"']([^\"']+)[\"']".to_string(),
        ]
    }
}
