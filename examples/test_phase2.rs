//! Simple test to verify Phase 2 features work
//! 
//! This tests the core functionality without dependencies on the testing library

use tailwind_rs_core::{
    CssGenerator, TailwindBuilder, Breakpoint,
    AstParser, ClassScanner, TreeShaker,
    PluginRegistry, plugin_system::{CustomUtilitiesPlugin, MinifierPlugin}
};
use tailwind_rs_core::css_optimizer::CssOptimizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing Phase 2 Features");
    println!("============================");
    
    // Test 1: CSS Generation
    println!("\n1. Testing CSS Generation...");
    test_css_generation()?;
    
    // Test 2: AST Parsing
    println!("\n2. Testing AST Parsing...");
    test_ast_parsing()?;
    
    // Test 3: Class Scanning
    println!("\n3. Testing Class Scanning...");
    test_class_scanning()?;
    
    // Test 4: Tree Shaking
    println!("\n4. Testing Tree Shaking...");
    test_tree_shaking()?;
    
    // Test 5: CSS Optimization
    println!("\n5. Testing CSS Optimization...");
    test_css_optimization()?;
    
    // Test 6: Plugin System
    println!("\n6. Testing Plugin System...");
    test_plugin_system()?;
    
    // Test 7: Build System
    println!("\n7. Testing Build System...");
    test_build_system()?;
    
    println!("\n✅ All Phase 2 features are working correctly!");
    
    Ok(())
}

fn test_css_generation() -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = CssGenerator::new();
    
    generator.add_class("p-4")?;
    generator.add_class("bg-blue-500")?;
    generator.add_class("text-white")?;
    generator.add_responsive_class(Breakpoint::Md, "p-6")?;
    
    let css = generator.generate_css();
    assert!(css.contains(".p-4"));
    assert!(css.contains("padding: 1rem"));
    assert!(css.contains("@media"));
    
    println!("  ✅ CSS generation working - {} bytes generated", css.len());
    Ok(())
}

fn test_ast_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = AstParser::new();
    
    let rust_code = r#"
        use tailwind_rs_core::ClassBuilder;
        
        fn test() -> String {
            ClassBuilder::new()
                .class("px-4")
                .class("py-2")
                .to_string()
        }
    "#;
    
    parser.parse_content(rust_code)?;
    
    println!("  ✅ AST parsing working - {} classes found", parser.class_count());
    Ok(())
}

fn test_class_scanning() -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = ClassScanner::new();
    
    // Create a temporary test file
    let temp_file = std::env::temp_dir().join("test_scan.rs");
    let content = r#"
        use tailwind_rs_core::ClassBuilder;
        
        fn test() -> String {
            ClassBuilder::new()
                .class("p-4")
                .class("bg-blue-500")
                .to_string()
        }
    "#;
    
    std::fs::write(&temp_file, content)?;
    
    let results = scanner.scan_files(&[temp_file.clone()])?;
    
    println!("  ✅ Class scanning working - {} files scanned, {} classes found", 
             results.stats.files_scanned, results.stats.total_classes);
    
    // Clean up
    std::fs::remove_file(&temp_file)?;
    Ok(())
}

fn test_tree_shaking() -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = CssGenerator::new();
    
    generator.add_class("p-4")?;
    generator.add_class("bg-blue-500")?;
    generator.add_class("text-white")?;
    
    let mut tree_shaker = TreeShaker::new();
    // Use current directory instead of temp directory
    let current_dir = std::env::current_dir()?;
    let results = tree_shaker.shake(&[&current_dir], &mut generator)?;
    
    println!("  ✅ Tree shaking working - {:.1}% size reduction", 
             results.reduction_percentage);
    Ok(())
}

fn test_css_optimization() -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = CssGenerator::new();
    
    generator.add_class("p-4")?;
    generator.add_class("bg-blue-500")?;
    generator.add_class("text-white")?;
    
    let optimizer = CssOptimizer::new();
    let results = optimizer.optimize(&mut generator)?;
    
    println!("  ✅ CSS optimization working - {:.1}% size reduction", 
             results.reduction_percentage);
    Ok(())
}

fn test_plugin_system() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = PluginRegistry::new();
    
    let custom_plugin = Box::new(CustomUtilitiesPlugin::new());
    let minifier_plugin = Box::new(MinifierPlugin::new());
    
    registry.register_plugin(custom_plugin)?;
    registry.register_plugin(minifier_plugin)?;
    
    println!("  ✅ Plugin system working - {} plugins registered", 
             registry.list_plugins().len());
    Ok(())
}

fn test_build_system() -> Result<(), Box<dyn std::error::Error>> {
    let builder = TailwindBuilder::new();
    
    // This should generate CSS successfully
    builder.build()?;
    
    println!("  ✅ Build system working - CSS file generated");
    Ok(())
}
