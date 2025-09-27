//! # tailwind-rs-postcss
//!
//! PostCSS integration for Tailwind-RS Core, providing advanced CSS processing
//! capabilities with plugin ecosystem compatibility.
//!
//! This crate provides the foundation for PostCSS integration, enabling:
//! - Advanced CSS processing with AST manipulation
//! - Plugin ecosystem compatibility (NPM plugins)
//! - Source map generation
//! - Performance optimization
//!
//! ## Features
//!
//! - **PostCSS Engine**: Full PostCSS integration with Rust bindings
//! - **Plugin System**: Support for NPM plugins and native Rust plugins
//! - **AST Processing**: Advanced CSS AST parsing and manipulation
//! - **Source Maps**: Complete source map generation and support
//! - **Performance**: Optimized for large-scale CSS processing
//!
//! ## Example
//!
//! ```rust
//! use tailwind_rs_postcss::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let engine = PostCSSEngine::new(PostCSSConfig::default())?;
//!     
//!     let input_css = "@tailwind base; @tailwind components; @tailwind utilities;";
//!     let result = engine.process_css(input_css).await?;
//!     
//!     println!("Generated CSS: {}", result.css);
//!     println!("Source map: {:?}", result.source_map);
//!     
//!     Ok(())
//! }
//! ```

pub mod advanced_features;
pub mod ast;
pub mod autoprefixer;
pub mod css_optimizer;
pub mod engine;
pub mod enhanced_plugin_loader;
pub mod error;
pub mod import_processor;
pub mod js_bridge;
pub mod parser;
pub mod plugin_loader;
pub mod purger;
pub mod source_map;
pub mod tailwind_processor;
pub mod test_integration;
pub mod transformer;

// Re-export main types
pub use advanced_features::{
    AdvancedSourceMapGenerator, CSSLinter, LintResult, LinterConfig, PostCSSDevTools,
    PostCSSPerformanceMonitor,
};
pub use ast::{CSSAtRule, CSSDeclaration, CSSNode, CSSRule};
pub use autoprefixer::{
    Autoprefixer, AutoprefixerConfig,
};
pub use autoprefixer::core::{PrefixOptions, PrefixResult, PrefixStatistics};
pub use css_optimizer::{
    CSSOptimizer, OptimizationConfig, OptimizationMetrics, OptimizationResult,
};
pub use engine::{PostCSSConfig, PostCSSEngine, ProcessedCSS};
pub use enhanced_plugin_loader::{EnhancedPluginLoader};
pub use enhanced_plugin_loader::core::{PluginInstance, PluginMetrics};
pub use error::{PostCSSError, Result};
pub use import_processor::{
    ImportConfig, ImportOptions, ImportProcessor, ImportResult, ImportStatistics,
};
pub use js_bridge::{JSBridge, JSRuntime};
pub use parser::{CSSParser, ParseOptions};
pub use plugin_loader::{PluginConfig, PluginLoader, PluginResult};
pub use purger::{CSSPurger, PurgeConfig, PurgeOptions, PurgeResult};
pub use source_map::{SourceMap, SourceMapGenerator};
pub use tailwind_processor::{ProcessingResult, TailwindConfig, TailwindProcessor};
pub use transformer::{CSSTransformer, TransformOptions};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration for PostCSS processing
pub fn default_config() -> PostCSSConfig {
    PostCSSConfig::default()
}

/// Create a new PostCSS engine with default configuration
pub fn new_engine() -> Result<PostCSSEngine> {
    PostCSSEngine::new(PostCSSConfig::default())
}

/// Process CSS with PostCSS using default configuration
pub async fn process_css(input: &str) -> Result<ProcessedCSS> {
    let engine = new_engine()?;
    engine.process_css(input).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constant() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.chars().any(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_default_config() {
        let config = default_config();
        assert!(config.plugins.is_empty());
        assert!(config.source_map);
    }

    #[tokio::test]
    async fn test_process_css() {
        let input = ".test { color: red; }";
        let result = process_css(input).await;
        assert!(result.is_ok());

        let css = result.unwrap();
        assert!(css.css.contains(".test"));
        assert!(css.css.contains("color: red"));
    }
}
