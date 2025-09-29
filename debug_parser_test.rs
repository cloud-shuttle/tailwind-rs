extern crate tailwind_rs_core;

use tailwind_rs_core::css_generator::{CssGenerator, trie::ParserType, parsers::UtilityParser};

fn main() {
    let generator = CssGenerator::new();

    let test_classes = vec![
        "from-slate-900",
        "to-pink-500",
        "via-cyan-500",
        "text-slate-900",
        "bg-slate-900",
    ];

    println!("🔍 DEBUG: Testing parser routing for gradient classes");
    println!("{}", "=".repeat(60));

    // Debug: Check trie directly
    println!("\n🔧 DEBUG: Checking trie directly:");
    for class in &test_classes {
        let parser_option = generator.parser_trie.find_parser(class);
        println!("  {} -> Parser found: {}", class, parser_option.is_some());
        if let Some(parser) = parser_option {
            println!("    Parser type: {:?}", parser);
        }
    }

    println!("\n🧪 Testing class_to_properties:");
    for class in &test_classes {
        println!("\nTesting: {}", class);

        // Try to find which parser would handle this
        match generator.class_to_properties(class) {
            Ok(properties) => {
                println!("  ✅ SUCCESS: {} properties", properties.len());
                for prop in &properties {
                    println!("    - {}: {}", prop.name, prop.value);
                }
            }
            Err(e) => {
                println!("  ❌ FAILED: {}", e);

                // Debug: Try the parser directly
                if let Some(parser) = generator.parser_trie.find_parser(class) {
                    println!("  🔍 DEBUG: Calling parser directly...");
                    let direct_result = match parser {
                        ParserType::Gradient(p) => {
                            println!("    Testing GradientParser.parse_gradient_stop_class...");
                            let stop_result = p.parse_gradient_stop_class(class);
                            println!("    parse_gradient_stop_class result: {:?}", stop_result.is_some());
                            if let Some(props) = &stop_result {
                                println!("    Properties: {} items", props.len());
                                for prop in props {
                                    println!("      - {}: {}", prop.name, prop.value);
                                }
                            }
                            p.parse_class(class)
                        },
                        ParserType::Typography(p) => p.parse_class(class),
                        ParserType::Background(p) => p.parse_class(class),
                        _ => None,
                    };
                    println!("    Direct parser result: {:?}", direct_result.is_some());
                    if let Some(props) = direct_result {
                        println!("    Direct parser returned {} properties", props.len());
                    }
                }
            }
        }
    }
}
