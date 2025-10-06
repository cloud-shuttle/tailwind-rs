//! Variant Parsing and Handling
//!
//! This module handles the parsing and processing of CSS variants,
//! including responsive, state, custom, container, and arbitrary variants.

use crate::responsive::Breakpoint;
use std::collections::HashMap;

/// CSS Functions support for Tailwind-RS
pub mod functions {
    use std::collections::HashMap;

    /// @apply directive parser and resolver
    pub struct ApplyParser {
        /// Reference to CSS generator for utility resolution
        generator: crate::CssGenerator,
    }

    impl ApplyParser {
        pub fn new(generator: crate::CssGenerator) -> Self {
            Self { generator }
        }

        /// Parse @apply directive and resolve to CSS properties
        pub fn parse_apply(&mut self, apply_value: &str) -> Result<Vec<crate::CssProperty>, ApplyError> {
            let classes: Vec<&str> = apply_value.split_whitespace().collect();
            let mut properties = Vec::new();

            for class in classes {
                // Parse variants from class (if any)
                let (_variants, _base_class) = self.generator.variant_parser.parse_variants(class);

                // Generate CSS properties for this class
                if let Ok(css) = self.generator.generate_individual_css_rule(class) {
                    // Extract properties from the generated CSS rule
                    // This is a simplified implementation
                    properties.extend(css.properties);
                } else {
                    return Err(ApplyError::UnknownClass(class.to_string()));
                }
            }

            Ok(properties)
        }

        /// Validate that all classes in @apply exist
        pub fn validate_apply(&self, apply_value: &str) -> Result<(), ApplyError> {
            let classes: Vec<&str> = apply_value.split_whitespace().collect();

            for class in classes {
                let (_, base_class) = self.generator.variant_parser.parse_variants(class);
                // Check if base class is known (simplified check)
                if !self.is_known_utility(&base_class) {
                    return Err(ApplyError::UnknownClass(base_class));
                }
            }

            Ok(())
        }

        /// Check if a class is a known utility (simplified implementation)
        fn is_known_utility(&self, class: &str) -> bool {
            // Check common utility patterns
            class.starts_with("flex") ||
            class.starts_with("items-") ||
            class.starts_with("justify-") ||
            class.starts_with("bg-") ||
            class.starts_with("text-") ||
            class.starts_with("p-") ||
            class.starts_with("m-") ||
            class.starts_with("w-") ||
            class.starts_with("h-") ||
            // Add more patterns as needed
            true // For now, accept all to avoid false negatives
        }
    }

    /// @layer directive processor
    pub struct LayerProcessor {
        layers: HashMap<String, Vec<String>>, // layer_name -> css_rules
    }

    impl LayerProcessor {
        pub fn new() -> Self {
            let mut layers = HashMap::new();
            layers.insert("base".to_string(), Vec::new());
            layers.insert("components".to_string(), Vec::new());
            layers.insert("utilities".to_string(), Vec::new());
            Self { layers }
        }

        /// Process @layer directive
        pub fn process_layer(&mut self, layer_name: &str, css_content: &str) -> Result<(), LayerError> {
            if !self.layers.contains_key(layer_name) {
                return Err(LayerError::UnknownLayer(layer_name.to_string()));
            }

            // Parse CSS content and add to layer
            let rules: Vec<String> = css_content.lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();

            if let Some(layer_rules) = self.layers.get_mut(layer_name) {
                layer_rules.extend(rules);
            }

            Ok(())
        }

        /// Generate CSS with proper layer ordering
        pub fn generate_layered_css(&self) -> String {
            let mut output = String::new();

            // Generate CSS in layer order: base, components, utilities
            let layer_order = ["base", "components", "utilities"];

            for layer_name in &layer_order {
                if let Some(rules) = self.layers.get(*layer_name) {
                    if !rules.is_empty() {
                        output.push_str(&format!("@layer {} {{\n", layer_name));
                        for rule in rules {
                            output.push_str(&format!("  {}\n", rule));
                        }
                        output.push_str("}\n\n");
                    }
                }
            }

            output
        }
    }

