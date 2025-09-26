use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🎨 Testing Additional Typography Utilities");
    println!("==========================================");

    let generator = CssGenerator::new();

    // Test classes for all additional typography utilities
    let test_classes = vec![
        // Letter spacing (tracking)
        "tracking-tighter",
        "tracking-tight",
        "tracking-normal",
        "tracking-wide",
        "tracking-wider",
        "tracking-widest",
        "tracking-(--my-tracking)",
        "tracking-[.25em]",
        "-tracking-2",
        // Line clamp
        "line-clamp-none",
        "line-clamp-1",
        "line-clamp-2",
        "line-clamp-3",
        "line-clamp-4",
        "line-clamp-5",
        "line-clamp-6",
        "line-clamp-(--my-line-count)",
        "line-clamp-[calc(var(--characters)/100)]",
        // Line height (leading)
        "leading-none",
        "leading-3",
        "leading-4",
        "leading-5",
        "leading-6",
        "leading-7",
        "leading-8",
        "leading-9",
        "leading-10",
        "leading-(--my-line-height)",
        "leading-[1.5]",
        // List style image
        "list-image-none",
        "list-image-[url(/img/checkmark.png)]",
        "list-image-(--my-list-image)",
        // List style position
        "list-inside",
        "list-outside",
        // List style type
        "list-disc",
        "list-decimal",
        "list-none",
        "list-(--my-marker)",
        "list-[upper-roman]",
        // Text align
        "text-left",
        "text-center",
        "text-right",
        "text-justify",
        "text-start",
        "text-end",
    ];

    let mut working_count = 0;
    let mut total_count = test_classes.len();

    println!("\n📝 Testing Additional Typography Utilities:");
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

    println!("📊 Additional Typography Coverage Results:");
    println!("=========================================");
    println!("✅ Working: {}/{} classes", working_count, total_count);
    println!("📈 Coverage: {:.1}%", coverage_percentage);

    if coverage_percentage >= 100.0 {
        println!("🎉 Perfect! All additional typography utilities are working!");
    } else if coverage_percentage >= 90.0 {
        println!("🚀 Excellent coverage! Almost all additional typography utilities are working!");
    } else if coverage_percentage >= 80.0 {
        println!("👍 Good coverage! Most additional typography utilities are working!");
    } else {
        println!("⚠️  Some additional typography utilities need attention.");
    }
}
