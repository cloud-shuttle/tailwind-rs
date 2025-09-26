//! Typography Utilities Test
//!
//! This example tests comprehensive typography utilities including:
//! - Font family utilities (font-sans, font-serif, font-mono, custom properties, arbitrary values)
//! - Font size utilities (text-xs through text-9xl, custom properties, arbitrary values, line-height)
//! - Font smoothing utilities (antialiased, subpixel-antialiased)
//! - Font style utilities (italic, not-italic)
//! - Font weight utilities (font-thin through font-black, custom properties, arbitrary values)
//! - Font stretch utilities (font-stretch-* with named values and percentages)
//! - Font variant numeric utilities (normal-nums, ordinal, slashed-zero, etc.)

use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let generator = CssGenerator::new();

    // Test typography utilities
    let test_classes = vec![
        // Font family utilities
        "font-sans",
        "font-serif",
        "font-mono",
        "font-(--my-font)",
        "font-[Open_Sans]",
        // Font size utilities
        "text-xs",
        "text-sm",
        "text-base",
        "text-lg",
        "text-xl",
        "text-2xl",
        "text-3xl",
        "text-4xl",
        "text-5xl",
        "text-6xl",
        "text-7xl",
        "text-8xl",
        "text-9xl",
        "text-(--my-text-size)",
        "text-[14px]",
        "text-sm/6",
        "text-lg/7",
        "text-base/tight",
        "text-xl/relaxed",
        // Font smoothing utilities
        "antialiased",
        "subpixel-antialiased",
        // Font style utilities
        "italic",
        "not-italic",
        // Font weight utilities
        "font-thin",
        "font-extralight",
        "font-light",
        "font-normal",
        "font-medium",
        "font-semibold",
        "font-bold",
        "font-extrabold",
        "font-black",
        "font-(--my-font-weight)",
        "font-[1000]",
        // Font stretch utilities
        "font-stretch-ultra-condensed",
        "font-stretch-extra-condensed",
        "font-stretch-condensed",
        "font-stretch-semi-condensed",
        "font-stretch-normal",
        "font-stretch-semi-expanded",
        "font-stretch-expanded",
        "font-stretch-extra-expanded",
        "font-stretch-ultra-expanded",
        "font-stretch-50%",
        "font-stretch-100%",
        "font-stretch-150%",
        "font-stretch-(--my-font-width)",
        "font-stretch-[66.66%]",
        // Font variant numeric utilities
        "normal-nums",
        "ordinal",
        "slashed-zero",
        "lining-nums",
        "oldstyle-nums",
        "proportional-nums",
        "tabular-nums",
        "diagonal-fractions",
        "stacked-fractions",
    ];

    println!("🔍 Typography Utilities Test");
    println!("===================================");

    let mut working_count = 0;
    let mut broken_count = 0;

    for class in test_classes {
        match generator.class_to_properties(&class) {
            Ok(properties) => {
                println!("✅ {} -> {} properties", class, properties.len());
                for prop in properties {
                    println!("   {}: {}", prop.name, prop.value);
                }
                working_count += 1;
            }
            Err(e) => {
                println!("❌ {} -> Error: {}", class, e);
                broken_count += 1;
            }
        }
    }

    let total = working_count + broken_count;
    let coverage = if total > 0 {
        (working_count as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!("\n📊 Results:");
    println!("===========");
    println!("  ✅ Working: {}/{}", working_count, total);
    println!("  ❌ Broken: {}/{}", broken_count, total);
    println!("  📈 Coverage: {:.1}%", coverage);

    if coverage >= 95.0 {
        println!("\n🎉 Excellent coverage! Ready for production!");
    } else if coverage >= 80.0 {
        println!("\n✅ Good coverage! Minor improvements needed.");
    } else {
        println!("\n⚠️  Coverage needs improvement.");
    }
}
