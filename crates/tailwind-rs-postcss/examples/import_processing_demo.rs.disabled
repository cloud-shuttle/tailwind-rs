//! Example: @import Processing System
//! 
//! This example demonstrates how to use the ImportProcessor to handle @import statements,
//! resolve dependencies, and optimize import usage in CSS.

use tailwind_rs_postcss::{ImportProcessor, ImportConfig, ImportOptions, CircularHandling};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 @import Processing System Demo");
    println!("=================================");
    
    // Example 1: Basic import processing
    println!("\n📝 Example 1: Basic import processing");
    let mut processor = ImportProcessor::new();
    
    let css = r#"
        @import 'base.css';
        @import 'components.css';
        
        .custom {
            color: red;
        }
    "#;
    
    let result = processor.process_imports(css, "/path/to/base");
    println!("✅ Basic import processing completed");
    println!("   - Result: {}", if result.is_ok() { "Success" } else { "Failed (expected - no files)" });
    
    // Example 2: Advanced import processing with options
    println!("\n📝 Example 2: Advanced import processing with options");
    let mut advanced_processor = ImportProcessor::new();
    
    let complex_css = r#"
        @import 'reset.css' screen;
        @import 'base.css' print;
        @import 'components.css';
        @import 'utilities.css';
        
        .main-content {
            display: flex;
            flex-direction: column;
        }
        
        .sidebar {
            flex: 0 0 250px;
        }
        
        .content {
            flex: 1;
        }
    "#;
    
    let options = ImportOptions {
        search_paths: vec!["/path/to/css".to_string(), "/path/to/styles".to_string()],
        extensions: vec![".css".to_string(), ".scss".to_string(), ".sass".to_string()],
        inline_imports: true,
        preserve_imports: false,
        optimize_imports: true,
        handle_circular: CircularHandling::Warn,
        max_depth: 5,
        source_map: true,
    };
    
    let advanced_result = advanced_processor.process_imports_advanced(complex_css, &options);
    println!("✅ Advanced import processing completed");
    println!("   - Result: {}", if advanced_result.is_ok() { "Success" } else { "Failed (expected - no files)" });
    
    // Example 3: Import optimization
    println!("\n📝 Example 3: Import optimization");
    let processor = ImportProcessor::new();
    
    let css_with_duplicates = r#"
        @import 'base.css';
        @import 'base.css';  // Duplicate
        @import 'components.css';
        @import 'base.css';  // Another duplicate
        @import 'utilities.css';
        
        .optimized {
            color: blue;
        }
    "#;
    
    let optimized_result = processor.optimize_imports(css_with_duplicates)?;
    println!("✅ Import optimization completed");
    println!("   - Original imports: {}", css_with_duplicates.matches("@import").count());
    println!("   - Optimized imports: {}", optimized_result.matches("@import").count());
    println!("   - Duplicates removed: {}", 
        css_with_duplicates.matches("@import").count() - optimized_result.matches("@import").count());
    
    // Example 4: Circular dependency detection
    println!("\n📝 Example 4: Circular dependency detection");
    let mut circular_processor = ImportProcessor::new();
    
    let css_with_circular = r#"
        @import 'a.css';  // a.css imports b.css, b.css imports a.css
    "#;
    
    let circular_result = circular_processor.process_imports(css_with_circular, "/path/to/base");
    println!("✅ Circular dependency detection completed");
    println!("   - Result: {}", if circular_result.is_ok() { "Success" } else { "Failed (expected - circular dependency)" });
    
    // Example 5: Media query handling
    println!("\n📝 Example 5: Media query handling");
    let mut media_processor = ImportProcessor::new();
    
    let css_with_media = r#"
        @import 'mobile.css' screen and (max-width: 768px);
        @import 'desktop.css' screen and (min-width: 769px);
        @import 'print.css' print;
        
        .responsive {
            width: 100%;
        }
    "#;
    
    let media_result = media_processor.process_imports(css_with_media, "/path/to/base");
    println!("✅ Media query handling completed");
    println!("   - Result: {}", if media_result.is_ok() { "Success" } else { "Failed (expected - no files)" });
    
    // Example 6: Import path resolution
    println!("\n📝 Example 6: Import path resolution");
    let processor = ImportProcessor::new();
    
    // Test different path types
    let path_tests = vec![
        ("relative.css", "/path/to/base"),
        ("../parent.css", "/path/to/base"),
        ("/absolute.css", "/path/to/base"),
        ("https://cdn.example.com/external.css", "/path/to/base"),
        ("//cdn.example.com/protocol-relative.css", "/path/to/base"),
    ];
    
    for (import_path, base_path) in path_tests {
        let resolved = processor.resolve_import_path(import_path, base_path);
        println!("   - {} -> {}", import_path, 
            if resolved.is_ok() { "Resolved" } else { "Failed (expected - no files)" });
    }
    
    // Example 7: Performance testing
    println!("\n📝 Example 7: Performance testing");
    let mut perf_processor = ImportProcessor::new();
    let large_css = generate_large_css_with_imports();
    
    let start_time = std::time::Instant::now();
    let perf_result = perf_processor.process_imports(&large_css, "/path/to/base");
    let processing_time = start_time.elapsed().as_millis();
    
    println!("✅ Performance test completed");
    println!("   - Large CSS size: {} bytes", large_css.len());
    println!("   - Processing time: {}ms", processing_time);
    println!("   - Result: {}", if perf_result.is_ok() { "Success" } else { "Failed (expected - no files)" });
    
    // Example 8: Configuration testing
    println!("\n📝 Example 8: Configuration testing");
    let mut config = ImportConfig::default();
    config.inline_imports = false;
    config.preserve_imports = true;
    config.optimize_imports = false;
    config.handle_circular = CircularHandling::Error;
    config.max_depth = 3;
    
    let config_processor = ImportProcessor::with_config(config);
    let config_css = r#"
        @import 'config-test.css';
        .configured { color: green; }
    "#;
    
    let config_result = config_processor.process_imports(config_css, "/path/to/base");
    println!("✅ Configuration testing completed");
    println!("   - Inline imports: {}", config_processor.config.inline_imports);
    println!("   - Preserve imports: {}", config_processor.config.preserve_imports);
    println!("   - Optimize imports: {}", config_processor.config.optimize_imports);
    println!("   - Handle circular: {:?}", config_processor.config.handle_circular);
    println!("   - Max depth: {}", config_processor.config.max_depth);
    println!("   - Result: {}", if config_result.is_ok() { "Success" } else { "Failed (expected - no files)" });
    
    println!("\n🎉 All @import processing examples completed successfully!");
    println!("========================================================");
    
    Ok(())
}

