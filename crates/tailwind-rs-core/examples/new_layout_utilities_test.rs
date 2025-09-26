//! New Layout Utilities Test
//!
//! This example tests the new layout utilities: overflow, overscroll, position, inset, visibility, and z-index.

use tailwind_rs_core::CssGenerator;

fn main() {
    println!("🔍 New Layout Utilities Test");
    println!("============================\n");

    let mut generator = CssGenerator::new();

    // Test classes for each new utility category
    let test_classes = vec![
        // Overflow utilities
        "overflow-auto",
        "overflow-hidden",
        "overflow-clip",
        "overflow-visible",
        "overflow-scroll",
        "overflow-x-auto",
        "overflow-x-hidden",
        "overflow-x-clip",
        "overflow-x-visible",
        "overflow-x-scroll",
        "overflow-y-auto",
        "overflow-y-hidden",
        "overflow-y-clip",
        "overflow-y-visible",
        "overflow-y-scroll",
        // Overscroll utilities
        "overscroll-auto",
        "overscroll-contain",
        "overscroll-none",
        "overscroll-x-auto",
        "overscroll-x-contain",
        "overscroll-x-none",
        "overscroll-y-auto",
        "overscroll-y-contain",
        "overscroll-y-none",
        // Position utilities
        "static",
        "fixed",
        "absolute",
        "relative",
        "sticky",
        // Inset utilities
        "inset-0",
        "inset-4",
        "inset-auto",
        "inset-full",
        "inset-x-0",
        "inset-x-4",
        "inset-y-0",
        "inset-y-4",
        "top-0",
        "top-4",
        "top-auto",
        "right-0",
        "right-4",
        "right-auto",
        "bottom-0",
        "bottom-4",
        "bottom-auto",
        "left-0",
        "left-4",
        "left-auto",
        "start-0",
        "start-4",
        "start-auto",
        "end-0",
        "end-4",
        "end-auto",
        "inset-[10px]",
        "top-[5px]",
        "right-[15px]",
        "bottom-[20px]",
        "left-[25px]",
        "inset-(--my-inset)",
        "top-(--my-top)",
        // Visibility utilities
        "visible",
        "invisible",
        "collapse",
        // Z-index utilities
        "z-0",
        "z-10",
        "z-20",
        "z-30",
        "z-40",
        "z-50",
        "z-auto",
        "-z-0",
        "-z-10",
        "-z-20",
        "-z-30",
        "-z-40",
        "-z-50",
        "z-[100]",
        "z-[999]",
        "z-(--my-z)",
    ];

    let mut working = 0;
    let mut broken = 0;

    println!(
        "📋 Testing New Layout Utilities ({} classes):",
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
