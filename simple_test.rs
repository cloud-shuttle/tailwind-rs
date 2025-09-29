use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();

    // Test some basic classes first
    println!("Testing basic classes:");
    let basic_classes = vec![
        "text-red-500", "bg-blue-500", "p-4", "m-2", "flex", "block", "hidden"
    ];

    for class in &basic_classes {
        match generator.add_class(class) {
            Ok(_) => println!("✅ {}", class),
            Err(e) => println!("❌ {} - {}", class, e),
        }
    }

    // Test advanced features
    println!("\nTesting advanced features:");
    let advanced_classes = vec![
        "perspective-1000",
        "rotate-x-12",
        "transform-style-preserve-3d",
        "animate-float",
        "animate-twinkle",
        "animate-rainbow",
        "bg-gradient-conic",
        "from-50%",
        "animate-particle-float",
    ];

    for class in &advanced_classes {
        match generator.add_class(class) {
            Ok(_) => println!("✅ {}", class),
            Err(e) => println!("❌ {} - {}", class, e),
        }
    }

    let css = generator.generate_css();
    println!("\nGenerated CSS length: {} characters", css.len());

    // Check for specific CSS features
    println!("\nChecking for CSS features:");
    println!("perspective: {}", css.contains("perspective:"));
    println!("rotateX: {}", css.contains("rotateX"));
    println!("preserve-3d: {}", css.contains("preserve-3d"));
    println!("conic-gradient: {}", css.contains("conic-gradient"));
    println!("50%: {}", css.contains("50%"));
}
