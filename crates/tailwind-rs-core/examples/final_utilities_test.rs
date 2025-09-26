use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🧪 Testing Final Utilities");
    println!("=========================");
    
    let test_classes = vec![
        // Display utilities
        "list-item",
        
        // Flexbox utilities
        "flex-grow",
        "flex-shrink",
        
        // SVG utilities
        "fill-none",
        "fill-current",
        "fill-transparent",
        "stroke-none",
        "stroke-current",
        "stroke-transparent",
        "stroke-1",
        "stroke-2",
        
        // Accessibility utilities
        "forced-color-adjust-auto",
        "forced-color-adjust-none"
    ];
    
    let mut generator = CssGenerator::new();
    
    for class in &test_classes {
        match generator.add_class(class) {
            Ok(_) => {
                println!("✅ {} - Added", class);
            }
            Err(e) => {
                println!("❌ {} - Failed: {}", class, e);
            }
        }
    }
    
    let css = generator.generate_css();
    println!("\n📝 Generated CSS:");
    for line in css.lines() {
        if line.contains("display") || line.contains("flex-") || line.contains("fill") || line.contains("stroke") || line.contains("forced-color") {
            println!("  {}", line);
        }
    }
}
