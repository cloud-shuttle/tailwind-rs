//! CSS generation system for tailwind-rs
//!
//! This module provides the core CSS generation functionality, converting
//! Tailwind class names into actual CSS rules.

use crate::error::{Result, TailwindError};
use crate::responsive::Breakpoint;
use std::collections::HashMap;

/// Represents a CSS rule with selector and properties
#[derive(Debug, Clone, PartialEq)]
pub struct CssRule {
    /// CSS selector (e.g., ".p-4", ".md:bg-blue-500")
    pub selector: String,
    /// CSS properties for this rule
    pub properties: Vec<CssProperty>,
    /// Media query for responsive rules
    pub media_query: Option<String>,
    /// CSS specificity score
    pub specificity: u32,
}

/// Represents a CSS property
#[derive(Debug, Clone, PartialEq)]
pub struct CssProperty {
    /// Property name (e.g., "padding", "background-color")
    pub name: String,
    /// Property value (e.g., "1rem", "#3b82f6")
    pub value: String,
    /// Whether the property is marked as !important
    pub important: bool,
}

/// CSS generator that converts Tailwind classes to CSS rules
#[derive(Debug, Clone)]
pub struct CssGenerator {
    /// Generated CSS rules
    rules: HashMap<String, CssRule>,
    /// Responsive breakpoints
    breakpoints: HashMap<Breakpoint, String>,
    /// Custom CSS properties
    custom_properties: HashMap<String, String>,
}

impl CssGenerator {
    /// Create a new CSS generator
    pub fn new() -> Self {
        let mut generator = Self {
            rules: HashMap::new(),
            breakpoints: HashMap::new(),
            custom_properties: HashMap::new(),
        };
        
        // Initialize default breakpoints
        generator.breakpoints.insert(Breakpoint::Sm, "(min-width: 640px)".to_string());
        generator.breakpoints.insert(Breakpoint::Md, "(min-width: 768px)".to_string());
        generator.breakpoints.insert(Breakpoint::Lg, "(min-width: 1024px)".to_string());
        generator.breakpoints.insert(Breakpoint::Xl, "(min-width: 1280px)".to_string());
        generator.breakpoints.insert(Breakpoint::Xl2, "(min-width: 1536px)".to_string());
        
        generator
    }

    /// Add a class to the generator
    pub fn add_class(&mut self, class: &str) -> Result<()> {
        let rule = self.class_to_css_rule(class)?;
        self.rules.insert(class.to_string(), rule);
        Ok(())
    }

    /// Add a responsive class
    pub fn add_responsive_class(&mut self, breakpoint: Breakpoint, class: &str) -> Result<()> {
        let mut rule = self.class_to_css_rule(class)?;
        rule.selector = format!("{}{}", breakpoint.prefix(), class);
        rule.media_query = self.breakpoints.get(&breakpoint).cloned();
        rule.specificity = 20; // Higher specificity for responsive rules
        
        let responsive_class = format!("{}:{}", breakpoint.prefix().trim_end_matches(':'), class);
        self.rules.insert(responsive_class, rule);
        Ok(())
    }

    /// Add a custom CSS property
    pub fn add_custom_property(&mut self, name: &str, value: &str) {
        self.custom_properties.insert(name.to_string(), value.to_string());
    }

    /// Generate CSS from all added classes
    pub fn generate_css(&self) -> String {
        let mut css = String::new();
        
        // Add custom properties
        if !self.custom_properties.is_empty() {
            css.push_str(":root {\n");
            for (name, value) in &self.custom_properties {
                css.push_str(&format!("  --{}: {};\n", name, value));
            }
            css.push_str("}\n\n");
        }
        
        // Group rules by media query
        let mut base_rules = Vec::new();
        let mut responsive_rules: HashMap<String, Vec<&CssRule>> = HashMap::new();
        
        for rule in self.rules.values() {
            if let Some(ref media_query) = rule.media_query {
                responsive_rules.entry(media_query.clone()).or_default().push(rule);
            } else {
                base_rules.push(rule);
            }
        }
        
        // Generate base rules
        for rule in base_rules {
            css.push_str(&self.rule_to_css(rule));
        }
        
        // Generate responsive rules
        for (media_query, rules) in responsive_rules {
            css.push_str(&format!("@media {} {{\n", media_query));
            for rule in rules {
                css.push_str(&format!("  {}\n", self.rule_to_css(rule)));
            }
            css.push_str("}\n\n");
        }
        
        css
    }

    /// Generate minified CSS
    pub fn generate_minified_css(&self) -> String {
        let css = self.generate_css();
        self.minify_css(&css)
    }

    /// Get all generated rules
    pub fn get_rules(&self) -> &HashMap<String, CssRule> {
        &self.rules
    }

    /// Get the number of generated rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Remove a CSS rule by selector
    pub fn remove_rule(&mut self, selector: &str) -> Option<CssRule> {
        self.rules.remove(selector)
    }

    /// Update a CSS rule
    pub fn update_rule(&mut self, selector: &str, rule: CssRule) {
        self.rules.insert(selector.to_string(), rule);
    }

    /// Parse variants from a class name and return (variants, base_class)
    fn parse_variants(&self, class: &str) -> (Vec<String>, String) {
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

    /// Convert a class name to a CSS rule
    fn class_to_css_rule(&self, class: &str) -> Result<CssRule> {
        let (variants, base_class) = self.parse_variants(class);
        let properties = self.class_to_properties(class)?;
        
        // Build selector with variants
        let mut selector = String::new();
        for variant in &variants {
            match variant.as_str() {
                "dark" => selector.push_str(".dark "),
                "hover" => selector.push_str(":hover"),
                "focus" => selector.push_str(":focus"),
                "active" => selector.push_str(":active"),
                "visited" => selector.push_str(":visited"),
                "disabled" => selector.push_str(":disabled"),
                "group-hover" => selector.push_str(".group:hover "),
                "group-focus" => selector.push_str(".group:focus "),
                "group-active" => selector.push_str(".group:active "),
                "group-disabled" => selector.push_str(".group:disabled "),
                "peer-hover" => selector.push_str(".peer:hover "),
                "peer-focus" => selector.push_str(".peer:focus "),
                "peer-active" => selector.push_str(".peer:active "),
                "peer-disabled" => selector.push_str(".peer:disabled "),
                "first" => selector.push_str(":first-child"),
                "last" => selector.push_str(":last-child"),
                "odd" => selector.push_str(":nth-child(odd)"),
                "even" => selector.push_str(":nth-child(even)"),
                _ => {} // Responsive variants handled separately
            }
        }
        
        // Add the base class
        selector.push_str(&format!(".{}", base_class));
        
        // Determine media query for responsive variants
        let media_query = variants.iter()
            .find(|v| matches!(v.as_str(), "sm" | "md" | "lg" | "xl" | "2xl"))
            .and_then(|variant| {
                match variant.as_str() {
                    "sm" => Some("(min-width: 640px)"),
                    "md" => Some("(min-width: 768px)"),
                    "lg" => Some("(min-width: 1024px)"),
                    "xl" => Some("(min-width: 1280px)"),
                    "2xl" => Some("(min-width: 1536px)"),
                    _ => None,
                }
            })
            .map(|s| s.to_string());
        
        // Determine specificity based on variants
        let specificity = 10 + (variants.len() as u32 * 10);
        
        Ok(CssRule {
            selector,
            properties,
            media_query,
            specificity,
        })
    }

    /// Convert a class name to CSS properties
    fn class_to_properties(&self, class: &str) -> Result<Vec<CssProperty>> {
        // First, parse variants and get the base class
        let (_variants, base_class) = self.parse_variants(class);
        
        // Try to parse the base class using comprehensive patterns
        if let Some(properties) = self.parse_spacing_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_color_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_typography_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_layout_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_flexbox_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_grid_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_border_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_effects_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_transform_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_animation_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_interactivity_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_sizing_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_background_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_filter_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_transition_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_text_shadow_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_mask_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_logical_properties_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_enhanced_backdrop_filter_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_modern_css_features_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_device_variant_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_css_nesting_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_advanced_plugin_system_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_enhanced_validation_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_advanced_performance_optimization_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_container_query_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_color_function_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_performance_optimization_class(&base_class) {
            return Ok(properties);
        }
        
        if let Some(properties) = self.parse_advanced_animation_class(&base_class) {
            return Ok(properties);
        }
        
        // Fallback to hardcoded classes for backwards compatibility
        match base_class.as_str() {
            // Display utilities
            "block" => Ok(vec![CssProperty { name: "display".to_string(), value: "block".to_string(), important: false }]),
            "inline" => Ok(vec![CssProperty { name: "display".to_string(), value: "inline".to_string(), important: false }]),
            "flex" => Ok(vec![CssProperty { name: "display".to_string(), value: "flex".to_string(), important: false }]),
            "grid" => Ok(vec![CssProperty { name: "display".to_string(), value: "grid".to_string(), important: false }]),
            "hidden" => Ok(vec![CssProperty { name: "display".to_string(), value: "none".to_string(), important: false }]),
            
            _ => Err(TailwindError::class_generation(format!("Unknown class: {}", class))),
        }
    }
    
    /// Parse spacing classes (padding, margin, etc.)
    fn parse_spacing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let spacing_map = [
            ("0", "0px"), ("px", "1px"), ("0.5", "0.125rem"), ("1", "0.25rem"),
            ("1.5", "0.375rem"), ("2", "0.5rem"), ("2.5", "0.625rem"), ("3", "0.75rem"),
            ("3.5", "0.875rem"), ("4", "1rem"), ("5", "1.25rem"), ("6", "1.5rem"),
            ("7", "1.75rem"), ("8", "2rem"), ("9", "2.25rem"), ("10", "2.5rem"),
            ("11", "2.75rem"), ("12", "3rem"), ("14", "3.5rem"), ("16", "4rem"),
            ("20", "5rem"), ("24", "6rem"), ("28", "7rem"), ("32", "8rem"),
            ("36", "9rem"), ("40", "10rem"), ("44", "11rem"), ("48", "12rem"),
            ("52", "13rem"), ("56", "14rem"), ("60", "15rem"), ("64", "16rem"),
            ("72", "18rem"), ("80", "20rem"), ("96", "24rem"),
        ];
        
        // Parse padding classes
        if class.starts_with("p-") {
            let value = &class[2..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![CssProperty { 
                    name: "padding".to_string(), 
                    value: css_value.to_string(), 
                    important: false 
                }]);
            }
        }
        
