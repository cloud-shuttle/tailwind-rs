//! Demo of CSS generation functionality
//! 
//! This demonstrates the basic CSS generation capabilities of tailwind-rs

use tailwind_rs_core::{CssGenerator, TailwindBuilder, Breakpoint};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Tailwind-RS CSS Generation Demo");
    println!("=====================================");
    
    // Create a CSS generator
    let mut generator = CssGenerator::new();
    
    // Add some basic classes
    println!("📝 Adding CSS classes...");
    generator.add_class("p-4")?;
    generator.add_class("bg-blue-500")?;
    generator.add_class("text-white")?;
    generator.add_class("rounded-md")?;
    generator.add_class("shadow-lg")?;
    
    // Add responsive classes
    println!("📱 Adding responsive classes...");
    generator.add_responsive_class(Breakpoint::Md, "p-6")?;
    generator.add_responsive_class(Breakpoint::Lg, "p-8")?;
    
    // Add custom properties
    println!("🎨 Adding custom properties...");
    generator.add_custom_property("primary-color", "#3b82f6");
    generator.add_custom_property("border-radius", "0.375rem");
    
    // Generate CSS
    println!("⚡ Generating CSS...");
    let css = generator.generate_css();
    
    // Display the generated CSS
    println!("\n📄 Generated CSS:");
    println!("{}", css);
    
    // Generate minified CSS
    println!("\n🗜️  Minified CSS:");
    let minified = generator.generate_minified_css();
    println!("{}", minified);
    
    // Show statistics
    println!("\n📊 Statistics:");
    println!("  - Total CSS rules: {}", generator.rule_count());
    println!("  - CSS size: {} bytes", css.len());
    println!("  - Minified size: {} bytes", minified.len());
    println!("  - Compression ratio: {:.1}%", 
             (1.0 - (minified.len() as f64 / css.len() as f64)) * 100.0);
    
    // Test the build system
    println!("\n🔨 Testing build system...");
    let builder = TailwindBuilder::new();
    builder.build()?;
    
    println!("\n✅ Demo completed successfully!");
    
    Ok(())
}
