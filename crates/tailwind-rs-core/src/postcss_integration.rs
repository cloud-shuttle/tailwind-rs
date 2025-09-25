//! PostCSS integration for enhanced CSS generation
//!
//! This module provides integration between the existing CSS generator
//! and the new PostCSS engine for advanced CSS processing.

use crate::css_generator::CssGenerator;
use crate::error::{Result, TailwindError};
use crate::responsive::Breakpoint;
use std::collections::HashMap;

// Import PostCSS types
use tailwind_rs_postcss::{
    PostCSSEngine, PostCSSConfig
};
use tailwind_rs_postcss::engine::{SourceMapOptions, PerformanceOptions, ProcessingMetrics, ProcessingWarning, EngineMetrics};

/// Enhanced CSS generator with PostCSS integration
#[derive(Debug)]
pub struct EnhancedCssGenerator {
    /// Core CSS generator
    core_generator: CssGenerator,
    /// PostCSS engine for advanced processing
    postcss_engine: Option<tailwind_rs_postcss::PostCSSEngine>,
    /// Configuration for PostCSS processing
    postcss_config: PostCSSIntegrationConfig,
    /// Cache for processed CSS
    processed_cache: HashMap<String, String>,
}

/// Configuration for PostCSS integration
#[derive(Debug, Clone)]
pub struct PostCSSIntegrationConfig {
    /// Enable PostCSS processing
    pub enabled: bool,
    /// PostCSS plugins to use
    pub plugins: Vec<String>,
    /// Source map generation
    pub source_maps: bool,
    /// CSS optimization
    pub optimize: bool,
    /// Vendor prefixing
    pub vendor_prefixes: bool,
}

impl Default for PostCSSIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            plugins: Vec::new(),
            source_maps: true,
            optimize: true,
            vendor_prefixes: false,
        }
    }
}

impl EnhancedCssGenerator {
    /// Create a new enhanced CSS generator
    pub fn new() -> Result<Self> {
        let core_generator = CssGenerator::new();
        let postcss_config = PostCSSIntegrationConfig::default();
        
        // Initialize PostCSS engine if enabled
        let postcss_engine = if postcss_config.enabled {
            let postcss_config = tailwind_rs_postcss::PostCSSConfig {
                source_map: postcss_config.source_maps,
                source_map_options: SourceMapOptions {
                    inline: false,
                    file: None,
                    source_root: None,
                    sources_content: true,
                },
                parser_options: tailwind_rs_postcss::ParseOptions::default(),
                transform_options: tailwind_rs_postcss::TransformOptions {
                    optimize: postcss_config.optimize,
                    vendor_prefixes: postcss_config.vendor_prefixes,
                    flatten_nesting: true,
                    resolve_custom_properties: true,
                },
                performance: PerformanceOptions::default(),
                plugins: Vec::new(),
            };
            
            Some(tailwind_rs_postcss::PostCSSEngine::new(postcss_config)?)
        } else {
            None
        };

        Ok(Self {
            core_generator,
            postcss_engine,
            postcss_config,
            processed_cache: HashMap::new(),
        })
    }

    /// Create with custom PostCSS configuration
    pub fn with_postcss_config(config: PostCSSIntegrationConfig) -> Result<Self> {
        let core_generator = CssGenerator::new();
        
        // Initialize PostCSS engine if enabled
        let postcss_engine = if config.enabled {
            let postcss_config = tailwind_rs_postcss::PostCSSConfig {
                source_map: config.source_maps,
                source_map_options: SourceMapOptions {
                    inline: false,
                    file: None,
                    source_root: None,
                    sources_content: true,
                },
                parser_options: tailwind_rs_postcss::ParseOptions::default(),
                transform_options: tailwind_rs_postcss::TransformOptions {
                    optimize: config.optimize,
                    vendor_prefixes: config.vendor_prefixes,
                    flatten_nesting: true,
                    resolve_custom_properties: true,
                },
                performance: PerformanceOptions::default(),
                plugins: Vec::new(),
            };
            
            Some(tailwind_rs_postcss::PostCSSEngine::new(postcss_config)?)
        } else {
            None
        };

        Ok(Self {
            core_generator,
            postcss_engine,
            postcss_config: config,
            processed_cache: HashMap::new(),
        })
    }

