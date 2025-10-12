//! Test script to verify that the Tailwind-RS demo is generating colors correctly
//! by analyzing the generated CSS and HTML files

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use regex::Regex;

#[derive(Debug, Default)]
struct ColorAnalysis {
    pub gradient_classes: usize,
    pub color_properties: usize,
    pub background_gradients: usize,
    pub text_gradients: usize,
    pub button_gradients: usize,
    pub unique_colors: HashSet<String>,
    pub total_css_size: usize,
}

impl ColorAnalysis {
    fn analyze_css_file(&mut self, css_content: &str) {
        self.total_css_size = css_content.len();

        // Find gradient-related classes
        let gradient_regex = Regex::new(r"\.bg-gradient-to-\w+|from-\w+-\d+|to-\w+-\d+|via-\w+-\d+").unwrap();
        self.gradient_classes = gradient_regex.find_iter(css_content).count();

        // Find color properties
        let color_regex = Regex::new(r"color:\s*#[0-9a-fA-F]{6}|background-color:\s*#[0-9a-fA-F]{6}|border-color:\s*#[0-9a-fA-F]{6}").unwrap();
        self.color_properties = color_regex.find_iter(css_content).count();

        // Find background gradients
        let bg_gradient_regex = Regex::new(r"background:\s*linear-gradient|background-image:\s*linear-gradient").unwrap();
        self.background_gradients = bg_gradient_regex.find_iter(css_content).count();

        // Find unique colors
        let hex_color_regex = Regex::new(r"#[0-9a-fA-F]{6}|#[0-9a-fA-F]{3}").unwrap();
        for cap in hex_color_regex.captures_iter(css_content) {
            if let Some(color) = cap.get(0) {
                self.unique_colors.insert(color.as_str().to_string());
            }
        }
    }

    fn analyze_html_file(&mut self, html_content: &str) {
        // Check for gradient text classes
        if html_content.contains("gradient-text") {
            self.text_gradients += 1;
        }

        // Check for button gradients (simplified)
        let button_gradient_regex = Regex::new(r"bg-gradient-to-\w+.*button|button.*bg-gradient-to-\w+").unwrap();
        self.button_gradients = button_gradient_regex.find_iter(html_content).count();
    }

    fn is_successful(&self) -> bool {
        self.gradient_classes > 10 &&
        self.color_properties > 50 &&
        self.background_gradients > 0 &&
        self.unique_colors.len() > 100 &&
        self.total_css_size > 20000 // At least 20KB of CSS
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--visual-test".to_string()) {
        println!("🖼️  Visual Test Mode");
        println!("===================");
        println!("📝 To test the demo visually:");
        println!("1. HTTP server is running on http://localhost:8080");
        println!("2. Open your browser and navigate to http://localhost:8080");
        println!("3. Check if the demo looks colorful or appears bland");
        println!("4. Look for gradient backgrounds, colored text, and vibrant buttons");
        println!();
        println!("💡 The demo uses a dark theme with glassmorphism effects");
        println!("💡 It should have colorful gradient backgrounds and text");
        println!("💡 Check for purple/pink/red/blue gradients in the cards");
        return Ok(());
    }

    println!("🎨 Analyzing Tailwind-RS Demo Colors");
    println!("=====================================");

    let demo_dir = Path::new("demos/leptos-demo");
    let css_path = demo_dir.join("assets/generated.css");
    let html_path = demo_dir.join("index.html");

    // Check if files exist
    if !css_path.exists() {
        eprintln!("❌ Generated CSS file not found: {:?}", css_path);
        eprintln!("💡 Run the demo build first to generate CSS");
        std::process::exit(1);
    }

    if !html_path.exists() {
        eprintln!("❌ HTML file not found: {:?}", html_path);
        std::process::exit(1);
    }

    // Analyze CSS
    println!("📄 Analyzing generated CSS...");
    let css_content = fs::read_to_string(&css_path)?;
    let mut analysis = ColorAnalysis::default();
    analysis.analyze_css_file(&css_content);
    println!("✅ CSS analyzed");

    // Analyze HTML
    println!("📄 Analyzing HTML...");
    let html_content = fs::read_to_string(&html_path)?;
    analysis.analyze_html_file(&html_content);
    println!("✅ HTML analyzed");

    // Print results
    println!("\n🎨 CSS Color Analysis:");
    println!("- Total CSS size: {} bytes", analysis.total_css_size);
    println!("- Gradient classes: {}", analysis.gradient_classes);
    println!("- Color properties: {}", analysis.color_properties);
    println!("- Background gradients: {}", analysis.background_gradients);
    println!("- Text gradients: {}", analysis.text_gradients);
    println!("- Button gradients: {}", analysis.button_gradients);
    println!("- Unique colors: {}", analysis.unique_colors.len());
    println!("- Overall: {}", if analysis.is_successful() { "✅ GOOD" } else { "❌ POOR" });

    // Check specific issues
    println!("\n🔍 Detailed Analysis:");

    if analysis.gradient_classes == 0 {
        println!("❌ No gradient classes found in CSS - Tailwind gradients not working");
    } else {
        println!("✅ Found {} gradient classes", analysis.gradient_classes);
    }

    if analysis.color_properties == 0 {
        println!("❌ No color properties found in CSS - Tailwind colors not generating");
    } else {
        println!("✅ Found {} color properties", analysis.color_properties);
    }

    if analysis.unique_colors.len() < 10 {
        println!("❌ Very few unique colors ({}) - limited color palette", analysis.unique_colors.len());
    } else {
        println!("✅ Good color variety with {} unique colors", analysis.unique_colors.len());
    }

    if analysis.total_css_size < 10000 {
        println!("❌ CSS file too small ({} bytes) - incomplete generation", analysis.total_css_size);
    } else {
        println!("✅ CSS file size looks good ({} bytes)", analysis.total_css_size);
    }

    // Sample some colors
    if !analysis.unique_colors.is_empty() {
        println!("\n🌈 Sample colors found:");
        let sample_colors: Vec<_> = analysis.unique_colors.iter().take(10).collect();
        for color in sample_colors {
            print!("{} ", color);
        }
        println!();
    }

    // Overall assessment
    if analysis.is_successful() {
        println!("\n🎉 SUCCESS: Tailwind-RS is generating colors correctly!");
        println!("✅ Comprehensive gradient support");
        println!("✅ Rich color palette");
        println!("✅ Large CSS output with many utilities");
    } else {
        println!("\n❌ FAILURE: Color generation issues detected");
        if analysis.gradient_classes == 0 {
            println!("💡 Gradients are completely missing - check gradient parser");
        }
        if analysis.color_properties == 0 {
            println!("💡 Colors are not being generated - check color parser");
        }
        if analysis.unique_colors.len() < 10 {
            println!("💡 Color palette is very limited - check color definitions");
        }
        if analysis.total_css_size < 10000 {
            println!("💡 CSS output is too small - check if generation is working");
        }
    }

    Ok(())
}
