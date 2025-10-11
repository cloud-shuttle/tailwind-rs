use tailwind_rs_core::css_generator::core::operations::ParserRegistry;

fn main() {
    let registry = ParserRegistry::new();

    println!("Testing animate-pulse...");

    if let Some(properties) = registry.parse_class("animate-pulse") {
        println!("✅ SUCCESS: animate-pulse parsed!");
        for prop in properties {
            println!("  {}: {}", prop.name, prop.value);
        }
    } else {
        println!("❌ FAILED: animate-pulse not parsed");
    }

    println!("\nTesting animate-spin...");

    if let Some(properties) = registry.parse_class("animate-spin") {
        println!("✅ SUCCESS: animate-spin parsed!");
        for prop in properties {
            println!("  {}: {}", prop.name, prop.value);
        }
    } else {
        println!("❌ FAILED: animate-spin not parsed");
    }
}