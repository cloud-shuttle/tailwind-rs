//! @tailwind Directive Processing System
//! 
//! This module provides comprehensive processing of Tailwind CSS directives
//! including @tailwind base, @tailwind components, @tailwind utilities,
//! @apply directives, and @layer directives.

use std::collections::HashMap;
use regex::Regex;
use thiserror::Error;

/// Main processor for Tailwind CSS directives
pub struct TailwindProcessor {
    base_styles: String,
    component_styles: String,
    utility_styles: String,
    config: TailwindConfig,
    cache: ProcessorCache,
    directive_parser: DirectiveParser,
    css_injector: CSSInjector,
}

/// Configuration for Tailwind directive processing
#[derive(Debug, Clone)]
pub struct TailwindConfig {
    pub base_styles: bool,
    pub component_styles: bool,
    pub utility_styles: bool,
    pub apply_processing: bool,
    pub layer_processing: bool,
    pub custom_directives: Vec<CustomDirective>,
}

/// Custom directive configuration
#[derive(Debug, Clone)]
pub struct CustomDirective {
    pub name: String,
    pub pattern: String,
    pub handler: String,
}

/// Cache for processed directives
#[derive(Debug, Clone)]
pub struct ProcessorCache {
    directive_cache: HashMap<String, String>,
    apply_cache: HashMap<String, String>,
    layer_cache: HashMap<String, String>,
}

/// Directive parser for @tailwind, @apply, and @layer directives
pub struct DirectiveParser {
    patterns: Vec<DirectivePattern>,
    config: ParserConfig,
}

/// Directive pattern matching
#[derive(Debug, Clone)]
pub struct DirectivePattern {
    pub name: String,
    pub regex: Regex,
    pub capture_groups: usize,
}

/// Parser configuration
#[derive(Debug, Clone)]
pub struct ParserConfig {
    pub case_sensitive: bool,
    pub multiline: bool,
    pub dotall: bool,
}

/// CSS injector for base, components, and utilities
pub struct CSSInjector {
    base_css: String,
    components_css: String,
    utilities_css: String,
    config: InjectionConfig,
}

/// Injection configuration
#[derive(Debug, Clone)]
pub struct InjectionConfig {
    pub preserve_order: bool,
    pub minify: bool,
    pub source_map: bool,
}

/// Tailwind directive types
#[derive(Debug, Clone, PartialEq)]
pub enum TailwindDirective {
    Base,
    Components,
    Utilities,
}

/// @apply directive structure
#[derive(Debug, Clone)]
pub struct ApplyDirective {
    pub classes: Vec<String>,
    pub selector: String,
    pub line_number: usize,
}

/// @layer directive structure
#[derive(Debug, Clone, PartialEq)]
pub enum LayerDirective {
    Base,
    Components,
    Utilities,
    Custom(String),
}

/// Directive match result
#[derive(Debug, Clone)]
pub struct DirectiveMatch {
    pub directive_type: String,
    pub content: String,
    pub line_number: usize,
    pub start_pos: usize,
    pub end_pos: usize,
}

/// Processing result with statistics
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    pub processed_css: String,
    pub directives_processed: Vec<DirectiveMatch>,
    pub statistics: ProcessingStatistics,
}

/// Processing statistics
#[derive(Debug, Clone)]
pub struct ProcessingStatistics {
    pub total_directives: usize,
    pub base_directives: usize,
    pub component_directives: usize,
    pub utility_directives: usize,
    pub apply_directives: usize,
    pub layer_directives: usize,
    pub processing_time_ms: u64,
}

/// Error types for Tailwind processing
#[derive(Debug, Error)]
pub enum TailwindProcessorError {
    #[error("Invalid @tailwind directive: {directive}")]
    InvalidDirective { directive: String },
    
    #[error("Unknown class in @apply: {class}")]
    UnknownClass { class: String },
    
    #[error("Circular @apply dependency: {class}")]
    CircularDependency { class: String },
    
    #[error("Invalid @layer directive: {layer}")]
    InvalidLayer { layer: String },
    
    #[error("CSS parsing error: {error}")]
    CSSParsingError { error: String },
    
    #[error("Configuration error: {error}")]
    ConfigurationError { error: String },
}

