//! Debug Custom Properties Detailed Test

use tailwind_rs_core::css_generator::parsers::sizing::SizingParser;
use tailwind_rs_core::css_generator::UtilityParser;

fn main() {
    let parser = SizingParser::new();

    // Test custom properties with detailed debugging
    let test_class = "w-(--my-width)";

    println!("🔍 Debug Custom Properties Detailed Test");
    println!("===================================");
    println!("Testing class: {}", test_class);

    // Test strip_prefix manually
    if let Some(value) = test_class.strip_prefix("w-(") {
        println!("✅ strip_prefix('w-(') worked: '{}'", value);
        if let Some(value) = value.strip_suffix(")") {
            println!("✅ strip_suffix(')') worked: '{}'", value);
            println!("✅ Final CSS value would be: var({})", value);
        } else {
            println!("❌ strip_suffix(')') failed");
        }
    } else {
        println!("❌ strip_prefix('w-(') failed");
    }

    // Test the actual parser
    match parser.parse_class(test_class) {
        Some(properties) => {
            println!("✅ Parser returned {} properties", properties.len());
            for prop in properties {
                println!("   {}: {}", prop.name, prop.value);
            }
        }
        None => {
            println!("❌ Parser returned None");
        }
    }
}
