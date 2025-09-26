//! New Flexbox Utilities Test
//!
//! This example tests the new flexbox utilities: flex-basis, flex-direction, flex-wrap, flex, flex-grow, and flex-shrink.

use tailwind_rs_core::CssGenerator;

fn main() {
    println!("🔍 New Flexbox Utilities Test");
    println!("==============================\n");

    let mut generator = CssGenerator::new();

    // Test classes for each new flexbox utility category
    let test_classes = vec![
        // Flex basis utilities
        "basis-0", "basis-4", "basis-8", "basis-16", "basis-32", "basis-64", "basis-auto", "basis-full",
        "basis-1/2", "basis-1/3", "basis-2/3", "basis-1/4", "basis-3/4", "basis-1/5", "basis-4/5",
        "basis-1/6", "basis-5/6", "basis-1/12", "basis-11/12",
        "basis-3xs", "basis-2xs", "basis-xs", "basis-sm", "basis-md", "basis-lg", "basis-xl", "basis-2xl",
        "basis-[30vw]", "basis-[200px]", "basis-(--my-basis)",
        
        // Flex direction utilities
        "flex-row", "flex-row-reverse", "flex-col", "flex-col-reverse",
        
        // Flex wrap utilities
        "flex-nowrap", "flex-wrap", "flex-wrap-reverse",
        
        // Flex utilities
        "flex-1", "flex-2", "flex-3", "flex-4", "flex-5", "flex-6", "flex-7", "flex-8", "flex-9", "flex-10", "flex-11", "flex-12",
        "flex-auto", "flex-initial", "flex-none",
        "flex-1/2", "flex-1/3", "flex-2/3", "flex-1/4", "flex-3/4", "flex-1/5", "flex-4/5",
        "flex-1/6", "flex-5/6", "flex-1/12", "flex-11/12",
        "flex-[3_1_auto]", "flex-[0_1_auto]", "flex-(--my-flex)",
        
        // Flex grow utilities
        "grow", "grow-0", "grow-1", "grow-2", "grow-3", "grow-4", "grow-5", "grow-6", "grow-7", "grow-8", "grow-9", "grow-10", "grow-11", "grow-12",
        "grow-[25vw]", "grow-[2.5]", "grow-(--my-grow)",
        
        // Flex shrink utilities
        "shrink", "shrink-0", "shrink-1", "shrink-2", "shrink-3", "shrink-4", "shrink-5", "shrink-6", "shrink-7", "shrink-8", "shrink-9", "shrink-10", "shrink-11", "shrink-12",
        "shrink-[calc(100vw-var(--sidebar))]", "shrink-[0.5]", "shrink-(--my-shrink)",
    ];

    let mut working = 0;
    let mut broken = 0;

    println!("📋 Testing New Flexbox Utilities ({} classes):", test_classes.len());
    
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