impl TailwindProcessor {
    /// Create a new TailwindProcessor with default configuration
    pub fn new() -> Self {
        Self::with_config(TailwindConfig::default())
    }
    
    /// Create a new TailwindProcessor with custom configuration
    pub fn with_config(config: TailwindConfig) -> Self {
        let directive_parser = DirectiveParser::new();
        let css_injector = CSSInjector::new();
        let cache = ProcessorCache::new();
        
        Self {
            base_styles: String::new(),
            component_styles: String::new(),
            utility_styles: String::new(),
            config,
            cache,
            directive_parser,
            css_injector,
        }
    }
    
    /// Create a new TailwindProcessor with cache
    pub fn with_cache(config: TailwindConfig, cache: ProcessorCache) -> Self {
        let directive_parser = DirectiveParser::new();
        let css_injector = CSSInjector::new();
        
        Self {
            base_styles: String::new(),
            component_styles: String::new(),
            utility_styles: String::new(),
            config,
            cache,
            directive_parser,
            css_injector,
        }
    }
    
    /// Process @tailwind directives in CSS
    pub fn process_directives(&mut self, css: &str) -> Result<ProcessingResult, TailwindProcessorError> {
        let start_time = std::time::Instant::now();
        
        // Parse all directives
        let directives = self.directive_parser.parse_tailwind_directives(css)?;
        let apply_directives = self.directive_parser.parse_apply_directives(css)?;
        let layer_directives = self.directive_parser.parse_layer_directives(css)?;
        
        // Process each directive type
        let mut processed_css = css.to_string();
        let mut processed_directives = Vec::new();
        
        // Process @tailwind directives
        for directive in &directives {
            let css_content = self.generate_css_for_directive(directive)?;
            processed_css = self.replace_directive(&processed_css, directive, &css_content)?;
            processed_directives.push(DirectiveMatch {
                directive_type: "tailwind".to_string(),
                content: format!("@tailwind {}", directive.to_string()),
                line_number: 0, // Will be filled by parser
                start_pos: 0,    // Will be filled by parser
                end_pos: 0,     // Will be filled by parser
            });
        }
        
        // Process @apply directives
        for apply in &apply_directives {
            let css_content = self.process_apply_directive(apply)?;
            processed_css = self.replace_apply_directive(&processed_css, apply, &css_content)?;
            processed_directives.push(DirectiveMatch {
                directive_type: "apply".to_string(),
                content: format!("@apply {}", apply.classes.join(" ")),
                line_number: apply.line_number,
                start_pos: 0,    // Will be filled by parser
                end_pos: 0,     // Will be filled by parser
            });
        }
        
        // Process @layer directives
        for layer in &layer_directives {
            let css_content = self.process_layer_directive(layer)?;
            processed_css = self.replace_layer_directive(&processed_css, layer, &css_content)?;
            processed_directives.push(DirectiveMatch {
                directive_type: "layer".to_string(),
                content: format!("@layer {}", layer.to_string()),
                line_number: 0, // Will be filled by parser
                start_pos: 0,    // Will be filled by parser
                end_pos: 0,     // Will be filled by parser
            });
        }
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        // Generate statistics
        let statistics = ProcessingStatistics {
            total_directives: processed_directives.len(),
            base_directives: directives.iter().filter(|d| matches!(d, TailwindDirective::Base)).count(),
            component_directives: directives.iter().filter(|d| matches!(d, TailwindDirective::Components)).count(),
            utility_directives: directives.iter().filter(|d| matches!(d, TailwindDirective::Utilities)).count(),
            apply_directives: apply_directives.len(),
            layer_directives: layer_directives.len(),
            processing_time_ms: processing_time,
        };
        
        Ok(ProcessingResult {
            processed_css,
            directives_processed: processed_directives,
            statistics,
        })
    }
    
    /// Process @apply directive
    pub fn process_apply(&mut self, css: &str) -> Result<String, TailwindProcessorError> {
        let apply_directives = self.directive_parser.parse_apply_directives(css)?;
        let mut processed_css = css.to_string();
        
        for apply in apply_directives {
            let css_content = self.process_apply_directive(&apply)?;
            processed_css = self.replace_apply_directive(&processed_css, &apply, &css_content)?;
        }
        
        Ok(processed_css)
    }
    
