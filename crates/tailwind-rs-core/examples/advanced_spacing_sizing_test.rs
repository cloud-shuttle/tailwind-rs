//! Advanced Spacing and Sizing Test
//!
//! This example tests advanced spacing and sizing utilities that might be missing
//! from our current implementation.

use tailwind_rs_core::CssGenerator;

fn main() {
    println!("🔍 Advanced Spacing and Sizing Test");
    println!("===================================\n");

    let mut generator = CssGenerator::new();

    // Test classes for advanced spacing and sizing utilities
    let test_classes = vec![
        // Logical properties for padding
        "ps-0",
        "ps-1",
        "ps-2",
        "ps-4",
        "ps-8",
        "ps-16",
        "ps-32",
        "ps-64",
        "pe-0",
        "pe-1",
        "pe-2",
        "pe-4",
        "pe-8",
        "pe-16",
        "pe-32",
        "pe-64",
        // Logical properties for margin
        "ms-0",
        "ms-1",
        "ms-2",
        "ms-4",
        "ms-8",
        "ms-16",
        "ms-32",
        "ms-64",
        "me-0",
        "me-1",
        "me-2",
        "me-4",
        "me-8",
        "me-16",
        "me-32",
        "me-64",
        // Container scale utilities
        "w-3xs",
        "w-2xs",
        "w-xs",
        "w-sm",
        "w-md",
        "w-lg",
        "w-xl",
        "w-2xl",
        "min-w-3xs",
        "min-w-2xs",
        "min-w-xs",
        "min-w-sm",
        "min-w-md",
        "min-w-lg",
        "min-w-xl",
        "min-w-2xl",
        "max-w-3xs",
        "max-w-2xs",
        "max-w-xs",
        "max-w-sm",
        "max-w-md",
        "max-w-lg",
        "max-w-xl",
        "max-w-2xl",
        // Viewport units
        "h-dvh",
        "h-lvh",
        "h-svh",
        "h-dvw",
        "h-lvw",
        "h-svw",
        "min-h-dvh",
        "min-h-lvh",
        "min-h-svh",
        "min-h-dvw",
        "min-h-lvw",
        "min-h-svw",
        "max-h-dvh",
        "max-h-lvh",
        "max-h-svh",
        "max-h-dvw",
        "max-h-lvw",
        "max-h-svw",
        // Size utilities (both width and height)
        "size-0",
        "size-1",
        "size-2",
        "size-4",
        "size-8",
        "size-16",
        "size-32",
        "size-64",
        "size-px",
        "size-0.5",
        "size-1.5",
        "size-2.5",
        "size-3.5",
        // Space utilities
        "space-x-0",
        "space-x-1",
        "space-x-2",
        "space-x-4",
        "space-x-8",
        "space-x-16",
        "space-x-32",
        "space-x-64",
        "space-y-0",
        "space-y-1",
        "space-y-2",
        "space-y-4",
        "space-y-8",
        "space-y-16",
        "space-y-32",
        "space-y-64",
        "space-x-reverse",
        "space-y-reverse",
        // Fractional utilities
        "w-1/2",
        "w-1/3",
        "w-2/3",
        "w-1/4",
        "w-3/4",
        "w-1/5",
        "w-2/5",
        "w-3/5",
        "w-4/5",
        "w-1/6",
        "w-5/6",
        "w-1/12",
        "w-2/12",
        "w-3/12",
        "w-4/12",
        "w-5/12",
        "w-6/12",
        "w-7/12",
        "w-8/12",
        "w-9/12",
        "w-10/12",
        "w-11/12",
        "w-full",
        "h-1/2",
        "h-1/3",
        "h-2/3",
        "h-1/4",
        "h-3/4",
        "h-1/5",
        "h-2/5",
        "h-3/5",
        "h-4/5",
        "h-1/6",
        "h-5/6",
        "h-full",
        // Arbitrary values
        "p-[5px]",
        "m-[10px]",
        "w-[100px]",
        "h-[50px]",
        "ps-[5px]",
        "pe-[10px]",
        "ms-[15px]",
        "me-[20px]",
        "size-[25px]",
        "space-x-[30px]",
        "space-y-[35px]",
        // Custom properties
        "p-(--my-padding)",
        "m-(--my-margin)",
        "w-(--my-width)",
        "h-(--my-height)",
        "ps-(--my-padding-start)",
        "pe-(--my-padding-end)",
        "ms-(--my-margin-start)",
        "me-(--my-margin-end)",
    ];

    let mut working = 0;
    let mut broken = 0;

    println!(
        "📋 Testing Advanced Spacing and Sizing Utilities ({} classes):",
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
