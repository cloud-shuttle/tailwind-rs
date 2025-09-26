//! Order and Grid Utilities Test
//!
//! This example tests the new order and grid utilities: order, grid-template-columns, grid-column,
//! grid-template-rows, grid-row, grid-auto-flow, grid-auto-columns, and grid-auto-rows.

use tailwind_rs_core::CssGenerator;

fn main() {
    println!("🔍 Order and Grid Utilities Test");
    println!("================================\n");

    let mut generator = CssGenerator::new();

    // Test classes for each new utility category
    let test_classes = vec![
        // Order utilities
        "order-1",
        "order-2",
        "order-3",
        "order-4",
        "order-5",
        "order-6",
        "order-7",
        "order-8",
        "order-9",
        "order-10",
        "order-11",
        "order-12",
        "-order-1",
        "-order-2",
        "-order-3",
        "-order-4",
        "-order-5",
        "-order-6",
        "-order-7",
        "-order-8",
        "-order-9",
        "-order-10",
        "-order-11",
        "-order-12",
        "order-first",
        "order-last",
        "order-none",
        "order-[min(var(--total-items),10)]",
        "order-(--my-order)",
        // Grid template columns utilities
        "grid-cols-1",
        "grid-cols-2",
        "grid-cols-3",
        "grid-cols-4",
        "grid-cols-5",
        "grid-cols-6",
        "grid-cols-7",
        "grid-cols-8",
        "grid-cols-9",
        "grid-cols-10",
        "grid-cols-11",
        "grid-cols-12",
        "grid-cols-none",
        "grid-cols-subgrid",
        "grid-cols-[200px_minmax(900px,_1fr)_100px]",
        "grid-cols-(--my-grid-cols)",
        // Grid column utilities
        "col-span-1",
        "col-span-2",
        "col-span-3",
        "col-span-4",
        "col-span-5",
        "col-span-6",
        "col-span-7",
        "col-span-8",
        "col-span-9",
        "col-span-10",
        "col-span-11",
        "col-span-12",
        "col-span-full",
        "col-start-1",
        "col-start-2",
        "col-start-3",
        "col-start-4",
        "col-start-5",
        "col-start-6",
        "col-start-7",
        "col-start-8",
        "col-start-9",
        "col-start-10",
        "col-start-11",
        "col-start-12",
        "col-start-auto",
        "-col-start-1",
        "-col-start-2",
        "-col-start-3",
        "-col-start-4",
        "-col-start-5",
        "-col-start-6",
        "-col-start-7",
        "-col-start-8",
        "-col-start-9",
        "-col-start-10",
        "-col-start-11",
        "-col-start-12",
        "col-end-1",
        "col-end-2",
        "col-end-3",
        "col-end-4",
        "col-end-5",
        "col-end-6",
        "col-end-7",
        "col-end-8",
        "col-end-9",
        "col-end-10",
        "col-end-11",
        "col-end-12",
        "col-end-auto",
        "-col-end-1",
        "-col-end-2",
        "-col-end-3",
        "-col-end-4",
        "-col-end-5",
        "-col-end-6",
        "-col-end-7",
        "-col-end-8",
        "-col-end-9",
        "-col-end-10",
        "-col-end-11",
        "-col-end-12",
        "col-[16_/_span_16]",
        "col-span-[3]",
        "col-start-[2]",
        "col-end-[4]",
        "col-(--my-columns)",
        "col-span-(--my-span)",
        "col-start-(--my-start)",
        "col-end-(--my-end)",
        // Grid template rows utilities
        "grid-rows-1",
        "grid-rows-2",
        "grid-rows-3",
        "grid-rows-4",
        "grid-rows-5",
        "grid-rows-6",
        "grid-rows-7",
        "grid-rows-8",
        "grid-rows-9",
        "grid-rows-10",
        "grid-rows-11",
        "grid-rows-12",
        "grid-rows-none",
        "grid-rows-subgrid",
        "grid-rows-[200px_minmax(900px,1fr)_100px]",
        "grid-rows-(--my-grid-rows)",
        // Grid row utilities
        "row-span-1",
        "row-span-2",
        "row-span-3",
        "row-span-4",
        "row-span-5",
        "row-span-6",
        "row-span-7",
        "row-span-8",
        "row-span-9",
        "row-span-10",
        "row-span-11",
        "row-span-12",
        "row-span-full",
        "row-start-1",
        "row-start-2",
        "row-start-3",
        "row-start-4",
        "row-start-5",
        "row-start-6",
        "row-start-7",
        "row-start-8",
        "row-start-9",
        "row-start-10",
        "row-start-11",
        "row-start-12",
        "row-start-auto",
        "-row-start-1",
        "-row-start-2",
        "-row-start-3",
        "-row-start-4",
        "-row-start-5",
        "-row-start-6",
        "-row-start-7",
        "-row-start-8",
        "-row-start-9",
        "-row-start-10",
        "-row-start-11",
        "-row-start-12",
        "row-end-1",
        "row-end-2",
        "row-end-3",
        "row-end-4",
        "row-end-5",
        "row-end-6",
        "row-end-7",
        "row-end-8",
        "row-end-9",
        "row-end-10",
        "row-end-11",
        "row-end-12",
        "row-end-auto",
        "-row-end-1",
        "-row-end-2",
        "-row-end-3",
        "-row-end-4",
        "-row-end-5",
        "-row-end-6",
        "-row-end-7",
        "-row-end-8",
        "-row-end-9",
        "-row-end-10",
        "-row-end-11",
        "-row-end-12",
        "row-[span_16_/_span_16]",
        "row-span-[3]",
        "row-start-[2]",
        "row-end-[4]",
        "row-(--my-rows)",
        "row-span-(--my-span)",
        "row-start-(--my-start)",
        "row-end-(--my-end)",
        // Grid auto flow utilities
        "grid-flow-row",
        "grid-flow-col",
        "grid-flow-dense",
        "grid-flow-row-dense",
        "grid-flow-col-dense",
        // Grid auto columns utilities
        "auto-cols-auto",
        "auto-cols-min",
        "auto-cols-max",
        "auto-cols-fr",
        "auto-cols-[minmax(0,2fr)]",
        "auto-cols-(--my-auto-cols)",
        // Grid auto rows utilities
        "auto-rows-auto",
        "auto-rows-min",
        "auto-rows-max",
        "auto-rows-fr",
        "auto-rows-[minmax(0,2fr)]",
        "auto-rows-(--my-auto-rows)",
    ];

    let mut working = 0;
    let mut broken = 0;

    println!(
        "📋 Testing Order and Grid Utilities ({} classes):",
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
