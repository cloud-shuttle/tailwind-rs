//! Variant Parsing and Handling
//! 
//! This module handles the parsing and processing of CSS variants,
//! including responsive, state, and custom variants.

use crate::responsive::Breakpoint;

/// Variant parser for handling CSS variants
#[derive(Debug, Clone)]
pub struct VariantParser {
    /// Supported variants
    variants: Vec<String>,
    /// Responsive breakpoints
    breakpoints: Vec<Breakpoint>,
}

impl VariantParser {
    /// Create a new variant parser
    pub fn new() -> Self {
        Self {
            variants: vec![
                "dark".to_string(),
                "hover".to_string(),
                "focus".to_string(),
                "active".to_string(),
                "visited".to_string(),
                "disabled".to_string(),
                "group-hover".to_string(),
                "group-focus".to_string(),
                "group-active".to_string(),
                "group-disabled".to_string(),
                "peer-hover".to_string(),
                "peer-focus".to_string(),
                "peer-active".to_string(),
                "peer-disabled".to_string(),
                "first".to_string(),
                "last".to_string(),
                "odd".to_string(),
                "even".to_string(),
                "sm".to_string(),
                "md".to_string(),
                "lg".to_string(),
                "xl".to_string(),
                "2xl".to_string(),
            ],
            breakpoints: vec![
                Breakpoint::Sm,
                Breakpoint::Md,
                Breakpoint::Lg,
                Breakpoint::Xl,
                Breakpoint::Xl2,
            ],
        }
    }

    /// Parse variants from a class string
    pub fn parse_variants(&self, class: &str) -> (Vec<String>, String) {
        let mut variants = Vec::new();
        let mut remaining = class.to_string();
        
        // Parse variants in order of specificity (most specific first)
        let variant_patterns = [
            ("dark:", "dark"),
            ("hover:", "hover"),
            ("focus:", "focus"),
            ("active:", "active"),
            ("visited:", "visited"),
            ("disabled:", "disabled"),
            ("group-hover:", "group-hover"),
            ("group-focus:", "group-focus"),
            ("group-active:", "group-active"),
            ("group-disabled:", "group-disabled"),
            ("peer-hover:", "peer-hover"),
            ("peer-focus:", "peer-focus"),
            ("peer-active:", "peer-active"),
            ("peer-disabled:", "peer-disabled"),
            ("first:", "first"),
            ("last:", "last"),
            ("odd:", "odd"),
            ("even:", "even"),
            // Device variants
            ("pointer-coarse:", "pointer-coarse"),
            ("pointer-fine:", "pointer-fine"),
            ("motion-reduce:", "motion-reduce"),
            ("motion-safe:", "motion-safe"),
            ("light:", "light"),
            // Responsive variants
            ("sm:", "sm"),
            ("md:", "md"),
            ("lg:", "lg"),
            ("xl:", "xl"),
            ("2xl:", "2xl"),
        ];
        
        for (prefix, variant) in variant_patterns {
            if remaining.starts_with(prefix) {
                variants.push(variant.to_string());
                remaining = remaining.strip_prefix(prefix).unwrap_or(&remaining).to_string();
                break; // Only parse one variant at a time for now
            }
        }
        
        (variants, remaining)
    }

    /// Get the CSS selector for a variant
    pub fn get_variant_selector(&self, variant: &str) -> String {
        match variant {
            "dark" => ".dark ".to_string(),
            "hover" => ":hover".to_string(),
            "focus" => ":focus".to_string(),
            "active" => ":active".to_string(),
            "visited" => ":visited".to_string(),
            "disabled" => ":disabled".to_string(),
            "group-hover" => ".group:hover ".to_string(),
            "group-focus" => ".group:focus ".to_string(),
            "group-active" => ".group:active ".to_string(),
            "group-disabled" => ".group:disabled ".to_string(),
            "peer-hover" => ".peer:hover ".to_string(),
            "peer-focus" => ".peer:focus ".to_string(),
            "peer-active" => ".peer:active ".to_string(),
            "peer-disabled" => ".peer:disabled ".to_string(),
            "first" => ":first-child".to_string(),
            "last" => ":last-child".to_string(),
            "odd" => ":nth-child(odd)".to_string(),
            "even" => ":nth-child(even)".to_string(),
            // Device variants use media queries, not selectors
            "pointer-coarse" | "pointer-fine" | "motion-reduce" | "motion-safe" | "light" => String::new(),
            _ => String::new(),
        }
    }

    /// Get the media query for device variants
    pub fn get_device_media_query(&self, variant: &str) -> Option<String> {
        match variant {
            "pointer-coarse" => Some("(pointer: coarse)".to_string()),
            "pointer-fine" => Some("(pointer: fine)".to_string()),
            "motion-reduce" => Some("(prefers-reduced-motion: reduce)".to_string()),
            "motion-safe" => Some("(prefers-reduced-motion: no-preference)".to_string()),
            "light" => Some("(prefers-color-scheme: light)".to_string()),
            _ => None,
        }
    }

    /// Get the media query for a responsive variant
    pub fn get_responsive_media_query(&self, variant: &str) -> Option<String> {
        match variant {
            "sm" => Some("(min-width: 640px)".to_string()),
            "md" => Some("(min-width: 768px)".to_string()),
            "lg" => Some("(min-width: 1024px)".to_string()),
            "xl" => Some("(min-width: 1280px)".to_string()),
            "2xl" => Some("(min-width: 1536px)".to_string()),
            _ => None,
        }
    }

    /// Check if a variant is supported
    pub fn is_supported_variant(&self, variant: &str) -> bool {
        self.variants.contains(&variant.to_string())
    }

    /// Get all supported variants
    pub fn get_supported_variants(&self) -> &[String] {
        &self.variants
    }

    /// Get all supported breakpoints
    pub fn get_supported_breakpoints(&self) -> &[Breakpoint] {
        &self.breakpoints
    }
}

impl Default for VariantParser {
    fn default() -> Self {
        Self::new()
    }
}
