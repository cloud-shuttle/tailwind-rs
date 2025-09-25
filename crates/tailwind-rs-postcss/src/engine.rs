//! PostCSS Engine implementation
//!
//! This module provides the core PostCSS processing engine with plugin support,
//! AST manipulation, and source map generation.

use crate::ast::{CSSNode, CSSRule};
use crate::parser::{CSSParser, ParseOptions};
use crate::transformer::{CSSTransformer, TransformOptions};
use crate::js_bridge::JSBridge;
use crate::plugin_loader::{PluginLoader, PluginConfig, PluginResult};
use crate::source_map::{SourceMapGenerator, SourceMap};
use crate::error::{PostCSSError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// PostCSS processing engine
#[derive(Debug)]
pub struct PostCSSEngine {
    config: PostCSSConfig,
    parser: CSSParser,
    transformer: CSSTransformer,
    js_bridge: Option<JSBridge>,
    plugin_loader: PluginLoader,
    source_map_generator: SourceMapGenerator,
    cache: Arc<RwLock<HashMap<String, ProcessedCSS>>>,
}

/// PostCSS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCSSConfig {
    /// Plugins to use for processing
    pub plugins: Vec<PluginConfig>,
    /// Source map generation
    pub source_map: bool,
    /// Source map options
    pub source_map_options: SourceMapOptions,
    /// Parser options
    pub parser_options: ParseOptions,
    /// Transformer options
    pub transform_options: TransformOptions,
    /// Performance options
    pub performance: PerformanceOptions,
}

/// Source map configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMapOptions {
    /// Generate inline source maps
    pub inline: bool,
    /// Source map file path
    pub file: Option<String>,
    /// Source root
    pub source_root: Option<String>,
    /// Include sources content
    pub sources_content: bool,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceOptions {
    /// Enable caching
    pub enable_cache: bool,
    /// Cache size limit
    pub cache_size_limit: usize,
    /// Parallel processing
    pub parallel_processing: bool,
    /// Memory optimization
    pub memory_optimization: bool,
}

/// Processed CSS result
#[derive(Debug, Clone)]
pub struct ProcessedCSS {
    /// Generated CSS
    pub css: String,
    /// Source map (if enabled)
    pub source_map: Option<SourceMap>,
    /// Processing warnings
    pub warnings: Vec<ProcessingWarning>,
    /// Processing metrics
    pub metrics: ProcessingMetrics,
}

/// Processing warning
#[derive(Debug, Clone)]
pub struct ProcessingWarning {
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub severity: WarningSeverity,
}

/// Warning severity levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarningSeverity {
    Info,
    Warning,
    Error,
}

/// Processing metrics
#[derive(Debug, Clone)]
pub struct ProcessingMetrics {
    pub parse_time: std::time::Duration,
    pub transform_time: std::time::Duration,
    pub generate_time: std::time::Duration,
    pub total_time: std::time::Duration,
    pub memory_usage: usize,
    pub rules_processed: usize,
    pub plugins_executed: usize,
}

impl Default for PostCSSConfig {
    fn default() -> Self {
        Self {
            plugins: Vec::new(),
            source_map: true,
            source_map_options: SourceMapOptions::default(),
            parser_options: ParseOptions::default(),
            transform_options: TransformOptions::default(),
            performance: PerformanceOptions::default(),
        }
    }
}

impl Default for SourceMapOptions {
    fn default() -> Self {
        Self {
            inline: false,
            file: None,
            source_root: None,
            sources_content: true,
        }
    }
}

impl Default for PerformanceOptions {
    fn default() -> Self {
        Self {
            enable_cache: true,
            cache_size_limit: 1000,
            parallel_processing: true,
            memory_optimization: true,
        }
    }
}

