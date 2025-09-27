//! Debug Flex Parsers Test
//!
//! This test debugs why FlexGrowParser and FlexShrinkParser are not working.

use tailwind_rs_core::*;

#[test]
fn debug_flex_parsers() -> Result<()> {
    println!("🔍 DEBUG FLEX PARSERS");
    println!("Debugging FlexGrowParser and FlexShrinkParser...\n");

    // Test FlexGrowParser
    println!("🔧 Testing FlexGrowParser:");
    let flex_grow_parser = FlexGrowParser::new();

    // Test different classes
    let test_classes = vec![
        "grow",
        "grow-0",
        "grow-1",
        "grow-2",
        "grow-3",
        "grow-4",
        "grow-5",
        "grow-6",
        "grow-7",
        "grow-8",
        "grow-9",
        "grow-10",
        "grow-11",
        "grow-12",
        "grow-[25vw]",
        "grow-(--my-grow)",
    ];

    for class in test_classes {
        let result = flex_grow_parser.parse_class(class);
        println!("  {} -> {:?}", class, result);
        if result.is_some() {
            println!("    ✅ WORKING");
        } else {
            println!("    ❌ STUB");
        }
    }

    // Test FlexShrinkParser
    println!("\n🔧 Testing FlexShrinkParser:");
    let flex_shrink_parser = FlexShrinkParser::new();

    // Test different classes
    let test_classes = vec![
        "shrink",
        "shrink-0",
        "shrink-1",
        "shrink-2",
        "shrink-3",
        "shrink-4",
        "shrink-5",
        "shrink-6",
        "shrink-7",
        "shrink-8",
        "shrink-9",
        "shrink-10",
        "shrink-11",
        "shrink-12",
        "shrink-[calc(100vw-var(--sidebar))]",
        "shrink-(--my-shrink)",
    ];

    for class in test_classes {
        let result = flex_shrink_parser.parse_class(class);
        println!("  {} -> {:?}", class, result);
        if result.is_some() {
            println!("    ✅ WORKING");
        } else {
            println!("    ❌ STUB");
        }
    }

    Ok(())
}
