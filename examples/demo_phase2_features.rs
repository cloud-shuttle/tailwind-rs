//! Demo of Phase 2 features: Source Scanning, Tree-Shaking, Optimization, and Plugins
//! 
//! This demonstrates the advanced features implemented in Phase 2 of the Tailwind-RS remediation plan.

use tailwind_rs_core::{
    AstParser, ClassScanner, CssGenerator, CssOptimizer, PluginRegistry, TreeShaker,
    Breakpoint, OptimizationConfig, TreeShakeConfig, ScanConfig,
    plugin_system::{CustomUtilitiesPlugin, MinifierPlugin}
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Tailwind-RS Phase 2 Features Demo");
    println!("=====================================");
    
    // 1. AST Parsing Demo
    println!("\n📝 1. AST Parsing Demo");
    println!("----------------------");
    demo_ast_parsing()?;
    
    // 2. Class Scanning Demo
    println!("\n🔍 2. Class Scanning Demo");
    println!("-------------------------");
    demo_class_scanning()?;
    
    // 3. Tree-Shaking Demo
    println!("\n🌳 3. Tree-Shaking Demo");
    println!("----------------------");
    demo_tree_shaking()?;
    
    // 4. CSS Optimization Demo
    println!("\n⚡ 4. CSS Optimization Demo");
    println!("--------------------------");
    demo_css_optimization()?;
    
    // 5. Plugin System Demo
    println!("\n🔌 5. Plugin System Demo");
    println!("------------------------");
    demo_plugin_system()?;
    
    // 6. Complete Pipeline Demo
    println!("\n🔄 6. Complete Pipeline Demo");
    println!("----------------------------");
    demo_complete_pipeline()?;
    
    println!("\n✅ Phase 2 Demo completed successfully!");
    println!("🎉 All advanced features are working!");
    
    Ok(())
}

fn demo_ast_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = AstParser::new();
    
    let rust_code = r#"
        use tailwind_rs_core::ClassBuilder;
        
        fn create_button() -> String {
            ClassBuilder::new()
                .class("px-4")
                .class("py-2")
                .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
                .text_color(Color::new(ColorPalette::White, ColorShade::Shade100))
                .responsive(Breakpoint::Md, |builder| {
                    builder.class("px-6")
                })
                .to_string()
        }
    "#;
    
    parser.parse_content(rust_code)?;
    
    println!("  📊 Parsed classes: {:?}", parser.get_classes());
    println!("  📱 Responsive classes: {:?}", parser.get_all_responsive_classes());
    println!("  🎯 Total classes found: {}", parser.class_count());
    
    Ok(())
}

fn demo_class_scanning() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = ClassScanner::new();
    
    // Create a temporary test file
    let temp_file = std::env::temp_dir().join("demo_scan.rs");
    let content = r#"
        use tailwind_rs_core::ClassBuilder;
        
        fn demo_component() -> String {
            ClassBuilder::new()
                .class("p-4")
                .class("bg-blue-500")
                .class("text-white")
                .class("rounded-lg")
                .class("shadow-md")
                .to_string()
        }
    "#;
    
    std::fs::write(&temp_file, content)?;
    
    let results = scanner.scan_files(&[temp_file.clone()])?;
    
    println!("  📁 Files scanned: {}", results.stats.files_scanned);
    println!("  📊 Classes found: {:?}", results.classes);
    println!("  ⏱️  Scan time: {}ms", results.stats.duration_ms);
    println!("  📏 Total file size: {} bytes", results.stats.total_file_size);
    
    // Clean up
    std::fs::remove_file(&temp_file)?;
    
    Ok(())
}

fn demo_tree_shaking() -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = CssGenerator::new();
    
    // Add some classes
    generator.add_class("p-4")?;
    generator.add_class("bg-blue-500")?;
    generator.add_class("text-white")?;
    generator.add_class("unused-class")?;
    generator.add_class("another-unused")?;
    
    let mut tree_shaker = TreeShaker::new();
    
    // Configure tree-shaking
    let mut config = TreeShakeConfig::default();
    config.keep_classes.insert("p-4".to_string());
    config.keep_classes.insert("bg-blue-500".to_string());
    config.keep_classes.insert("text-white".to_string());
    tree_shaker.set_config(config);
    
    let temp_dir = std::env::temp_dir();
    let results = tree_shaker.shake(&[&temp_dir], &mut generator)?;
    
    println!("  📊 Original size: {} bytes", results.original_size);
    println!("  📊 Optimized size: {} bytes", results.optimized_size);
    println!("  📉 Size reduction: {:.1}%", results.reduction_percentage);
    println!("  🗑️  Classes removed: {}", results.stats.classes_removed);
    println!("  ⏱️  Processing time: {}ms", results.stats.processing_time_ms);
    
    Ok(())
}

