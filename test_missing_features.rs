use tailwind_rs_core::css_generator::CssGenerator;

#[test]
fn test_missing_advanced_features() {
    let mut generator = CssGenerator::new();

    // Test 3D transforms
    let perspective_1000 = generator.add_class("perspective-1000");
    let rotate_x_12 = generator.add_class("rotate-x-12");
    let transform_style_preserve_3d = generator.add_class("transform-style-preserve-3d");

    // Test complex animations
    let animate_float = generator.add_class("animate-float");
    let animate_twinkle = generator.add_class("animate-twinkle");
    let animate_rainbow = generator.add_class("animate-rainbow");

    // Test advanced gradients
    let bg_gradient_conic = generator.add_class("bg-gradient-conic");
    let from_50_percent = generator.add_class("from-50%");

    // Test particle systems (these are CSS animations)
    let animate_particle_float = generator.add_class("animate-particle-float");

    // Generate CSS and check what we got
    let css = generator.generate_css();
    println!("Generated CSS length: {}", css.len());
    println!("First 2000 chars of CSS:\n{}", &css[..css.len().min(2000)]);

    // Check if we have the expected features
    assert!(css.contains("perspective:"), "perspective-1000 should generate perspective property");
    assert!(css.contains("rotateX"), "rotate-x-12 should generate rotateX transform");
    assert!(css.contains("preserve-3d"), "transform-style-preserve-3d should work");
    assert!(css.contains("conic-gradient"), "bg-gradient-conic should work");
    assert!(css.contains("50%"), "from-50% should work with percentage stops");
}