impl PostCSSEngine {
    /// Create a new PostCSS engine with configuration
    pub fn new(config: PostCSSConfig) -> Result<Self> {
        let parser = CSSParser::new(config.parser_options.clone());
        let transformer = CSSTransformer::new(config.transform_options.clone());
        let plugin_loader = PluginLoader::new();
        let source_map_generator = SourceMapGenerator::new();
        
        // Initialize JavaScript bridge if needed
        let js_bridge = if config.plugins.iter().any(|p| p.requires_js()) {
            Some(JSBridge::new()?)
        } else {
            None
        };

        Ok(Self {
            config,
            parser,
            transformer,
            js_bridge,
            plugin_loader,
            source_map_generator,
            cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Process CSS input through PostCSS pipeline
    pub async fn process_css(&self, input: &str) -> Result<ProcessedCSS> {
        let start_time = std::time::Instant::now();
        
        // Check cache first
        if self.config.performance.enable_cache {
            if let Some(cached) = self.get_cached_result(input).await {
                return Ok(cached);
            }
        }

        // Parse CSS into AST
        let parse_start = std::time::Instant::now();
        let ast = self.parser.parse(input)?;
        let parse_time = parse_start.elapsed();

        // Apply transformations
        let transform_start = std::time::Instant::now();
        let transformed_ast = self.apply_transformations(ast).await?;
        let transform_time = transform_start.elapsed();

        // Generate output CSS
        let generate_start = std::time::Instant::now();
        let css = self.generate_css(&transformed_ast)?;
        let generate_time = generate_start.elapsed();

        // Generate source map if enabled
        let source_map = if self.config.source_map {
            let source_map_options = crate::source_map::SourceMapOptions {
                inline: self.config.source_map_options.inline,
                file: self.config.source_map_options.file.clone(),
                source_root: self.config.source_map_options.source_root.clone(),
                sources_content: self.config.source_map_options.sources_content,
            };
            Some(self.source_map_generator.generate(
                input,
                &css,
                &source_map_options,
            )?)
        } else {
            None
        };

        let total_time = start_time.elapsed();
        let metrics = ProcessingMetrics {
            parse_time,
            transform_time,
            generate_time,
            total_time,
            memory_usage: self.get_memory_usage(),
            rules_processed: self.count_rules(&transformed_ast),
            plugins_executed: self.config.plugins.len(),
        };

        let result = ProcessedCSS {
            css,
            source_map,
            warnings: Vec::new(), // TODO: Collect warnings during processing
            metrics,
        };

        // Cache result if enabled
        if self.config.performance.enable_cache {
            self.cache_result(input, &result).await;
        }

        Ok(result)
    }

    /// Apply all configured transformations to the AST
    async fn apply_transformations(&self, mut ast: CSSNode) -> Result<CSSNode> {
        for plugin_config in &self.config.plugins {
            let plugin_result = self.plugin_loader.load_plugin(plugin_config).await?;
            
            match plugin_result {
                PluginResult::Native(plugin) => {
                    ast = plugin.transform(ast)?;
                }
                PluginResult::JavaScript(js_plugin) => {
                    if let Some(js_bridge) = &self.js_bridge {
                        ast = js_bridge.execute_plugin(&js_plugin.name, ast).await?;
                    } else {
                        return Err(PostCSSError::JavaScriptBridgeNotAvailable);
                    }
                }
            }
        }

        // Apply built-in transformations
        ast = self.transformer.transform(ast)?;

        Ok(ast)
    }

    /// Generate CSS from transformed AST
    fn generate_css(&self, ast: &CSSNode) -> Result<String> {
        match ast {
            CSSNode::Stylesheet(rules) => {
                let mut css = String::new();
                for rule in rules {
                    css.push_str(&self.rule_to_css(rule)?);
                    css.push('\n');
                }
                Ok(css)
            }
            _ => Err(PostCSSError::InvalidAST("Expected stylesheet".to_string())),
        }
    }

    /// Convert a CSS rule to CSS string
    fn rule_to_css(&self, rule: &CSSRule) -> Result<String> {
        let mut css = String::new();
        
        // Add selector
        css.push_str(&rule.selector);
        css.push_str(" {\n");
        
        // Add declarations
        for declaration in &rule.declarations {
            css.push_str("  ");
            css.push_str(&declaration.property);
            css.push_str(": ");
            css.push_str(&declaration.value);
            if declaration.important {
                css.push_str(" !important");
            }
            css.push_str(";\n");
        }
        
        css.push('}');
        Ok(css)
    }

    /// Get cached result if available
    async fn get_cached_result(&self, input: &str) -> Option<ProcessedCSS> {
        let cache = self.cache.read().await;
        cache.get(input).cloned()
    }

    /// Cache processing result
    async fn cache_result(&self, input: &str, result: &ProcessedCSS) {
        let mut cache = self.cache.write().await;
        
        // Check cache size limit
        if cache.len() >= self.config.performance.cache_size_limit {
            // Remove oldest entries (simple LRU)
            let keys_to_remove: Vec<String> = cache.keys().take(cache.len() / 2).cloned().collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }
        
        cache.insert(input.to_string(), result.clone());
    }

    /// Get current memory usage
    fn get_memory_usage(&self) -> usize {
        // Simple memory usage estimation
        // In a real implementation, this would use proper memory profiling
        std::mem::size_of::<Self>() + self.cache.try_read().map(|c| c.len() * 1024).unwrap_or(0)
    }

    /// Count rules in AST
    fn count_rules(&self, ast: &CSSNode) -> usize {
        match ast {
            CSSNode::Stylesheet(rules) => rules.len(),
            _ => 0,
        }
    }

    /// Get engine metrics
    pub async fn get_metrics(&self) -> EngineMetrics {
        let cache = self.cache.read().await;
        EngineMetrics {
            cache_size: cache.len(),
            memory_usage: self.get_memory_usage(),
            plugins_loaded: self.config.plugins.len(),
            js_bridge_available: self.js_bridge.is_some(),
        }
    }

    /// Clear cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

/// Engine metrics
#[derive(Debug, Clone)]
pub struct EngineMetrics {
    pub cache_size: usize,
    pub memory_usage: usize,
    pub plugins_loaded: usize,
    pub js_bridge_available: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_creation() {
        let config = PostCSSConfig::default();
        let engine = PostCSSEngine::new(config);
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_css_processing() {
        let engine = PostCSSEngine::new(PostCSSConfig::default()).unwrap();
        let input = ".test { color: red; }";
        let result = engine.process_css(input).await;
        assert!(result.is_ok());
        
        let css = result.unwrap();
        assert!(css.css.contains(".test"));
        assert!(css.css.contains("color: red"));
    }

    #[tokio::test]
    async fn test_caching() {
        let mut config = PostCSSConfig::default();
        config.performance.enable_cache = true;
        
        let engine = PostCSSEngine::new(config).unwrap();
        let input = ".test { color: red; }";
        
        // First processing
        let result1 = engine.process_css(input).await.unwrap();
        
        // Second processing (should use cache)
        let result2 = engine.process_css(input).await.unwrap();
        
        assert_eq!(result1.css, result2.css);
    }
}