    /// Process @layer directive
    pub fn process_layer(&mut self, css: &str) -> Result<String, TailwindProcessorError> {
        let layer_directives = self.directive_parser.parse_layer_directives(css)?;
        let mut processed_css = css.to_string();
        
        for layer in layer_directives {
            let css_content = self.process_layer_directive(&layer)?;
            processed_css = self.replace_layer_directive(&processed_css, &layer, &css_content)?;
        }
        
        Ok(processed_css)
    }
    
    /// Generate CSS for a specific directive
    fn generate_css_for_directive(&self, directive: &TailwindDirective) -> Result<String, TailwindProcessorError> {
        match directive {
            TailwindDirective::Base => {
                if self.config.base_styles {
                    Ok(self.css_injector.inject_base(&self.base_styles)?)
                } else {
                    Ok(String::new())
                }
            },
            TailwindDirective::Components => {
                if self.config.component_styles {
                    Ok(self.css_injector.inject_components(&self.component_styles)?)
                } else {
                    Ok(String::new())
                }
            },
            TailwindDirective::Utilities => {
                if self.config.utility_styles {
                    Ok(self.css_injector.inject_utilities(&self.utility_styles)?)
                } else {
                    Ok(String::new())
                }
            },
        }
    }
    
    /// Process @apply directive
    fn process_apply_directive(&mut self, apply: &ApplyDirective) -> Result<String, TailwindProcessorError> {
        let mut css = String::new();
        
        for class in &apply.classes {
            let class_css = self.resolve_class_to_css(class)?;
            css.push_str(&format!("{} {{ {} }}\n", apply.selector, class_css));
        }
        
        Ok(css)
    }
    
    /// Process @layer directive
    fn process_layer_directive(&self, layer: &LayerDirective) -> Result<String, TailwindProcessorError> {
        match layer {
            LayerDirective::Base => Ok(self.css_injector.inject_base(&self.base_styles)?),
            LayerDirective::Components => Ok(self.css_injector.inject_components(&self.component_styles)?),
            LayerDirective::Utilities => Ok(self.css_injector.inject_utilities(&self.utility_styles)?),
            LayerDirective::Custom(name) => {
                // Handle custom layers
                Ok(format!("/* Custom layer: {} */\n", name))
            },
        }
    }
    
    /// Resolve class name to CSS
    fn resolve_class_to_css(&mut self, class: &str) -> Result<String, TailwindProcessorError> {
        // Check cache first
        if let Some(cached) = self.cache.get_cached_apply(class) {
            return Ok(cached.clone());
        }
        
        // TODO: Implement actual class resolution
        // This would integrate with the existing CSS generator
        let css = format!("/* TODO: Resolve class {} */", class);
        
        // Cache the result
        self.cache.cache_apply(class.to_string(), css.clone());
        
        Ok(css)
    }
    
    /// Replace directive in CSS
    fn replace_directive(&self, css: &str, directive: &TailwindDirective, content: &str) -> Result<String, TailwindProcessorError> {
        let pattern = match directive {
            TailwindDirective::Base => r"@tailwind\s+base;",
            TailwindDirective::Components => r"@tailwind\s+components;",
            TailwindDirective::Utilities => r"@tailwind\s+utilities;",
        };
        
        let regex = Regex::new(pattern).map_err(|e| TailwindProcessorError::CSSParsingError { 
            error: format!("Invalid regex pattern: {}", e) 
        })?;
        
        Ok(regex.replace_all(css, content).to_string())
    }
    
    /// Replace @apply directive in CSS
    fn replace_apply_directive(&self, css: &str, apply: &ApplyDirective, content: &str) -> Result<String, TailwindProcessorError> {
        let pattern = format!(r"@apply\s+{};", apply.classes.join(r"\s+"));
        let regex = Regex::new(&pattern).map_err(|e| TailwindProcessorError::CSSParsingError { 
            error: format!("Invalid regex pattern: {}", e) 
        })?;
        
        Ok(regex.replace_all(css, content).to_string())
    }
    