fn demo_css_optimization() -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = CssGenerator::new();
    
    // Add some classes
    generator.add_class("p-4")?;
    generator.add_class("bg-blue-500")?;
    generator.add_class("text-white")?;
    generator.add_class("rounded-lg")?;
    generator.add_class("shadow-md")?;
    
    let optimizer = CssOptimizer::new();
    let results = optimizer.optimize(&mut generator)?;
    
    println!("  📊 Original size: {} bytes", results.original_size);
    println!("  📊 Optimized size: {} bytes", results.optimized_size);
    println!("  📉 Size reduction: {:.1}%", results.reduction_percentage);
    println!("  🔧 Rules merged: {}", results.stats.rules_merged);
    println!("  ⚡ Properties optimized: {}", results.stats.properties_optimized);
    println!("  ⏱️  Processing time: {}ms", results.stats.processing_time_ms);
    
    // Test CSS compression
    let css = r#"
        /* This is a comment */
        .test {
            padding: 1rem;
            margin: 0px;
            color: #ffffff;
            background-color: #000000;
        }
    "#;
    
    let compressed = optimizer.compress_css(css)?;
    println!("  🗜️  CSS compression: {} -> {} bytes", css.len(), compressed.len());
    
    Ok(())
}

fn demo_plugin_system() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = PluginRegistry::new();
    
    // Register plugins
    let custom_plugin = Box::new(CustomUtilitiesPlugin::new());
    let minifier_plugin = Box::new(MinifierPlugin::new());
    
    registry.register_plugin(custom_plugin)?;
    registry.register_plugin(minifier_plugin)?;
    
    println!("  🔌 Registered plugins: {:?}", registry.list_plugins());
    
    // Configure plugins
    let custom_config = serde_json::json!({
        "utilities": [
            {
                "name": "custom-shadow",
                "properties": {
                    "box-shadow": "0 4px 6px -1px rgba(0, 0, 0, 0.1)"
                }
            }
        ]
    });
    
    let minifier_config = serde_json::json!({
        "enabled": true
    });
    
    registry.set_plugin_config("custom-utilities", custom_config)?;
    registry.set_plugin_config("minifier", minifier_config)?;
    
    println!("  ⚙️  Plugin configurations set");
    
    // Execute hooks
    registry.execute_hook(tailwind_rs_core::plugin_system::PluginHook::BeforeGenerate)?;
    registry.execute_hook(tailwind_rs_core::plugin_system::PluginHook::OnOptimize)?;
    registry.execute_hook(tailwind_rs_core::plugin_system::PluginHook::AfterGenerate)?;
    
    println!("  🎯 Plugin hooks executed successfully");
    
    Ok(())
}

fn demo_complete_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    println!("  🔄 Running complete pipeline...");
    
    // 1. Create source files
    let temp_dir = std::env::temp_dir().join("tailwind_demo");
    std::fs::create_dir_all(&temp_dir)?;
    
    let source_file = temp_dir.join("components.rs");
    let content = r#"
        use tailwind_rs_core::ClassBuilder;
        
        pub fn button() -> String {
            ClassBuilder::new()
                .class("px-4")
                .class("py-2")
                .class("bg-blue-500")
                .class("text-white")
                .class("rounded-lg")
                .class("shadow-md")
                .responsive(Breakpoint::Md, |builder| {
                    builder.class("px-6")
                })
                .to_string()
        }
        
        pub fn card() -> String {
            ClassBuilder::new()
                .class("p-6")
                .class("bg-white")
                .class("rounded-xl")
                .class("shadow-lg")
                .to_string()
        }
    "#;
    
    std::fs::write(&source_file, content)?;
    
    // 2. Scan for classes
    let mut scanner = ClassScanner::new();
    let scan_results = scanner.scan_directory(&temp_dir)?;
    
    println!("    📊 Scanned {} files, found {} classes", 
             scan_results.stats.files_scanned, 
             scan_results.stats.total_classes);
    
    // 3. Generate CSS
    let mut generator = CssGenerator::new();
    for class in &scan_results.classes {
        generator.add_class(class)?;
    }
    
    let original_css = generator.generate_css();
    println!("    📄 Generated {} bytes of CSS", original_css.len());
    
    // 4. Tree-shake unused classes
    let mut tree_shaker = TreeShaker::new();
    let tree_shake_results = tree_shaker.shake(&[&temp_dir], &mut generator)?;
    
    println!("    🌳 Tree-shaking: {:.1}% size reduction", 
             tree_shake_results.reduction_percentage);
    
    // 5. Optimize CSS
    let optimizer = CssOptimizer::new();
    let optimization_results = optimizer.optimize(&mut generator)?;
    
    println!("    ⚡ Optimization: {:.1}% size reduction", 
             optimization_results.reduction_percentage);
    
    // 6. Final CSS output
    let final_css = generator.generate_minified_css();
    println!("    🎯 Final CSS: {} bytes", final_css.len());
    
    // 7. Plugin system integration
    let mut registry = PluginRegistry::new();
    let custom_plugin = Box::new(CustomUtilitiesPlugin::new());
    registry.register_plugin(custom_plugin)?;
    
    registry.execute_hook(tailwind_rs_core::plugin_system::PluginHook::AfterGenerate)?;
    println!("    🔌 Plugin system integrated");
    
    // 8. Write final output
    let output_file = temp_dir.join("styles.css");
    std::fs::write(&output_file, final_css)?;
    println!("    💾 CSS written to: {:?}", output_file);
    
    // Clean up
    std::fs::remove_dir_all(&temp_dir)?;
    
    println!("  ✅ Complete pipeline executed successfully!");
    
    Ok(())
}
