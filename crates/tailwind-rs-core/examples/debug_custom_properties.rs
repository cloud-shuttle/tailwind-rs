//! Debug Custom Properties Test

use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let generator = CssGenerator::new();

    // Test custom properties
    let test_classes = vec![
        "w-(--my-width)",
        "h-(--my-height)",
        "w-(--my-var)",
        "h-(--my-var)",
    ];

    println!("🔍 Debug Custom Properties Test");
    println!("===================================");

    for class in test_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => {
                println!("✅ {} -> {} properties", class, properties.len());
                for prop in properties {
                    println!("   {}: {}", prop.name, prop.value);
                }
            }
            Err(e) => {
                println!("❌ {} -> Error: {}", class, e);
            }
        }
    }
}
