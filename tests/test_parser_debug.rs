use tailwind_rs_core::css_generator::core::operations::ParserRegistry;

fn main() {
    println!("Testing ParserRegistry pattern matching...");

    let registry = ParserRegistry::new();

    println!("Testing 'animate-pulse' pattern matching:");
    let result = registry.parse_class("animate-pulse");

    match result {
        Some(properties) => {
            println!("✅ SUCCESS: 'animate-pulse' parsed!");
            println!("Properties ({}):", properties.len());
            for prop in &properties {
                println!("  {}: {}", prop.name, prop.value);
            }
        }
        None => {
            println!("❌ FAILED: 'animate-pulse' not parsed");
        }
    }

    println!("\nTesting other animation classes:");
    let test_classes = vec!["animate-spin", "animate-bounce", "animate-float"];

    for class in test_classes {
        match registry.parse_class(class) {
            Some(_) => println!("✅ {}: parsed", class),
            None => println!("❌ {}: not parsed", class),
        }
    }

    println!("\nTesting pattern variations:");
    let patterns = vec!["animate-", "animate*", "animate"];

    for pattern in patterns {
        println!("Pattern '{}' would match 'animate-pulse': {}", pattern, "animate-pulse".starts_with(&pattern.replace("*", "").replace("-", "")));
    }
}
