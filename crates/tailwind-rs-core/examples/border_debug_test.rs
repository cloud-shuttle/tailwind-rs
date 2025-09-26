use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🧪 Testing Border Utilities - Debug Test");
    println!("========================================");

    let test_classes = vec![
        "rounded-t",
        "rounded-r",
        "rounded-b",
        "rounded-l",
        "rounded-tl",
        "rounded-tr",
        "rounded-br",
        "rounded-bl",
        "rounded-t-lg",
        "rounded-r-md",
        "rounded-b-sm",
        "rounded-l-xl",
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
        if line.contains("border-radius") {
            println!("  {}", line);
        }
    }
}
