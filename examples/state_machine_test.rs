use tailwind_rs_core::state_machine::{StateMachineRegistry, GradientDefinition, ArbitraryValue, ArbitraryValueType, VariantCombination, ParsedVariant, VariantKind};

fn main() {
    println!("🧪 Testing Enhanced State Machine Architecture");

    let mut registry = StateMachineRegistry::new();

    // Test 1: Gradient parsing
    println!("\n🌈 Testing Gradient Machine:");
    let gradient_input = "bg-gradient-to-r from-blue-500 via-purple-300 to-pink-500";

    match registry.process_gradient(gradient_input) {
        Ok(gradient) => {
            println!("✅ Gradient parsed successfully!");
            println!("  Direction: {}", gradient.direction);
            println!("  Stops: {}", gradient.stops.len());
            for (i, stop) in gradient.stops.iter().enumerate() {
                println!("    Stop {}: {} (pos: {:?})", i + 1, stop.color, stop.position);
            }
        }
        Err(err) => println!("❌ Gradient parsing failed: {}", err),
    }

    // Test 2: Arbitrary value parsing
    println!("\n🔧 Testing Arbitrary Value Machine:");
    let arbitrary_inputs = vec![
        "[#ff0000]",
        "[24px]",
        "[50%]",
        "[url('image.jpg')]",
        "[invalid value]",
    ];

    for input in arbitrary_inputs {
        match registry.process_arbitrary_value(input) {
            Ok(arbitrary) => {
                println!("✅ '{}' -> {} (type: {:?})",
                    input, arbitrary.content, arbitrary.value_type);
            }
            Err(err) => println!("❌ '{}' failed: {}", input, err),
        }
    }

    // Test 3: Variant combination parsing
    println!("\n🎭 Testing Variant Combination Machine:");
    let variant_input = "hover:focus:dark:bg-blue-500";

    match registry.process_variant_combination(variant_input) {
        Ok(combination) => {
            println!("✅ Variant combination parsed successfully!");
            println!("  Variants: {}", combination.variants.len());
            for (i, variant) in combination.variants.iter().enumerate() {
                println!("    Variant {}: {} ({:?})", i + 1, variant.name, variant.kind);
            }
        }
        Err(err) => println!("❌ Variant combination parsing failed: {}", err),
    }

    // Test 4: Complex scenario - gradient with variants
    println!("\n🚀 Testing Complex Scenario:");
    let complex_input = "lg:hover:bg-gradient-to-r lg:hover:from-green-400 lg:hover:via-blue-500 lg:hover:to-purple-600";

    // This would be handled by combining multiple state machines
    println!("Complex input: {}", complex_input);
    println!("This would combine variant parsing + gradient parsing");
    println!("- Variants: lg:, hover:");
    println!("- Gradient: to-r, from-green-400, via-blue-500, to-purple-600");

    println!("\n✅ Enhanced State Machine Architecture test completed!");
}
