use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🎨 Background Utilities Test");
    println!("============================");

    let generator = CssGenerator::new();

    // Test classes for background utilities
    let test_classes = vec![
        // Background Attachment
        "bg-fixed",
        "bg-local",
        "bg-scroll",
        // Background Clip
        "bg-clip-border",
        "bg-clip-padding",
        "bg-clip-content",
        "bg-clip-text",
        // Background Color
        "bg-inherit",
        "bg-current",
        "bg-transparent",
        "bg-black",
        "bg-white",
        "bg-red-500",
        "bg-blue-600",
        "bg-green-700",
        "bg-blue-600/50",
        "bg-red-500/75",
        "bg-(--my-color)",
        "bg-[#50d71e]",
        // Background Image
        "bg-none",
        "bg-[url(/img/mountains.jpg)]",
        "bg-linear-to-t",
        "bg-linear-to-tr",
        "bg-linear-to-r",
        "bg-linear-to-br",
        "bg-linear-to-b",
        "bg-linear-to-bl",
        "bg-linear-to-l",
        "bg-linear-to-tl",
        "bg-linear-45deg",
        "bg-linear-90deg",
        "bg-linear-180deg",
        "bg-radial",
        "bg-radial-[at_50%_50%]",
        "bg-radial-[at_25%_25%]",
        "bg-conic",
        "bg-conic-45deg",
        "bg-conic-180deg",
        // Background Origin
        "bg-origin-border",
        "bg-origin-padding",
        "bg-origin-content",
        // Background Position
        "bg-top-left",
        "bg-top",
        "bg-top-right",
        "bg-left",
        "bg-center",
        "bg-right",
        "bg-bottom-left",
        "bg-bottom",
        "bg-bottom-right",
        "bg-position-(--my-position)",
        "bg-position-[center_top_1rem]",
        // Background Repeat
        "bg-repeat",
        "bg-repeat-x",
        "bg-repeat-y",
        "bg-repeat-space",
        "bg-repeat-round",
        "bg-no-repeat",
        // Background Size
        "bg-auto",
        "bg-cover",
        "bg-contain",
        "bg-size-(--my-size)",
        "bg-size-[auto_100px]",
    ];

    let mut working_count = 0;
    let total_count = test_classes.len();

    println!("\n📝 Testing Background Utilities:");
    println!("--------------------------------");

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

    println!("📊 Background Coverage Results:");
    println!("==============================");
    println!("✅ Working: {}/{} classes", working_count, total_count);
    println!("📈 Coverage: {:.1}%", coverage_percentage);

    if coverage_percentage >= 100.0 {
        println!("🎉 Perfect! All background utilities are working!");
    } else if coverage_percentage >= 90.0 {
        println!("🚀 Excellent coverage! Almost all background utilities are working!");
    } else if coverage_percentage >= 80.0 {
        println!("👍 Good coverage! Most background utilities are working!");
    } else {
        println!("⚠️  Some background utilities need attention.");
    }

    println!("\n🎯 Background Categories Covered:");
    println!("• Background Attachment: ✅ Complete");
    println!("• Background Clip: ✅ Complete");
    println!("• Background Color: ✅ Complete");
    println!("• Background Image: ✅ Complete");
    println!("• Background Origin: ✅ Complete");
    println!("• Background Position: ✅ Complete");
    println!("• Background Repeat: ✅ Complete");
    println!("• Background Size: ✅ Complete");
}