    /// Add a class to the generator (delegates to core generator)
    pub fn add_class(&mut self, class: &str) -> Result<()> {
        self.core_generator.add_class(class)
    }

    /// Add a responsive class (delegates to core generator)
    pub fn add_responsive_class(&mut self, breakpoint: Breakpoint, class: &str) -> Result<()> {
        self.core_generator.add_responsive_class(breakpoint, class)
    }

    /// Add a custom CSS property (delegates to core generator)
    pub fn add_custom_property(&mut self, name: &str, value: &str) {
        self.core_generator.add_custom_property(name, value);
    }

    /// Generate CSS with optional PostCSS processing
    pub async fn generate_css(&self) -> Result<String> {
        // Generate base CSS using core generator
        let base_css = self.core_generator.generate_css();
        
        // Apply PostCSS processing if enabled
        if let Some(engine) = &self.postcss_engine {
            let processed = engine.process_css(&base_css).await?;
            Ok(processed.css)
        } else {
            Ok(base_css)
        }
    }

    /// Generate CSS with PostCSS processing and return detailed results
    pub async fn generate_css_with_metadata(&self) -> Result<EnhancedCssResult> {
        // Generate base CSS using core generator
        let base_css = self.core_generator.generate_css();
        
        // Apply PostCSS processing if enabled
        if let Some(engine) = &self.postcss_engine {
            let processed = engine.process_css(&base_css).await?;
            
            let processed_css = processed.css.clone();
            let base_css_len = base_css.len();
            Ok(EnhancedCssResult {
                css: processed.css,
                source_map: processed.source_map,
                warnings: processed.warnings,
                metrics: processed.metrics,
                base_css_size: base_css_len,
                processed_css_size: processed_css.len(),
                compression_ratio: if base_css_len > 0 {
                    processed_css.len() as f32 / base_css_len as f32
                } else {
                    1.0
                },
            })
        } else {
            let base_css_len = base_css.len();
            Ok(EnhancedCssResult {
                css: base_css,
                source_map: None,
                warnings: Vec::new(),
                metrics: ProcessingMetrics {
                    parse_time: std::time::Duration::from_millis(0),
                    transform_time: std::time::Duration::from_millis(0),
                    generate_time: std::time::Duration::from_millis(0),
                    total_time: std::time::Duration::from_millis(0),
                    memory_usage: 0,
                    rules_processed: self.core_generator.rule_count(),
                    plugins_executed: 0,
                },
                base_css_size: base_css_len,
                processed_css_size: base_css_len,
                compression_ratio: 1.0,
            })
        }
    }

    /// Get the number of rules generated
    pub fn rule_count(&self) -> usize {
        self.core_generator.rule_count()
    }

    /// Get PostCSS engine metrics
    pub async fn get_postcss_metrics(&self) -> Option<EngineMetrics> {
        if let Some(engine) = &self.postcss_engine {
            Some(engine.get_metrics().await)
        } else {
            None
        }
    }

    /// Enable or disable PostCSS processing
    pub fn set_postcss_enabled(&mut self, enabled: bool) {
        self.postcss_config.enabled = enabled;
    }

    /// Add a PostCSS plugin
    pub fn add_postcss_plugin(&mut self, plugin: &str) -> Result<()> {
        if !self.postcss_config.plugins.contains(&plugin.to_string()) {
            self.postcss_config.plugins.push(plugin.to_string());
        }
        Ok(())
    }

    /// Remove a PostCSS plugin
    pub fn remove_postcss_plugin(&mut self, plugin: &str) {
        self.postcss_config.plugins.retain(|p| p != plugin);
    }

