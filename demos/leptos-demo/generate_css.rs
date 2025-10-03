use std::fs;

use tailwind_rs_core::CssGenerator;

fn main() {
    println!("🎨 Generating comprehensive CSS for Tailwind-RS demo...");

    let mut generator = CssGenerator::new();

    // Test with a simple class first
    println!("Testing with a simple class...");
    let _ = generator.add_class("bg-blue-500");
    let simple_css = generator.generate_css();
    println!("Simple CSS generated: {} characters", simple_css.len());

    // Reset generator for comprehensive test
    let mut generator = CssGenerator::new();

    // Add comprehensive demo classes
    let demo_classes = vec![
        // Layout
        "container", "mx-auto", "flex", "grid", "block", "inline", "hidden",

        // Spacing
        "p-4", "m-2", "px-8", "py-6", "mt-8", "mb-4",

        // Colors
        "bg-blue-500", "bg-red-600", "bg-green-400", "text-white", "text-gray-800",
        "border-gray-300", "border-2",

        // Typography
        "text-lg", "font-bold", "text-center", "leading-relaxed",

        // Effects (our new shadow classes!)
        "shadow-sm", "shadow", "shadow-md", "shadow-lg", "shadow-xl", "shadow-2xl",
        "shadow-none", "shadow-inner",
        "shadow-blue-500/25", "shadow-purple-500/25",

        // Responsive variants
        "md:flex", "lg:grid", "sm:text-center",

        // Hover states
        "hover:bg-blue-600", "hover:shadow-lg",
    ];

    println!("Adding {} demo classes...", demo_classes.len());
    for class in demo_classes {
        let _ = generator.add_class(class);
    }

    let css = generator.generate_css();
    let css_len = css.len();
    println!("Comprehensive CSS length: {} characters", css_len);

    // Show first 500 characters for debugging
    if css_len > 0 {
        println!("First 500 characters of CSS:");
        println!("{}", &css[..std::cmp::min(500, css_len)]);
    }

    // Ensure the assets directory exists
    fs::create_dir_all("assets").unwrap_or_default();

    // Write the CSS to the assets file
    fs::write("assets/generated.css", css).expect("Failed to write CSS file");

    println!("✅ Generated CSS file with {} characters", css_len);
    println!("📁 CSS saved to: assets/generated.css");
}