    /// Replace @layer directive in CSS
    fn replace_layer_directive(&self, css: &str, layer: &LayerDirective, content: &str) -> Result<String, TailwindProcessorError> {
        let pattern = match layer {
            LayerDirective::Base => r"@layer\s+base;",
            LayerDirective::Components => r"@layer\s+components;",
            LayerDirective::Utilities => r"@layer\s+utilities;",
            LayerDirective::Custom(name) => &format!(r"@layer\s+{};", regex::escape(name)),
        };
        
        let regex = Regex::new(pattern).map_err(|e| TailwindProcessorError::CSSParsingError { 
            error: format!("Invalid regex pattern: {}", e) 
        })?;
        
        Ok(regex.replace_all(css, content).to_string())
    }
}

impl Default for TailwindConfig {
    fn default() -> Self {
        Self {
            base_styles: true,
            component_styles: true,
            utility_styles: true,
            apply_processing: true,
            layer_processing: true,
            custom_directives: Vec::new(),
        }
    }
}

impl ProcessorCache {
    /// Create a new processor cache
    pub fn new() -> Self {
        Self {
            directive_cache: HashMap::new(),
            apply_cache: HashMap::new(),
            layer_cache: HashMap::new(),
        }
    }
    
    /// Get cached directive
    pub fn get_cached_directive(&self, directive: &str) -> Option<&String> {
        self.directive_cache.get(directive)
    }
    
    /// Cache directive
    pub fn cache_directive(&mut self, directive: String, css: String) {
        self.directive_cache.insert(directive, css);
    }
    
    /// Get cached apply
    pub fn get_cached_apply(&self, class: &str) -> Option<&String> {
        self.apply_cache.get(class)
    }
    
    /// Cache apply
    pub fn cache_apply(&mut self, class: String, css: String) {
        self.apply_cache.insert(class, css);
    }
}

impl DirectiveParser {
    /// Create a new directive parser
    pub fn new() -> Self {
        let patterns = vec![
            DirectivePattern {
                name: "tailwind_base".to_string(),
                regex: Regex::new(r"@tailwind\s+base;").unwrap(),
                capture_groups: 0,
            },
            DirectivePattern {
                name: "tailwind_components".to_string(),
                regex: Regex::new(r"@tailwind\s+components;").unwrap(),
                capture_groups: 0,
            },
            DirectivePattern {
                name: "tailwind_utilities".to_string(),
                regex: Regex::new(r"@tailwind\s+utilities;").unwrap(),
                capture_groups: 0,
            },
            DirectivePattern {
                name: "apply".to_string(),
                regex: Regex::new(r"@apply\s+([^;]+);").unwrap(),
                capture_groups: 1,
            },
            DirectivePattern {
                name: "layer".to_string(),
                regex: Regex::new(r"@layer\s+([^;{]+);").unwrap(),
                capture_groups: 1,
            },
        ];
        
        Self {
            patterns,
            config: ParserConfig {
                case_sensitive: false,
                multiline: true,
                dotall: false,
            },
        }
    }
    
    /// Parse @tailwind directives
    pub fn parse_tailwind_directives(&self, css: &str) -> Result<Vec<TailwindDirective>, TailwindProcessorError> {
        let mut directives = Vec::new();
        
        // Parse base directives
        if self.patterns[0].regex.is_match(css) {
            directives.push(TailwindDirective::Base);
        }
        
        // Parse components directives
        if self.patterns[1].regex.is_match(css) {
            directives.push(TailwindDirective::Components);
        }
        
        // Parse utilities directives
        if self.patterns[2].regex.is_match(css) {
            directives.push(TailwindDirective::Utilities);
        }
        
        Ok(directives)
    }
    
    /// Parse @apply directives
    pub fn parse_apply_directives(&self, css: &str) -> Result<Vec<ApplyDirective>, TailwindProcessorError> {
        let mut directives = Vec::new();
        let apply_pattern = &self.patterns[3];
        
        for (line_num, line) in css.lines().enumerate() {
            for cap in apply_pattern.regex.captures_iter(line) {
                let classes_str = &cap[1];
                let classes: Vec<String> = classes_str.split_whitespace().map(|s| s.to_string()).collect();
                
                directives.push(ApplyDirective {
                    classes,
                    selector: "".to_string(), // Will be determined by context
                    line_number: line_num + 1,
                });
            }
        }
        
        Ok(directives)
    }
    