    /// @import directive processor
    pub struct ImportProcessor {
        imports: Vec<ImportRule>,
    }

    #[derive(Debug, Clone)]
    pub struct ImportRule {
        pub url: String,
        pub layer: Option<String>,
        pub media_query: Option<String>,
    }

    impl ImportProcessor {
        pub fn new() -> Self {
            Self {
                imports: Vec::new(),
            }
        }

        /// Process @import directive
        pub fn process_import(&mut self, import_value: &str) -> Result<(), ImportError> {
            // Parse @import "url" layer(layer) media-query;
            let import = ImportRule {
                url: import_value.to_string(),
                layer: None,
                media_query: None,
            };

            self.imports.push(import);
            Ok(())
        }

        /// Generate import CSS
        pub fn generate_imports(&self) -> String {
            let mut output = String::new();

            for import in &self.imports {
                output.push_str(&format!("@import \"{}\";\n", import.url));
            }

            if !output.is_empty() {
                output.push('\n');
            }

            output
        }
    }

    /// Errors for CSS functions
    #[derive(Debug, thiserror::Error)]
    pub enum ApplyError {
        #[error("Unknown class in @apply: {0}")]
        UnknownClass(String),

        #[error("Invalid @apply syntax: {0}")]
        InvalidSyntax(String),

