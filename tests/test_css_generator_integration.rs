use tailwind_rs_core::css_generator::core::CssGenerator;

fn main() {
    println!("Testing CssGenerator integration with element-based processing...");

    let mut generator = CssGenerator::new();

    // Test gradient processing
    let classes = vec!["bg-gradient-to-r", "from-pink-400", "to-cyan-400"];
    let css = generator.process_element_classes(&classes);
    println!("Gradient CSS:\n{}", css);
    assert!(css.contains("--tw-gradient-from"));
    assert!(css.contains("--tw-gradient-to"));
    assert!(css.contains("background-image"));

    // Test shadow processing
    let classes = vec!["shadow-lg"];
    let css = generator.process_element_classes(&classes);
    println!("\nShadow CSS:\n{}", css);
    assert!(css.contains("box-shadow"));
    assert!(css.contains("10px 15px"));

    // Test transform processing
    let classes = vec!["scale-110", "rotate-45"];
    let css = generator.process_element_classes(&classes);
    println!("\nTransform CSS:\n{}", css);
    assert!(css.contains("transform"));
    assert!(css.contains("scale(1.1)"));
    assert!(css.contains("rotate(45deg)"));

    // Test combined processing
    let classes = vec!["shadow-lg", "scale-110", "bg-gradient-to-r", "from-pink-400"];
    let css = generator.process_element_classes(&classes);
    println!("\nCombined CSS:\n{}", css);
    assert!(css.contains("box-shadow"));
    assert!(css.contains("transform"));
    assert!(css.contains("--tw-gradient-from"));

    // Test variant processing
    let classes = vec!["hover:shadow-lg"];
    let css = generator.process_element_classes(&classes);
    println!("\nVariant CSS:\n{}", css);
    assert!(css.contains(".hover\\:shadow-lg:hover"));
    assert!(css.contains("box-shadow"));

    println!("\n✅ All integration tests passed!");
    println!("🎉 CssGenerator now supports full element-based processing!");
}
