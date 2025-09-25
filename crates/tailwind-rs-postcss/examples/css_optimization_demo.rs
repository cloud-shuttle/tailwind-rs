//! Example: CSS Optimization System
//! 
//! This example demonstrates how to use the CSSOptimizer to optimize CSS
//! with rule merging, property optimization, selector optimization, and minification.

use tailwind_rs_postcss::{CSSOptimizer, OptimizationConfig, OptimizationLevel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 CSS Optimization System Demo");
    println!("===============================");
    
    // Example 1: Basic CSS optimization
    println!("\n📝 Example 1: Basic CSS optimization");
    let mut optimizer = CSSOptimizer::new();
    
    let css = r#"
        .btn {
            color: red;
            color: blue;
            background: white;
            padding: 10px;
        }
        
        .btn {
            margin: 5px;
            border: 1px solid black;
        }
        
        .container {
            width: 100%;
            height: 100vh;
        }
        
        .container {
            display: flex;
            flex-direction: column;
        }
    "#;
    
    let result = optimizer.optimize(css)?;
    println!("✅ Basic optimization completed");
    println!("   - Original size: {} bytes", result.metrics.original_size);
    println!("   - Optimized size: {} bytes", result.metrics.optimized_size);
    println!("   - Size reduction: {:.1}%", result.metrics.size_reduction_percentage);
    println!("   - Rules merged: {}", result.metrics.rules_merged);
    println!("   - Properties optimized: {}", result.metrics.properties_optimized);
    println!("   - Duplicates removed: {}", result.metrics.duplicates_removed);
    println!("   - Processing time: {}ms", result.metrics.processing_time.as_millis());
    
    // Example 2: Advanced optimization with custom configuration
    println!("\n📝 Example 2: Advanced optimization with custom configuration");
    let mut config = OptimizationConfig::default();
    config.level = OptimizationLevel::Aggressive;
    config.merge_rules = true;
    config.optimize_properties = true;
    config.optimize_selectors = true;
    config.minify = true;
    
    let mut advanced_optimizer = CSSOptimizer::with_config(config);
    
    let complex_css = r#"
        /* This is a comment that will be removed */
        .header {
            background-color: #ffffff;
            color: #000000;
            padding-top: 20px;
            padding-right: 20px;
            padding-bottom: 20px;
            padding-left: 20px;
            margin-top: 10px;
            margin-right: 10px;
            margin-bottom: 10px;
            margin-left: 10px;
        }
        
        .header {
            font-size: 18px;
            font-weight: bold;
        }
        
        .content {
            width: 100%;
            max-width: 1200px;
            margin: 0 auto;
        }
        
        .content {
            padding: 20px;
            background: linear-gradient(45deg, #ff0000, #00ff00);
        }
        
        .sidebar {
            width: 250px;
            height: 100vh;
            position: fixed;
            top: 0;
            left: 0;
        }
        
        .sidebar {
            background: #f5f5f5;
            border-right: 1px solid #cccccc;
        }
        
        .footer {
            text-align: center;
            padding: 20px;
            background: #333333;
            color: #ffffff;
        }
        
        .footer {
            margin-top: 50px;
            border-top: 1px solid #666666;
        }
    "#;
    
    let advanced_result = advanced_optimizer.optimize(complex_css)?;
    println!("✅ Advanced optimization completed");
    println!("   - Original size: {} bytes", advanced_result.metrics.original_size);
    println!("   - Optimized size: {} bytes", advanced_result.metrics.optimized_size);
    println!("   - Size reduction: {:.1}%", advanced_result.metrics.size_reduction_percentage);
    println!("   - Rules merged: {}", advanced_result.metrics.rules_merged);
    println!("   - Properties optimized: {}", advanced_result.metrics.properties_optimized);
    println!("   - Duplicates removed: {}", advanced_result.metrics.duplicates_removed);
    println!("   - Selectors optimized: {}", advanced_result.metrics.selectors_optimized);
    println!("   - Processing time: {}ms", advanced_result.metrics.processing_time.as_millis());
    
    // Example 3: Optimization level comparison
    println!("\n📝 Example 3: Optimization level comparison");
    let test_css = r#"
        .test {
            color: red;
            color: blue;
            background: white;
            padding: 10px;
            margin: 5px;
        }
        
        .test {
            border: 1px solid black;
            font-size: 14px;
        }
        
        .another {
            width: 100%;
            height: 50px;
        }
    "#;
    
    // Test different optimization levels
    let levels = vec![
        (OptimizationLevel::Minimal, "Minimal"),
        (OptimizationLevel::Standard, "Standard"),
        (OptimizationLevel::Aggressive, "Aggressive"),
    ];
    
    for (level, name) in levels {
        let mut config = OptimizationConfig::default();
        config.level = level.clone();
        
        let mut optimizer = CSSOptimizer::with_config(config);
        let result = optimizer.optimize(test_css)?;
        
        println!("   - {} optimization:", name);
        println!("     * Size reduction: {:.1}%", result.metrics.size_reduction_percentage);
        println!("     * Rules merged: {}", result.metrics.rules_merged);
        println!("     * Properties optimized: {}", result.metrics.properties_optimized);
        println!("     * Processing time: {}ms", result.metrics.processing_time.as_millis());
    }
    
    // Example 4: Performance testing with large CSS
    println!("\n📝 Example 4: Performance testing with large CSS");
    let large_css = generate_large_css();
    let start_time = std::time::Instant::now();
    
    let mut perf_optimizer = CSSOptimizer::new();
    let perf_result = perf_optimizer.optimize(&large_css)?;
    
    let total_time = start_time.elapsed();
    println!("✅ Performance test completed");
    println!("   - Large CSS size: {} bytes", large_css.len());
    println!("   - Optimized size: {} bytes", perf_result.metrics.optimized_size);
    println!("   - Size reduction: {:.1}%", perf_result.metrics.size_reduction_percentage);
    println!("   - Total processing time: {}ms", total_time.as_millis());
    println!("   - Optimization time: {}ms", perf_result.metrics.processing_time.as_millis());
    
    // Example 5: CSS analysis
    println!("\n📝 Example 5: CSS analysis");
    let analysis_css = r#"
        .btn {
            color: red;
            background: blue;
            padding: 10px;
        }
        
        .btn:hover {
            color: white;
            background: darkblue;
        }
        
        .container {
            width: 100%;
            height: 100vh;
            display: flex;
            flex-direction: column;
        }
        
        .content {
            flex: 1;
            padding: 20px;
        }
    "#;
    
    let mut analysis_optimizer = CSSOptimizer::new();
    let analysis_result = analysis_optimizer.optimize(analysis_css)?;
    
    println!("✅ CSS analysis completed");
    println!("   - Total rules: {}", analysis_result.analysis.rules.total_rules);
    println!("   - Empty rules: {}", analysis_result.analysis.rules.empty_rules.len());
    println!("   - Duplicate rules: {}", analysis_result.analysis.rules.duplicate_rules.len());
    println!("   - Total properties: {}", analysis_result.analysis.properties.total_properties);
    println!("   - Duplicate properties: {}", analysis_result.analysis.properties.duplicate_properties.len());
    println!("   - Total selectors: {}", analysis_result.analysis.selectors.total_selectors);
    println!("   - Redundant selectors: {}", analysis_result.analysis.selectors.redundant_selectors.len());
    println!("   - Analysis time: {}ms", analysis_result.analysis.analysis_time.as_millis());
    
    // Example 6: Minification demonstration
    println!("\n📝 Example 6: Minification demonstration");
    let minify_css = r#"
        /* This comment will be removed */
        .test {
            color: #ff0000;
            background: #ffffff;
            padding: 10px 20px;
            margin: 5px;
        }
        
        .test:hover {
            color: #00ff00;
            background: #000000;
        }
    "#;
    
    let mut minify_optimizer = CSSOptimizer::new();
    let minify_result = minify_optimizer.optimize(minify_css)?;
    
    println!("✅ Minification demonstration completed");
    println!("   - Original CSS:");
    println!("{}", minify_css);
    println!("   - Minified CSS:");
    println!("{}", minify_result.optimized_css);
    println!("   - Size reduction: {:.1}%", minify_result.metrics.size_reduction_percentage);
    
    println!("\n🎉 All CSS optimization examples completed successfully!");
    println!("=====================================================");
    
    Ok(())
}

/// Generate large CSS for performance testing
fn generate_large_css() -> String {
    let mut css = String::new();
    
    // Generate 1000 CSS rules with various properties
    for i in 0..1000 {
        css.push_str(&format!(
            r#".class-{} {{
                color: #{:06x};
                background: #{:06x};
                padding: {}px;
                margin: {}px;
                border: 1px solid #{:06x};
                font-size: {}px;
            }}
            "#,
            i,
            i * 1000 % 0xffffff,
            i * 2000 % 0xffffff,
            i * 2,
            i * 3,
            i * 4000 % 0xffffff,
            i % 20 + 10
        ));
        
        // Add some duplicate rules for optimization
        if i % 10 == 0 {
            css.push_str(&format!(
                r#".class-{} {{
                    color: #{:06x};
                    background: #{:06x};
                }}
                "#,
                i,
                i * 1000 % 0xffffff,
                i * 2000 % 0xffffff
            ));
        }
    }
    
    css
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_optimization() {
        let mut optimizer = CSSOptimizer::new();
        let css = r#"
            .test {
                color: red;
                color: blue;
            }
        "#;
        
        let result = optimizer.optimize(css);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(result.metrics.duplicates_removed > 0);
        assert!(result.optimized_css.len() < css.len());
    }
    
    #[test]
    fn test_rule_merging() {
        let mut optimizer = CSSOptimizer::new();
        let css = r#"
            .btn { color: red; }
            .btn { background: blue; }
        "#;
        
        let result = optimizer.optimize(css);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(result.metrics.rules_merged > 0);
    }
    
    #[test]
    fn test_property_optimization() {
        let mut optimizer = CSSOptimizer::new();
        let css = r#"
            .test {
                color: red;
                color: blue;
                margin-top: 10px;
                margin-right: 10px;
                margin-bottom: 10px;
                margin-left: 10px;
            }
        "#;
        
        let result = optimizer.optimize(css);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(result.metrics.properties_optimized > 0);
    }
    
    #[test]
    fn test_css_minification() {
        let mut optimizer = CSSOptimizer::new();
        let css = r#"
            .test {
                color: red;
                background: blue;
            }
        "#;
        
        let result = optimizer.optimize(css);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(result.optimized_css.len() < css.len());
    }
    
    #[test]
    fn test_optimization_config() {
        let config = OptimizationConfig::default();
        assert!(config.merge_rules);
        assert!(config.optimize_properties);
        assert!(config.optimize_selectors);
        assert!(config.minify);
        assert_eq!(config.level, OptimizationLevel::Standard);
    }
    
    #[test]
    fn test_optimization_metrics() {
        let metrics = OptimizationMetrics::new();
        assert_eq!(metrics.original_size, 0);
        assert_eq!(metrics.optimized_size, 0);
        assert_eq!(metrics.size_reduction_percentage, 0.0);
        assert_eq!(metrics.rules_merged, 0);
        assert_eq!(metrics.properties_optimized, 0);
        assert_eq!(metrics.duplicates_removed, 0);
        assert_eq!(metrics.selectors_optimized, 0);
    }
}
