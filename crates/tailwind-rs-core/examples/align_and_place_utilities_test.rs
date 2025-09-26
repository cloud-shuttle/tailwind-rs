//! Align and Place Utilities Test
//!
//! This example tests the new align and place utilities: align-content, align-items, align-self,
//! place-content, place-items, and place-self.

use tailwind_rs_core::CssGenerator;

fn main() {
    println!("🔍 Align and Place Utilities Test");
    println!("==================================\n");

    let mut generator = CssGenerator::new();

    // Test classes for each new utility category
    let test_classes = vec![
        // Align content utilities
        "content-normal",
        "content-center",
        "content-start",
        "content-end",
        "content-between",
        "content-around",
        "content-evenly",
        "content-baseline",
        "content-stretch",
        // Align items utilities
        "items-start",
        "items-end",
        "items-end-safe",
        "items-center",
        "items-center-safe",
        "items-baseline",
        "items-baseline-last",
        "items-stretch",
        // Align self utilities
        "self-auto",
        "self-start",
        "self-end",
        "self-end-safe",
        "self-center",
        "self-center-safe",
        "self-stretch",
        "self-baseline",
        "self-baseline-last",
        // Place content utilities
        "place-content-center",
        "place-content-center-safe",
        "place-content-start",
        "place-content-end",
        "place-content-end-safe",
        "place-content-between",
        "place-content-around",
        "place-content-evenly",
        "place-content-baseline",
        "place-content-stretch",
        // Place items utilities
        "place-items-start",
        "place-items-end",
        "place-items-end-safe",
        "place-items-center",
        "place-items-center-safe",
        "place-items-baseline",
        "place-items-stretch",
        // Place self utilities
        "place-self-auto",
        "place-self-start",
        "place-self-end",
        "place-self-end-safe",
        "place-self-center",
        "place-self-center-safe",
        "place-self-stretch",
    ];

    let mut working = 0;
    let mut broken = 0;

    println!(
        "📋 Testing Align and Place Utilities ({} classes):",
        test_classes.len()
    );

    for class in &test_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => {
                println!("  ✅ {} -> {} properties", class, properties.len());
                working += 1;
            }
            Err(e) => {
                println!("  ❌ {} -> Error: {}", class, e);
                broken += 1;
            }
        }
    }

    let coverage = (working as f64 / test_classes.len() as f64) * 100.0;

    println!("\n📊 Results:");
    println!("===========");
    println!("  ✅ Working: {}/{}", working, test_classes.len());
    println!("  ❌ Broken: {}/{}", broken, test_classes.len());
    println!("  📈 Coverage: {:.1}%", coverage);

    if coverage >= 90.0 {
        println!("\n🎉 Excellent coverage! Ready for production!");
    } else if coverage >= 80.0 {
        println!("\n🚀 Good coverage! Consider release candidate.");
    } else {
        println!("\n⚠️  Coverage needs improvement.");
    }
}
