//! Enhanced Autoprefixer Integration
//! 
//! This module provides comprehensive vendor prefixing functionality for CSS properties
//! based on browser compatibility data, essential for cross-browser CSS support.

use std::collections::HashMap;
use regex::Regex;
use thiserror::Error;

/// Main autoprefixer for adding vendor prefixes to CSS properties
pub struct Autoprefixer {
    browser_data: BrowserData,
    caniuse_data: CanIUseData,
    config: AutoprefixerConfig,
    cache: PrefixCache,
}

impl Autoprefixer {
    /// Create a new autoprefixer with default configuration
    pub fn new() -> Self {
        Self::with_config(AutoprefixerConfig::default())
    }
    
    /// Create a new autoprefixer with custom configuration
    pub fn with_config(config: AutoprefixerConfig) -> Self {
        Self {
            browser_data: BrowserData::new(),
            caniuse_data: CanIUseData::new(),
            config,
            cache: PrefixCache::new(),
        }
    }
    
    /// Add vendor prefixes based on browser support
    pub fn add_prefixes(&self, css: &str, browsers: &[String]) -> Result<String, AutoprefixerError> {
        let mut result = String::new();
        let mut in_rule = false;
        let mut current_rule = String::new();
        
        for line in css.lines() {
            if line.trim().ends_with('{') {
                in_rule = true;
                current_rule = line.to_string();
            } else if in_rule && line.trim() == "}" {
                current_rule.push_str(line);
                
                let prefixed_rule = self.prefix_rule(&current_rule, browsers)?;
                result.push_str(&prefixed_rule);
                result.push('\n');
                
                in_rule = false;
                current_rule.clear();
            } else if in_rule {
                current_rule.push_str(line);
                current_rule.push('\n');
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }
        
        Ok(result)
    }
    
    /// Add prefixes with advanced options
    pub fn add_prefixes_advanced(&self, css: &str, options: &PrefixOptions) -> Result<PrefixResult, AutoprefixerError> {
        let start_time = std::time::Instant::now();
        let original_size = css.len();
        
        let prefixed_css = self.add_prefixes(css, &options.browsers)?;
        let prefixed_size = prefixed_css.len();
        
        let prefixes_added = self.count_prefixes_added(&css, &prefixed_css);
        let prefixes_removed = self.count_prefixes_removed(&css, &prefixed_css);
        let properties_processed = self.count_properties_processed(&css);
        
        let processing_time = start_time.elapsed().as_millis() as usize;
        
        Ok(PrefixResult {
            prefixed_css,
            prefixes_added: HashMap::new(),
            prefixes_removed: HashMap::new(),
            statistics: PrefixStatistics {
                original_size,
                prefixed_size,
                prefixes_added,
                prefixes_removed,
                properties_processed,
                processing_time_ms: processing_time,
            },
        })
    }
    
    /// Check if property needs prefixes
    pub fn needs_prefix(&self, property: &str, browsers: &[String]) -> bool {
        self.check_browser_compatibility(property, browsers)
    }
    
    /// Prefix a single CSS rule
    fn prefix_rule(&self, rule: &str, browsers: &[String]) -> Result<String, AutoprefixerError> {
        let mut prefixed_rule = String::new();
        let mut in_declaration = false;
        let mut current_declaration = String::new();
        
        for line in rule.lines() {
            if line.trim().ends_with('{') {
                prefixed_rule.push_str(line);
                prefixed_rule.push('\n');
            } else if line.trim() == "}" {
                if in_declaration {
                    let prefixed_declaration = self.prefix_declaration(&current_declaration, browsers)?;
                    prefixed_rule.push_str(&prefixed_declaration);
                    in_declaration = false;
                    current_declaration.clear();
                }
                prefixed_rule.push_str(line);
                prefixed_rule.push('\n');
            } else if line.trim().ends_with(';') {
                current_declaration.push_str(line);
                let prefixed_declaration = self.prefix_declaration(&current_declaration, browsers)?;
                prefixed_rule.push_str(&prefixed_declaration);
                current_declaration.clear();
            } else {
                current_declaration.push_str(line);
                current_declaration.push('\n');
                in_declaration = true;
            }
        }
        
        Ok(prefixed_rule)
    }
    
    /// Prefix a single CSS declaration
    fn prefix_declaration(&self, declaration: &str, browsers: &[String]) -> Result<String, AutoprefixerError> {
        let property_pattern = Regex::new(r"([a-zA-Z-]+)\s*:\s*([^;]+);").unwrap();
        
        if let Some(cap) = property_pattern.captures(declaration) {
            let property_name = &cap[1];
            let property_value = &cap[2];
            
            if self.needs_prefix(property_name, browsers) {
                let prefixes = self.get_prefixes_for_property(property_name);
                let mut prefixed_declaration = String::new();
                
                for prefix in prefixes {
                    let prefixed_name = format!("{}{}", prefix, property_name);
                    prefixed_declaration.push_str(&format!("{}: {};\n", prefixed_name, property_value));
                }
                
                // Add original property
                prefixed_declaration.push_str(&format!("{}: {};\n", property_name, property_value));
                
                Ok(prefixed_declaration)
            } else {
                Ok(declaration.to_string())
            }
        } else {
            Ok(declaration.to_string())
        }
    }
    
    /// Get vendor prefixes for a property
    fn get_prefixes_for_property(&self, property: &str) -> Vec<String> {
        let mut prefixes = Vec::new();
        
        // Common vendor prefixes
        if self.needs_webkit_prefix(property) {
            prefixes.push("-webkit-".to_string());
        }
        if self.needs_moz_prefix(property) {
            prefixes.push("-moz-".to_string());
        }
        if self.needs_ms_prefix(property) {
            prefixes.push("-ms-".to_string());
        }
        if self.needs_o_prefix(property) {
            prefixes.push("-o-".to_string());
        }
        
        prefixes
    }
    
    /// Check if property needs webkit prefix
    fn needs_webkit_prefix(&self, property: &str) -> bool {
        matches!(property, 
            "display" | "flex" | "flex-direction" | "flex-wrap" | "flex-flow" |
            "justify-content" | "align-items" | "align-content" | "align-self" |
            "flex-grow" | "flex-shrink" | "flex-basis" | "order" |
            "transform" | "transform-origin" | "transform-style" |
            "transition" | "transition-property" | "transition-duration" |
            "transition-timing-function" | "transition-delay" |
            "animation" | "animation-name" | "animation-duration" |
            "animation-timing-function" | "animation-delay" |
            "animation-iteration-count" | "animation-direction" |
            "animation-fill-mode" | "animation-play-state" |
            "filter" | "backdrop-filter" | "mask" | "mask-image" |
            "mask-size" | "mask-position" | "mask-repeat" |
            "mask-origin" | "mask-clip" | "mask-composite" |
            "clip-path" | "shape-outside" | "shape-margin" |
            "text-decoration" | "text-decoration-line" |
            "text-decoration-style" | "text-decoration-color" |
            "text-decoration-skip" | "text-underline-position" |
            "text-emphasis" | "text-emphasis-style" |
            "text-emphasis-color" | "text-emphasis-position" |
            "text-shadow" | "box-shadow" | "border-radius" |
            "border-image" | "border-image-source" |
            "border-image-slice" | "border-image-width" |
            "border-image-outset" | "border-image-repeat" |
            "background" | "background-image" | "background-size" |
            "background-position" | "background-repeat" |
            "background-attachment" | "background-clip" |
            "background-origin" | "background-color"
        )
    }
    
    /// Check if property needs moz prefix
    fn needs_moz_prefix(&self, property: &str) -> bool {
        matches!(property,
            "display" | "flex" | "flex-direction" | "flex-wrap" | "flex-flow" |
            "justify-content" | "align-items" | "align-content" | "align-self" |
            "flex-grow" | "flex-shrink" | "flex-basis" | "order" |
            "transform" | "transform-origin" | "transform-style" |
            "transition" | "transition-property" | "transition-duration" |
            "transition-timing-function" | "transition-delay" |
            "animation" | "animation-name" | "animation-duration" |
            "animation-timing-function" | "animation-delay" |
            "animation-iteration-count" | "animation-direction" |
            "animation-fill-mode" | "animation-play-state" |
            "filter" | "backdrop-filter" | "mask" | "mask-image" |
            "mask-size" | "mask-position" | "mask-repeat" |
            "mask-origin" | "mask-clip" | "mask-composite" |
            "clip-path" | "shape-outside" | "shape-margin" |
            "text-decoration" | "text-decoration-line" |
            "text-decoration-style" | "text-decoration-color" |
            "text-decoration-skip" | "text-underline-position" |
            "text-emphasis" | "text-emphasis-style" |
            "text-emphasis-color" | "text-emphasis-position" |
            "text-shadow" | "box-shadow" | "border-radius" |
            "border-image" | "border-image-source" |
            "border-image-slice" | "border-image-width" |
            "border-image-outset" | "border-image-repeat" |
            "background" | "background-image" | "background-size" |
            "background-position" | "background-repeat" |
            "background-attachment" | "background-clip" |
            "background-origin" | "background-color"
        )
    }
    
    /// Check if property needs ms prefix
    fn needs_ms_prefix(&self, property: &str) -> bool {
        matches!(property,
            "display" | "flex" | "flex-direction" | "flex-wrap" | "flex-flow" |
            "justify-content" | "align-items" | "align-content" | "align-self" |
            "flex-grow" | "flex-shrink" | "flex-basis" | "order" |
            "transform" | "transform-origin" | "transform-style" |
            "transition" | "transition-property" | "transition-duration" |
            "transition-timing-function" | "transition-delay" |
            "animation" | "animation-name" | "animation-duration" |
            "animation-timing-function" | "animation-delay" |
            "animation-iteration-count" | "animation-direction" |
            "animation-fill-mode" | "animation-play-state" |
            "filter" | "backdrop-filter" | "mask" | "mask-image" |
            "mask-size" | "mask-position" | "mask-repeat" |
            "mask-origin" | "mask-clip" | "mask-composite" |
            "clip-path" | "shape-outside" | "shape-margin" |
            "text-decoration" | "text-decoration-line" |
            "text-decoration-style" | "text-decoration-color" |
            "text-decoration-skip" | "text-underline-position" |
            "text-emphasis" | "text-emphasis-style" |
            "text-emphasis-color" | "text-emphasis-position" |
            "text-shadow" | "box-shadow" | "border-radius" |
            "border-image" | "border-image-source" |
            "border-image-slice" | "border-image-width" |
            "border-image-outset" | "border-image-repeat" |
            "background" | "background-image" | "background-size" |
            "background-position" | "background-repeat" |
            "background-attachment" | "background-clip" |
            "background-origin" | "background-color"
        )
    }
    
    /// Check if property needs o prefix
    fn needs_o_prefix(&self, property: &str) -> bool {
        matches!(property,
            "display" | "flex" | "flex-direction" | "flex-wrap" | "flex-flow" |
            "justify-content" | "align-items" | "align-content" | "align-self" |
            "flex-grow" | "flex-shrink" | "flex-basis" | "order" |
            "transform" | "transform-origin" | "transform-style" |
            "transition" | "transition-property" | "transition-duration" |
            "transition-timing-function" | "transition-delay" |
            "animation" | "animation-name" | "animation-duration" |
            "animation-timing-function" | "animation-delay" |
            "animation-iteration-count" | "animation-direction" |
            "animation-fill-mode" | "animation-play-state" |
            "filter" | "backdrop-filter" | "mask" | "mask-image" |
            "mask-size" | "mask-position" | "mask-repeat" |
            "mask-origin" | "mask-clip" | "mask-composite" |
            "clip-path" | "shape-outside" | "shape-margin" |
            "text-decoration" | "text-decoration-line" |
            "text-decoration-style" | "text-decoration-color" |
            "text-decoration-skip" | "text-underline-position" |
            "text-emphasis" | "text-emphasis-style" |
            "text-emphasis-color" | "text-emphasis-position" |
            "text-shadow" | "box-shadow" | "border-radius" |
            "border-image" | "border-image-source" |
            "border-image-slice" | "border-image-width" |
            "border-image-outset" | "border-image-repeat" |
            "background" | "background-image" | "background-size" |
            "background-position" | "background-repeat" |
            "background-attachment" | "background-clip" |
            "background-origin" | "background-color"
        )
    }
    
    /// Check browser compatibility for a property
    fn check_browser_compatibility(&self, property: &str, browsers: &[String]) -> bool {
        for browser in browsers {
            let support = self.browser_data.get_feature_support(property, browser);
            match support {
                Some(SupportLevel::Full) => continue,
                Some(SupportLevel::Partial) => return true, // Needs prefix
                Some(SupportLevel::None) => return true, // Needs prefix
                Some(SupportLevel::Unknown) => return true, // Assume needs prefix
                None => return true, // Unknown feature, assume needs prefix
            }
        }
        false
    }
    
    /// Count prefixes added
    fn count_prefixes_added(&self, original: &str, prefixed: &str) -> usize {
        let original_prefixes = self.count_prefixes_in_css(original);
        let prefixed_prefixes = self.count_prefixes_in_css(prefixed);
        prefixed_prefixes - original_prefixes
    }
    
    /// Count prefixes removed
    fn count_prefixes_removed(&self, original: &str, prefixed: &str) -> usize {
        let original_prefixes = self.count_prefixes_in_css(original);
        let prefixed_prefixes = self.count_prefixes_in_css(prefixed);
        if original_prefixes > prefixed_prefixes {
            original_prefixes - prefixed_prefixes
        } else {
            0
        }
    }
    
    /// Count prefixes in CSS
    fn count_prefixes_in_css(&self, css: &str) -> usize {
        let prefix_patterns = ["-webkit-", "-moz-", "-ms-", "-o-"];
        let mut count = 0;
        
        for pattern in &prefix_patterns {
            count += css.matches(pattern).count();
        }
        
        count
    }
    
    /// Count properties processed
    fn count_properties_processed(&self, css: &str) -> usize {
        let property_pattern = Regex::new(r"([a-zA-Z-]+)\s*:\s*([^;]+);").unwrap();
        property_pattern.captures_iter(css).count()
    }
}

/// Browser data for compatibility checking
pub struct BrowserData {
    browsers: HashMap<String, BrowserInfo>,
    features: HashMap<String, FeatureSupport>,
    versions: HashMap<String, Vec<String>>,
}

impl BrowserData {
    /// Create new browser data
    pub fn new() -> Self {
        let mut browsers = HashMap::new();
        let features = HashMap::new();
        let mut versions = HashMap::new();
        
        // Initialize with common browsers
        browsers.insert("chrome".to_string(), BrowserInfo {
            name: "Chrome".to_string(),
            versions: vec!["30".to_string(), "40".to_string(), "50".to_string(), "60".to_string(), "70".to_string(), "80".to_string(), "90".to_string(), "100".to_string()],
            current_version: "100".to_string(),
            support_level: SupportLevel::Full,
        });
        
        browsers.insert("firefox".to_string(), BrowserInfo {
            name: "Firefox".to_string(),
            versions: vec!["25".to_string(), "30".to_string(), "40".to_string(), "50".to_string(), "60".to_string(), "70".to_string(), "80".to_string(), "90".to_string()],
            current_version: "90".to_string(),
            support_level: SupportLevel::Full,
        });
        
        browsers.insert("safari".to_string(), BrowserInfo {
            name: "Safari".to_string(),
            versions: vec!["7".to_string(), "8".to_string(), "9".to_string(), "10".to_string(), "11".to_string(), "12".to_string(), "13".to_string(), "14".to_string()],
            current_version: "14".to_string(),
            support_level: SupportLevel::Full,
        });
        
        browsers.insert("ie".to_string(), BrowserInfo {
            name: "Internet Explorer".to_string(),
            versions: vec!["8".to_string(), "9".to_string(), "10".to_string(), "11".to_string()],
            current_version: "11".to_string(),
            support_level: SupportLevel::Partial,
        });
        
        browsers.insert("edge".to_string(), BrowserInfo {
            name: "Edge".to_string(),
            versions: vec!["12".to_string(), "13".to_string(), "14".to_string(), "15".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), "19".to_string()],
            current_version: "19".to_string(),
            support_level: SupportLevel::Full,
        });
        
