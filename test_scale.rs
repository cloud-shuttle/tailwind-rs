extern crate tailwind_rs_core;
use tailwind_rs_core::css_generator::{CssGenerator, trie::ParserType};

fn main() {
    let generator = CssGenerator::new();

    let classes = vec!["scale-105", "scale-110"];

    for class in &classes {
        println!("Testing: {}", class);

        // Debug: Check which parser is selected
        let parser_option = generator.parser_trie.find_parser(class);
        println!("  Parser found: {:?}", parser_option.is_some());
        if let Some(parser) = parser_option {
            println!("  Parser type: {:?}", parser);
        }

        match generator.class_to_properties(class) {
            Ok(properties) => {
                println!("  ✅ Found {} properties", properties.len());
                for prop in &properties {
                    println!("    - {}: {}", prop.name, prop.value);
                }
            }
            Err(e) => {
                println!("  ❌ Error: {}", e);
            }
        }
        println!();
    }
}
