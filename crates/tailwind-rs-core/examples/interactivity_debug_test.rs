use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🧪 Testing Interactivity Utilities - Debug Test");
    println!("===============================================");

    let test_classes = vec![
        "touch-auto",
        "touch-none",
        "touch-pan-x",
        "touch-pan-y",
        "touch-pan-left",
        "touch-pan-right",
        "touch-pan-up",
        "touch-pan-down",
        "touch-pinch-zoom",
        "touch-manipulation",
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
        if line.contains("touch-action") {
            println!("  {}", line);
        }
    }
}
