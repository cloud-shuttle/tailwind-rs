use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();

    // Test prose classes
    let prose_classes = vec![
        "prose",
        "prose-sm",
        "prose-lg",
        "prose-xl",
        "prose-2xl",
        "prose-invert",
        "dark:prose-invert",
    ];

    println!("🔍 Testing Prose Classes");
    println!("========================");

    for class in prose_classes {
        println!("\n📝 Testing class: {}", class);

        match generator.add_class(class) {
            Ok(_) => {
                println!("  ✅ Successfully added class");
                // Check if it generated CSS
                let css = generator.generate_css();
                if css.contains(&format!(".\\{}", class.replace(":", "\\:"))) {
                    println!("  ✅ CSS rule generated");
                } else {
                    println!("  ❌ No CSS rule found in output");
                }
            }
            Err(e) => {
                println!("  ❌ Failed to add class: {}", e);
            }
        }
    }

    println!("\n🎨 Generated CSS:");
    println!("==================");
    let css = generator.generate_css();
    println!("{}", css);
}
