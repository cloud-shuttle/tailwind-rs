use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🧪 Testing Transform Utilities");
    println!("=============================");

    let test_classes = vec![
        // Backface visibility
        "backface-hidden",
        "backface-visible",
        // Perspective
        "perspective-dramatic",
        "perspective-near",
        "perspective-normal",
        "perspective-midrange",
        "perspective-distant",
        "perspective-none",
        // Perspective origin
        "perspective-origin-center",
        "perspective-origin-top",
        "perspective-origin-top-right",
        "perspective-origin-right",
        "perspective-origin-bottom-right",
        "perspective-origin-bottom",
        "perspective-origin-bottom-left",
        "perspective-origin-left",
        "perspective-origin-top-left",
        // Transform style
        "transform-3d",
        "transform-flat",
        // Basic transforms
        "transform",
        "transform-gpu",
        "transform-cpu",
        "transform-none",
        // Rotate
        "rotate-none",
        "rotate-45",
        "rotate-90",
        "rotate-180",
        "-rotate-45",
        "-rotate-90",
        // Scale
        "scale-75",
        "scale-100",
        "scale-125",
        "scale-150",
        "-scale-75",
        "-scale-125",
        // Skew
        "skew-3",
        "skew-6",
        "skew-12",
        "-skew-3",
        "-skew-6",
        "-skew-12",
        // Skew X
        "skew-x-3",
        "skew-x-6",
        "skew-x-12",
        "-skew-x-3",
        "-skew-x-6",
        "-skew-x-12",
        // Skew Y
        "skew-y-3",
        "skew-y-6",
        "skew-y-12",
        "-skew-y-3",
        "-skew-y-6",
        "-skew-y-12",
        // Transform origin
        "origin-center",
        "origin-top",
        "origin-top-right",
        "origin-right",
        "origin-bottom-right",
        "origin-bottom",
        "origin-bottom-left",
        "origin-left",
        "origin-top-left",
    ];

    let mut generator = CssGenerator::new();

    for class in &test_classes {
        match generator.add_class(class) {
            Ok(_) => {
                println!("✅ {} - Added", class);
            }
            Err(e) => {
                println!("❌ {} - Failed: {}", class, e);
            }
        }
    }

    let css = generator.generate_css();
    println!("\n📝 Generated CSS:");
    for line in css.lines() {
        if line.contains("backface")
            || line.contains("perspective")
            || line.contains("transform")
            || line.contains("rotate")
            || line.contains("scale")
            || line.contains("skew")
            || line.contains("origin")
        {
            println!("  {}", line);
        }
    }
}
