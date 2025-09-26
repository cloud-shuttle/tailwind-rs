//! Integration test for PostCSS functionality
//!
//! This module provides basic integration tests to verify that the PostCSS
//! engine works correctly with the existing Tailwind-RS Core.

use crate::engine::{PerformanceOptions, SourceMapOptions};
use crate::*;

/// Test basic PostCSS engine functionality
pub async fn test_basic_postcss_processing() -> Result<()> {
    let engine = PostCSSEngine::new(PostCSSConfig::default())?;
    let input_css = ".test { color: red; font-size: 16px; }";
    let result = engine.process_css(input_css).await?;

    assert!(result.css.contains(".test"));
    assert!(result.css.contains("color: red"));
    assert!(result.css.contains("font-size: 16px"));

    Ok(())
}

/// Test CSS parsing functionality
pub fn test_css_parsing() -> Result<()> {
    let parser = CSSParser::new(ParseOptions::default());
    let input = ".test { color: red; }";
    let result = parser.parse(input)?;

    match result {
        CSSNode::Stylesheet(rules) => {
            assert_eq!(rules.len(), 1);
            assert_eq!(rules[0].selector, ".test");
            assert_eq!(rules[0].declarations.len(), 1);
            assert_eq!(rules[0].declarations[0].property, "color");
            assert_eq!(rules[0].declarations[0].value, "red");
        }
        _ => panic!("Expected stylesheet"),
    }

    Ok(())
}

/// Test error handling
pub fn test_error_handling() {
    let parser = CSSParser::new(ParseOptions::default());
    let invalid_css = ".test { color: red; font-size: }"; // Missing value

    // This should either parse successfully (if we're lenient) or fail gracefully
    let result = parser.parse(invalid_css);
    // For now, we'll just ensure it doesn't panic
    let _ = result;
}

/// Test configuration
pub async fn test_configuration() -> Result<()> {
    let config = PostCSSConfig {
        source_map: true,
        source_map_options: SourceMapOptions {
            inline: false,
            file: Some("output.css.map".to_string()),
            source_root: Some("/src".to_string()),
            sources_content: true,
        },
        parser_options: ParseOptions {
            track_positions: true,
            strict_mode: false,
            custom_properties: true,
            nesting: true,
            container_queries: true,
            cascade_layers: true,
            max_nesting_depth: 10,
            plugins: Vec::new(),
        },
        transform_options: TransformOptions {
            optimize: true,
            vendor_prefixes: false,
            flatten_nesting: true,
            resolve_custom_properties: true,
        },
        performance: PerformanceOptions {
            enable_cache: true,
            cache_size_limit: 1000,
            parallel_processing: true,
            memory_optimization: true,
        },
        plugins: Vec::new(),
    };

    let engine = PostCSSEngine::new(config)?;
    assert!(engine.get_metrics().await.plugins_loaded == 0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        test_css_parsing().unwrap();
    }

    #[test]
    fn test_error_handling() {
        let parser = CSSParser::new(ParseOptions::default());
        let invalid_css = ".test { color: red; font-size: }"; // Missing value

        // This should either parse successfully (if we're lenient) or fail gracefully
        let result = parser.parse(invalid_css);
        // For now, we'll just ensure it doesn't panic
        let _ = result;
    }

    #[tokio::test]
    async fn test_configuration_async() {
        test_configuration().await.unwrap();
    }

    #[tokio::test]
    async fn test_basic_processing() {
        test_basic_postcss_processing().await.unwrap();
    }
}
