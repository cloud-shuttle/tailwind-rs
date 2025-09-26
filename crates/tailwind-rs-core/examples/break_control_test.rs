use tailwind_rs_core::*;

/// Test break control utilities
fn main() {
    println!("🔍 Break Control Utilities Test");
    println!("================================");

    let generator = CssGenerator::new();

    // Test break control classes
    let break_control_classes = vec![
        // break-after
        "break-after-auto",
        "break-after-avoid",
        "break-after-all",
        "break-after-avoid-page",
        "break-after-page",
        "break-after-left",
        "break-after-right",
        "break-after-column",
        // break-before
        "break-before-auto",
        "break-before-avoid",
        "break-before-all",
        "break-before-avoid-page",
        "break-before-page",
        "break-before-left",
        "break-before-right",
        "break-before-column",
        // break-inside
        "break-inside-auto",
        "break-inside-avoid",
        "break-inside-avoid-page",
        "break-inside-avoid-column",
    ];

    println!(
        "\n📋 Testing Break Control Classes ({} classes):",
        break_control_classes.len()
    );
    let mut working = 0;
    let mut broken = 0;

    for class in &break_control_classes {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
                broken += 1;
            }
        }
    }

    let coverage = (working as f32 / break_control_classes.len() as f32) * 100.0;
    println!(
        "  📊 Break Control: {}/{} ({:.1}%)",
        working,
        break_control_classes.len(),
        coverage
    );

    println!("\n📊 Results:");
    println!("===========");
    println!("  ✅ Working: {}/{}", working, break_control_classes.len());
    println!("  ❌ Broken: {}/{}", broken, break_control_classes.len());
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
