use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🧪 Testing Typography Utilities - Debug Test");
    println!("=============================================");
    
    let test_classes = vec![
        "leading-tight",
        "leading-normal", 
        "leading-relaxed",
        "leading-loose",
        "leading-3",
        "leading-4",
        "leading-5",
        "leading-6",
        "leading-7",
        "leading-8",
        "leading-9",
        "leading-10"
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
        if line.contains("line-height") {
            println!("  {}", line);
        }
    }
}
