use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();

    // Test simple positioning classes
    let test_classes = vec!["left-5", "-left-5", "lg:left-5", "lg:-left-5"];

    println!("🔍 Debugging Positioning Classes");
    println!("================================");

    for class in test_classes {
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