        // Initialize versions
        for (browser, info) in &browsers {
            versions.insert(browser.clone(), info.versions.clone());
        }
        
        Self {
            browsers,
            features,
            versions,
        }
    }
    
    /// Get browser support for a feature
    pub fn get_feature_support(&self, feature: &str, browser: &str) -> Option<SupportLevel> {
        // Simplified support checking
        match browser {
            "chrome" | "firefox" | "safari" | "edge" => Some(SupportLevel::Full),
            "ie" => Some(SupportLevel::Partial),
            _ => Some(SupportLevel::Unknown),
        }
    }
    
    /// Check if browser supports feature
    pub fn supports_feature(&self, _feature: &str, browser: &str, _version: &str) -> bool {
        match self.get_feature_support(_feature, browser) {
            Some(SupportLevel::Full) => true,
            Some(SupportLevel::Partial) => true,
            Some(SupportLevel::None) => false,
            Some(SupportLevel::Unknown) => false,
            None => false,
        }
    }
}

/// Can I Use data for browser compatibility
pub struct CanIUseData {
    features: HashMap<String, FeatureSupport>,
}

impl CanIUseData {
    /// Create new Can I Use data
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
        }
    }
    
    /// Get support for a feature
    pub fn get_support(&self, _feature: &str, browser: &str) -> Option<SupportLevel> {
        // Simplified support checking
        match browser {
            "chrome" | "firefox" | "safari" | "edge" => Some(SupportLevel::Full),
            "ie" => Some(SupportLevel::Partial),
            _ => Some(SupportLevel::Unknown),
        }
    }
}