    /// Parse @layer directives
    pub fn parse_layer_directives(&self, css: &str) -> Result<Vec<LayerDirective>, TailwindProcessorError> {
        let mut directives = Vec::new();
        let layer_pattern = &self.patterns[4];
        
        for cap in layer_pattern.regex.captures_iter(css) {
            let layer_name = &cap[1];
            let directive = match layer_name {
                "base" => LayerDirective::Base,
                "components" => LayerDirective::Components,
                "utilities" => LayerDirective::Utilities,
                name => LayerDirective::Custom(name.to_string()),
            };
            directives.push(directive);
        }
        
        Ok(directives)
    }
}

impl CSSInjector {
    /// Create a new CSS injector
    pub fn new() -> Self {
        Self {
            base_css: String::new(),
            components_css: String::new(),
            utilities_css: String::new(),
            config: InjectionConfig {
                preserve_order: true,
                minify: false,
                source_map: false,
            },
        }
    }
    
    /// Inject base styles
    pub fn inject_base(&self, css: &str) -> Result<String, TailwindProcessorError> {
        // TODO: Implement actual base CSS injection
        Ok(format!("/* Base styles */\n{}", css))
    }
    
    /// Inject component styles
    pub fn inject_components(&self, css: &str) -> Result<String, TailwindProcessorError> {
        // TODO: Implement actual component CSS injection
        Ok(format!("/* Component styles */\n{}", css))
    }
    
    /// Inject utility styles
    pub fn inject_utilities(&self, css: &str) -> Result<String, TailwindProcessorError> {
        // TODO: Implement actual utility CSS injection
        Ok(format!("/* Utility styles */\n{}", css))
    }
}

impl std::fmt::Display for TailwindDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TailwindDirective::Base => write!(f, "base"),
            TailwindDirective::Components => write!(f, "components"),
            TailwindDirective::Utilities => write!(f, "utilities"),
        }
    }
}

impl std::fmt::Display for LayerDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayerDirective::Base => write!(f, "base"),
            LayerDirective::Components => write!(f, "components"),
            LayerDirective::Utilities => write!(f, "utilities"),
            LayerDirective::Custom(name) => write!(f, "{}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tailwind_directive_processing() {
        let mut processor = TailwindProcessor::new();
        let css = "@tailwind base; @tailwind components; @tailwind utilities;";
        let result = processor.process_directives(css);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_apply_directive_processing() {
        let mut processor = TailwindProcessor::new();
        let css = ".btn { @apply bg-blue-500 text-white px-4 py-2; }";
        let result = processor.process_apply(css);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_layer_directive_processing() {
        let mut processor = TailwindProcessor::new();
        let css = "@layer base; @layer components; @layer utilities;";
        let result = processor.process_layer(css);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_directive_parser() {
        let parser = DirectiveParser::new();
        let css = "@tailwind base; @tailwind components; @tailwind utilities;";
        let directives = parser.parse_tailwind_directives(css).unwrap();
        assert_eq!(directives.len(), 3);
        assert!(directives.contains(&TailwindDirective::Base));
        assert!(directives.contains(&TailwindDirective::Components));
        assert!(directives.contains(&TailwindDirective::Utilities));
    }
    
    #[test]
    fn test_apply_parser() {
        let parser = DirectiveParser::new();
        let css = ".btn { @apply bg-blue-500 text-white px-4 py-2; }";
        let directives = parser.parse_apply_directives(css).unwrap();
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].classes.len(), 4);
        assert!(directives[0].classes.contains(&"bg-blue-500".to_string()));
    }
    
    #[test]
    fn test_layer_parser() {
        let parser = DirectiveParser::new();
        let css = "@layer base; @layer components; @layer utilities;";
        let directives = parser.parse_layer_directives(css).unwrap();
        assert_eq!(directives.len(), 3);
        assert!(directives.contains(&LayerDirective::Base));
        assert!(directives.contains(&LayerDirective::Components));
        assert!(directives.contains(&LayerDirective::Utilities));
    }
}
