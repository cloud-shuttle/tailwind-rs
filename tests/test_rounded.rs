extern crate tailwind_rs_core;
use tailwind_rs_core::css_generator::parsers::{BorderRadiusParser, UtilityParser};

fn main() {
    let parser = BorderRadiusParser::new();
    let classes = vec!["rounded", "rounded-full", "rounded-xl", "rounded-2xl"];

    for class in &classes {
        match parser.parse_class(class) {
            Some(properties) => {
                println!("✅ {} -> {} properties", class, properties.len());
                for prop in &properties {
                    println!("   - {}: {}", prop.name, prop.value);
                }
            }
            None => {
                println!("❌ {} -> None", class);
            }
        }
    }
}