/// Prefix generator for vendor prefixes
pub struct PrefixGenerator {
    prefixes: HashMap<String, Vec<String>>,
    config: GeneratorConfig,
}

impl PrefixGenerator {
    /// Create new prefix generator
    pub fn new() -> Self {
        Self {
            prefixes: HashMap::new(),
            config: GeneratorConfig::default(),
        }
    }
    
    /// Generate vendor prefixes for property
    pub fn generate_prefixes(&self, property: &str, browsers: &[String]) -> Vec<String> {
        let mut prefixes = Vec::new();
        
        for browser in browsers {
            match browser.as_str() {
                "chrome" | "safari" => {
                    if self.needs_webkit_prefix(property) {
                        prefixes.push("-webkit-".to_string());
                    }
                },
                "firefox" => {
                    if self.needs_moz_prefix(property) {
                        prefixes.push("-moz-".to_string());
                    }
                },
                "ie" | "edge" => {
                    if self.needs_ms_prefix(property) {
                        prefixes.push("-ms-".to_string());
                    }
                },
                _ => {}
            }
        }
        
        prefixes
    }
    
    /// Check if property needs webkit prefix
    fn needs_webkit_prefix(&self, property: &str) -> bool {
        matches!(property,
            "display" | "flex" | "flex-direction" | "flex-wrap" | "flex-flow" |
            "justify-content" | "align-items" | "align-content" | "align-self" |
            "flex-grow" | "flex-shrink" | "flex-basis" | "order" |
            "transform" | "transform-origin" | "transform-style" |
            "transition" | "transition-property" | "transition-duration" |
            "transition-timing-function" | "transition-delay" |
            "animation" | "animation-name" | "animation-duration" |
            "animation-timing-function" | "animation-delay" |
            "animation-iteration-count" | "animation-direction" |
            "animation-fill-mode" | "animation-play-state" |
            "filter" | "backdrop-filter" | "mask" | "mask-image" |
            "mask-size" | "mask-position" | "mask-repeat" |
            "mask-origin" | "mask-clip" | "mask-composite" |
            "clip-path" | "shape-outside" | "shape-margin" |
            "text-decoration" | "text-decoration-line" |
            "text-decoration-style" | "text-decoration-color" |
            "text-decoration-skip" | "text-underline-position" |
            "text-emphasis" | "text-emphasis-style" |
            "text-emphasis-color" | "text-emphasis-position" |
            "text-shadow" | "box-shadow" | "border-radius" |
            "border-image" | "border-image-source" |
            "border-image-slice" | "border-image-width" |
            "border-image-outset" | "border-image-repeat" |
            "background" | "background-image" | "background-size" |
            "background-position" | "background-repeat" |
            "background-attachment" | "background-clip" |
            "background-origin" | "background-color"
        )
    }
    
