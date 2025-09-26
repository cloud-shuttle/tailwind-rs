use tailwind_rs_core::*;

/// Test box utilities
fn main() {
    println!("🔍 Box Utilities Test");
    println!("====================");

    let generator = CssGenerator::new();

    // Test box utilities classes
    let box_utilities_classes = vec![
        // box-decoration-break
        "box-decoration-clone",
        "box-decoration-slice",
        // box-sizing
        "box-border",
        "box-content",
        // display utilities
        "flow-root",
        "table-caption",
        "table-column",
        "table-column-group",
        "table-header-group",
        "table-row-group",
        "table-footer-group",
        "sr-only",
        "not-sr-only",
    ];

    println!(
        "\n📋 Testing Box Utilities Classes ({} classes):",
        box_utilities_classes.len()
    );
    let mut working = 0;
    let mut broken = 0;

    for class in &box_utilities_classes {
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

    let coverage = (working as f32 / box_utilities_classes.len() as f32) * 100.0;
    println!(
        "  📊 Box Utilities: {}/{} ({:.1}%)",
        working,
        box_utilities_classes.len(),
        coverage
    );

    println!("\n📊 Results:");
    println!("===========");
    println!("  ✅ Working: {}/{}", working, box_utilities_classes.len());
    println!("  ❌ Broken: {}/{}", broken, box_utilities_classes.len());
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
