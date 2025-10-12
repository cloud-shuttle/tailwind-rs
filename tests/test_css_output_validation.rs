extern crate tailwind_rs_core;

use tailwind_rs_core::css_generator::LegacyCssGenerator as CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();

    // Test some key classes and validate their CSS output
    let test_cases = vec![
        ("w-4", "width: 1rem;"),
        ("h-8", "height: 2rem;"),
        ("bg-blue-500", "background-color: rgb(59 130 246);"),
        ("text-red-600", "color: rgb(220 38 38);"),
        ("p-4", "padding: 1rem;"),
        ("m-2", "margin: -0.5rem;"),
        ("border", "border-width: 1px;\nborder-style: solid;"),
        ("rounded-lg", "border-radius: 0.5rem;"),
        ("shadow-lg", "box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);"),
        ("hover:bg-blue-600", "/* hover:bg-blue-600 */"),
        ("sm:w-full", "/* sm:w-full */"),
    ];

    println!("CSS Output Validation Test");
    println!("==========================");

    let mut passed = 0;
    let mut total = test_cases.len();

    for (class, expected) in test_cases {
        match generator.generate_individual_css_rule(class) {
            Ok(css) => {
                println!("\nClass: {}", class);
                println!("Generated: {}", css);
                println!("Expected:  {}", expected);

                // Simple validation - check if key parts are present
                let css_lower = css.to_lowercase();
                let expected_lower = expected.to_lowercase();

                let matches = if expected.contains("/*") {
                    // For variants, just check that it generated something
                    !css.is_empty()
                } else {
                    // For regular classes, check that key CSS properties are present
                    expected_lower.split(';').filter(|s| !s.trim().is_empty()).all(|prop| {
                        let prop = prop.trim();
                        if prop.contains(':') {
                            let parts: Vec<&str> = prop.split(':').collect();
                            if parts.len() >= 2 {
                                let property = parts[0].trim();
                                let value = parts[1].trim();
                                css_lower.contains(property) && css_lower.contains(value)
                            } else {
                                false
                            }
                        } else {
                            css_lower.contains(prop)
                        }
                    })
                };

                if matches {
                    println!("✅ PASS");
                    passed += 1;
                } else {
                    println!("❌ FAIL");
                }
            }
            Err(e) => {
                println!("\nClass: {} - ERROR: {}", class, e);
                println!("❌ FAIL");
            }
        }
    }

    println!("\n==========================");
    println!("Results: {}/{} passed ({:.1}%)", passed, total, (passed as f32 / total as f32) * 100.0);
}