    /// Check if property needs moz prefix
    fn needs_moz_prefix(&self, property: &str) -> bool {
        matches!(property,
            "display" | "flex" | "flex-direction" | "flex-wrap" | "flex-flow" |
            "justify-content" | "align-items" | "align-content" | "align-self" |
            "flex-grow" | "flex-shrink" | "flex-basis" | "order" |
            "transform" | "transform-origin" | "transform-style" |
            "transition" | "transition-property" | "transition-duration" |
            "transition-timing-function" | "transition-delay" |
            "animation" | "animation-name" | "animation-duration" |
            "animation-timing-function" | "animation-delay" |
            "animation-iteration-count" | "animation-direction" |
            "animation-fill-mode" | "animation-play-state" |
            "filter" | "backdrop-filter" | "mask" | "mask-image" |
            "mask-size" | "mask-position" | "mask-repeat" |
            "mask-origin" | "mask-clip" | "mask-composite" |
            "clip-path" | "shape-outside" | "shape-margin" |
            "text-decoration" | "text-decoration-line" |
            "text-decoration-style" | "text-decoration-color" |
            "text-decoration-skip" | "text-underline-position" |
            "text-emphasis" | "text-emphasis-style" |
            "text-emphasis-color" | "text-emphasis-position" |
            "text-shadow" | "box-shadow" | "border-radius" |
            "border-image" | "border-image-source" |
            "border-image-slice" | "border-image-width" |
            "border-image-outset" | "border-image-repeat" |
            "background" | "background-image" | "background-size" |
            "background-position" | "background-repeat" |
            "background-attachment" | "background-clip" |
            "background-origin" | "background-color"
        )
    }
    
