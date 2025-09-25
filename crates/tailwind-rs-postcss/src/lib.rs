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

pub mod engine;
pub mod ast;
pub mod parser;
pub mod transformer;
pub mod js_bridge;
pub mod plugin_loader;
pub mod source_map;
pub mod error;
pub mod test_integration;
pub mod tailwind_processor;
pub mod purger;
pub mod autoprefixer;
pub mod import_processor;
pub mod css_optimizer;
pub mod enhanced_plugin_loader;
pub mod advanced_features;

// Re-export main types
pub use engine::{PostCSSEngine, PostCSSConfig, ProcessedCSS};
pub use ast::{CSSNode, CSSRule, CSSDeclaration, CSSAtRule};
pub use parser::{CSSParser, ParseOptions};
pub use transformer::{CSSTransformer, TransformOptions};
pub use js_bridge::{JSBridge, JSRuntime};
pub use plugin_loader::{PluginLoader, PluginConfig, PluginResult};
pub use source_map::{SourceMapGenerator, SourceMap};
pub use error::{PostCSSError, Result};
pub use tailwind_processor::{TailwindProcessor, TailwindConfig, ProcessingResult};
pub use purger::{CSSPurger, PurgeConfig, PurgeResult, PurgeOptions};
pub use autoprefixer::{Autoprefixer, AutoprefixerConfig, PrefixResult, PrefixOptions, PrefixStatistics};
pub use import_processor::{ImportProcessor, ImportConfig, ImportResult, ImportOptions, ImportStatistics};
pub use css_optimizer::{CSSOptimizer, OptimizationConfig, OptimizationResult, OptimizationMetrics};
pub use enhanced_plugin_loader::{EnhancedPluginLoader, PluginInstance, PluginMetrics};
pub use advanced_features::{CSSLinter, LinterConfig, LintResult, AdvancedSourceMapGenerator, PostCSSPerformanceMonitor, PostCSSDevTools};

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