        // Parse margin classes
        if class.starts_with("m-") {
            let value = &class[2..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![CssProperty { 
                    name: "margin".to_string(), 
                    value: css_value.to_string(), 
                    important: false 
                }]);
            }
        }
        
        // Parse padding top/bottom/left/right
        if class.starts_with("pt-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![CssProperty { 
                    name: "padding-top".to_string(), 
                    value: css_value.to_string(), 
                    important: false 
                }]);
            }
        }
        
        if class.starts_with("pb-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![CssProperty { 
                    name: "padding-bottom".to_string(), 
                    value: css_value.to_string(), 
                    important: false 
                }]);
            }
        }
        
        if class.starts_with("pl-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![CssProperty { 
                    name: "padding-left".to_string(), 
                    value: css_value.to_string(), 
                    important: false 
                }]);
            }
        }
        
        if class.starts_with("pr-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![CssProperty { 
                    name: "padding-right".to_string(), 
                    value: css_value.to_string(), 
                    important: false 
                }]);
            }
        }
        
        // Parse padding x/y (horizontal/vertical)
        if class.starts_with("px-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![
                    CssProperty { 
                        name: "padding-left".to_string(), 
                        value: css_value.to_string(), 
                        important: false 
                    },
                    CssProperty { 
                        name: "padding-right".to_string(), 
                        value: css_value.to_string(), 
                        important: false 
                    }
                ]);
            }
        }
        
        if class.starts_with("py-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![
                    CssProperty { 
                        name: "padding-top".to_string(), 
                        value: css_value.to_string(), 
                        important: false 
                    },
                    CssProperty { 
                        name: "padding-bottom".to_string(), 
                        value: css_value.to_string(), 
                        important: false 
                    }
                ]);
            }
        }
        
        // Parse margin top/bottom/left/right
        if class.starts_with("mt-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![CssProperty { 
                    name: "margin-top".to_string(), 
                    value: css_value.to_string(), 
                    important: false 
                }]);
            }
        }
        
        if class.starts_with("mb-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![CssProperty { 
                    name: "margin-bottom".to_string(), 
                    value: css_value.to_string(), 
                    important: false 
                }]);
            }
        }
        
        if class.starts_with("ml-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![CssProperty { 
                    name: "margin-left".to_string(), 
                    value: css_value.to_string(), 
                    important: false 
                }]);
            }
        }
        
        if class.starts_with("mr-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![CssProperty { 
                    name: "margin-right".to_string(), 
                    value: css_value.to_string(), 
                    important: false 
                }]);
            }
        }
        
        // Parse margin x/y (horizontal/vertical)
        if class.starts_with("mx-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![
                    CssProperty { 
                        name: "margin-left".to_string(), 
                        value: css_value.to_string(), 
                        important: false 
                    },
                    CssProperty { 
                        name: "margin-right".to_string(), 
                        value: css_value.to_string(), 
                        important: false 
                    }
                ]);
            }
        }
        
        if class.starts_with("my-") {
            let value = &class[3..];
            if let Some((_, css_value)) = spacing_map.iter().find(|(k, _)| *k == value) {
                return Some(vec![
                    CssProperty { 
                        name: "margin-top".to_string(), 
                        value: css_value.to_string(), 
                        important: false 
                    },
                    CssProperty { 
                        name: "margin-bottom".to_string(), 
                        value: css_value.to_string(), 
                        important: false 
                    }
                ]);
            }
        }
        
        None
    }
    
    /// Parse color classes (background, text, border colors)
    fn parse_color_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        let color_map = [
            // Grays
            ("gray-50", "#f9fafb"), ("gray-100", "#f3f4f6"), ("gray-200", "#e5e7eb"), 
            ("gray-300", "#d1d5db"), ("gray-400", "#9ca3af"), ("gray-500", "#6b7280"),
            ("gray-600", "#4b5563"), ("gray-700", "#374151"), ("gray-800", "#1f2937"), ("gray-900", "#111827"),
            ("gray-950", "#030712"),
            
            // Zinc colors
            ("zinc-50", "#fafafa"), ("zinc-100", "#f4f4f5"), ("zinc-200", "#e4e4e7"), 
            ("zinc-300", "#d4d4d8"), ("zinc-400", "#a1a1aa"), ("zinc-500", "#71717a"),
            ("zinc-600", "#52525b"), ("zinc-700", "#3f3f46"), ("zinc-800", "#27272a"), ("zinc-900", "#18181b"),
            ("zinc-950", "#09090b"),
            
            // Blues
            ("blue-50", "#eff6ff"), ("blue-100", "#dbeafe"), ("blue-200", "#bfdbfe"), 
            ("blue-300", "#93c5fd"), ("blue-400", "#60a5fa"), ("blue-500", "#3b82f6"),
            ("blue-600", "#2563eb"), ("blue-700", "#1d4ed8"), ("blue-800", "#1e40af"), ("blue-900", "#1e3a8a"),
            ("blue-950", "#172554"),
            
            // Teal colors
            ("teal-50", "#f0fdfa"), ("teal-100", "#ccfbf1"), ("teal-200", "#99f6e4"), 
            ("teal-300", "#5eead4"), ("teal-400", "#2dd4bf"), ("teal-500", "#14b8a6"),
            ("teal-600", "#0d9488"), ("teal-700", "#0f766e"), ("teal-800", "#115e59"), ("teal-900", "#134e4a"),
            ("teal-950", "#042f2e"),
            
            // Emerald colors
            ("emerald-50", "#ecfdf5"), ("emerald-100", "#d1fae5"), ("emerald-200", "#a7f3d0"), 
            ("emerald-300", "#6ee7b7"), ("emerald-400", "#34d399"), ("emerald-500", "#10b981"),
            ("emerald-600", "#059669"), ("emerald-700", "#047857"), ("emerald-800", "#065f46"), ("emerald-900", "#064e3b"),
            ("emerald-950", "#022c22"),
            
            // Reds
            ("red-50", "#fef2f2"), ("red-100", "#fee2e2"), ("red-200", "#fecaca"), 
            ("red-300", "#fca5a5"), ("red-400", "#f87171"), ("red-500", "#ef4444"),
            ("red-600", "#dc2626"), ("red-700", "#b91c1c"), ("red-800", "#991b1b"), ("red-900", "#7f1d1d"),
            ("red-950", "#450a0a"),
            
            // Greens
            ("green-50", "#f0fdf4"), ("green-100", "#dcfce7"), ("green-200", "#bbf7d0"), 
            ("green-300", "#86efac"), ("green-400", "#4ade80"), ("green-500", "#22c55e"),
            ("green-600", "#16a34a"), ("green-700", "#15803d"), ("green-800", "#166534"), ("green-900", "#14532d"),
            ("green-950", "#052e16"),
            
            // Yellows
            ("yellow-50", "#fefce8"), ("yellow-100", "#fef3c7"), ("yellow-200", "#fde68a"), 
            ("yellow-300", "#fcd34d"), ("yellow-400", "#fbbf24"), ("yellow-500", "#f59e0b"),
            ("yellow-600", "#d97706"), ("yellow-700", "#b45309"), ("yellow-800", "#92400e"), ("yellow-900", "#78350f"),
            ("yellow-950", "#451a03"),
            
            // Purples
            ("purple-50", "#faf5ff"), ("purple-100", "#f3e8ff"), ("purple-200", "#e9d5ff"), 
            ("purple-300", "#d8b4fe"), ("purple-400", "#c084fc"), ("purple-500", "#a855f7"),
            ("purple-600", "#9333ea"), ("purple-700", "#7c3aed"), ("purple-800", "#6b21a8"), ("purple-900", "#581c87"),
            ("purple-950", "#3b0764"),
            
            // Special colors
            ("white", "#ffffff"), ("black", "#000000"), ("transparent", "transparent"),
        ];
        
        // Parse background colors
        if class.starts_with("bg-") {
            let color = &class[3..];
            
            // Check for opacity modifier (e.g., bg-blue-500/50)
            if let Some(slash_pos) = color.find('/') {
                let base_color = &color[..slash_pos];
                let opacity_str = &color[slash_pos + 1..];
                
                if let Some((_, hex_value)) = color_map.iter().find(|(k, _)| *k == base_color) {
                    if let Ok(opacity) = opacity_str.parse::<f32>() {
                        let rgba_value = self.hex_to_rgba(hex_value, opacity / 100.0);
                        return Some(vec![CssProperty { 
                            name: "background-color".to_string(), 
                            value: rgba_value, 
                            important: false 
                        }]);
                    }
                }
            } else {
                // No opacity modifier
                if let Some((_, hex_value)) = color_map.iter().find(|(k, _)| *k == color) {
                    return Some(vec![CssProperty { 
                        name: "background-color".to_string(), 
                        value: hex_value.to_string(), 
                        important: false 
                    }]);
                }
            }
        }
        
        // Parse text colors
        if class.starts_with("text-") {
            let color = &class[5..];
            
            // Check for opacity modifier (e.g., text-blue-500/50)
            if let Some(slash_pos) = color.find('/') {
                let base_color = &color[..slash_pos];
                let opacity_str = &color[slash_pos + 1..];
                
                if let Some((_, hex_value)) = color_map.iter().find(|(k, _)| *k == base_color) {
                    if let Ok(opacity) = opacity_str.parse::<f32>() {
                        let rgba_value = self.hex_to_rgba(hex_value, opacity / 100.0);
                        return Some(vec![CssProperty { 
                            name: "color".to_string(), 
                            value: rgba_value, 
                            important: false 
                        }]);
                    }
                }
            } else {
                // No opacity modifier
                if let Some((_, hex_value)) = color_map.iter().find(|(k, _)| *k == color) {
                    return Some(vec![CssProperty { 
                        name: "color".to_string(), 
                        value: hex_value.to_string(), 
                        important: false 
                    }]);
                }
            }
        }
        
        // Parse border colors
        if class.starts_with("border-") {
            let color = &class[7..];
            if let Some((_, hex_value)) = color_map.iter().find(|(k, _)| *k == color) {
                return Some(vec![CssProperty { 
                    name: "border-color".to_string(), 
                    value: hex_value.to_string(), 
                    important: false 
                }]);
            }
        }
        
        None
    }
    
    /// Convert hex color to RGBA with opacity
    fn hex_to_rgba(&self, hex: &str, opacity: f32) -> String {
        // Remove # if present
        let hex = hex.trim_start_matches('#');
        
        // Parse hex to RGB
        if hex.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                return format!("rgba({}, {}, {}, {})", r, g, b, opacity);
            }
        }
        
        // Fallback to original hex if parsing fails
        format!("{}", hex)
    }
    
    /// Parse typography classes
    fn parse_typography_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            // Font sizes
            "text-xs" => Some(vec![CssProperty { name: "font-size".to_string(), value: "0.75rem".to_string(), important: false }]),
            "text-sm" => Some(vec![CssProperty { name: "font-size".to_string(), value: "0.875rem".to_string(), important: false }]),
            "text-base" => Some(vec![CssProperty { name: "font-size".to_string(), value: "1rem".to_string(), important: false }]),
            "text-lg" => Some(vec![CssProperty { name: "font-size".to_string(), value: "1.125rem".to_string(), important: false }]),
            "text-xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "1.25rem".to_string(), important: false }]),
            "text-2xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "1.5rem".to_string(), important: false }]),
            "text-3xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "1.875rem".to_string(), important: false }]),
            "text-4xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "2.25rem".to_string(), important: false }]),
            "text-5xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "3rem".to_string(), important: false }]),
            "text-6xl" => Some(vec![CssProperty { name: "font-size".to_string(), value: "3.75rem".to_string(), important: false }]),
            
            // Font weights
            "font-thin" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "100".to_string(), important: false }]),
            "font-extralight" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "200".to_string(), important: false }]),
            "font-light" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "300".to_string(), important: false }]),
            "font-normal" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "400".to_string(), important: false }]),
            "font-medium" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "500".to_string(), important: false }]),
            "font-semibold" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "600".to_string(), important: false }]),
            "font-bold" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "700".to_string(), important: false }]),
            "font-extrabold" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "800".to_string(), important: false }]),
            "font-black" => Some(vec![CssProperty { name: "font-weight".to_string(), value: "900".to_string(), important: false }]),
            
            // Text alignment
            "text-left" => Some(vec![CssProperty { name: "text-align".to_string(), value: "left".to_string(), important: false }]),
            "text-center" => Some(vec![CssProperty { name: "text-align".to_string(), value: "center".to_string(), important: false }]),
            "text-right" => Some(vec![CssProperty { name: "text-align".to_string(), value: "right".to_string(), important: false }]),
            "text-justify" => Some(vec![CssProperty { name: "text-align".to_string(), value: "justify".to_string(), important: false }]),
            
            // Letter spacing (tracking)
            "tracking-tighter" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "-0.05em".to_string(), important: false }]),
            "tracking-tight" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "-0.025em".to_string(), important: false }]),
            "tracking-normal" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "0em".to_string(), important: false }]),
            "tracking-wide" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "0.025em".to_string(), important: false }]),
            "tracking-wider" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "0.05em".to_string(), important: false }]),
            "tracking-widest" => Some(vec![CssProperty { name: "letter-spacing".to_string(), value: "0.1em".to_string(), important: false }]),
            
            _ => None,
        }
    }
    
    /// Parse layout classes
    fn parse_layout_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "block" => Some(vec![CssProperty { name: "display".to_string(), value: "block".to_string(), important: false }]),
            "inline" => Some(vec![CssProperty { name: "display".to_string(), value: "inline".to_string(), important: false }]),
            "inline-block" => Some(vec![CssProperty { name: "display".to_string(), value: "inline-block".to_string(), important: false }]),
            "flex" => Some(vec![CssProperty { name: "display".to_string(), value: "flex".to_string(), important: false }]),
            "inline-flex" => Some(vec![CssProperty { name: "display".to_string(), value: "inline-flex".to_string(), important: false }]),
            "grid" => Some(vec![CssProperty { name: "display".to_string(), value: "grid".to_string(), important: false }]),
            "inline-grid" => Some(vec![CssProperty { name: "display".to_string(), value: "inline-grid".to_string(), important: false }]),
            "hidden" => Some(vec![CssProperty { name: "display".to_string(), value: "none".to_string(), important: false }]),
            
            // Position
            "static" => Some(vec![CssProperty { name: "position".to_string(), value: "static".to_string(), important: false }]),
            "fixed" => Some(vec![CssProperty { name: "position".to_string(), value: "fixed".to_string(), important: false }]),
            "absolute" => Some(vec![CssProperty { name: "position".to_string(), value: "absolute".to_string(), important: false }]),
            "relative" => Some(vec![CssProperty { name: "position".to_string(), value: "relative".to_string(), important: false }]),
            "sticky" => Some(vec![CssProperty { name: "position".to_string(), value: "sticky".to_string(), important: false }]),
            
            // Positioning utilities
            "inset-0" => Some(vec![CssProperty { name: "top".to_string(), value: "0px".to_string(), important: false }, CssProperty { name: "right".to_string(), value: "0px".to_string(), important: false }, CssProperty { name: "bottom".to_string(), value: "0px".to_string(), important: false }, CssProperty { name: "left".to_string(), value: "0px".to_string(), important: false }]),
            "inset-x-0" => Some(vec![CssProperty { name: "left".to_string(), value: "0px".to_string(), important: false }, CssProperty { name: "right".to_string(), value: "0px".to_string(), important: false }]),
            "inset-y-0" => Some(vec![CssProperty { name: "top".to_string(), value: "0px".to_string(), important: false }, CssProperty { name: "bottom".to_string(), value: "0px".to_string(), important: false }]),
            "top-0" => Some(vec![CssProperty { name: "top".to_string(), value: "0px".to_string(), important: false }]),
            "right-0" => Some(vec![CssProperty { name: "right".to_string(), value: "0px".to_string(), important: false }]),
            "bottom-0" => Some(vec![CssProperty { name: "bottom".to_string(), value: "0px".to_string(), important: false }]),
            "left-0" => Some(vec![CssProperty { name: "left".to_string(), value: "0px".to_string(), important: false }]),
            
            // Negative positioning
            "-inset-x-4" => Some(vec![CssProperty { name: "left".to_string(), value: "-1rem".to_string(), important: false }, CssProperty { name: "right".to_string(), value: "-1rem".to_string(), important: false }]),
            "-inset-y-6" => Some(vec![CssProperty { name: "top".to_string(), value: "-1.5rem".to_string(), important: false }, CssProperty { name: "bottom".to_string(), value: "-1.5rem".to_string(), important: false }]),
            
            // Z-index utilities
            "z-0" => Some(vec![CssProperty { name: "z-index".to_string(), value: "0".to_string(), important: false }]),
            "z-10" => Some(vec![CssProperty { name: "z-index".to_string(), value: "10".to_string(), important: false }]),
            "z-20" => Some(vec![CssProperty { name: "z-index".to_string(), value: "20".to_string(), important: false }]),
            "z-30" => Some(vec![CssProperty { name: "z-index".to_string(), value: "30".to_string(), important: false }]),
            "z-40" => Some(vec![CssProperty { name: "z-index".to_string(), value: "40".to_string(), important: false }]),
            "z-50" => Some(vec![CssProperty { name: "z-index".to_string(), value: "50".to_string(), important: false }]),
            
            _ => None,
        }
    }
    
    /// Parse flexbox classes
    fn parse_flexbox_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "flex-row" => Some(vec![CssProperty { name: "flex-direction".to_string(), value: "row".to_string(), important: false }]),
            "flex-row-reverse" => Some(vec![CssProperty { name: "flex-direction".to_string(), value: "row-reverse".to_string(), important: false }]),
            "flex-col" => Some(vec![CssProperty { name: "flex-direction".to_string(), value: "column".to_string(), important: false }]),
            "flex-col-reverse" => Some(vec![CssProperty { name: "flex-direction".to_string(), value: "column-reverse".to_string(), important: false }]),
            
            "flex-wrap" => Some(vec![CssProperty { name: "flex-wrap".to_string(), value: "wrap".to_string(), important: false }]),
            "flex-wrap-reverse" => Some(vec![CssProperty { name: "flex-wrap".to_string(), value: "wrap-reverse".to_string(), important: false }]),
            "flex-nowrap" => Some(vec![CssProperty { name: "flex-wrap".to_string(), value: "nowrap".to_string(), important: false }]),
            
            "justify-start" => Some(vec![CssProperty { name: "justify-content".to_string(), value: "flex-start".to_string(), important: false }]),
            "justify-end" => Some(vec![CssProperty { name: "justify-content".to_string(), value: "flex-end".to_string(), important: false }]),
            "justify-center" => Some(vec![CssProperty { name: "justify-content".to_string(), value: "center".to_string(), important: false }]),
            "justify-between" => Some(vec![CssProperty { name: "justify-content".to_string(), value: "space-between".to_string(), important: false }]),
            "justify-around" => Some(vec![CssProperty { name: "justify-content".to_string(), value: "space-around".to_string(), important: false }]),
            "justify-evenly" => Some(vec![CssProperty { name: "justify-content".to_string(), value: "space-evenly".to_string(), important: false }]),
            
            "items-start" => Some(vec![CssProperty { name: "align-items".to_string(), value: "flex-start".to_string(), important: false }]),
            "items-end" => Some(vec![CssProperty { name: "align-items".to_string(), value: "flex-end".to_string(), important: false }]),
            "items-center" => Some(vec![CssProperty { name: "align-items".to_string(), value: "center".to_string(), important: false }]),
            "items-baseline" => Some(vec![CssProperty { name: "align-items".to_string(), value: "baseline".to_string(), important: false }]),
            "items-stretch" => Some(vec![CssProperty { name: "align-items".to_string(), value: "stretch".to_string(), important: false }]),
            
            // Order utilities
            "order-first" => Some(vec![CssProperty { name: "order".to_string(), value: "-1".to_string(), important: false }]),
            "order-last" => Some(vec![CssProperty { name: "order".to_string(), value: "9999".to_string(), important: false }]),
            "order-none" => Some(vec![CssProperty { name: "order".to_string(), value: "0".to_string(), important: false }]),
            "order-1" => Some(vec![CssProperty { name: "order".to_string(), value: "1".to_string(), important: false }]),
            "order-2" => Some(vec![CssProperty { name: "order".to_string(), value: "2".to_string(), important: false }]),
            "order-3" => Some(vec![CssProperty { name: "order".to_string(), value: "3".to_string(), important: false }]),
            "order-4" => Some(vec![CssProperty { name: "order".to_string(), value: "4".to_string(), important: false }]),
            "order-5" => Some(vec![CssProperty { name: "order".to_string(), value: "5".to_string(), important: false }]),
            "order-6" => Some(vec![CssProperty { name: "order".to_string(), value: "6".to_string(), important: false }]),
            "order-7" => Some(vec![CssProperty { name: "order".to_string(), value: "7".to_string(), important: false }]),
            "order-8" => Some(vec![CssProperty { name: "order".to_string(), value: "8".to_string(), important: false }]),
            "order-9" => Some(vec![CssProperty { name: "order".to_string(), value: "9".to_string(), important: false }]),
            "order-10" => Some(vec![CssProperty { name: "order".to_string(), value: "10".to_string(), important: false }]),
            "order-11" => Some(vec![CssProperty { name: "order".to_string(), value: "11".to_string(), important: false }]),
            "order-12" => Some(vec![CssProperty { name: "order".to_string(), value: "12".to_string(), important: false }]),
            
            _ => None,
        }
    }
    
    /// Parse grid classes
    fn parse_grid_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "grid-cols-1" => Some(vec![CssProperty { name: "grid-template-columns".to_string(), value: "repeat(1, minmax(0, 1fr))".to_string(), important: false }]),
            "grid-cols-2" => Some(vec![CssProperty { name: "grid-template-columns".to_string(), value: "repeat(2, minmax(0, 1fr))".to_string(), important: false }]),
            "grid-cols-3" => Some(vec![CssProperty { name: "grid-template-columns".to_string(), value: "repeat(3, minmax(0, 1fr))".to_string(), important: false }]),
            "grid-cols-4" => Some(vec![CssProperty { name: "grid-template-columns".to_string(), value: "repeat(4, minmax(0, 1fr))".to_string(), important: false }]),
            "grid-cols-5" => Some(vec![CssProperty { name: "grid-template-columns".to_string(), value: "repeat(5, minmax(0, 1fr))".to_string(), important: false }]),
            "grid-cols-6" => Some(vec![CssProperty { name: "grid-template-columns".to_string(), value: "repeat(6, minmax(0, 1fr))".to_string(), important: false }]),
            "grid-cols-12" => Some(vec![CssProperty { name: "grid-template-columns".to_string(), value: "repeat(12, minmax(0, 1fr))".to_string(), important: false }]),
            
            "grid-rows-1" => Some(vec![CssProperty { name: "grid-template-rows".to_string(), value: "repeat(1, minmax(0, 1fr))".to_string(), important: false }]),
            "grid-rows-2" => Some(vec![CssProperty { name: "grid-template-rows".to_string(), value: "repeat(2, minmax(0, 1fr))".to_string(), important: false }]),
            "grid-rows-3" => Some(vec![CssProperty { name: "grid-template-rows".to_string(), value: "repeat(3, minmax(0, 1fr))".to_string(), important: false }]),
            "grid-rows-4" => Some(vec![CssProperty { name: "grid-template-rows".to_string(), value: "repeat(4, minmax(0, 1fr))".to_string(), important: false }]),
            "grid-rows-5" => Some(vec![CssProperty { name: "grid-template-rows".to_string(), value: "repeat(5, minmax(0, 1fr))".to_string(), important: false }]),
            "grid-rows-6" => Some(vec![CssProperty { name: "grid-template-rows".to_string(), value: "repeat(6, minmax(0, 1fr))".to_string(), important: false }]),
            
            _ => None,
        }
    }
    
    /// Parse border classes
    fn parse_border_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "border" => Some(vec![CssProperty { name: "border-width".to_string(), value: "1px".to_string(), important: false }]),
            "border-0" => Some(vec![CssProperty { name: "border-width".to_string(), value: "0px".to_string(), important: false }]),
            "border-2" => Some(vec![CssProperty { name: "border-width".to_string(), value: "2px".to_string(), important: false }]),
            "border-4" => Some(vec![CssProperty { name: "border-width".to_string(), value: "4px".to_string(), important: false }]),
            "border-8" => Some(vec![CssProperty { name: "border-width".to_string(), value: "8px".to_string(), important: false }]),
            
            "rounded" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0.25rem".to_string(), important: false }]),
            "rounded-sm" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0.125rem".to_string(), important: false }]),
            "rounded-md" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0.375rem".to_string(), important: false }]),
            "rounded-lg" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0.5rem".to_string(), important: false }]),
            "rounded-xl" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0.75rem".to_string(), important: false }]),
            "rounded-2xl" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "1rem".to_string(), important: false }]),
            "rounded-3xl" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "1.5rem".to_string(), important: false }]),
            "rounded-full" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "9999px".to_string(), important: false }]),
            "rounded-none" => Some(vec![CssProperty { name: "border-radius".to_string(), value: "0px".to_string(), important: false }]),
            
            _ => None,
        }
    }
    
    /// Parse effects classes (shadows, etc.)
    fn parse_effects_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "shadow-sm" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(), important: false }]),
            "shadow" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(), important: false }]),
            "shadow-md" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(), important: false }]),
            "shadow-lg" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(), important: false }]),
            "shadow-xl" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string(), important: false }]),
            "shadow-2xl" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(), important: false }]),
            "shadow-none" => Some(vec![CssProperty { name: "box-shadow".to_string(), value: "0 0 #0000".to_string(), important: false }]),
            
            _ => None,
        }
    }
    
    /// Parse transform classes
    fn parse_transform_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "transform" => Some(vec![CssProperty { name: "transform".to_string(), value: "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))".to_string(), important: false }]),
            "scale-0" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0)".to_string(), important: false }]),
            "scale-50" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.5)".to_string(), important: false }]),
            "scale-75" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.75)".to_string(), important: false }]),
            "scale-90" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.9)".to_string(), important: false }]),
            "scale-95" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(0.95)".to_string(), important: false }]),
            "scale-100" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1)".to_string(), important: false }]),
            "scale-105" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.05)".to_string(), important: false }]),
            "scale-110" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.1)".to_string(), important: false }]),
            "scale-125" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.25)".to_string(), important: false }]),
            "scale-150" => Some(vec![CssProperty { name: "transform".to_string(), value: "scale(1.5)".to_string(), important: false }]),
            
            _ => None,
        }
    }
    
    /// Parse animation classes
    fn parse_animation_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "animate-none" => Some(vec![CssProperty { name: "animation".to_string(), value: "none".to_string(), important: false }]),
            "animate-spin" => Some(vec![CssProperty { name: "animation".to_string(), value: "spin 1s linear infinite".to_string(), important: false }]),
            "animate-ping" => Some(vec![CssProperty { name: "animation".to_string(), value: "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(), important: false }]),
            "animate-pulse" => Some(vec![CssProperty { name: "animation".to_string(), value: "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(), important: false }]),
            "animate-bounce" => Some(vec![CssProperty { name: "animation".to_string(), value: "bounce 1s infinite".to_string(), important: false }]),
            
            _ => None,
        }
    }
    
    /// Parse interactivity classes
    fn parse_interactivity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "cursor-auto" => Some(vec![CssProperty { name: "cursor".to_string(), value: "auto".to_string(), important: false }]),
            "cursor-default" => Some(vec![CssProperty { name: "cursor".to_string(), value: "default".to_string(), important: false }]),
            "cursor-pointer" => Some(vec![CssProperty { name: "cursor".to_string(), value: "pointer".to_string(), important: false }]),
            "cursor-wait" => Some(vec![CssProperty { name: "cursor".to_string(), value: "wait".to_string(), important: false }]),
            "cursor-text" => Some(vec![CssProperty { name: "cursor".to_string(), value: "text".to_string(), important: false }]),
            "cursor-move" => Some(vec![CssProperty { name: "cursor".to_string(), value: "move".to_string(), important: false }]),
            "cursor-help" => Some(vec![CssProperty { name: "cursor".to_string(), value: "help".to_string(), important: false }]),
            "cursor-not-allowed" => Some(vec![CssProperty { name: "cursor".to_string(), value: "not-allowed".to_string(), important: false }]),
            
            "select-none" => Some(vec![CssProperty { name: "user-select".to_string(), value: "none".to_string(), important: false }]),
            "select-text" => Some(vec![CssProperty { name: "user-select".to_string(), value: "text".to_string(), important: false }]),
            "select-all" => Some(vec![CssProperty { name: "user-select".to_string(), value: "all".to_string(), important: false }]),
            "select-auto" => Some(vec![CssProperty { name: "user-select".to_string(), value: "auto".to_string(), important: false }]),
            
            _ => None,
        }
    }

    /// Convert a CSS rule to CSS string
    fn rule_to_css(&self, rule: &CssRule) -> String {
        let mut css = format!("{} {{\n", rule.selector);
        for property in &rule.properties {
            let important = if property.important { " !important" } else { "" };
            css.push_str(&format!("  {}: {}{};\n", property.name, property.value, important));
        }
        css.push_str("}\n");
        css
    }

    /// Minify CSS by removing unnecessary whitespace
    fn minify_css(&self, css: &str) -> String {
        css.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join("")
            .replace(" {", "{")
            .replace("} ", "}")
            .replace("; ", ";")
            .replace(" ", "")
    }
}

impl Default for CssGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for comprehensive CSS generation
#[derive(Debug, Clone)]
pub struct CssGenerationConfig {
    /// Include color palettes
    pub include_colors: bool,
    /// Include spacing utilities
    pub include_spacing: bool,
    /// Include typography utilities
    pub include_typography: bool,
    /// Include layout utilities
    pub include_layout: bool,
    /// Include flexbox utilities
    pub include_flexbox: bool,
    /// Include grid utilities
    pub include_grid: bool,
    /// Include border utilities
    pub include_borders: bool,
    /// Include effects utilities
    pub include_effects: bool,
    /// Include transform utilities
    pub include_transforms: bool,
    /// Include animation utilities
    pub include_animations: bool,
    /// Include interactivity utilities
    pub include_interactivity: bool,
    /// Include sizing utilities
    pub include_sizing: bool,
    /// Include background utilities
    pub include_backgrounds: bool,
    /// Include filter utilities
    pub include_filters: bool,
    /// Include transition utilities
    pub include_transitions: bool,
    /// Include text shadow utilities
    pub include_text_shadow: bool,
    /// Include mask utilities
    pub include_mask: bool,
    /// Include logical properties
    pub include_logical_properties: bool,
    /// Include enhanced backdrop filters
    pub include_enhanced_backdrop_filters: bool,
    /// Include modern CSS features
    pub include_modern_css_features: bool,
    /// Include device variants
    pub include_device_variants: bool,
    /// Include CSS nesting
    pub include_css_nesting: bool,
    /// Include advanced plugin system
    pub include_advanced_plugin_system: bool,
    /// Include enhanced validation
    pub include_enhanced_validation: bool,
    /// Include advanced performance optimization
    pub include_advanced_performance_optimization: bool,
    /// Include container queries
    pub include_container_queries: bool,
    /// Include color functions
    pub include_color_functions: bool,
    /// Include performance optimization
    pub include_performance_optimization: bool,
    /// Include advanced animations
    pub include_advanced_animations: bool,
    /// Color palettes to include
    pub color_palettes: Vec<String>,
    /// Generate responsive variants
    pub include_responsive: bool,
    /// Generate dark mode variants
    pub include_dark_mode: bool,
}

impl Default for CssGenerationConfig {
    fn default() -> Self {
        Self {
            include_colors: true,
            include_spacing: true,
            include_typography: true,
            include_layout: true,
            include_flexbox: true,
            include_grid: true,
            include_borders: true,
            include_effects: true,
            include_transforms: true,
            include_animations: true,
            include_interactivity: true,
            include_sizing: true,
            include_backgrounds: true,
            include_filters: true,
            include_transitions: true,
            include_text_shadow: true,
            include_mask: true,
            include_logical_properties: true,
            include_enhanced_backdrop_filters: true,
            include_modern_css_features: true,
            include_device_variants: true,
            include_css_nesting: true,
            include_advanced_plugin_system: true,
            include_enhanced_validation: true,
            include_advanced_performance_optimization: true,
            include_container_queries: true,
            include_color_functions: true,
            include_performance_optimization: true,
            include_advanced_animations: true,
            color_palettes: vec![
                "gray".to_string(),
                "blue".to_string(),
                "red".to_string(),
                "green".to_string(),
                "yellow".to_string(),
                "purple".to_string(),
            ],
            include_responsive: true,
            include_dark_mode: true,
        }
    }
}

impl CssGenerator {
    /// Generate comprehensive CSS with all Tailwind utilities
    pub fn generate_comprehensive_css(&mut self, config: &CssGenerationConfig) -> Result<String> {
        // Generate spacing utilities
        if config.include_spacing {
            self.generate_spacing_utilities()?;
        }
        
        // Generate color utilities
        if config.include_colors {
            self.generate_color_utilities(&config.color_palettes)?;
        }
        
        // Generate typography utilities
        if config.include_typography {
            self.generate_typography_utilities()?;
        }
        
        // Generate layout utilities
        if config.include_layout {
            self.generate_layout_utilities()?;
        }
        
        // Generate flexbox utilities
        if config.include_flexbox {
            self.generate_flexbox_utilities()?;
        }
        
        // Generate grid utilities
        if config.include_grid {
            self.generate_grid_utilities()?;
        }
        
        // Generate border utilities
        if config.include_borders {
            self.generate_border_utilities()?;
        }
        
        // Generate effects utilities
        if config.include_effects {
            self.generate_effects_utilities()?;
        }
        
        // Generate transform utilities
        if config.include_transforms {
            self.generate_transform_utilities()?;
        }
        
        // Generate animation utilities
        if config.include_animations {
            self.generate_animation_utilities()?;
        }
        
        // Generate interactivity utilities
        if config.include_interactivity {
            self.generate_interactivity_utilities()?;
        }
        
        // Generate sizing utilities
        if config.include_sizing {
            self.generate_sizing_utilities()?;
        }
        
        // Generate background utilities
        if config.include_backgrounds {
            self.generate_background_utilities()?;
        }
        
        // Generate filter utilities
        if config.include_filters {
            self.generate_filter_utilities()?;
        }
        
        // Generate transition utilities
        if config.include_transitions {
            self.generate_transition_utilities()?;
        }
        
        // Generate text shadow utilities
        if config.include_text_shadow {
            self.generate_text_shadow_utilities()?;
        }
        
        // Generate mask utilities
        if config.include_mask {
            self.generate_mask_utilities()?;
        }
        
        // Generate logical properties utilities
        if config.include_logical_properties {
            self.generate_logical_properties_utilities()?;
        }
        
        // Generate enhanced backdrop filter utilities
        if config.include_enhanced_backdrop_filters {
            self.generate_enhanced_backdrop_filter_utilities()?;
        }
        
        // Generate modern CSS features utilities
        if config.include_modern_css_features {
            self.generate_modern_css_features_utilities()?;
        }
        
        // Generate device variant utilities
        if config.include_device_variants {
            self.generate_device_variant_utilities()?;
        }
        
        // Generate CSS nesting utilities
        if config.include_css_nesting {
            self.generate_css_nesting_utilities()?;
        }
        
        // Generate advanced plugin system utilities
        if config.include_advanced_plugin_system {
            self.generate_advanced_plugin_system_utilities()?;
        }
        
        // Generate enhanced validation utilities
        if config.include_enhanced_validation {
            self.generate_enhanced_validation_utilities()?;
        }
        
        // Generate advanced performance optimization utilities
        if config.include_advanced_performance_optimization {
            self.generate_advanced_performance_optimization_utilities()?;
        }
        
        // Generate container query utilities
        if config.include_container_queries {
            self.generate_container_query_utilities()?;
        }
        
        // Generate color function utilities
        if config.include_color_functions {
            self.generate_color_function_utilities()?;
        }
        
        // Generate performance optimization utilities
        if config.include_performance_optimization {
            self.generate_performance_optimization_utilities()?;
        }
        
        // Generate advanced animation utilities
        if config.include_advanced_animations {
            self.generate_advanced_animation_utilities()?;
        }
        
        // Generate responsive variants
        if config.include_responsive {
            self.generate_responsive_variants()?;
        }
        
        // Generate dark mode variants
        if config.include_dark_mode {
            self.generate_dark_mode_variants()?;
        }
        
        Ok(self.generate_css())
    }
    
    /// Generate spacing utilities (padding, margin, etc.)
    pub fn generate_spacing_utilities(&mut self) -> Result<()> {
        let spacing_values = [
            "0", "px", "0.5", "1", "1.5", "2", "2.5", "3", "3.5", "4", "5", "6", "7", "8", "9", "10",
            "11", "12", "14", "16", "20", "24", "28", "32", "36", "40", "44", "48", "52", "56", "60", "64",
            "72", "80", "96"
        ];
        
        for value in &spacing_values {
            // Padding
            self.add_class(&format!("p-{}", value))?;
            self.add_class(&format!("pt-{}", value))?;
            self.add_class(&format!("pr-{}", value))?;
            self.add_class(&format!("pb-{}", value))?;
            self.add_class(&format!("pl-{}", value))?;
            self.add_class(&format!("px-{}", value))?;
            self.add_class(&format!("py-{}", value))?;
            
            // Margin
            self.add_class(&format!("m-{}", value))?;
            self.add_class(&format!("mt-{}", value))?;
            self.add_class(&format!("mr-{}", value))?;
            self.add_class(&format!("mb-{}", value))?;
            self.add_class(&format!("ml-{}", value))?;
            self.add_class(&format!("mx-{}", value))?;
            self.add_class(&format!("my-{}", value))?;
        }
        
        Ok(())
    }
    
    /// Generate color utilities for specified palettes
    pub fn generate_color_utilities(&mut self, palettes: &[String]) -> Result<()> {
        let color_shades = ["50", "100", "200", "300", "400", "500", "600", "700", "800", "900", "950"];
        
        for palette in palettes {
            for shade in &color_shades {
                // Background colors
                self.add_class(&format!("bg-{}-{}", palette, shade))?;
                // Text colors
                self.add_class(&format!("text-{}-{}", palette, shade))?;
                // Border colors
                self.add_class(&format!("border-{}-{}", palette, shade))?;
            }
        }
        
        // Special colors
        self.add_class("bg-white")?;
        self.add_class("bg-black")?;
        self.add_class("bg-transparent")?;
        self.add_class("text-white")?;
        self.add_class("text-black")?;
        self.add_class("text-transparent")?;
        
        Ok(())
    }
    
    /// Generate typography utilities
    pub fn generate_typography_utilities(&mut self) -> Result<()> {
        // Font sizes
        let font_sizes = ["xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl"];
        for size in &font_sizes {
            self.add_class(&format!("text-{}", size))?;
        }
        
        // Font weights
        let font_weights = ["thin", "extralight", "light", "normal", "medium", "semibold", "bold", "extrabold", "black"];
        for weight in &font_weights {
            self.add_class(&format!("font-{}", weight))?;
        }
        
        // Text alignment
        let alignments = ["left", "center", "right", "justify"];
        for alignment in &alignments {
            self.add_class(&format!("text-{}", alignment))?;
        }
        
        Ok(())
    }
    
    /// Generate layout utilities
    pub fn generate_layout_utilities(&mut self) -> Result<()> {
        let displays = ["block", "inline", "inline-block", "flex", "inline-flex", "grid", "inline-grid", "hidden"];
        for display in &displays {
            self.add_class(display)?;
        }
        
        let positions = ["static", "fixed", "absolute", "relative", "sticky"];
        for position in &positions {
            self.add_class(position)?;
        }
        
        Ok(())
    }
    
    /// Generate flexbox utilities
    pub fn generate_flexbox_utilities(&mut self) -> Result<()> {
        let flex_directions = ["flex-row", "flex-row-reverse", "flex-col", "flex-col-reverse"];
        for direction in &flex_directions {
            self.add_class(direction)?;
        }
        
        let flex_wraps = ["flex-wrap", "flex-wrap-reverse", "flex-nowrap"];
        for wrap in &flex_wraps {
            self.add_class(wrap)?;
        }
        
        let justify_contents = ["justify-start", "justify-end", "justify-center", "justify-between", "justify-around", "justify-evenly"];
        for justify in &justify_contents {
            self.add_class(justify)?;
        }
        
        let align_items = ["items-start", "items-end", "items-center", "items-baseline", "items-stretch"];
        for align in &align_items {
            self.add_class(align)?;
        }
        
        Ok(())
    }
    
    /// Generate grid utilities
    pub fn generate_grid_utilities(&mut self) -> Result<()> {
        let grid_cols = ["1", "2", "3", "4", "5", "6", "12"];
        for cols in &grid_cols {
            self.add_class(&format!("grid-cols-{}", cols))?;
        }
        
        let grid_rows = ["1", "2", "3", "4", "5", "6"];
        for rows in &grid_rows {
            self.add_class(&format!("grid-rows-{}", rows))?;
        }
        
        Ok(())
    }
    
    /// Generate border utilities
    pub fn generate_border_utilities(&mut self) -> Result<()> {
        let border_widths = ["0", "2", "4", "8"];
        for width in &border_widths {
            self.add_class(&format!("border-{}", width))?;
        }
        
        let border_radius = ["none", "sm", "md", "lg", "xl", "2xl", "3xl", "full"];
        for radius in &border_radius {
            self.add_class(&format!("rounded-{}", radius))?;
        }
        
        Ok(())
    }
    
    /// Generate effects utilities
    pub fn generate_effects_utilities(&mut self) -> Result<()> {
        let shadows = ["none", "sm", "md", "lg", "xl", "2xl"];
        for shadow in &shadows {
            self.add_class(&format!("shadow-{}", shadow))?;
        }
        
        Ok(())
    }
    
    /// Generate transform utilities
    pub fn generate_transform_utilities(&mut self) -> Result<()> {
        self.add_class("transform")?;
        
        let scales = ["0", "50", "75", "90", "95", "100", "105", "110", "125", "150"];
        for scale in &scales {
            self.add_class(&format!("scale-{}", scale))?;
        }
        
        Ok(())
    }
    
    /// Generate animation utilities
    pub fn generate_animation_utilities(&mut self) -> Result<()> {
        let animations = ["none", "spin", "ping", "pulse", "bounce"];
        for animation in &animations {
            self.add_class(&format!("animate-{}", animation))?;
        }
        
        Ok(())
    }
    
    /// Generate interactivity utilities
    pub fn generate_interactivity_utilities(&mut self) -> Result<()> {
        let cursors = ["auto", "default", "pointer", "wait", "text", "move", "help", "not-allowed"];
        for cursor in &cursors {
            self.add_class(&format!("cursor-{}", cursor))?;
        }
        
        let selects = ["none", "text", "all", "auto"];
        for select in &selects {
            self.add_class(&format!("select-{}", select))?;
        }
        
        Ok(())
    }
    
    /// Generate responsive variants
    pub fn generate_responsive_variants(&mut self) -> Result<()> {
        let breakpoints = [Breakpoint::Sm, Breakpoint::Md, Breakpoint::Lg, Breakpoint::Xl, Breakpoint::Xl2];
        let common_classes = ["p-4", "m-4", "bg-blue-500", "text-white", "flex", "grid"];
        
        for breakpoint in &breakpoints {
            for class in &common_classes {
                self.add_responsive_class(*breakpoint, class)?;
            }
        }
        
        Ok(())
    }
    
    /// Generate dark mode variants
    pub fn generate_dark_mode_variants(&mut self) -> Result<()> {
        // This would require dark mode support in the CSS generator
        // For now, we'll add some basic dark mode classes
        let dark_classes = ["dark:bg-gray-800", "dark:text-white", "dark:border-gray-600"];
        
        for class in &dark_classes {
            // Parse dark mode classes and add them
            if let Some(actual_class) = class.strip_prefix("dark:") {
                self.add_class(actual_class)?;
            }
        }
        
        Ok(())
    }
    
    /// Generate sizing utilities (width, height, min/max dimensions)
    pub fn generate_sizing_utilities(&mut self) -> Result<()> {
        let sizing_values = [
            "0", "px", "0.5", "1", "1.5", "2", "2.5", "3", "3.5", "4", "5", "6", "7", "8", "9", "10",
            "11", "12", "14", "16", "20", "24", "28", "32", "36", "40", "44", "48", "52", "56", "60", "64",
            "72", "80", "96", "auto", "full", "screen", "min", "max", "fit"
        ];
        
        for value in &sizing_values {
            // Width utilities
            self.add_class(&format!("w-{}", value))?;
            self.add_class(&format!("min-w-{}", value))?;
            self.add_class(&format!("max-w-{}", value))?;
            
            // Height utilities
            self.add_class(&format!("h-{}", value))?;
            self.add_class(&format!("min-h-{}", value))?;
            self.add_class(&format!("max-h-{}", value))?;
        }
        
        // Special sizing utilities
        let special_sizing = [
            "w-screen", "h-screen", "w-full", "h-full", "w-auto", "h-auto",
            "w-fit", "h-fit", "w-max", "h-max", "w-min", "h-min",
            "w-1/2", "w-1/3", "w-2/3", "w-1/4", "w-3/4", "w-1/5", "w-2/5", "w-3/5", "w-4/5",
            "h-1/2", "h-1/3", "h-2/3", "h-1/4", "h-3/4", "h-1/5", "h-2/5", "h-3/5", "h-4/5",
        ];
        
        for class in &special_sizing {
            self.add_class(class)?;
        }
        
        Ok(())
    }
    
    /// Generate background utilities (images, gradients, attachments)
    pub fn generate_background_utilities(&mut self) -> Result<()> {
        // Background size utilities
        let background_sizes = ["auto", "cover", "contain"];
        for size in &background_sizes {
            self.add_class(&format!("bg-{}", size))?;
        }
        
        // Background position utilities
        let background_positions = [
            "bottom", "center", "left", "left-bottom", "left-top", "right", "right-bottom", "right-top", "top"
        ];
        for position in &background_positions {
            self.add_class(&format!("bg-{}", position))?;
        }
        
        // Background repeat utilities
        let background_repeats = ["no-repeat", "repeat", "repeat-x", "repeat-y", "round", "space"];
        for repeat in &background_repeats {
            self.add_class(&format!("bg-{}", repeat))?;
        }
        
        // Background attachment utilities
        let background_attachments = ["fixed", "local", "scroll"];
        for attachment in &background_attachments {
            self.add_class(&format!("bg-{}", attachment))?;
        }
        
        // Background clip utilities
        let background_clips = ["border", "padding", "content", "text"];
        for clip in &background_clips {
            self.add_class(&format!("bg-clip-{}", clip))?;
        }
        
        // Background origin utilities
        let background_origins = ["border", "padding", "content"];
        for origin in &background_origins {
            self.add_class(&format!("bg-origin-{}", origin))?;
        }
        
        Ok(())
    }
    
    /// Generate filter utilities (blur, brightness, contrast, etc.)
    pub fn generate_filter_utilities(&mut self) -> Result<()> {
        // Blur utilities
        let blur_values = ["none", "sm", "0", "1", "2", "3", "xl", "2xl", "3xl"];
        for blur in &blur_values {
            self.add_class(&format!("blur-{}", blur))?;
        }
        
        // Brightness utilities
        let brightness_values = ["0", "50", "75", "90", "95", "100", "105", "110", "125", "150", "200"];
        for brightness in &brightness_values {
            self.add_class(&format!("brightness-{}", brightness))?;
        }
        
        // Contrast utilities
        let contrast_values = ["0", "50", "75", "100", "125", "150", "200"];
        for contrast in &contrast_values {
            self.add_class(&format!("contrast-{}", contrast))?;
        }
        
        // Grayscale utilities
        let grayscale_values = ["0", "100"];
        for grayscale in &grayscale_values {
            self.add_class(&format!("grayscale-{}", grayscale))?;
        }
        
        // Hue rotate utilities
        let hue_rotate_values = ["0", "15", "30", "60", "90", "180"];
        for hue_rotate in &hue_rotate_values {
            self.add_class(&format!("hue-rotate-{}", hue_rotate))?;
        }
        
        // Invert utilities
        let invert_values = ["0", "100"];
        for invert in &invert_values {
            self.add_class(&format!("invert-{}", invert))?;
        }
        
        // Saturate utilities
        let saturate_values = ["0", "50", "100", "150", "200"];
        for saturate in &saturate_values {
            self.add_class(&format!("saturate-{}", saturate))?;
        }
        
        // Sepia utilities
        let sepia_values = ["0", "100"];
        for sepia in &sepia_values {
            self.add_class(&format!("sepia-{}", sepia))?;
        }
        
        // Drop shadow utilities
        let drop_shadow_values = ["none", "sm", "0", "1", "2", "3", "xl", "2xl", "3xl"];
        for drop_shadow in &drop_shadow_values {
            self.add_class(&format!("drop-shadow-{}", drop_shadow))?;
        }
        
        Ok(())
    }
    
    /// Generate transition utilities (properties, duration, timing)
    pub fn generate_transition_utilities(&mut self) -> Result<()> {
        // Transition property utilities
        let transition_properties = ["none", "all", "colors", "opacity", "shadow", "transform"];
        for property in &transition_properties {
            self.add_class(&format!("transition-{}", property))?;
        }
        
        // Transition duration utilities
        let transition_durations = ["75", "100", "150", "200", "300", "500", "700", "1000"];
        for duration in &transition_durations {
            self.add_class(&format!("duration-{}", duration))?;
        }
        
        // Transition timing function utilities
        let transition_timing = ["linear", "in", "out", "in-out"];
        for timing in &transition_timing {
            self.add_class(&format!("ease-{}", timing))?;
        }
        
        // Transition delay utilities
        let transition_delays = ["75", "100", "150", "200", "300", "500", "700", "1000"];
        for delay in &transition_delays {
            self.add_class(&format!("delay-{}", delay))?;
        }
        
        Ok(())
    }
    
    /// Generate text shadow utilities
    pub fn generate_text_shadow_utilities(&mut self) -> Result<()> {
        let text_shadow_values = ["none", "sm", "0", "1", "2", "3", "xl", "2xl", "3xl"];
        for shadow in &text_shadow_values {
            self.add_class(&format!("text-shadow-{}", shadow))?;
        }
        
        Ok(())
    }
    
    /// Generate mask utilities
    pub fn generate_mask_utilities(&mut self) -> Result<()> {
        // Mask size utilities
        let mask_sizes = ["auto", "contain", "cover"];
        for size in &mask_sizes {
            self.add_class(&format!("mask-{}", size))?;
        }
        
        // Mask position utilities
        let mask_positions = [
            "bottom", "center", "left", "left-bottom", "left-top", "right", "right-bottom", "right-top", "top"
        ];
        for position in &mask_positions {
            self.add_class(&format!("mask-{}", position))?;
        }
        
        // Mask repeat utilities
        let mask_repeats = ["no-repeat", "repeat", "repeat-x", "repeat-y", "round", "space"];
        for repeat in &mask_repeats {
            self.add_class(&format!("mask-{}", repeat))?;
        }
        
        // Mask origin utilities
        let mask_origins = ["border", "padding", "content"];
        for origin in &mask_origins {
            self.add_class(&format!("mask-origin-{}", origin))?;
        }
        
        // Mask clip utilities
        let mask_clips = ["border", "padding", "content", "text"];
        for clip in &mask_clips {
            self.add_class(&format!("mask-clip-{}", clip))?;
        }
        
        Ok(())
    }
    
    /// Generate logical properties utilities
    pub fn generate_logical_properties_utilities(&mut self) -> Result<()> {
        // Logical border utilities
        let logical_borders = ["border-inline", "border-block", "border-inline-start", "border-inline-end", "border-block-start", "border-block-end"];
        for border in &logical_borders {
            self.add_class(border)?;
        }
        
        // Logical margin utilities
        let logical_margins = ["m-inline", "m-block", "m-inline-start", "m-inline-end", "m-block-start", "m-block-end"];
        for margin in &logical_margins {
            self.add_class(margin)?;
        }
        
        // Logical padding utilities
        let logical_paddings = ["p-inline", "p-block", "p-inline-start", "p-inline-end", "p-block-start", "p-block-end"];
        for padding in &logical_paddings {
            self.add_class(padding)?;
        }
        
        // Logical text alignment utilities
        let logical_text_alignments = ["text-inline-start", "text-inline-end", "text-block-start", "text-block-end"];
        for alignment in &logical_text_alignments {
            self.add_class(alignment)?;
        }
        
        Ok(())
    }
    
    /// Generate enhanced backdrop filter utilities
    pub fn generate_enhanced_backdrop_filter_utilities(&mut self) -> Result<()> {
        // Backdrop blur utilities
        let backdrop_blur_values = ["none", "sm", "0", "1", "2", "3", "xl", "2xl", "3xl"];
        for blur in &backdrop_blur_values {
            self.add_class(&format!("backdrop-blur-{}", blur))?;
        }
        
        // Backdrop brightness utilities
        let backdrop_brightness_values = ["0", "50", "75", "90", "95", "100", "105", "110", "125", "150", "200"];
        for brightness in &backdrop_brightness_values {
            self.add_class(&format!("backdrop-brightness-{}", brightness))?;
        }
        
        // Backdrop contrast utilities
        let backdrop_contrast_values = ["0", "50", "75", "100", "125", "150", "200"];
        for contrast in &backdrop_contrast_values {
            self.add_class(&format!("backdrop-contrast-{}", contrast))?;
        }
        
        // Backdrop grayscale utilities
        let backdrop_grayscale_values = ["0", "100"];
        for grayscale in &backdrop_grayscale_values {
            self.add_class(&format!("backdrop-grayscale-{}", grayscale))?;
        }
        
        // Backdrop hue rotate utilities
        let backdrop_hue_rotate_values = ["0", "15", "30", "60", "90", "180"];
        for hue_rotate in &backdrop_hue_rotate_values {
            self.add_class(&format!("backdrop-hue-rotate-{}", hue_rotate))?;
        }
        
        // Backdrop invert utilities
        let backdrop_invert_values = ["0", "100"];
        for invert in &backdrop_invert_values {
            self.add_class(&format!("backdrop-invert-{}", invert))?;
        }
        
        // Backdrop opacity utilities
        let backdrop_opacity_values = ["0", "5", "10", "20", "25", "30", "40", "50", "60", "70", "75", "80", "90", "95", "100"];
        for opacity in &backdrop_opacity_values {
            self.add_class(&format!("backdrop-opacity-{}", opacity))?;
        }
        
        // Backdrop saturate utilities
        let backdrop_saturate_values = ["0", "50", "100", "150", "200"];
        for saturate in &backdrop_saturate_values {
            self.add_class(&format!("backdrop-saturate-{}", saturate))?;
        }
        
        // Backdrop sepia utilities
        let backdrop_sepia_values = ["0", "100"];
        for sepia in &backdrop_sepia_values {
            self.add_class(&format!("backdrop-sepia-{}", sepia))?;
        }
        
        Ok(())
    }
    
    /// Generate modern CSS features utilities
    pub fn generate_modern_css_features_utilities(&mut self) -> Result<()> {
        // Cascade layer utilities
        let cascade_layers = ["base", "components", "utilities"];
        for layer in &cascade_layers {
            self.add_class(&format!("layer-{}", layer))?;
        }
        
        // Custom property utilities
        let custom_properties = ["primary", "secondary", "accent", "neutral", "base-100", "base-200", "base-300"];
        for property in &custom_properties {
            self.add_class(&format!("--{}", property))?;
        }
        
        // Container query utilities
        let container_queries = ["@container", "@container-sm", "@container-md", "@container-lg", "@container-xl"];
        for query in &container_queries {
            self.add_class(query)?;
        }
        
        // CSS nesting utilities
        let nesting_utilities = ["&", "&:hover", "&:focus", "&:active", "&:disabled"];
        for nesting in &nesting_utilities {
            self.add_class(nesting)?;
        }
        
        Ok(())
    }
    
    /// Generate device variant utilities
    pub fn generate_device_variant_utilities(&mut self) -> Result<()> {
        // Device-specific utilities
        let device_variants = [
            "mobile", "tablet", "desktop", "touch", "no-touch", "hover", "no-hover",
            "pointer-coarse", "pointer-fine", "pointer-none", "any-pointer-coarse", "any-pointer-fine", "any-pointer-none"
        ];
        
        for device in &device_variants {
            self.add_class(&format!("{}:", device))?;
        }
        
        Ok(())
    }
    
    /// Generate CSS nesting utilities
    pub fn generate_css_nesting_utilities(&mut self) -> Result<()> {
        // CSS nesting selectors
        let nesting_selectors = [
            "&", "&:hover", "&:focus", "&:active", "&:disabled", "&:checked", "&:indeterminate",
            "&:default", "&:required", "&:valid", "&:invalid", "&:in-range", "&:out-of-range",
            "&:placeholder-shown", "&:autofill", "&:read-only", "&:before", "&:after",
            "&:first-letter", "&:first-line", "&:marker", "&:selection", "&:file",
            "&:backdrop", "&:placeholder", "&:any-link", "&:link", "&:visited", "&:target",
            "&:scope", "&:current", "&:past", "&:future", "&:playing", "&:paused",
            "&:seeking", "&:buffering", "&:stalled", "&:muted", "&:volume-locked"
        ];
        
        for selector in &nesting_selectors {
            self.add_class(selector)?;
        }
        
        Ok(())
    }
    
    /// Generate advanced plugin system utilities
    pub fn generate_advanced_plugin_system_utilities(&mut self) -> Result<()> {
        // Plugin type utilities
        let plugin_types = ["utility", "component", "base", "variant"];
        for plugin_type in &plugin_types {
            self.add_class(&format!("plugin-{}", plugin_type))?;
        }
        
        // Plugin priority utilities
        let plugin_priorities = ["low", "normal", "high", "critical"];
        for priority in &plugin_priorities {
            self.add_class(&format!("plugin-priority-{}", priority))?;
        }
        
        // Plugin lifecycle utilities
        let plugin_lifecycles = ["initialize", "execute", "cleanup", "validate"];
        for lifecycle in &plugin_lifecycles {
            self.add_class(&format!("plugin-{}", lifecycle))?;
        }
        
        // Plugin composition utilities
        let plugin_compositions = ["merge", "extend", "override", "prepend", "append"];
        for composition in &plugin_compositions {
            self.add_class(&format!("plugin-{}", composition))?;
        }
        
        Ok(())
    }
    
    /// Generate enhanced validation utilities
    pub fn generate_enhanced_validation_utilities(&mut self) -> Result<()> {
        // Validation state utilities
        let validation_states = ["valid", "invalid", "pending", "required", "optional"];
        for state in &validation_states {
            self.add_class(&format!("validation-{}", state))?;
        }
        
        // Validation rule utilities
        let validation_rules = ["required", "pattern", "length", "range", "custom"];
        for rule in &validation_rules {
            self.add_class(&format!("validation-rule-{}", rule))?;
        }
        
        // Validation error utilities
        let validation_errors = ["error", "warning", "info", "success"];
        for error in &validation_errors {
            self.add_class(&format!("validation-{}", error))?;
        }
        
        Ok(())
    }
    
    /// Generate advanced performance optimization utilities
    pub fn generate_advanced_performance_optimization_utilities(&mut self) -> Result<()> {
        // Performance optimization utilities
        let performance_optimizations = [
            "will-change-auto", "will-change-scroll", "will-change-contents", "will-change-transform",
            "contain-none", "contain-strict", "contain-content", "contain-layout", "contain-paint", "contain-size",
            "isolation-auto", "isolation-isolate",
            "backface-visibility-visible", "backface-visibility-hidden",
            "perspective-none", "perspective-1000", "perspective-1500", "perspective-2000", "perspective-2500", "perspective-3000",
            "transform-gpu", "transform-cpu", "transform-3d", "transform-2d"
        ];
        
        for optimization in &performance_optimizations {
            self.add_class(optimization)?;
        }
        
        Ok(())
    }
    
    /// Generate container query utilities
    pub fn generate_container_query_utilities(&mut self) -> Result<()> {
        // Container query utilities
        let container_queries = [
            "@container", "@container-sm", "@container-md", "@container-lg", "@container-xl", "@container-2xl",
            "container-type-size", "container-type-inline-size", "container-type-normal",
            "container-name", "container-query"
        ];
        
        for query in &container_queries {
            self.add_class(query)?;
        }
        
        Ok(())
    }
    
    /// Generate color function utilities
    pub fn generate_color_function_utilities(&mut self) -> Result<()> {
        // Color function utilities
        let color_functions = [
            "rgb", "rgba", "hsl", "hsla", "hwb", "lab", "lch", "oklab", "oklch",
            "color-mix", "color-contrast", "color-scheme", "color-adjust"
        ];
        
        for function in &color_functions {
            self.add_class(&format!("color-{}", function))?;
        }
        
        Ok(())
    }
    
    /// Generate performance optimization utilities
    pub fn generate_performance_optimization_utilities(&mut self) -> Result<()> {
        // Performance optimization utilities
        let performance_utilities = [
            "optimize-speed", "optimize-quality", "optimize-auto",
            "will-change-auto", "will-change-scroll", "will-change-contents", "will-change-transform",
            "contain-none", "contain-strict", "contain-content", "contain-layout", "contain-paint", "contain-size"
        ];
        
        for utility in &performance_utilities {
            self.add_class(utility)?;
        }
        
        Ok(())
    }
    
    /// Generate advanced animation utilities
    pub fn generate_advanced_animation_utilities(&mut self) -> Result<()> {
        // Advanced animation utilities
        let advanced_animations = [
            "animate-bounce", "animate-ping", "animate-pulse", "animate-spin",
            "animate-ping-slow", "animate-ping-fast", "animate-pulse-slow", "animate-pulse-fast",
            "animate-spin-slow", "animate-spin-fast", "animate-bounce-slow", "animate-bounce-fast",
            "animate-fade-in", "animate-fade-out", "animate-slide-in", "animate-slide-out",
            "animate-zoom-in", "animate-zoom-out", "animate-rotate-in", "animate-rotate-out",
            "animate-scale-in", "animate-scale-out", "animate-flip-in", "animate-flip-out"
        ];
        
        for animation in &advanced_animations {
            self.add_class(animation)?;
        }
        
        Ok(())
    }
    
    /// Parse sizing classes (width, height, min/max dimensions)
    fn parse_sizing_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Width utilities
        if class.starts_with("w-") {
            let value = &class[2..];
            return Some(vec![CssProperty { 
                name: "width".to_string(), 
                value: self.parse_sizing_value(value), 
                important: false 
            }]);
        }
        
        // Height utilities
        if class.starts_with("h-") {
            let value = &class[2..];
            return Some(vec![CssProperty { 
                name: "height".to_string(), 
                value: self.parse_sizing_value(value), 
                important: false 
            }]);
        }
        
        // Min width utilities
        if class.starts_with("min-w-") {
            let value = &class[6..];
            return Some(vec![CssProperty { 
                name: "min-width".to_string(), 
                value: self.parse_sizing_value(value), 
                important: false 
            }]);
        }
        
        // Max width utilities
        if class.starts_with("max-w-") {
            let value = &class[6..];
            return Some(vec![CssProperty { 
                name: "max-width".to_string(), 
                value: self.parse_sizing_value(value), 
                important: false 
            }]);
        }
        
        // Min height utilities
        if class.starts_with("min-h-") {
            let value = &class[6..];
            return Some(vec![CssProperty { 
                name: "min-height".to_string(), 
                value: self.parse_sizing_value(value), 
                important: false 
            }]);
        }
        
        // Max height utilities
        if class.starts_with("max-h-") {
            let value = &class[6..];
            return Some(vec![CssProperty { 
                name: "max-height".to_string(), 
                value: self.parse_sizing_value(value), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse sizing values
    fn parse_sizing_value(&self, value: &str) -> String {
        match value {
            "0" => "0px".to_string(),
            "px" => "1px".to_string(),
            "auto" => "auto".to_string(),
            "full" => "100%".to_string(),
            "screen" => "100vh".to_string(),
            "min" => "min-content".to_string(),
            "max" => "max-content".to_string(),
            "fit" => "fit-content".to_string(),
            "1/2" => "50%".to_string(),
            "1/3" => "33.333333%".to_string(),
            "2/3" => "66.666667%".to_string(),
            "1/4" => "25%".to_string(),
            "3/4" => "75%".to_string(),
            "1/5" => "20%".to_string(),
            "2/5" => "40%".to_string(),
            "3/5" => "60%".to_string(),
            "4/5" => "80%".to_string(),
            _ => {
                // Handle numeric values
                if let Ok(num) = value.parse::<f64>() {
                    if num < 1.0 {
                        format!("{}rem", num * 0.25)
                    } else {
                        format!("{}rem", num * 0.25)
                    }
                } else {
                    value.to_string()
                }
            }
        }
    }
    
    /// Parse background classes
    fn parse_background_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("bg-") {
            let value = &class[3..];
            return Some(vec![CssProperty { 
                name: "background-size".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("bg-clip-") {
            let value = &class[8..];
            return Some(vec![CssProperty { 
                name: "background-clip".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("bg-origin-") {
            let value = &class[10..];
            return Some(vec![CssProperty { 
                name: "background-origin".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse filter classes
    fn parse_filter_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("blur-") {
            let value = &class[5..];
            return Some(vec![CssProperty { 
                name: "filter".to_string(), 
                value: format!("blur({})", self.parse_blur_value(value)), 
                important: false 
            }]);
        }
        
        if class.starts_with("brightness-") {
            let value = &class[11..];
            return Some(vec![CssProperty { 
                name: "filter".to_string(), 
                value: format!("brightness({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("contrast-") {
            let value = &class[9..];
            return Some(vec![CssProperty { 
                name: "filter".to_string(), 
                value: format!("contrast({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("grayscale-") {
            let value = &class[10..];
            return Some(vec![CssProperty { 
                name: "filter".to_string(), 
                value: format!("grayscale({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("hue-rotate-") {
            let value = &class[11..];
            return Some(vec![CssProperty { 
                name: "filter".to_string(), 
                value: format!("hue-rotate({}deg)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("invert-") {
            let value = &class[7..];
            return Some(vec![CssProperty { 
                name: "filter".to_string(), 
                value: format!("invert({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("saturate-") {
            let value = &class[9..];
            return Some(vec![CssProperty { 
                name: "filter".to_string(), 
                value: format!("saturate({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("sepia-") {
            let value = &class[6..];
            return Some(vec![CssProperty { 
                name: "filter".to_string(), 
                value: format!("sepia({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("drop-shadow-") {
            let value = &class[12..];
            return Some(vec![CssProperty { 
                name: "filter".to_string(), 
                value: format!("drop-shadow({})", self.parse_drop_shadow_value(value)), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse blur values
    fn parse_blur_value(&self, value: &str) -> String {
        match value {
            "none" => "0px".to_string(),
            "sm" => "4px".to_string(),
            "0" => "0px".to_string(),
            "1" => "4px".to_string(),
            "2" => "8px".to_string(),
            "3" => "12px".to_string(),
            "xl" => "24px".to_string(),
            "2xl" => "40px".to_string(),
            "3xl" => "64px".to_string(),
            _ => format!("{}px", value),
        }
    }
    
    /// Parse drop shadow values
    fn parse_drop_shadow_value(&self, value: &str) -> String {
        match value {
            "none" => "0 0 #0000".to_string(),
            "sm" => "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
            "0" => "0 0 #0000".to_string(),
            "1" => "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(),
            "2" => "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
            "3" => "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(),
            "xl" => "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string(),
            "2xl" => "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(),
            "3xl" => "0 35px 60px -12px rgb(0 0 0 / 0.3)".to_string(),
            _ => format!("0 0 #0000"),
        }
    }
    
    /// Parse transition classes
    fn parse_transition_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("transition-") {
            let value = &class[11..];
            return Some(vec![CssProperty { 
                name: "transition-property".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("duration-") {
            let value = &class[9..];
            return Some(vec![CssProperty { 
                name: "transition-duration".to_string(), 
                value: format!("{}ms", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("ease-") {
            let value = &class[5..];
            return Some(vec![CssProperty { 
                name: "transition-timing-function".to_string(), 
                value: self.parse_ease_value(value), 
                important: false 
            }]);
        }
        
        if class.starts_with("delay-") {
            let value = &class[6..];
            return Some(vec![CssProperty { 
                name: "transition-delay".to_string(), 
                value: format!("{}ms", value), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse ease values
    fn parse_ease_value(&self, value: &str) -> String {
        match value {
            "linear" => "linear".to_string(),
            "in" => "cubic-bezier(0.4, 0, 1, 1)".to_string(),
            "out" => "cubic-bezier(0, 0, 0.2, 1)".to_string(),
            "in-out" => "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
            _ => value.to_string(),
        }
    }
    
    /// Parse text shadow classes
    fn parse_text_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("text-shadow-") {
            let value = &class[12..];
            return Some(vec![CssProperty { 
                name: "text-shadow".to_string(), 
                value: self.parse_text_shadow_value(value), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse text shadow values
    fn parse_text_shadow_value(&self, value: &str) -> String {
        match value {
            "none" => "none".to_string(),
            "sm" => "0 1px 2px rgb(0 0 0 / 0.05)".to_string(),
            "0" => "none".to_string(),
            "1" => "0 1px 3px rgb(0 0 0 / 0.1), 0 1px 2px rgb(0 0 0 / 0.1)".to_string(),
            "2" => "0 4px 6px rgb(0 0 0 / 0.1), 0 2px 4px rgb(0 0 0 / 0.1)".to_string(),
            "3" => "0 10px 15px rgb(0 0 0 / 0.1), 0 4px 6px rgb(0 0 0 / 0.1)".to_string(),
            "xl" => "0 20px 25px rgb(0 0 0 / 0.1), 0 8px 10px rgb(0 0 0 / 0.1)".to_string(),
            "2xl" => "0 25px 50px rgb(0 0 0 / 0.25)".to_string(),
            "3xl" => "0 35px 60px rgb(0 0 0 / 0.3)".to_string(),
            _ => "none".to_string(),
        }
    }
    
    /// Parse mask classes
    fn parse_mask_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("mask-") {
            let value = &class[5..];
            return Some(vec![CssProperty { 
                name: "mask-size".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("mask-origin-") {
            let value = &class[12..];
            return Some(vec![CssProperty { 
                name: "mask-origin".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("mask-clip-") {
            let value = &class[10..];
            return Some(vec![CssProperty { 
                name: "mask-clip".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse logical properties classes
    fn parse_logical_properties_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("border-inline") {
            return Some(vec![CssProperty { 
                name: "border-inline".to_string(), 
                value: "1px solid".to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("border-block") {
            return Some(vec![CssProperty { 
                name: "border-block".to_string(), 
                value: "1px solid".to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("m-inline") {
            return Some(vec![CssProperty { 
                name: "margin-inline".to_string(), 
                value: "0.25rem".to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("m-block") {
            return Some(vec![CssProperty { 
                name: "margin-block".to_string(), 
                value: "0.25rem".to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("p-inline") {
            return Some(vec![CssProperty { 
                name: "padding-inline".to_string(), 
                value: "0.25rem".to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("p-block") {
            return Some(vec![CssProperty { 
                name: "padding-block".to_string(), 
                value: "0.25rem".to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("text-inline-start") {
            return Some(vec![CssProperty { 
                name: "text-align".to_string(), 
                value: "start".to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("text-inline-end") {
            return Some(vec![CssProperty { 
                name: "text-align".to_string(), 
                value: "end".to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("text-block-start") {
            return Some(vec![CssProperty { 
                name: "text-align".to_string(), 
                value: "start".to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("text-block-end") {
            return Some(vec![CssProperty { 
                name: "text-align".to_string(), 
                value: "end".to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse enhanced backdrop filter classes
    fn parse_enhanced_backdrop_filter_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("backdrop-blur-") {
            let value = &class[14..];
            return Some(vec![CssProperty { 
                name: "backdrop-filter".to_string(), 
                value: format!("blur({})", self.parse_blur_value(value)), 
                important: false 
            }]);
        }
        
        if class.starts_with("backdrop-brightness-") {
            let value = &class[19..];
            return Some(vec![CssProperty { 
                name: "backdrop-filter".to_string(), 
                value: format!("brightness({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("backdrop-contrast-") {
            let value = &class[17..];
            return Some(vec![CssProperty { 
                name: "backdrop-filter".to_string(), 
                value: format!("contrast({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("backdrop-grayscale-") {
            let value = &class[18..];
            return Some(vec![CssProperty { 
                name: "backdrop-filter".to_string(), 
                value: format!("grayscale({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("backdrop-hue-rotate-") {
            let value = &class[19..];
            return Some(vec![CssProperty { 
                name: "backdrop-filter".to_string(), 
                value: format!("hue-rotate({}deg)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("backdrop-invert-") {
            let value = &class[16..];
            return Some(vec![CssProperty { 
                name: "backdrop-filter".to_string(), 
                value: format!("invert({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("backdrop-opacity-") {
            let value = &class[17..];
            return Some(vec![CssProperty { 
                name: "backdrop-filter".to_string(), 
                value: format!("opacity({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("backdrop-saturate-") {
            let value = &class[18..];
            return Some(vec![CssProperty { 
                name: "backdrop-filter".to_string(), 
                value: format!("saturate({}%)", value), 
                important: false 
            }]);
        }
        
        if class.starts_with("backdrop-sepia-") {
            let value = &class[15..];
            return Some(vec![CssProperty { 
                name: "backdrop-filter".to_string(), 
                value: format!("sepia({}%)", value), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse modern CSS features classes
    fn parse_modern_css_features_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("layer-") {
            let value = &class[6..];
            return Some(vec![CssProperty { 
                name: "layer".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("--") {
            let value = &class[2..];
            return Some(vec![CssProperty { 
                name: "custom-property".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("@container") {
            return Some(vec![CssProperty { 
                name: "container-query".to_string(), 
                value: "true".to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse device variant classes
    fn parse_device_variant_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.ends_with(":") {
            let device = &class[..class.len()-1];
            return Some(vec![CssProperty { 
                name: "device-variant".to_string(), 
                value: device.to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse CSS nesting classes
    fn parse_css_nesting_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("&") {
            return Some(vec![CssProperty { 
                name: "nesting-selector".to_string(), 
                value: class.to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse advanced plugin system classes
    fn parse_advanced_plugin_system_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("plugin-") {
            let value = &class[7..];
            return Some(vec![CssProperty { 
                name: "plugin-type".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("plugin-priority-") {
            let value = &class[16..];
            return Some(vec![CssProperty { 
                name: "plugin-priority".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse enhanced validation classes
    fn parse_enhanced_validation_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("validation-") {
            let value = &class[11..];
            return Some(vec![CssProperty { 
                name: "validation-state".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("validation-rule-") {
            let value = &class[15..];
            return Some(vec![CssProperty { 
                name: "validation-rule".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse advanced performance optimization classes
    fn parse_advanced_performance_optimization_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("will-change-") {
            let value = &class[11..];
            return Some(vec![CssProperty { 
                name: "will-change".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("contain-") {
            let value = &class[8..];
            return Some(vec![CssProperty { 
                name: "contain".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("isolation-") {
            let value = &class[10..];
            return Some(vec![CssProperty { 
                name: "isolation".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("backface-visibility-") {
            let value = &class[19..];
            return Some(vec![CssProperty { 
                name: "backface-visibility".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("perspective-") {
            let value = &class[12..];
            return Some(vec![CssProperty { 
                name: "perspective".to_string(), 
                value: self.parse_perspective_value(value), 
                important: false 
            }]);
        }
        
        if class.starts_with("transform-") {
            let value = &class[10..];
            return Some(vec![CssProperty { 
                name: "transform".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse perspective values
    fn parse_perspective_value(&self, value: &str) -> String {
        match value {
            "none" => "none".to_string(),
            "1000" => "1000px".to_string(),
            "1500" => "1500px".to_string(),
            "2000" => "2000px".to_string(),
            "2500" => "2500px".to_string(),
            "3000" => "3000px".to_string(),
            _ => format!("{}px", value),
        }
    }
    
    /// Parse container query classes
    fn parse_container_query_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("@container") {
            return Some(vec![CssProperty { 
                name: "container-query".to_string(), 
                value: "true".to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("container-type-") {
            let value = &class[15..];
            return Some(vec![CssProperty { 
                name: "container-type".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("container-name") {
            return Some(vec![CssProperty { 
                name: "container-name".to_string(), 
                value: "true".to_string(), 
                important: false 
            }]);
        }
        
        if class == "container-query" {
            return Some(vec![CssProperty { 
                name: "container-query".to_string(), 
                value: "true".to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse color function classes
    fn parse_color_function_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("color-") {
            let value = &class[6..];
            return Some(vec![CssProperty { 
                name: "color-function".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse performance optimization classes
    fn parse_performance_optimization_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("optimize-") {
            let value = &class[9..];
            return Some(vec![CssProperty { 
                name: "optimization".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("will-change-") {
            let value = &class[11..];
            return Some(vec![CssProperty { 
                name: "will-change".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        if class.starts_with("contain-") {
            let value = &class[8..];
            return Some(vec![CssProperty { 
                name: "contain".to_string(), 
                value: value.to_string(), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse advanced animation classes
    fn parse_advanced_animation_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if class.starts_with("animate-") {
            let value = &class[7..];
            return Some(vec![CssProperty { 
                name: "animation".to_string(), 
                value: self.parse_animation_value(value), 
                important: false 
            }]);
        }
        
        None
    }
    
    /// Parse animation values
    fn parse_animation_value(&self, value: &str) -> String {
        match value {
            "bounce" => "bounce 1s infinite".to_string(),
            "ping" => "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
            "pulse" => "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
            "spin" => "spin 1s linear infinite".to_string(),
            "ping-slow" => "ping 2s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
            "ping-fast" => "ping 0.5s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
            "pulse-slow" => "pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
            "pulse-fast" => "pulse 1s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
            "spin-slow" => "spin 2s linear infinite".to_string(),
            "spin-fast" => "spin 0.5s linear infinite".to_string(),
            "bounce-slow" => "bounce 2s infinite".to_string(),
            "bounce-fast" => "bounce 0.5s infinite".to_string(),
            "fade-in" => "fadeIn 0.5s ease-in".to_string(),
            "fade-out" => "fadeOut 0.5s ease-out".to_string(),
            "slide-in" => "slideIn 0.5s ease-in".to_string(),
            "slide-out" => "slideOut 0.5s ease-out".to_string(),
            "zoom-in" => "zoomIn 0.5s ease-in".to_string(),
            "zoom-out" => "zoomOut 0.5s ease-out".to_string(),
            "rotate-in" => "rotateIn 0.5s ease-in".to_string(),
            "rotate-out" => "rotateOut 0.5s ease-out".to_string(),
            "scale-in" => "scaleIn 0.5s ease-in".to_string(),
            "scale-out" => "scaleOut 0.5s ease-out".to_string(),
            "flip-in" => "flipIn 0.5s ease-in".to_string(),
            "flip-out" => "flipOut 0.5s ease-out".to_string(),
            _ => value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_generator_creation() {
        let generator = CssGenerator::new();
        assert_eq!(generator.rule_count(), 0);
        assert!(!generator.breakpoints.is_empty());
    }

    #[test]
    fn test_add_class() {
        let mut generator = CssGenerator::new();
        generator.add_class("p-4").unwrap();
        
        assert_eq!(generator.rule_count(), 1);
        let rules = generator.get_rules();
        assert!(rules.contains_key("p-4"));
    }

    #[test]
    fn test_generate_css() {
        let mut generator = CssGenerator::new();
        generator.add_class("p-4").unwrap();
        generator.add_class("bg-blue-500").unwrap();
        
        let css = generator.generate_css();
        assert!(css.contains(".p-4"));
        assert!(css.contains("padding: 1rem"));
        assert!(css.contains(".bg-blue-500"));
        assert!(css.contains("background-color: #3b82f6"));
    }

    #[test]
    fn test_responsive_class() {
        let mut generator = CssGenerator::new();
        generator.add_responsive_class(Breakpoint::Md, "p-4").unwrap();
        
        let css = generator.generate_css();
        assert!(css.contains("@media (min-width: 768px)"));
        assert!(css.contains("md:p-4"));
    }

    #[test]
    fn test_custom_properties() {
        let mut generator = CssGenerator::new();
        generator.add_custom_property("primary-color", "#3b82f6");
        
        let css = generator.generate_css();
        assert!(css.contains(":root"));
        assert!(css.contains("--primary-color: #3b82f6"));
    }

    #[test]
    fn test_minified_css() {
        let mut generator = CssGenerator::new();
        generator.add_class("p-4").unwrap();
        
        let minified = generator.generate_minified_css();
        assert!(!minified.contains('\n'));
        assert!(!minified.contains(' '));
    }

    #[test]
    fn test_unknown_class() {
        let mut generator = CssGenerator::new();
        let result = generator.add_class("unknown-class");
        assert!(result.is_err());
    }
}
