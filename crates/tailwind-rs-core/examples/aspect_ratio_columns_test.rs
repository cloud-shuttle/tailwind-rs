use tailwind_rs_core::*;

/// Test aspect ratio and columns utilities
fn main() {
    println!("🔍 Aspect Ratio & Columns Utilities Test");
    println!("========================================");

    let generator = CssGenerator::new();

    // Test aspect ratio classes
    let aspect_ratio_classes = vec![
        "aspect-auto",
        "aspect-square",
        "aspect-video",
        "aspect-3/2",
        "aspect-4/3",
        "aspect-16/9",
        "aspect-[2/1]",
        "aspect-[calc(4*3+1)/3]",
        "aspect-(--my-aspect-ratio)",
    ];

    // Test columns classes
    let columns_classes = vec![
        "columns-1",
        "columns-2",
        "columns-3",
        "columns-4",
        "columns-5",
        "columns-6",
        "columns-3xs",
        "columns-2xs",
        "columns-xs",
        "columns-sm",
        "columns-md",
        "columns-lg",
        "columns-xl",
        "columns-2xl",
        "columns-3xl",
        "columns-4xl",
        "columns-5xl",
        "columns-6xl",
        "columns-7xl",
        "columns-[30vw]",
        "columns-(--my-columns)",
    ];

    println!(
        "\n📋 Testing Aspect Ratio Classes ({} classes):",
        aspect_ratio_classes.len()
    );
    let mut aspect_working = 0;
    let mut aspect_broken = 0;

    for class in &aspect_ratio_classes {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                aspect_working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
                aspect_broken += 1;
            }
        }
    }

    let aspect_coverage = (aspect_working as f32 / aspect_ratio_classes.len() as f32) * 100.0;
    println!(
        "  📊 Aspect Ratio: {}/{} ({:.1}%)",
        aspect_working,
        aspect_ratio_classes.len(),
        aspect_coverage
    );

    println!(
        "\n📋 Testing Columns Classes ({} classes):",
        columns_classes.len()
    );
    let mut columns_working = 0;
    let mut columns_broken = 0;

    for class in &columns_classes {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                columns_working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
                columns_broken += 1;
            }
        }
    }

    let columns_coverage = (columns_working as f32 / columns_classes.len() as f32) * 100.0;
    println!(
        "  📊 Columns: {}/{} ({:.1}%)",
        columns_working,
        columns_classes.len(),
        columns_coverage
    );

    let total_working = aspect_working + columns_working;
    let total_broken = aspect_broken + columns_broken;
    let total_classes = total_working + total_broken;
    let overall_coverage = (total_working as f32 / total_classes as f32) * 100.0;

    println!("\n📊 Overall Results:");
    println!("==================");
    println!("  ✅ Working: {}/{}", total_working, total_classes);
    println!("  ❌ Broken: {}/{}", total_broken, total_classes);
    println!("  📈 Overall Coverage: {:.1}%", overall_coverage);

    if overall_coverage >= 95.0 {
        println!("\n🎉 Excellent coverage! Ready for production!");
    } else if overall_coverage >= 90.0 {
        println!("\n🚀 Good coverage! Consider release candidate.");
    } else if overall_coverage >= 80.0 {
        println!("\n⚠️  Coverage needs improvement before release.");
    } else {
        println!("\n❌ Coverage needs significant improvement.");
    }
}
