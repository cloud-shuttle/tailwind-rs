use tailwind_rs_core::css_generator::core::operations::ParserRegistry;

fn main() {
    let registry = ParserRegistry::new();

    println!("Testing ParserRegistry with gradient stops...");

    // Test gradient stops
    let gradient_classes = vec!["from-blue-400", "via-purple-500", "to-pink-500"];

    // Test translate classes
    let translate_classes = vec!["translate-x-4", "translate-y-2", "-translate-x-1", "translate-x-full"];

    println!("\nTesting translate classes...");
    for class in translate_classes {
        match registry.parse_class(class) {
            Some(properties) => {
                println!("✅ '{}' parsed successfully!", class);
                println!("   Properties:");
                for prop in &properties {
                    println!("     {}: {}", prop.name, prop.value);
                }
            }
            None => {
                println!("❌ '{}' failed to parse", class);
            }
        }
    }

    for class in gradient_classes {
        match registry.parse_class(class) {
            Some(properties) => {
                println!("✅ '{}' parsed successfully!", class);
                println!("   Properties:");
                for prop in &properties {
                    println!("     {}: {}", prop.name, prop.value);
                }

                // Check for expected properties
                let has_from = properties.iter().any(|p| p.name == "--tw-gradient-from");
                let has_via = properties.iter().any(|p| p.name == "--tw-gradient-via");
                let has_to = properties.iter().any(|p| p.name == "--tw-gradient-to");
                let has_stops = properties.iter().any(|p| p.name == "--tw-gradient-stops");

                if class.starts_with("from-") {
                    if has_from && has_stops {
                        println!("   ✅ Has --tw-gradient-from and --tw-gradient-stops");
                    } else {
                        println!("   ❌ Missing expected properties");
                    }
                } else if class.starts_with("via-") {
                    if has_via {
                        println!("   ✅ Has --tw-gradient-via");
                    } else {
                        println!("   ❌ Missing --tw-gradient-via");
                    }
                } else if class.starts_with("to-") {
                    if has_to {
                        println!("   ✅ Has --tw-gradient-to");
                    } else {
                        println!("   ❌ Missing --tw-gradient-to");
                    }
                }
            }
            None => {
                println!("❌ '{}' not parsed", class);
            }
        }
        println!();
    }

    println!("Testing ParserRegistry with 'animate-pulse'...");

    match registry.parse_class("animate-pulse") {
        Some(properties) => {
            println!("✅ 'animate-pulse' parsed successfully!");
            println!("   Properties: {:?}", properties);
        }
        None => {
            println!("❌ 'animate-pulse' not parsed");
        }
    }

    println!("\nTesting core classes...");
    let test_classes = vec![
        "p-4", "bg-blue-500", "text-white", "flex", "animate-pulse", "animate-spin",
        "hover:bg-blue-600", "sm:p-6", "invalid-class"
    ];

    for class in test_classes {
        match registry.parse_class(class) {
            Some(_) => println!("✅ {}: parsed", class),
            None => println!("❌ {}: not parsed", class),
        }
    }
}
