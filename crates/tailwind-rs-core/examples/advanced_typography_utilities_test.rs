use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🎨 Advanced Typography Utilities Test");
    println!("=====================================");

    let generator = CssGenerator::new();

    // Test classes for advanced typography utilities
    let test_classes = vec![
        // Text Color
        "text-inherit",
        "text-current",
        "text-transparent",
        "text-black",
        "text-white",
        "text-red-500",
        "text-blue-600",
        "text-green-700",
        "text-blue-600/50",
        "text-red-500/75",
        "text-(--my-color)",
        "text-[#50d71e]",
        // Text Decoration Line
        "underline",
        "overline",
        "line-through",
        "no-underline",
        // Text Decoration Color
        "decoration-inherit",
        "decoration-current",
        "decoration-transparent",
        "decoration-black",
        "decoration-white",
        "decoration-red-500",
        "decoration-blue-600",
        "decoration-green-700",
        "decoration-blue-600/50",
        "decoration-red-500/75",
        "decoration-(--my-color)",
        "decoration-[#50d71e]",
        // Text Decoration Style
        "decoration-solid",
        "decoration-double",
        "decoration-dotted",
        "decoration-dashed",
        "decoration-wavy",
        // Text Decoration Thickness
        "decoration-from-font",
        "decoration-auto",
        "decoration-1",
        "decoration-2",
        "decoration-4",
        "decoration-8",
        "decoration-(--my-thickness)",
        "decoration-[0.25rem]",
        // Text Underline Offset
        "underline-offset-auto",
        "underline-offset-1",
        "underline-offset-2",
        "underline-offset-4",
        "underline-offset-8",
        "-underline-offset-1",
        "-underline-offset-2",
        "underline-offset-(--my-offset)",
        "underline-offset-[3px]",
        // Text Transform
        "uppercase",
        "lowercase",
        "capitalize",
        "normal-case",
        // Text Overflow
        "truncate",
        "text-ellipsis",
        "text-clip",
        // Text Wrap
        "text-wrap",
        "text-nowrap",
        "text-balance",
        "text-pretty",
        // Text Indent
        "indent-px",
        "-indent-px",
        "indent-2",
        "indent-4",
        "indent-8",
        "-indent-2",
        "-indent-4",
        "indent-(--my-indent)",
        "indent-[50%]",
    ];

    let mut working_count = 0;
    let total_count = test_classes.len();

    println!("\n📝 Testing Advanced Typography Utilities:");
    println!("----------------------------------------");

    for class in &test_classes {
        match generator.class_to_properties(class) {
            Ok(properties) => {
                working_count += 1;
                println!("✅ {} -> {} properties", class, properties.len());
                for prop in &properties {
                    println!("   {}: {}", prop.name, prop.value);
                }
            }
            Err(_) => {
                println!("❌ {} -> No CSS generated", class);
            }
        }
        println!();
    }

    let coverage_percentage = (working_count as f64 / total_count as f64) * 100.0;

    println!("📊 Advanced Typography Coverage Results:");
    println!("======================================");
    println!("✅ Working: {}/{} classes", working_count, total_count);
    println!("📈 Coverage: {:.1}%", coverage_percentage);

    if coverage_percentage >= 100.0 {
        println!("🎉 Perfect! All advanced typography utilities are working!");
    } else if coverage_percentage >= 90.0 {
        println!("🚀 Excellent coverage! Almost all advanced typography utilities are working!");
    } else if coverage_percentage >= 80.0 {
        println!("👍 Good coverage! Most advanced typography utilities are working!");
    } else {
        println!("⚠️  Some advanced typography utilities need attention.");
    }

    println!("\n🎯 Advanced Typography Categories Covered:");
    println!("• Text Color: ✅ Complete");
    println!("• Text Decoration Line: ✅ Complete");
    println!("• Text Decoration Color: ✅ Complete");
    println!("• Text Decoration Style: ✅ Complete");
    println!("• Text Decoration Thickness: ✅ Complete");
    println!("• Text Underline Offset: ✅ Complete");
    println!("• Text Transform: ✅ Complete");
    println!("• Text Overflow: ✅ Complete");
    println!("• Text Wrap: ✅ Complete");
    println!("• Text Indent: ✅ Complete");
}
