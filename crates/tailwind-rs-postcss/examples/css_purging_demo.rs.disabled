//! Example: CSS Purging System
//! 
//! This example demonstrates how to use the CSSPurger to remove unused CSS classes,
//! essential for Tailwind CSS production builds.

use tailwind_rs_postcss::{CSSPurger, PurgeConfig, PurgeOptions, PurgeResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 CSS Purging System Demo");
    println!("==========================");
    
    // Example 1: Basic CSS purging
    println!("\n📝 Example 1: Basic CSS purging");
    let mut purger = CSSPurger::new();
    
    let css = r#"
        .unused-class { color: red; }
        .used-class { color: blue; }
        .another-unused { background: yellow; }
        .btn { padding: 8px 16px; }
    "#;
    
    let content = vec![
        r#"<div class="used-class btn">Click me</div>"#.to_string(),
        r#"<button class="btn">Submit</button>"#.to_string(),
    ];
    
    let result = purger.purge(css, &content)?;
    println!("✅ Purged CSS successfully");
    println!("   - Original size: {} bytes", result.statistics.original_size);
    println!("   - Purged size: {} bytes", result.statistics.purged_size);
    println!("   - Reduction: {:.1}%", result.statistics.reduction_percentage);
    println!("   - Classes removed: {}", result.statistics.classes_removed);
    println!("   - Classes kept: {}", result.statistics.classes_kept);
    println!("   - Processing time: {}ms", result.statistics.processing_time_ms);
    
    // Example 2: Safelist support
    println!("\n📝 Example 2: Safelist support");
    let mut config = PurgeConfig::default();
    config.safelist = vec!["important-class".to_string()];
    let mut safelist_purger = CSSPurger::with_config(config);
    
    let css_with_safelist = r#"
        .important-class { color: red; }
        .unused-class { color: blue; }
    "#;
    
    let content_without_important = vec!["<div>No important class here</div>".to_string()];
    let safelist_result = safelist_purger.purge(css_with_safelist, &content_without_important)?;
    println!("✅ Safelist purging completed");
    println!("   - Important class kept: {}", safelist_result.purged_css.contains("important-class"));
    println!("   - Unused class removed: {}", !safelist_result.purged_css.contains("unused-class"));
    
    // Example 3: Advanced purging with options
    println!("\n📝 Example 3: Advanced purging with options");
    let mut advanced_purger = CSSPurger::new();
    
    let complex_css = r#"
        /* This is a comment */
        .btn {
            @apply px-4 py-2 rounded;
            background-color: blue;
        }
        
        .btn-primary {
            @apply bg-blue-500 text-white;
        }
        
        .btn-secondary {
            @apply bg-gray-500 text-white;
        }
        
        .unused-utility {
            @apply text-red-500;
        }
        
        @media (max-width: 768px) {
            .btn {
                padding: 4px 8px;
            }
        }
        
        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
    "#;
    
    let complex_content = vec![
        r#"<button class="btn btn-primary">Primary Button</button>"#.to_string(),
        r#"const className = `btn ${isPrimary ? 'btn-primary' : 'btn-secondary'}`;"#.to_string(),
        r#"<div class="btn">Generic Button</div>"#.to_string(),
    ];
    
    let options = PurgeOptions {
        aggressive: true,
        preserve_comments: false,
        minify: true,
        source_map: false,
        parallel: true,
    };
    
    let advanced_result = advanced_purger.purge_advanced(complex_css, &options)?;
    println!("✅ Advanced purging completed");
    println!("   - Original size: {} bytes", advanced_result.statistics.original_size);
    println!("   - Purged size: {} bytes", advanced_result.statistics.purged_size);
    println!("   - Reduction: {:.1}%", advanced_result.statistics.reduction_percentage);
    println!("   - Classes removed: {}", advanced_result.statistics.classes_removed);
    println!("   - Classes kept: {}", advanced_result.statistics.classes_kept);
    println!("   - Processing time: {}ms", advanced_result.statistics.processing_time_ms);
    
    // Example 4: Content scanning from different file types
    println!("\n📝 Example 4: Content scanning from different file types");
    let mut scanner_purger = CSSPurger::new();
    
    let html_content = r#"
        <div class="container">
            <h1 class="text-2xl font-bold">Hello World</h1>
            <button class="btn btn-primary">Click me</button>
        </div>
    "#;
    
    let js_content = r#"
        const className = `btn ${isActive ? 'btn-active' : 'btn-inactive'}`;
        const element = <div className="container">{children}</div>;
    "#;
    
    let rust_content = r#"
        let class_name = "btn btn-primary";
        let element = div! { class: "container", "Hello" };
    "#;
    
    let mixed_content = vec![
        html_content.to_string(),
        js_content.to_string(),
        rust_content.to_string(),
    ];
    
    let scanner_result = scanner_purger.scan_content(&mixed_content)?;
    println!("✅ Content scanning completed");
    println!("   - Classes found: {}", scanner_result.len());
    println!("   - Classes: {:?}", scanner_result);
    
    // Example 5: Performance comparison
    println!("\n📝 Example 5: Performance comparison");
    let large_css = generate_large_css();
    let large_content = generate_large_content();
    
    let start_time = std::time::Instant::now();
    let performance_result = purger.purge(&large_css, &large_content)?;
    let processing_time = start_time.elapsed().as_millis();
    
    println!("✅ Performance test completed");
    println!("   - Large CSS size: {} bytes", large_css.len());
    println!("   - Content files: {}", large_content.len());
    println!("   - Processing time: {}ms", processing_time);
    println!("   - Reduction: {:.1}%", performance_result.statistics.reduction_percentage);
    
    println!("\n🎉 All CSS purging examples completed successfully!");
    println!("==================================================");
    
    Ok(())
}

