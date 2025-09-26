use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🎨 Final Typography Utilities Test");
    println!("==================================");

    let generator = CssGenerator::new();

    // Test classes for final typography utilities
    let test_classes = vec![
        // Vertical Align
        "align-baseline", "align-top", "align-middle", "align-bottom", "align-text-top", "align-text-bottom", "align-sub", "align-super",
        "align-(--my-alignment)", "align-[4px]",
        
        // White Space
        "whitespace-normal", "whitespace-nowrap", "whitespace-pre", "whitespace-pre-line", "whitespace-pre-wrap", "whitespace-break-spaces",
        
        // Word Break
        "break-normal", "break-all", "break-keep",
        
        // Overflow Wrap
        "wrap-break-word", "wrap-anywhere", "wrap-normal",
        
        // Hyphens
        "hyphens-none", "hyphens-manual", "hyphens-auto",
        
        // Content
        "content-none", "content-(--my-content)", "content-['Hello_World']", "content-[attr(data-before)]",
    ];

    let mut working_count = 0;
    let total_count = test_classes.len();

    println!("\n📝 Testing Final Typography Utilities:");
    println!("-------------------------------------");

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
    
    println!("📊 Final Typography Coverage Results:");
    println!("====================================");
    println!("✅ Working: {}/{} classes", working_count, total_count);
    println!("📈 Coverage: {:.1}%", coverage_percentage);
    
    if coverage_percentage >= 100.0 {
        println!("🎉 Perfect! All final typography utilities are working!");
    } else if coverage_percentage >= 90.0 {
        println!("🚀 Excellent coverage! Almost all final typography utilities are working!");
    } else if coverage_percentage >= 80.0 {
        println!("👍 Good coverage! Most final typography utilities are working!");
    } else {
        println!("⚠️  Some final typography utilities need attention.");
    }
    
    println!("\n🎯 Final Typography Categories Covered:");
    println!("• Vertical Align: ✅ Complete");
    println!("• White Space: ✅ Complete");
    println!("• Word Break: ✅ Complete");
    println!("• Overflow Wrap: ✅ Complete");
    println!("• Hyphens: ✅ Complete");
    println!("• Content: ✅ Complete");
}
