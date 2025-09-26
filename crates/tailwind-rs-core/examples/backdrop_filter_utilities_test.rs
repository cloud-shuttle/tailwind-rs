use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🧪 Testing Backdrop Filter Utilities");
    println!("====================================");

    let test_classes = vec![
        "backdrop-filter-none",
        "backdrop-blur-sm",
        "backdrop-blur-lg",
        "backdrop-brightness-50",
        "backdrop-brightness-150",
        "backdrop-contrast-75",
        "backdrop-contrast-125",
        "backdrop-grayscale",
        "backdrop-grayscale-50",
        "backdrop-hue-rotate-90",
        "backdrop-hue-rotate-180",
        "backdrop-invert",
        "backdrop-invert-20",
        "backdrop-opacity-50",
        "backdrop-opacity-75",
        "backdrop-saturate-50",
        "backdrop-saturate-150",
        "backdrop-sepia",
        "backdrop-sepia-75",
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
        if line.contains("backdrop-filter") {
            println!("  {}", line);
        }
    }
}
