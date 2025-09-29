extern crate tailwind_rs_core;
use tailwind_rs_core::css_generator::{CssGenerator, trie::ParserType, parsers::{EffectsParser, UtilityParser}};

fn main() {
    let generator = CssGenerator::new();

    let test_classes = vec![
        "border-purple-500/50",
        "border-purple-500",
        "shadow-purple-500/50",
        "shadow-purple-500",
        "text-blue-500/20",
        "text-blue-500",
        "bg-green-500/30",
        "rounded",
        "rounded-full",
        "rounded-xl",
        "rounded-2xl"
    ];

    // First check which parser is selected
    println!("🔍 Checking parser selection:");
    for class in &test_classes {
        let parser_option = generator.parser_trie.find_parser(class);
        println!("  {} -> Parser: {:?}", class, parser_option.map(|p| match p {
            ParserType::Color(_) => "Color",
            ParserType::Background(_) => "Background",
            ParserType::BorderUtilities(_) => "BorderUtilities",
            ParserType::Effects(_) => "Effects",
            ParserType::Shadow(_) => "Shadow",
            _ => "Other",
        }));
    }

    println!("\n🧪 Testing class parsing:");
    for class in &test_classes {
        match generator.class_to_css_rule(class) {
            Ok(rule) => {
                println!("✅ {} -> {} properties", class, rule.properties.len());
                for prop in &rule.properties {
                    println!("   - {}: {}", prop.name, prop.value);
                }
            }
            Err(e) => {
                println!("❌ {} -> {}", class, e);
            }
        }
    }

    // Test Effects parser directly
    println!("\n🔧 Testing Effects parser directly:");
    let effects_parser = EffectsParser::new();
    let shadow_classes = vec!["shadow-purple-500/50", "shadow-purple-500"];
    for class in &shadow_classes {
        match effects_parser.parse_class(class) {
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

    // Test Color parser directly
    println!("\n🎨 Testing Color parser directly:");
    use tailwind_rs_core::css_generator::parsers::ColorParser;
    let color_parser = ColorParser::new();
    let text_classes = vec!["text-blue-500/20", "text-blue-500"];
    for class in &text_classes {
        match color_parser.parse_class(class) {
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
