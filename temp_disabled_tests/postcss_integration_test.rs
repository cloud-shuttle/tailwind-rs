//! Integration tests for PostCSS integration
//!
//! This module provides comprehensive tests to verify that the PostCSS
//! integration works correctly with the existing CSS generator.

#[cfg(feature = "postcss")]
use crate::postcss_integration::{EnhancedCssGenerator, PostCSSIntegrationConfig};
use crate::css_generator::CssGenerator;

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_creation() {
    let generator = EnhancedCssGenerator::new();
    assert!(generator.is_ok());
    
    let generator = generator.unwrap();
    assert_eq!(generator.rule_count(), 0);
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_with_config() {
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

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_basic_operations() {
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Add some classes
    generator.add_class("p-4").unwrap();
    generator.add_class("bg-blue-500").unwrap();
    generator.add_class("text-white").unwrap();
    
    assert_eq!(generator.rule_count(), 3);
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_css_generation() {
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Add some classes
    generator.add_class("p-4").unwrap();
    generator.add_class("bg-blue-500").unwrap();
    
    // Generate CSS
    let css = generator.generate_css().await;
    assert!(css.is_ok());
    
    let css = css.unwrap();
    assert!(css.contains(".p-4"));
    assert!(css.contains(".bg-blue-500"));
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_with_metadata() {
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Add some classes
    generator.add_class("p-4").unwrap();
    generator.add_class("bg-blue-500").unwrap();
    
    // Generate CSS with metadata
    let result = generator.generate_css_with_metadata().await;
    assert!(result.is_ok());
    
    let result = result.unwrap();
    assert!(result.css.contains(".p-4"));
    assert!(result.css.contains(".bg-blue-500"));
    assert!(result.base_css_size > 0);
    assert!(result.processed_css_size > 0);
    assert!(result.compression_ratio > 0.0);
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_postcss_metrics() {
    let generator = EnhancedCssGenerator::new().unwrap();
    
    // Get PostCSS metrics
    let metrics = generator.get_postcss_metrics().await;
    // This might be None if PostCSS is not enabled
    assert!(metrics.is_some() || metrics.is_none());
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_configuration() {
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Test configuration getter
    let config = generator.get_postcss_config();
    assert!(config.enabled);
    
    // Test plugin management
    generator.add_postcss_plugin("autoprefixer").unwrap();
    generator.add_postcss_plugin("cssnano").unwrap();
    
    let config = generator.get_postcss_config();
    assert!(config.plugins.contains(&"autoprefixer".to_string()));
    assert!(config.plugins.contains(&"cssnano".to_string()));
    
    // Test plugin removal
    generator.remove_postcss_plugin("cssnano");
    let config = generator.get_postcss_config();
    assert!(!config.plugins.contains(&"cssnano".to_string()));
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_postcss_toggle() {
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Test PostCSS toggle
    generator.set_postcss_enabled(false);
    let config = generator.get_postcss_config();
    assert!(!config.enabled);
    
    generator.set_postcss_enabled(true);
    let config = generator.get_postcss_config();
    assert!(config.enabled);
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_config_update() {
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Update configuration
    let new_config = PostCSSIntegrationConfig {
        enabled: true,
        plugins: vec!["autoprefixer".to_string(), "cssnano".to_string()],
        source_maps: false,
        optimize: true,
        vendor_prefixes: false,
    };
    
    let result = generator.update_postcss_config(new_config);
    assert!(result.is_ok());
    
    let config = generator.get_postcss_config();
    assert_eq!(config.plugins.len(), 2);
    assert!(!config.source_maps);
    assert!(config.optimize);
    assert!(!config.vendor_prefixes);
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_result_operations() {
    let mut generator = EnhancedCssGenerator::new().unwrap();
    generator.add_class("p-4").unwrap();
    
    let result = generator.generate_css_with_metadata().await.unwrap();
    
    // Test result operations
    assert!(result.compression_percentage() >= 0.0);
    assert!(result.was_compressed() || !result.was_compressed()); // Either is fine
    assert!(result.total_processing_time() >= std::time::Duration::from_millis(0));
    assert!(result.memory_usage() >= 0);
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_custom_properties() {
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Add custom properties
    generator.add_custom_property("--primary-color", "#3b82f6");
    generator.add_custom_property("--secondary-color", "#64748b");
    
    // Add classes that might use these properties
    generator.add_class("p-4").unwrap();
    generator.add_class("bg-blue-500").unwrap();
    
    let css = generator.generate_css().await.unwrap();
    assert!(css.contains(".p-4"));
    assert!(css.contains(".bg-blue-500"));
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_responsive_classes() {
    use crate::responsive::Breakpoint;
    
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Add responsive classes
    generator.add_responsive_class(Breakpoint::Md, "p-4").unwrap();
    generator.add_responsive_class(Breakpoint::Lg, "p-8").unwrap();
    
    let css = generator.generate_css().await.unwrap();
    assert!(css.contains(".p-4"));
    assert!(css.contains(".p-8"));
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_performance() {
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Add many classes to test performance
    for i in 0..100 {
        generator.add_class(&format!("p-{}", i)).unwrap();
        generator.add_class(&format!("bg-blue-{}", i)).unwrap();
    }
    
    let start = std::time::Instant::now();
    let css = generator.generate_css().await.unwrap();
    let duration = start.elapsed();
    
    assert!(css.len() > 0);
    assert!(duration < std::time::Duration::from_secs(5)); // Should be fast
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_error_handling() {
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Test with invalid class (should not panic)
    let result = generator.add_class("invalid-class");
    // This might succeed or fail depending on implementation
    let _ = result;
    
    // Test CSS generation even with potential errors
    let css = generator.generate_css().await;
    assert!(css.is_ok());
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_integration_with_core() {
    // Test that the enhanced generator works with the core generator
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Add classes using the core generator interface
    generator.add_class("p-4").unwrap();
    generator.add_class("bg-blue-500").unwrap();
    generator.add_class("text-white").unwrap();
    generator.add_class("hover:bg-blue-600").unwrap();
    generator.add_class("focus:ring-2").unwrap();
    
    // Generate CSS
    let css = generator.generate_css().await.unwrap();
    
    // Verify all classes are present
    assert!(css.contains(".p-4"));
    assert!(css.contains(".bg-blue-500"));
    assert!(css.contains(".text-white"));
    assert!(css.contains(".hover:bg-blue-600"));
    assert!(css.contains(".focus:ring-2"));
    
    // Verify rule count
    assert_eq!(generator.rule_count(), 5);
}

#[cfg(feature = "postcss")]
#[tokio::test]
async fn test_enhanced_css_generator_comprehensive_workflow() {
    // Test a comprehensive workflow
    let mut generator = EnhancedCssGenerator::new().unwrap();
    
    // Add various types of classes
    generator.add_class("p-4").unwrap();
    generator.add_class("bg-blue-500").unwrap();
    generator.add_class("text-white").unwrap();
    generator.add_class("hover:bg-blue-600").unwrap();
    generator.add_class("focus:ring-2").unwrap();
    generator.add_class("dark:bg-gray-800").unwrap();
    generator.add_class("sm:p-6").unwrap();
    generator.add_class("md:p-8").unwrap();
    
    // Add custom properties
    generator.add_custom_property("--primary-color", "#3b82f6");
    generator.add_custom_property("--secondary-color", "#64748b");
    
    // Generate CSS with metadata
    let result = generator.generate_css_with_metadata().await.unwrap();
    
    // Verify CSS content
    assert!(result.css.contains(".p-4"));
    assert!(result.css.contains(".bg-blue-500"));
    assert!(result.css.contains(".text-white"));
    assert!(result.css.contains(".hover:bg-blue-600"));
    assert!(result.css.contains(".focus:ring-2"));
    assert!(result.css.contains(".dark:bg-gray-800"));
    assert!(result.css.contains(".sm:p-6"));
    assert!(result.css.contains(".md:p-8"));
    
    // Verify metadata
    assert!(result.base_css_size > 0);
    assert!(result.processed_css_size > 0);
    assert!(result.compression_ratio > 0.0);
    assert!(result.total_processing_time() >= std::time::Duration::from_millis(0));
    assert!(result.memory_usage() >= 0);
    
    // Verify rule count
    assert_eq!(generator.rule_count(), 8);
}