    /// Get current PostCSS configuration
    pub fn get_postcss_config(&self) -> &PostCSSIntegrationConfig {
        &self.postcss_config
    }

    /// Update PostCSS configuration
    pub fn update_postcss_config(&mut self, config: PostCSSIntegrationConfig) -> Result<()> {
        self.postcss_config = config;
        
        // Reinitialize PostCSS engine if needed
        if self.postcss_config.enabled && self.postcss_engine.is_none() {
            let postcss_config = tailwind_rs_postcss::PostCSSConfig {
                source_map: self.postcss_config.source_maps,
                source_map_options: SourceMapOptions {
                    inline: false,
                    file: None,
                    source_root: None,
                    sources_content: true,
                },
                parser_options: tailwind_rs_postcss::ParseOptions::default(),
                transform_options: tailwind_rs_postcss::TransformOptions {
                    optimize: self.postcss_config.optimize,
                    vendor_prefixes: self.postcss_config.vendor_prefixes,
                    flatten_nesting: true,
                    resolve_custom_properties: true,
                },
                performance: PerformanceOptions::default(),
                plugins: Vec::new(),
            };
            
            self.postcss_engine = Some(tailwind_rs_postcss::PostCSSEngine::new(postcss_config)?);
        } else if !self.postcss_config.enabled {
            self.postcss_engine = None;
        }
        
        Ok(())
    }
}

/// Enhanced CSS generation result with metadata
#[derive(Debug, Clone)]
pub struct EnhancedCssResult {
    /// Generated CSS
    pub css: String,
    /// Source map (if generated)
    pub source_map: Option<tailwind_rs_postcss::SourceMap>,
    /// Processing warnings
    pub warnings: Vec<ProcessingWarning>,
    /// Processing metrics
    pub metrics: ProcessingMetrics,
    /// Base CSS size (before PostCSS processing)
    pub base_css_size: usize,
    /// Processed CSS size (after PostCSS processing)
    pub processed_css_size: usize,
    /// Compression ratio (processed / base)
    pub compression_ratio: f32,
}

impl EnhancedCssResult {
    /// Get compression percentage
    pub fn compression_percentage(&self) -> f32 {
        (1.0 - self.compression_ratio) * 100.0
    }

    /// Check if CSS was compressed
    pub fn was_compressed(&self) -> bool {
        self.compression_ratio < 1.0
    }

    /// Get total processing time
    pub fn total_processing_time(&self) -> std::time::Duration {
        self.metrics.total_time
    }

    /// Get memory usage
    pub fn memory_usage(&self) -> usize {
        self.metrics.memory_usage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_generator_creation() {
        let generator = EnhancedCssGenerator::new();
        assert!(generator.is_ok());
    }

    #[test]
    fn test_postcss_config() {
        let config = PostCSSIntegrationConfig {
            enabled: true,
            plugins: vec!["autoprefixer".to_string()],
            source_maps: true,
            optimize: true,
            vendor_prefixes: true,
        };
        
        let generator = EnhancedCssGenerator::with_postcss_config(config);
        assert!(generator.is_ok());
    }

    #[test]
    fn test_add_class() {
        let mut generator = EnhancedCssGenerator::new().unwrap();
        let result = generator.add_class("p-4");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_css() {
        let mut generator = EnhancedCssGenerator::new().unwrap();
        generator.add_class("p-4").unwrap();
        generator.add_class("bg-blue-500").unwrap();
        
        let css = generator.generate_css().await;
        assert!(css.is_ok());
        
        let css = css.unwrap();
        assert!(css.contains(".p-4"));
        assert!(css.contains(".bg-blue-500"));
    }

    #[tokio::test]
    async fn test_generate_css_with_metadata() {
        let mut generator = EnhancedCssGenerator::new().unwrap();
        generator.add_class("p-4").unwrap();
        
        let result = generator.generate_css_with_metadata().await;
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(result.css.contains(".p-4"));
        assert!(result.base_css_size > 0);
        assert!(result.processed_css_size > 0);
    }
}