    /// Check if property needs ms prefix
    fn needs_ms_prefix(&self, property: &str) -> bool {
        matches!(property,
            "display" | "flex" | "flex-direction" | "flex-wrap" | "flex-flow" |
            "justify-content" | "align-items" | "align-content" | "align-self" |
            "flex-grow" | "flex-shrink" | "flex-basis" | "order" |
            "transform" | "transform-origin" | "transform-style" |
            "transition" | "transition-property" | "transition-duration" |
            "transition-timing-function" | "transition-delay" |
            "animation" | "animation-name" | "animation-duration" |
            "animation-timing-function" | "animation-delay" |
            "animation-iteration-count" | "animation-direction" |
            "animation-fill-mode" | "animation-play-state" |
            "filter" | "backdrop-filter" | "mask" | "mask-image" |
            "mask-size" | "mask-position" | "mask-repeat" |
            "mask-origin" | "mask-clip" | "mask-composite" |
            "clip-path" | "shape-outside" | "shape-margin" |
            "text-decoration" | "text-decoration-line" |
            "text-decoration-style" | "text-decoration-color" |
            "text-decoration-skip" | "text-underline-position" |
            "text-emphasis" | "text-emphasis-style" |
            "text-emphasis-color" | "text-emphasis-position" |
            "text-shadow" | "box-shadow" | "border-radius" |
            "border-image" | "border-image-source" |
            "border-image-slice" | "border-image-width" |
            "border-image-outset" | "border-image-repeat" |
            "background" | "background-image" | "background-size" |
            "background-position" | "background-repeat" |
            "background-attachment" | "background-clip" |
            "background-origin" | "background-color"
        )
    }
}

