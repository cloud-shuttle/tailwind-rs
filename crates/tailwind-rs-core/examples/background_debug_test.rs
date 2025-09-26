use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🧪 Testing Background Utilities - Debug Test");
    println!("=============================================");

    let test_classes = vec![
        "bg-gradient-to-b",
        "bg-size-cover",
        "bg-position-center",
        "bg-size-contain",
        "bg-size-auto",
        "bg-position-top",
        "bg-position-bottom",
        "bg-position-left",
        "bg-position-right",
        "bg-position-top-left",
        "bg-position-top-right",
        "bg-position-bottom-left",
        "bg-position-bottom-right",
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
        if line.contains("background") {
            println!("  {}", line);
        }
    }
}
