use tailwind_rs_core::*;

/// Test layout utilities
fn main() {
    println!("🔍 Layout Utilities Test");
    println!("========================");

    let generator = CssGenerator::new();

    // Test layout utilities classes
    let layout_utilities_classes = vec![
        // float
        "float-right",
        "float-left",
        "float-start",
        "float-end",
        "float-none",
        // clear
        "clear-left",
        "clear-right",
        "clear-both",
        "clear-start",
        "clear-end",
        "clear-none",
        // isolation
        "isolate",
        "isolation-auto",
        // object-position
        "object-top-left",
        "object-top",
        "object-top-right",
        "object-left",
        "object-center",
        "object-right",
        "object-bottom-left",
        "object-bottom",
        "object-bottom-right",
        // arbitrary values
        "object-[25%_75%]",
        "object-(--my-object)",
    ];

    println!(
        "\n📋 Testing Layout Utilities Classes ({} classes):",
        layout_utilities_classes.len()
    );
    let mut working = 0;
    let mut broken = 0;

    for class in &layout_utilities_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => {
                println!("  ✅ {} -> {} properties", class, properties.len());
                working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
                broken += 1;
            }
        }
    }

    let coverage = (working as f32 / layout_utilities_classes.len() as f32) * 100.0;
    println!(
        "  📊 Layout Utilities: {}/{} ({:.1}%)",
        working,
        layout_utilities_classes.len(),
        coverage
    );

    println!("\n📊 Results:");
    println!("===========");
    println!(
        "  ✅ Working: {}/{}",
        working,
        layout_utilities_classes.len()
    );
    println!("  ❌ Broken: {}/{}", broken, layout_utilities_classes.len());
    println!("  📈 Coverage: {:.1}%", coverage);

    if coverage >= 95.0 {
        println!("\n🎉 Excellent coverage! Ready for production!");
    } else if coverage >= 90.0 {
        println!("\n🚀 Good coverage! Consider release candidate.");
    } else if coverage >= 80.0 {
        println!("\n⚠️  Coverage needs improvement before release.");
    } else {
        println!("\n❌ Coverage needs significant improvement.");
    }
}