/// Prefix cache for performance optimization
pub struct PrefixCache {
    property_cache: HashMap<String, Vec<String>>,
    browser_cache: HashMap<String, SupportLevel>,
    css_cache: HashMap<String, String>,
}

impl PrefixCache {
    /// Create new prefix cache
    pub fn new() -> Self {
        Self {
            property_cache: HashMap::new(),
            browser_cache: HashMap::new(),
            css_cache: HashMap::new(),
        }
    }
    
    /// Get cached prefixes for property
    pub fn get_cached_prefixes(&self, property: &str) -> Option<&Vec<String>> {
        self.property_cache.get(property)
    }
    
    /// Cache prefixes for property
    pub fn cache_prefixes(&mut self, property: String, prefixes: Vec<String>) {
        self.property_cache.insert(property, prefixes);
    }
    
    /// Get cached CSS
    pub fn get_cached_css(&self, css: &str) -> Option<&String> {
        self.css_cache.get(css)
    }
    
    /// Cache CSS
    pub fn cache_css(&mut self, css: String, prefixed: String) {
        self.css_cache.insert(css, prefixed);
    }
}

/// Configuration for autoprefixer
#[derive(Debug, Clone)]
pub struct AutoprefixerConfig {
    pub browsers: Vec<String>,
    pub cascade: bool,
    pub add: bool,
    pub remove: bool,
    pub supports: bool,
    pub flexbox: FlexboxMode,
    pub grid: GridMode,
    pub stats: Option<BrowserStats>,
}