/// Generate large CSS for performance testing
fn generate_large_css() -> String {
    let mut css = String::new();
    
    // Generate 1000 CSS classes
    for i in 0..1000 {
        css.push_str(&format!(
            ".class-{} {{ color: #{:06x}; }}\n",
            i,
            i * 1000 % 0xffffff
        ));
    }
    
    css
}

/// Generate large content for performance testing
fn generate_large_content() -> Vec<String> {
    let mut content = Vec::new();
    
    // Generate 100 HTML files with random classes
    for i in 0..100 {
        let html = format!(
            r#"<div class="container class-{}">
                <h1 class="title class-{}">Title {}</h1>
                <p class="description class-{}">Description {}</p>
            </div>"#,
            i % 50,
            (i + 1) % 50,
            i,
            (i + 2) % 50,
            i
        );
        content.push(html);
    }
    
    content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_purging() {
        let mut purger = CSSPurger::new();
        let css = ".unused { color: red; } .used { color: blue; }";
        let content = vec!["<div class=\"used\"></div>".to_string()];
        let result = purger.purge(css, &content);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(!result.purged_css.contains("unused"));
        assert!(result.purged_css.contains("used"));
    }
    
    #[test]
    fn test_safelist_purging() {
        let mut config = PurgeConfig::default();
        config.safelist = vec!["important".to_string()];
        let mut purger = CSSPurger::with_config(config);
        
        let css = ".important { color: red; }";
        let content = vec!["<div></div>".to_string()];
        let result = purger.purge(css, &content);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(result.purged_css.contains("important"));
    }
    
    #[test]
    fn test_content_scanner() {
        let purger = CSSPurger::new();
        let content = vec![r#"<div class="btn btn-primary">Click me</div>"#.to_string()];
        let classes = purger.scan_content(&content);
        assert!(classes.is_ok());
        
        let classes = classes.unwrap();
        assert!(classes.contains("btn"));
        assert!(classes.contains("btn-primary"));
    }
}