/// Generate large CSS with multiple imports for performance testing
fn generate_large_css_with_imports() -> String {
    let mut css = String::new();
    
    // Generate 100 import statements
    for i in 0..100 {
        css.push_str(&format!("@import 'import-{}.css';\n", i));
    }
    
    // Add some CSS content
    for i in 0..50 {
        css.push_str(&format!(
            r#".class-{} {{
                color: #{:06x};
                background: #{:06x};
                border: 1px solid #{:06x};
                margin: {}px;
                padding: {}px;
            }}
            "#,
            i,
            i * 1000 % 0xffffff,
            i * 2000 % 0xffffff,
            i * 3000 % 0xffffff,
            i * 2,
            i * 3
        ));
    }
    
    css
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_import_statement_parsing() {
        let processor = ImportProcessor::new();
        let line = "@import 'styles.css' screen and (max-width: 768px);";
        let statement = processor.parse_import_statement(line);
        assert!(statement.is_ok());
        
        let statement = statement.unwrap();
        assert_eq!(statement.import_path, "styles.css");
        assert_eq!(statement.media_query, Some("screen and (max-width: 768px)".to_string()));
    }
    
    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        
        // Add dependencies
        assert!(graph.add_dependency("a.css", "b.css").is_ok());
        assert!(graph.add_dependency("b.css", "c.css").is_ok());
        
        // Test circular dependency
        assert!(graph.add_dependency("c.css", "a.css").is_err());
    }
    
    #[test]
    fn test_import_cache() {
        let mut cache = ImportCache::new();
        cache.cache_content("test.css".to_string(), "body { color: red; }".to_string());
        
        let cached = cache.get_cached_content("test.css");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap(), "body { color: red; }");
    }
    
    #[test]
    fn test_import_optimization() {
        let processor = ImportProcessor::new();
        let css = r#"
            @import 'styles.css';
            @import 'styles.css';
            @import 'components.css';
            .custom { color: red; }
        "#;
        
        let result = processor.optimize_imports(css);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        let import_count = optimized.matches("@import").count();
        assert_eq!(import_count, 2); // Duplicate removed
    }
    
    #[test]
    fn test_import_config() {
        let config = ImportConfig::default();
        assert_eq!(config.inline_imports, true);
        assert_eq!(config.preserve_imports, false);
        assert_eq!(config.optimize_imports, true);
        assert_eq!(config.max_depth, 10);
    }
    
    #[test]
    fn test_import_options() {
        let options = ImportOptions {
            search_paths: vec!["/path1".to_string(), "/path2".to_string()],
            extensions: vec![".css".to_string()],
            inline_imports: true,
            preserve_imports: false,
            optimize_imports: true,
            handle_circular: CircularHandling::Warn,
            max_depth: 5,
            source_map: true,
        };
        
        assert_eq!(options.search_paths.len(), 2);
        assert_eq!(options.extensions.len(), 1);
        assert_eq!(options.inline_imports, true);
        assert_eq!(options.source_map, true);
    }
}
