//! Example: Advanced PostCSS Features
//! 
//! This example demonstrates how to use the advanced PostCSS features including
//! CSS linting, source maps, performance monitoring, and development tools.

use tailwind_rs_postcss::{
    CSSLinter, LinterConfig, LintOptions, LintResult,
    AdvancedSourceMapGenerator, SourceFile, SourceMap,
    PostCSSPerformanceMonitor, PerformanceMetrics,
    PostCSSDevTools, DebugResult, AnalysisResult
};
use std::time::SystemTime;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Advanced PostCSS Features Demo");
    println!("=================================");
    
    // Example 1: CSS Linting
    println!("\n📝 Example 1: CSS Linting");
    let config = LinterConfig::default();
    let linter = CSSLinter::new(config);
    
    let css_with_issues = r#"
        .test { color: red; }
        .test { color: blue; }  /* Duplicate selector */
        .empty { }              /* Empty rule */
        .important { color: red !important; }  /* Important declaration */
        #id .class .class .class { color: red; }  /* High specificity */
    "#;
    
    let lint_result = linter.lint_css(css_with_issues, &LintOptions::default())?;
    println!("✅ CSS linting completed");
    println!("   - Total issues: {}", lint_result.statistics.total_issues);
    println!("   - Errors: {}", lint_result.statistics.error_count);
    println!("   - Warnings: {}", lint_result.statistics.warning_count);
    println!("   - Fixable issues: {}", lint_result.statistics.fixable_count);
    
    for issue in &lint_result.issues {
        println!("   - {}: {} (line {}, col {})", 
                 issue.rule, issue.message, issue.line, issue.column);
    }
    
    // Example 2: Advanced Source Maps
    println!("\n📝 Example 2: Advanced Source Maps");
    let mut source_map_generator = AdvancedSourceMapGenerator::new();
    
    let source_files = vec![
        SourceFile {
            path: "input.css".to_string(),
            content: css_with_issues.to_string(),
            mtime: SystemTime::now(),
            size: css_with_issues.len(),
        }
    ];
    
    let source_map = source_map_generator.generate_source_map(css_with_issues, &source_files)?;
    println!("✅ Source map generated");
    println!("   - Version: {}", source_map.version);
    println!("   - Sources: {}", source_map.sources.len());
    println!("   - Mappings length: {}", source_map.mappings.len());
    
    // Example 3: Performance Monitoring
    println!("\n📝 Example 3: Performance Monitoring");
    let mut monitor = PostCSSPerformanceMonitor::new();
    
    // Start monitoring an operation
    let timer = monitor.start_monitoring("css_processing");
    
    // Simulate CSS processing
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Record metrics
    let metrics = tailwind_rs_postcss::OperationMetrics {
        operation: "css_processing".to_string(),
        duration: timer.elapsed(),
        memory_delta: 1024,
        cpu_usage: 0.5,
        input_size: css_with_issues.len(),
        output_size: css_with_issues.len() * 2,
    };
    
    monitor.record_metrics("css_processing", metrics);
    
    let report = monitor.generate_report();
    println!("✅ Performance monitoring completed");
    println!("   - Total time: {:?}", report.total_time);
    println!("   - Memory usage: {} bytes", report.memory_usage);
    println!("   - CPU usage: {:.2}%", report.cpu_usage * 100.0);
    println!("   - Operations: {}", report.operations.len());
    println!("   - Alerts: {}", report.alerts.len());
    
    // Example 4: Development Tools
    println!("\n📝 Example 4: Development Tools");
    let dev_tools = PostCSSDevTools::new();
    
    // CSS Inspection
    let inspection = dev_tools.inspect_css(css_with_issues)?;
    println!("✅ CSS inspection completed");
    println!("   - Total rules: {}", inspection.total_rules);
    println!("   - Total properties: {}", inspection.total_properties);
    println!("   - Complexity score: {:.2}", inspection.complexity_score);
    println!("   - Maintainability score: {:.2}", inspection.maintainability_score);
    println!("   - Issues found: {}", inspection.issues.len());
    
    for issue in &inspection.issues {
        println!("     - {}", issue);
    }
    
    for recommendation in &inspection.recommendations {
        println!("     - Recommendation: {}", recommendation);
    }
    
    // CSS Analysis
    let analysis = dev_tools.analyze_css_structure(css_with_issues)?;
    println!("✅ CSS analysis completed");
    println!("   - Selectors: {}", analysis.analysis.selectors.total_selectors);
    println!("   - Properties: {}", analysis.analysis.properties.total_properties);
    println!("   - Max specificity: {}", analysis.analysis.specificity.max_specificity);
    println!("   - Avg specificity: {:.2}", analysis.analysis.specificity.avg_specificity);
    println!("   - Performance complexity: {:.2}", analysis.analysis.performance.complexity_score);
    println!("   - Maintainability: {:.2}", analysis.analysis.maintainability.maintainability_score);
    
    // Example 5: CSS Formatting
    println!("\n📝 Example 5: CSS Formatting");
    let unformatted_css = ".test{color:red;background:blue;}.another{font-size:14px;}";
    let format_options = tailwind_rs_postcss::FormatOptions::default();
    
    let formatted_css = dev_tools.format_css(unformatted_css, &format_options)?;
    println!("✅ CSS formatting completed");
    println!("   - Original length: {} chars", unformatted_css.len());
    println!("   - Formatted length: {} chars", formatted_css.len());
    println!("   - Formatted CSS:");
    println!("{}", formatted_css);
    
    // Example 6: Lint Fixes
    println!("\n📝 Example 6: Lint Fixes");
    if !lint_result.fixes.is_empty() {
        let fixed_css = linter.fix_css(css_with_issues, &lint_result.fixes)?;
        println!("✅ CSS fixes applied");
        println!("   - Fixes applied: {}", lint_result.fixes.len());
        println!("   - Fixed CSS length: {} chars", fixed_css.len());
    } else {
        println!("ℹ️  No automatic fixes available");
    }
    
    // Example 7: Source Map JSON
    println!("\n📝 Example 7: Source Map JSON");
    let source_map_json = source_map.to_json()?;
    println!("✅ Source map JSON generated");
    println!("   - JSON length: {} chars", source_map_json.len());
    
    // Parse it back
    let parsed_source_map = SourceMap::from_json(&source_map_json)?;
    println!("   - Successfully parsed back from JSON");
    println!("   - Version: {}", parsed_source_map.version);
    
    // Example 8: Performance Alerts
    println!("\n📝 Example 8: Performance Alerts");
    if !report.alerts.is_empty() {
        println!("⚠️  Performance alerts detected:");
        for alert in &report.alerts {
            println!("   - {}: {} ({:?})", 
                     alert.operation, alert.issue, alert.severity);
        }
    } else {
        println!("✅ No performance alerts");
    }
    
    // Example 9: Recommendations
    println!("\n📝 Example 9: Recommendations");
    if !report.recommendations.is_empty() {
        println!("💡 Performance recommendations:");
        for recommendation in &report.recommendations {
            println!("   - {}", recommendation);
        }
    }
    
    if !analysis.recommendations.is_empty() {
        println!("💡 Analysis recommendations:");
        for recommendation in &analysis.recommendations {
            println!("   - {}", recommendation);
        }
    }
    
    // Example 10: Advanced Linting Rules
    println!("\n📝 Example 10: Advanced Linting Rules");
    let mut advanced_config = LinterConfig::default();
    advanced_config.auto_fix = true;
    
    let advanced_linter = CSSLinter::new(advanced_config);
    let advanced_result = advanced_linter.lint_css(css_with_issues, &LintOptions {
        auto_fix: true,
        ignore_warnings: false,
        max_issues: Some(10),
        include_patterns: vec!["*.css".to_string()],
        exclude_patterns: vec![],
    })?;
    
    println!("✅ Advanced linting completed");
    println!("   - Issues found: {}", advanced_result.statistics.total_issues);
    println!("   - Suggestions: {}", advanced_result.suggestions.len());
    
    for suggestion in &advanced_result.suggestions {
        println!("   - Suggestion: {} (line {})", 
                 suggestion.message, suggestion.line);
    }
    
    println!("\n🎉 All advanced PostCSS features examples completed successfully!");
    println!("=================================================================");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_css_linting() {
        let config = LinterConfig::default();
        let linter = CSSLinter::new(config);
        let css = ".test { color: red; } .test { color: blue; }";
        let result = linter.lint_css(css, &LintOptions::default());
        assert!(result.is_ok());
        
        let lint_result = result.unwrap();
        assert!(!lint_result.issues.is_empty());
    }
    
    #[test]
    fn test_source_map_generation() {
        let mut generator = AdvancedSourceMapGenerator::new();
        let css = ".test { color: red; }";
        let source_files = vec![
            SourceFile {
                path: "input.css".to_string(),
                content: css.to_string(),
                mtime: SystemTime::now(),
                size: css.len(),
            }
        ];
        
        let result = generator.generate_source_map(css, &source_files);
        assert!(result.is_ok());
        
        let source_map = result.unwrap();
        assert_eq!(source_map.version, 3);
    }
    
    #[test]
    fn test_performance_monitoring() {
        let mut monitor = PostCSSPerformanceMonitor::new();
        let timer = monitor.start_monitoring("test_operation");
        
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        let metrics = tailwind_rs_postcss::OperationMetrics {
            operation: "test_operation".to_string(),
            duration: timer.elapsed(),
            memory_delta: 1024,
            cpu_usage: 0.5,
            input_size: 100,
            output_size: 200,
        };
        
        monitor.record_metrics("test_operation", metrics);
        let report = monitor.generate_report();
        assert!(report.operations.contains_key("test_operation"));
    }
    
    #[test]
    fn test_dev_tools() {
        let tools = PostCSSDevTools::new();
        let css = ".test { color: red; }";
        let result = tools.inspect_css(css);
        assert!(result.is_ok());
        
        let inspection = result.unwrap();
        assert_eq!(inspection.total_rules, 1);
    }
    
    #[test]
    fn test_css_formatting() {
        let tools = PostCSSDevTools::new();
        let css = ".test{color:red;}";
        let options = tailwind_rs_postcss::FormatOptions::default();
        let result = tools.format_css(css, &options);
        assert!(result.is_ok());
        
        let formatted = result.unwrap();
        assert!(formatted.contains('\n'));
    }
    
    #[test]
    fn test_css_analysis() {
        let tools = PostCSSDevTools::new();
        let css = ".test { color: red; }";
        let result = tools.analyze_css_structure(css);
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        assert_eq!(analysis.analysis.selectors.total_selectors, 1);
    }
}