impl Default for AutoprefixerConfig {
    fn default() -> Self {
        Self {
            browsers: vec![
                "chrome 30".to_string(),
                "firefox 25".to_string(),
                "safari 7".to_string(),
                "ie 10".to_string(),
                "edge 12".to_string(),
            ],
            cascade: true,
            add: true,
            remove: false,
            supports: true,
            flexbox: FlexboxMode::All,
            grid: GridMode::Autoplace,
            stats: None,
        }
    }
}

/// Flexbox mode configuration
#[derive(Debug, Clone)]
pub enum FlexboxMode {
    No2009,
    No2012,
    All,
}

/// Grid mode configuration
#[derive(Debug, Clone)]
pub enum GridMode {
    NoAutoplace,
    Autoplace,
}

/// Prefix options for advanced prefixing
#[derive(Debug, Clone)]
pub struct PrefixOptions {
    pub browsers: Vec<String>,
    pub cascade: bool,
    pub add: bool,
    pub remove: bool,
    pub supports: bool,
    pub flexbox: FlexboxMode,
    pub grid: GridMode,
    pub stats: Option<BrowserStats>,
}

/// Result of prefixing operation
#[derive(Debug, Clone)]
pub struct PrefixResult {
    pub prefixed_css: String,
    pub prefixes_added: HashMap<String, Vec<String>>,
    pub prefixes_removed: HashMap<String, Vec<String>>,
    pub statistics: PrefixStatistics,
}

/// Statistics for prefixing operation
#[derive(Debug, Clone)]
pub struct PrefixStatistics {
    pub original_size: usize,
    pub prefixed_size: usize,
    pub prefixes_added: usize,
    pub prefixes_removed: usize,
    pub properties_processed: usize,
    pub processing_time_ms: usize,
}