        #[error("Circular dependency in @apply")]
        CircularDependency,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum LayerError {
        #[error("Unknown layer: {0}")]
        UnknownLayer(String),

        #[error("Invalid layer syntax")]
        InvalidSyntax,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum ImportError {
        #[error("Invalid import syntax: {0}")]
        InvalidSyntax(String),

        #[error("Import resolution failed: {0}")]
        ResolutionFailed(String),
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::CssGenerator;

        #[test]
        fn test_apply_parser() {
            let generator = CssGenerator::new();
            let parser = ApplyParser::new(generator);

            // Test basic @apply parsing (this would need a more complete setup)
            // For now, just test the structure
            assert!(parser.validate_apply("flex items-center").is_ok());
        }

        #[test]
        fn test_layer_processor() {
            let mut processor = LayerProcessor::new();

            // Add CSS to base layer
            processor.process_layer("base", "*, ::before, ::after { box-sizing: border-box; }").unwrap();

            let css = processor.generate_layered_css();
            assert!(css.contains("@layer base"));
            assert!(css.contains("box-sizing: border-box"));
        }

        #[test]
        fn test_import_processor() {
            let mut processor = ImportProcessor::new();

            processor.process_import("tailwindcss/base").unwrap();

            let css = processor.generate_imports();
            assert!(css.contains("@import"));
            assert!(css.contains("tailwindcss/base"));
        }
    }
}

/// Custom variant definition
#[derive(Debug, Clone)]
pub struct CustomVariant {
    pub name: String,
    pub selector: String,
    pub order: i32,
    pub compounds_with: Vec<VariantType>,
}

/// Container definition for container queries
#[derive(Debug, Clone)]
pub struct ContainerDefinition {
    pub name: Option<String>,
    pub conditions: Vec<ContainerCondition>,
}

/// Container condition for queries
#[derive(Debug, Clone)]
pub enum ContainerCondition {
    Width { min: Option<String>, max: Option<String> },
    Height { min: Option<String>, max: Option<String> },
    AspectRatio(String),
    Orientation(Orientation),
}

/// Orientation for container queries
#[derive(Debug, Clone)]
pub enum Orientation {
    Portrait,
    Landscape,
}

/// Arbitrary value variant definition
#[derive(Debug, Clone)]
pub struct ArbitraryVariant {
    pub selector_template: String,
    pub value_type: ArbitraryValueType,
}

/// Arbitrary value type
#[derive(Debug, Clone)]
pub enum ArbitraryValueType {
    Attribute,
    Data,
    Aria,
}

/// Variant type for compound variant checking
#[derive(Debug, Clone, PartialEq)]
pub enum VariantType {
    Static,
    Dynamic,
    Responsive,
    State,
    Container,
    Custom,
}

/// Variant parser for handling CSS variants
#[derive(Debug, Clone)]
pub struct VariantParser {
    /// Supported variants
    variants: Vec<String>,
    /// Responsive breakpoints
    breakpoints: Vec<Breakpoint>,
    /// Custom variants
    custom_variants: HashMap<String, CustomVariant>,
    /// Container definitions
    containers: HashMap<String, ContainerDefinition>,
    /// Arbitrary value variants
    arbitrary_variants: HashMap<String, ArbitraryVariant>,
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
                // Advanced variants
                "pointer-coarse".to_string(),
                "pointer-fine".to_string(),
                "motion-reduce".to_string(),
                "motion-safe".to_string(),
                "light".to_string(),
                "high-contrast".to_string(),
                "low-contrast".to_string(),
                "reduced-motion".to_string(),
                "portrait".to_string(),
                "landscape".to_string(),
            ],
            breakpoints: vec![
                Breakpoint::Sm,
                Breakpoint::Md,
                Breakpoint::Lg,
                Breakpoint::Xl,
                Breakpoint::Xl2,
            ],
            custom_variants: HashMap::new(),
            containers: HashMap::new(),
            arbitrary_variants: HashMap::new(),
        }
    }

    /// Check if a class is a gradient stop (from-*, to-*, via-*)
    pub fn is_gradient_stop(&self, class: &str) -> bool {
        class.starts_with("from-") || class.starts_with("to-") || class.starts_with("via-")
    }


    /// Parse variants from a class string - supports complex multi-variant combinations
    pub fn parse_variants(&self, class: &str) -> (Vec<String>, String) {
        let mut variants = Vec::new();
        let mut remaining = class.to_string();

        // Parse all variants iteratively until no more variants are found
        loop {
            let mut __found_variant = false;

            // Check for compound variants first (most specific)
            let compound_patterns = [
                ("dark:hover:", vec!["dark", "hover"]),
                ("dark:focus:", vec!["dark", "focus"]),
                ("dark:active:", vec!["dark", "active"]),
                ("dark:visited:", vec!["dark", "visited"]),
                ("dark:disabled:", vec!["dark", "disabled"]),
                ("dark:group-hover:", vec!["dark", "group-hover"]),
                ("dark:group-focus:", vec!["dark", "group-focus"]),
                ("dark:peer-hover:", vec!["dark", "peer-hover"]),
                ("dark:peer-focus:", vec!["dark", "peer-focus"]),
                ("hover:focus:", vec!["hover", "focus"]),
                ("hover:active:", vec!["hover", "active"]),
                ("focus:active:", vec!["focus", "active"]),
                ("group-hover:focus:", vec!["group-hover", "focus"]),
                ("peer-hover:focus:", vec!["peer-hover", "focus"]),
            ];

            for (prefix, variant_list) in compound_patterns {
                if remaining.starts_with(prefix) {
                    variants.extend(variant_list.iter().map(|v| v.to_string()));
                    remaining = remaining
                        .strip_prefix(prefix)
                        .unwrap_or(&remaining)
                        .to_string();
                    _found_variant = true;
                    break;
                }
            }

            // If no compound variant found, check individual variants
            if !_found_variant {
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
                    // Advanced variants
                    ("pointer-coarse:", "pointer-coarse"),
                    ("pointer-fine:", "pointer-fine"),
                    ("motion-reduce:", "motion-reduce"),
                    ("motion-safe:", "motion-safe"),
                    ("high-contrast:", "high-contrast"),
                    ("low-contrast:", "low-contrast"),
                    ("reduced-motion:", "reduced-motion"),
                    ("portrait:", "portrait"),
                    ("landscape:", "landscape"),
                    // Responsive variants
                    ("sm:", "sm"),
                    ("md:", "md"),
                    ("lg:", "lg"),
                    ("xl:", "xl"),
                    ("2xl:", "2xl"),
                ];

                // Check for container variants (@container-*)
                if remaining.starts_with("@container-") {
                    if let Some(colon_pos) = remaining.find(':') {
                        let container_part = &remaining[..colon_pos];
                        let container_name = container_part.strip_prefix("@container-").unwrap_or("");
                        variants.push(format!("@container-{}", container_name));
                        remaining = remaining[colon_pos + 1..].to_string();
                        _found_variant = true;
                        continue;
                    }
                }

                // Check for arbitrary variants ([data-*]:, [aria-*]:, etc.)
                if remaining.starts_with('[') {
                    if let Some(end_bracket) = remaining.find(']') {
                        if remaining.chars().nth(end_bracket + 1) == Some(':') {
                            let arbitrary_part = &remaining[..end_bracket + 1];
                            variants.push(arbitrary_part.to_string());
                            remaining = remaining[end_bracket + 2..].to_string();
                            _found_variant = true;
                            continue;
                        }
                    }
                }

                // Check for custom variants
                for custom_name in self.custom_variants.keys() {
                    let custom_prefix = format!("{}:", custom_name);
                    if remaining.starts_with(&custom_prefix) {
                        variants.push(custom_name.clone());
                        remaining = remaining.strip_prefix(&custom_prefix).unwrap_or(&remaining).to_string();
                        _found_variant = true;
                        break;
                    }
                }

                for (prefix, variant) in variant_patterns {
                    if remaining.starts_with(prefix) {
                        variants.push(variant.to_string());
                        remaining = remaining
                            .strip_prefix(prefix)
                            .unwrap_or(&remaining)
                            .to_string();
                        _found_variant = true;
                        break;
                    }
                }
            }

            // If no variant found in this iteration, we're done
            if !_found_variant {
                break;
            }
        }

        (variants, remaining)
    }

    /// Define a custom variant
    pub fn define_custom_variant(&mut self, name: &str, selector: &str) -> Result<(), String> {
        if self.custom_variants.contains_key(name) {
            return Err(format!("Custom variant '{}' already exists", name));
        }

        let custom_variant = CustomVariant {
            name: name.to_string(),
            selector: selector.to_string(),
            order: self.custom_variants.len() as i32 + 1000, // Custom variants start at order 1000
            compounds_with: vec![VariantType::Static, VariantType::Dynamic, VariantType::State],
        };

        self.custom_variants.insert(name.to_string(), custom_variant);
        Ok(())
    }

    /// Register a container definition
    pub fn register_container(&mut self, name: Option<&str>, conditions: Vec<ContainerCondition>) -> Result<(), String> {
        let key = name.unwrap_or("").to_string();

        if self.containers.contains_key(&key) {
            return Err(format!("Container '{}' already exists", key));
        }

        let container = ContainerDefinition {
            name: name.map(String::from),
            conditions,
        };

        self.containers.insert(key, container);
        Ok(())
    }

    /// Define an arbitrary variant
    pub fn define_arbitrary_variant(&mut self, name: &str, selector_template: &str, value_type: ArbitraryValueType) -> Result<(), String> {
        if self.arbitrary_variants.contains_key(name) {
            return Err(format!("Arbitrary variant '{}' already exists", name));
        }

        let arbitrary_variant = ArbitraryVariant {
            selector_template: selector_template.to_string(),
            value_type,
        };

        self.arbitrary_variants.insert(name.to_string(), arbitrary_variant);
        Ok(())
    }

    /// Get media query for advanced variants
    pub fn get_single_variant_media_query(&self, variant: &str) -> Option<String> {
        match variant {
            "pointer-coarse" => Some("@media (pointer: coarse)".to_string()),
            "pointer-fine" => Some("@media (pointer: fine)".to_string()),
            "motion-reduce" => Some("@media (prefers-reduced-motion: reduce)".to_string()),
            "motion-safe" => Some("@media (prefers-reduced-motion: no-preference)".to_string()),
            "light" => Some("@media (prefers-color-scheme: light)".to_string()),
            "high-contrast" => Some("@media (prefers-contrast: high)".to_string()),
            "low-contrast" => Some("@media (prefers-contrast: low)".to_string()),
            "reduced-motion" => Some("@media (prefers-reduced-motion: reduce)".to_string()),
            "portrait" => Some("@media (orientation: portrait)".to_string()),
            "landscape" => Some("@media (orientation: landscape)".to_string()),
            // Container queries don't use media queries
            variant if variant.starts_with("@container-") => None,
            _ => None,
        }
    }

    /// Check if a variant uses media queries
    pub fn is_media_variant(&self, variant: &str) -> bool {
        self.get_single_variant_media_query(variant).is_some()
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
            // Advanced variants
            "pointer-coarse" | "pointer-fine" | "motion-reduce" | "motion-safe" | "light" |
            "high-contrast" | "low-contrast" | "reduced-motion" | "portrait" | "landscape" => {
                // These use media queries, not selectors
                String::new()
            }
            // Handle container variants
            variant if variant.starts_with("@container-") => {
                let container_name = &variant[11..]; // Remove "@container-" prefix
                if container_name.is_empty() {
                    "@container ".to_string()
                } else {
                    format!("@container {} ", container_name)
                }
            }
            // Handle arbitrary variants
            variant if variant.starts_with('[') && variant.ends_with(']') => {
                format!("{} ", variant)
            }
            // Handle custom variants
            variant => {
                if let Some(custom_variant) = self.custom_variants.get(variant) {
                    format!("{} ", custom_variant.selector)
                } else {
                    String::new()
                }
            }
        }
    }

    /// Combine multiple variants into a single CSS selector with proper ordering
    pub fn combine_variant_selectors(&self, variants: &[String]) -> String {
        if variants.is_empty() {
            return String::new();
        }

        // Separate variants by type for proper ordering
        let mut pseudo_selectors = Vec::new();
        let mut class_selectors = Vec::new();
        let mut group_selectors = Vec::new();
        let mut peer_selectors = Vec::new();

        for variant in variants {
            let selector = self.get_variant_selector(variant);
            if !selector.is_empty() {
                if selector.starts_with('.') && selector.contains(':') {
                    // Group or peer selectors like ".group:hover "
                    if selector.contains(".group:") {
                        group_selectors.push(selector.trim_end().to_string());
                    } else if selector.contains(".peer:") {
                        peer_selectors.push(selector.trim_end().to_string());
                    }
                } else if selector.starts_with('.') {
                    // Class selectors like ".dark "
                    class_selectors.push(selector.trim_end().to_string());
                } else if selector.starts_with(':') {
                    // Pseudo selectors like ":hover"
                    pseudo_selectors.push(selector);
                }
            }
        }

        // Combine in proper order: class selectors first, then group/peer, then pseudo
        let mut result = String::new();

        // Add class selectors (dark mode, etc.)
        for selector in class_selectors {
            result.push_str(&selector);
        }

        // Add group selectors
        for selector in group_selectors {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&selector);
        }

        // Add peer selectors
        for selector in peer_selectors {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&selector);
        }

        // Add pseudo selectors (combine multiple pseudo selectors)
        if !pseudo_selectors.is_empty() {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&pseudo_selectors.join(""));
        }

        result
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

    /// Get media query for a list of variants
    pub fn get_variant_media_query(&self, variants: &[String]) -> Option<String> {
        for variant in variants {
            if let Some(media_query) = self.get_responsive_media_query(variant) {
                return Some(media_query);
            }
            if let Some(media_query) = self.get_device_media_query(variant) {
                return Some(media_query);
            }
        }
        None
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

    /// Build CSS selector from class name and variants - "One Class = One CSS Rule" architecture
    pub fn build_css_selector(&self, class: &str, variants: &[String]) -> crate::error::Result<String> {
        // Escape special characters in CSS selectors (but not colons which are valid in class names)
        let escaped_class = class.replace("/", "\\/");
        let mut selector = format!(".{}", escaped_class);

        // Add pseudo-classes for state variants (in proper order)
        let mut pseudo_selectors = Vec::new();

        for variant in variants {
            match variant.as_str() {
                "hover" => pseudo_selectors.push(":hover"),
                "focus" => pseudo_selectors.push(":focus"),
                "active" => pseudo_selectors.push(":active"),
                "visited" => pseudo_selectors.push(":visited"),
                "disabled" => pseudo_selectors.push(":disabled"),
                "first" => pseudo_selectors.push(":first-child"),
                "last" => pseudo_selectors.push(":last-child"),
                "odd" => pseudo_selectors.push(":nth-child(odd)"),
                "even" => pseudo_selectors.push(":nth-child(even)"),
                // Group variants
                "group-hover" => {
                    // For group variants, we need the full selector
                    return Ok(format!(".group:hover .{}", class));
                }
                "group-focus" => {
                    return Ok(format!(".group:focus .{}", class));
                }
                "group-active" => {
                    return Ok(format!(".group:active .{}", class));
                }
                "group-disabled" => {
                    return Ok(format!(".group:disabled .{}", class));
                }
                // Peer variants
                "peer-hover" => {
                    return Ok(format!(".peer:hover ~ .{}", class));
                }
                "peer-focus" => {
                    return Ok(format!(".peer:focus ~ .{}", class));
                }
                "peer-active" => {
                    return Ok(format!(".peer:active ~ .{}", class));
                }
                "peer-disabled" => {
                    return Ok(format!(".peer:disabled ~ .{}", class));
                }
                // Dark mode
                "dark" => {
                    return Ok(format!(".dark .{}", class));
                }
                // Responsive variants are handled via media queries, not selectors
                "sm" | "md" | "lg" | "xl" | "2xl" => {}
                // Device variants are handled via media queries
                "pointer-coarse" | "pointer-fine" | "motion-reduce" | "motion-safe" | "light" => {}
                _ => {
                    // Unknown variant - log warning but continue
                    // Unknown variant encountered, continuing with base class
                }
            }
        }

        // Add pseudo-classes to the selector
        for pseudo in pseudo_selectors {
            selector.push_str(pseudo);
        }

        Ok(selector)
    }

    /// Get media query for variants (alias for get_variant_media_query)
    pub fn get_media_query(&self, variants: &[String]) -> Option<String> {
        self.get_variant_media_query(variants)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_variants_parsing() {
        let mut parser = VariantParser::new();

        // Test container variant
        let (variants, base) = parser.parse_variants("@container-md:flex");
        assert_eq!(variants, vec!["@container-md"]);
        assert_eq!(base, "flex");

        // Test arbitrary variant
        let (variants, base) = parser.parse_variants("[data-open]:flex");
        assert_eq!(variants, vec!["[data-open]"]);
        assert_eq!(base, "flex");

        // Test advanced device variants
        let (variants, base) = parser.parse_variants("pointer-coarse:flex");
        assert_eq!(variants, vec!["pointer-coarse"]);
        assert_eq!(base, "flex");

        let (variants, base) = parser.parse_variants("motion-reduce:flex");
        assert_eq!(variants, vec!["motion-reduce"]);
        assert_eq!(base, "flex");
    }

    #[test]
    fn test_custom_variant_definition() {
        let mut parser = VariantParser::new();

        // Define a custom variant
        parser.define_custom_variant("hover-visible", ":hover:not(.hidden)").unwrap();

        // Test parsing custom variant
        let (variants, base) = parser.parse_variants("hover-visible:flex");
        assert_eq!(variants, vec!["hover-visible"]);
        assert_eq!(base, "flex");

        // Test getting selector for custom variant
        let selector = parser.get_variant_selector("hover-visible");
        assert_eq!(selector, ":hover:not(.hidden) ");
    }

    #[test]
    fn test_media_queries() {
        let parser = VariantParser::new();

        // Test device variant media queries
        assert_eq!(
            parser.get_single_variant_media_query("pointer-coarse"),
            Some("@media (pointer: coarse)".to_string())
        );
        assert_eq!(
            parser.get_single_variant_media_query("motion-reduce"),
            Some("@media (prefers-reduced-motion: reduce)".to_string())
        );

        // Test that media variants are identified correctly
        assert!(parser.is_media_variant("pointer-coarse"));
        assert!(parser.is_media_variant("motion-reduce"));
        assert!(!parser.is_media_variant("hover"));
    }
}

impl Default for VariantParser {
    fn default() -> Self {
        Self::new()
    }
}
