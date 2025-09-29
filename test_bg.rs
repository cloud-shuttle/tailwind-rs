extern crate tailwind_rs_core;
use tailwind_rs_core::css_generator::{CssGenerator, generator_parsers::CssGeneratorParsers};

fn main() {
    let generator = CssGenerator::new();

    let classes = vec!["bg-white/10", "bg-white/5", "bg-black/5"];

    for class in &classes {
        println!("Testing: {}", class);

        // Debug: Check variant parsing
        let (variants, base_class) = generator.parse_variants(class);
        println!("  Variants: {:?}, Base class: {}", variants, &base_class);

        match generator.class_to_properties(&base_class) {
            Ok(properties) => {
                println!("  ✅ Base class '{}' -> {} properties", &base_class, properties.len());
                for prop in &properties {
                    println!("    - {}: {}", prop.name, prop.value);
                }
            }
            Err(e) => {
                println!("  ❌ Base class '{}' error: {}", &base_class, e);
            }
        }

        match generator.class_to_css_rule(class) {
            Ok(rule) => {
                println!("  ✅ Full class -> {} properties", rule.properties.len());
                for prop in &rule.properties {
                    println!("    - {}: {}", prop.name, prop.value);
                }
            }
            Err(e) => {
                println!("  ❌ Full class error: {}", e);
            }
        }

        println!();
    }
}
