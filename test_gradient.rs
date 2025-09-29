use tailwind_rs_core::css_generator::CssGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = CssGenerator::new();

    // Test gradient combination
    let classes = &["from-blue-500", "to-red-500", "bg-gradient-to-r"];

    generator.add_classes_for_element(classes)?;

    let css = generator.generate_css();
    println!("Generated CSS:\n{}", css);

    // Check if gradient was generated
    if css.contains("linear-gradient") {
        println!("✅ Gradient parsing works!");
    } else {
        println!("❌ Gradient parsing failed");
    }

    Ok(())
}
