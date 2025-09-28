use std::fs;

use tailwind_rs_leptos_demo_working::css_generator::DemoCssGenerator;

fn main() {
    println!("🎨 Generating comprehensive CSS for Tailwind-RS demo...");

    let mut generator = DemoCssGenerator::new();

    // Test with a simple class first
    println!("Testing with a simple class...");
    let _ = generator.add_class("bg-blue-500");
    let simple_css = generator.generate_css();
    println!("Simple CSS generated: {} characters", simple_css.len());

    // Reset generator for comprehensive test
    let mut generator = DemoCssGenerator::new();

    // Generate CSS with all comprehensive classes
    match generator.generate_demo_css() {
        Ok(css) => {
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
            println!("📁 CSS saved to: demos/leptos-demo/assets/generated.css");
        }
        Err(e) => {
            eprintln!("❌ Failed to generate CSS: {}", e);
            std::process::exit(1);
        }
    }
}