/// Browser information
#[derive(Debug, Clone)]
pub struct BrowserInfo {
    pub name: String,
    pub versions: Vec<String>,
    pub current_version: String,
    pub support_level: SupportLevel,
}

/// Support level for features
#[derive(Debug, Clone)]
pub enum SupportLevel {
    Full,
    Partial,
    None,
    Unknown,
}

/// Feature support information
#[derive(Debug, Clone)]
pub struct FeatureSupport {
    pub feature: String,
    pub browsers: HashMap<String, SupportInfo>,
}

/// Support information for specific browser
#[derive(Debug, Clone)]
pub struct SupportInfo {
    pub version: String,
    pub support: SupportLevel,
    pub prefix: Option<String>,
    pub notes: Option<String>,
}

/// Generator configuration
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub optimize: bool,
    pub remove_unused: bool,
    pub add_missing: bool,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            optimize: true,
            remove_unused: false,
            add_missing: true,
        }
    }
}

/// Browser statistics
#[derive(Debug, Clone)]
pub struct BrowserStats {
    pub usage: HashMap<String, f64>,
}

/// Error types for autoprefixer
#[derive(Debug, Error)]
pub enum AutoprefixerError {
    #[error("Invalid CSS syntax: {error}")]
    InvalidCSS { error: String },
    
    #[error("Unsupported browser: {browser}")]
    UnsupportedBrowser { browser: String },
    
    #[error("Failed to load browser data: {error}")]
    BrowserDataError { error: String },
    
    #[error("Prefix generation failed: {property}")]
    PrefixGenerationError { property: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_prefixing() {
        let autoprefixer = Autoprefixer::new();
        let css = ".test { display: flex; }";
        let result = autoprefixer.add_prefixes(css, &["chrome 30".to_string()]);
        assert!(result.is_ok());
        let prefixed = result.unwrap();
        assert!(prefixed.contains("-webkit-"));
    }
    
    #[test]
    fn test_flexbox_prefixing() {
        let mut config = AutoprefixerConfig::default();
        config.flexbox = FlexboxMode::All;
        let autoprefixer = Autoprefixer::with_config(config);
        
        let css = ".container { display: flex; }";
        let result = autoprefixer.add_prefixes(css, &["ie 10".to_string()]);
        assert!(result.is_ok());
        let prefixed = result.unwrap();
        assert!(prefixed.contains("-ms-"));
    }
    
    #[test]
    fn test_complex_prefixing() {
        let autoprefixer = Autoprefixer::new();
        
        let css = r#"
            .container {
                display: flex;
                flex-direction: column;
                align-items: center;
            }
            
            .item {
                flex: 1;
                transform: translateX(10px);
            }
        "#;
        
        let browsers = vec![
            "chrome 30".to_string(),
            "firefox 25".to_string(),
            "safari 7".to_string(),
        ];
        
        let result = autoprefixer.add_prefixes(css, &browsers);
        assert!(result.is_ok());
        
        let prefixed = result.unwrap();
        assert!(prefixed.contains("-webkit-"));
        assert!(prefixed.contains("-moz-"));
    }
    
    #[test]
    fn test_prefix_generator() {
        let generator = PrefixGenerator::new();
        let prefixes = generator.generate_prefixes("display", &["chrome".to_string(), "firefox".to_string()]);
        assert!(!prefixes.is_empty());
    }
    
    #[test]
    fn test_browser_data() {
        let browser_data = BrowserData::new();
        let support = browser_data.get_feature_support("display", "chrome");
        assert!(support.is_some());
    }
    
    #[test]
    fn test_prefix_cache() {
        let mut cache = PrefixCache::new();
        let prefixes = vec!["-webkit-".to_string(), "-moz-".to_string()];
        cache.cache_prefixes("display".to_string(), prefixes);
        
        let cached = cache.get_cached_prefixes("display");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 2);
    }
}
