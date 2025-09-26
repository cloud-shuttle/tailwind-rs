//! Gap and Justify Utilities Test
//!
//! This example tests the new gap and justify utilities: gap, justify-content, justify-items, and justify-self.

use tailwind_rs_core::CssGenerator;

fn main() {
    println!("🔍 Gap and Justify Utilities Test");
    println!("=================================\n");

    let mut generator = CssGenerator::new();

    // Test classes for each new utility category
    let test_classes = vec![
        // Gap utilities
        "gap-0", "gap-px", "gap-0.5", "gap-1", "gap-1.5", "gap-2", "gap-2.5", "gap-3", "gap-3.5", "gap-4",
        "gap-5", "gap-6", "gap-7", "gap-8", "gap-9", "gap-10", "gap-11", "gap-12", "gap-14", "gap-16",
        "gap-20", "gap-24", "gap-28", "gap-32", "gap-36", "gap-40", "gap-44", "gap-48", "gap-52", "gap-56",
        "gap-60", "gap-64", "gap-72", "gap-80", "gap-96",
        "gap-x-0", "gap-x-px", "gap-x-0.5", "gap-x-1", "gap-x-1.5", "gap-x-2", "gap-x-2.5", "gap-x-3", "gap-x-3.5", "gap-x-4",
        "gap-x-5", "gap-x-6", "gap-x-7", "gap-x-8", "gap-x-9", "gap-x-10", "gap-x-11", "gap-x-12", "gap-x-14", "gap-x-16",
        "gap-x-20", "gap-x-24", "gap-x-28", "gap-x-32", "gap-x-36", "gap-x-40", "gap-x-44", "gap-x-48", "gap-x-52", "gap-x-56",
        "gap-x-60", "gap-x-64", "gap-x-72", "gap-x-80", "gap-x-96",
        "gap-y-0", "gap-y-px", "gap-y-0.5", "gap-y-1", "gap-y-1.5", "gap-y-2", "gap-y-2.5", "gap-y-3", "gap-y-3.5", "gap-y-4",
        "gap-y-5", "gap-y-6", "gap-y-7", "gap-y-8", "gap-y-9", "gap-y-10", "gap-y-11", "gap-y-12", "gap-y-14", "gap-y-16",
        "gap-y-20", "gap-y-24", "gap-y-28", "gap-y-32", "gap-y-36", "gap-y-40", "gap-y-44", "gap-y-48", "gap-y-52", "gap-y-56",
        "gap-y-60", "gap-y-64", "gap-y-72", "gap-y-80", "gap-y-96",
        "gap-[10vw]", "gap-x-[5rem]", "gap-y-[2vh]",
        "gap-(--my-gap)", "gap-x-(--my-gap-x)", "gap-y-(--my-gap-y)",
        
        // Justify content utilities
        "justify-start", "justify-end", "justify-end-safe", "justify-center", "justify-center-safe",
        "justify-between", "justify-around", "justify-evenly", "justify-stretch", "justify-baseline", "justify-normal",
        
        // Justify items utilities
        "justify-items-start", "justify-items-end", "justify-items-end-safe", "justify-items-center", "justify-items-center-safe",
        "justify-items-stretch", "justify-items-normal",
        
        // Justify self utilities
        "justify-self-auto", "justify-self-start", "justify-self-center", "justify-self-center-safe",
        "justify-self-end", "justify-self-end-safe", "justify-self-stretch",
    ];

    let mut working = 0;
    let mut broken = 0;

    println!("📋 Testing Gap and Justify Utilities ({} classes):", test_classes.len());
    
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
