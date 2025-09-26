use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🧪 Testing Tailwind-RS Mask Utilities - Complete Coverage Test");
    println!("================================================================");

    // Test classes that were previously failing
    let test_classes = vec![
        // Previously failing classes
        "mask-x-to-90%",
        "mask-y-to-90%",
        "mask-radial-to-85%",
        "mask-conic-to-75%",
        "mask-conic-45",
        // Additional comprehensive mask tests
        "mask-none",
        "mask-alpha",
        "mask-luminance",
        "mask-match",
        "mask-origin-border",
        "mask-origin-padding",
        "mask-origin-content",
        "mask-origin-fill",
        "mask-origin-stroke",
        "mask-origin-view",
        "mask-top-left",
        "mask-top",
        "mask-top-right",
        "mask-left",
        "mask-center",
        "mask-right",
        "mask-bottom-left",
        "mask-bottom",
        "mask-bottom-right",
        "mask-repeat",
        "mask-no-repeat",
        "mask-repeat-x",
        "mask-repeat-y",
        "mask-repeat-space",
        "mask-repeat-round",
        "mask-auto",
        "mask-cover",
        "mask-contain",
        "mask-type-alpha",
        "mask-type-luminance",
        // Linear gradient masks
        "mask-linear-45",
        "-mask-linear-45",
        "mask-linear-from-50%",
        "mask-linear-from-20",
        "mask-t-from-50%",
        "mask-r-from-30%",
        "mask-b-from-20%",
        "mask-l-from-50%",
        "mask-x-from-70%",
        "mask-y-from-80%",
        // Radial gradient masks
        "mask-radial-from-75%",
        "mask-radial-from-50",
        // Conic gradient masks
        "mask-conic-from-60%",
        "mask-conic-from-40",
        // Custom properties and arbitrary values
        "mask-[url(/img/mask.png)]",
        "mask-(--custom-mask)",
        "mask-position-[center_top_1rem]",
        "mask-position-(--custom-position)",
        "mask-size-[50%_50%]",
        "mask-size-(--custom-size)",
    ];

    let mut generator = CssGenerator::new();
    let mut successful_classes = 0;
    let mut failed_classes = Vec::new();

    for class in &test_classes {
        match generator.add_class(class) {
            Ok(_) => {
                println!("✅ {} - Added", class);
                successful_classes += 1;
            }
            Err(e) => {
                println!("❌ {} - Failed: {}", class, e);
                failed_classes.push(class);
            }
        }
    }

    println!("\n📊 Results:");
    println!(
        "✅ Classes added: {}/{}",
        successful_classes,
        test_classes.len()
    );
    println!("❌ Classes failed: {}", failed_classes.len());

    if !failed_classes.is_empty() {
        println!("\n❌ Failed classes:");
        for class in &failed_classes {
            println!("  - {}", class);
        }
    }

    // Generate CSS
    let css = generator.generate_css();
    println!("\n✅ CSS generated successfully");
    println!("📊 Generated {} CSS rules", css.lines().count());

    // Show a sample of the generated CSS
    let lines: Vec<&str> = css.lines().take(10).collect();
    println!("\n📝 Sample CSS output:");
    for line in lines {
        println!("  {}", line);
    }
    if css.lines().count() > 10 {
        println!("  ... and {} more lines", css.lines().count() - 10);
    }

    // Calculate coverage percentage
    let coverage = (successful_classes as f64 / test_classes.len() as f64) * 100.0;
    println!("\n🎯 Coverage: {:.1}%", coverage);

    if coverage >= 100.0 {
        println!("🎉 Perfect! All mask utilities are working!");
    } else if coverage >= 95.0 {
        println!("🔥 Excellent! Nearly perfect coverage!");
    } else if coverage >= 90.0 {
        println!("✅ Great! Good coverage achieved!");
    } else {
        println!("⚠️  Some mask utilities need attention.");
    }
}
