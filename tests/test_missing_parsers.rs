use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();

    // Test some "missing" classes
    let test_classes = vec![
        "animate-spin",
        "scale-105",
        "rotate-45",
        "blur-sm",
        "backdrop-blur-lg",
        "hover:bg-blue-600",
        "md:text-lg",
        "dark:bg-gray-800",
    ];

    for class in test_classes {
        match generator.add_class(class) {
            Ok(_) => println!("✅ {} - Added successfully", class),
            Err(e) => println!("❌ {} - Failed: {}", class, e),
        }
    }

    let css = generator.generate_css();
    println!("\nGenerated CSS ({} chars):\n{}", css.len(), css);
}
