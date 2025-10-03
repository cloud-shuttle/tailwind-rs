use tailwind_rs_core::boundary::{BoundaryValidator, LanguageBoundaryRules, TemplateLanguage, utils};

fn main() {
    println!("🧪 Testing Boundary Classification System");

    // Test HTML boundary validation
    let html_input = br#"<div class="flex items-center px-4 bg-blue-500"></div>"#;
    let validator = BoundaryValidator::new(html_input);

    println!("📄 HTML Input: {:?}", String::from_utf8_lossy(html_input));

    // Test valid class positions
    let valid_positions = vec![
        (12, 16), // "flex"
        (17, 29), // "items-center"
        (30, 34), // "px-4"
        (35, 47), // "bg-blue-500"
    ];

    println!("✅ Valid class positions:");
    for (start, end) in &valid_positions {
        let class_bytes = &html_input[*start..*end];
        let class_name = String::from_utf8_lossy(class_bytes);
        let is_valid = validator.is_valid_class_position(*start, *end);
        println!("  '{}' at ({}, {}): {}", class_name, start, end, is_valid);
    }

    // Test invalid positions
    let invalid_positions = vec![
        (1, 4),   // "div" (no proper boundaries)
        (48, 52), // "div>" (invalid context)
    ];

    println!("❌ Invalid class positions:");
    for (start, end) in &invalid_positions {
        if *end <= html_input.len() {
            let class_bytes = &html_input[*start..*end];
            let class_name = String::from_utf8_lossy(class_bytes);
            let is_valid = validator.is_valid_class_position(*start, *end);
            println!("  '{}' at ({}, {}): {}", class_name, start, end, is_valid);
        }
    }

    // Test language-specific rules
    println!("\n🌐 Language-specific boundary rules:");

    let vue_rules = LanguageBoundaryRules::for_language(TemplateLanguage::Vue);
    println!("Vue ':' boundary: {:?}", vue_rules.classify_with_language(b':'));

    let svelte_rules = LanguageBoundaryRules::for_language(TemplateLanguage::Svelte);
    println!("Svelte '{{' boundary: {:?}", svelte_rules.classify_with_language(b'{'));
    println!("Svelte '}}' boundary: {:?}", svelte_rules.classify_with_language(b'}'));

    let angular_rules = LanguageBoundaryRules::for_language(TemplateLanguage::Angular);
    println!("Angular '[' boundary: {:?}", angular_rules.classify_with_language(b'['));
    println!("Angular ']' boundary: {:?}", angular_rules.classify_with_language(b']'));

    // Test utility functions
    println!("\n🔧 Utility function tests:");
    let test_input = r#"<div class="flex invalid-class items-center" data-class="should-not-match">"#;
    let potential_classes = vec!["flex", "invalid-class", "items-center", "should-not-match"];

    let valid_classes = utils::extract_valid_classes(test_input, &potential_classes);
    println!("Extracted valid classes: {:?}", valid_classes);

    println!("\n✅ Boundary Classification System test completed!");
}
