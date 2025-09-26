//! Debug Sizing Parser Test

use tailwind_rs_core::css_generator::parsers::sizing::SizingParser;
use tailwind_rs_core::css_generator::UtilityParser;

fn main() {
    let parser = SizingParser::new();
    
    // Test custom properties directly with the sizing parser
    let test_classes = vec![
        "w-(--my-width)",
        "h-(--my-height)",
        "w-(--my-var)",
        "h-(--my-var)",
    ];
    
    println!("🔍 Debug Sizing Parser Test");
    println!("===================================");
    
    for class in test_classes {
        match parser.parse_class(class) {
            Some(properties) => {
                println!("✅ {} -> {} properties", class, properties.len());
                for prop in properties {
                    println!("   {}: {}", prop.name, prop.value);
                }
            }
            None => {
                println!("❌ {} -> No properties found", class);
            }
        }
    }
}
