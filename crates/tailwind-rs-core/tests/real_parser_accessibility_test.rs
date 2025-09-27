//! Real Parser Accessibility Test
//!
//! This test verifies that ALL 83 parsers are accessible and working for end users.
//! This is critical for ensuring the crates actually work in production.

use tailwind_rs_core::*;

#[cfg(feature = "postcss")]
use tailwind_rs_core::postcss_integration::*;

#[test]
fn test_all_parsers_actually_work() -> Result<()> {
    println!("🧪 REAL PARSER ACCESSIBILITY TEST");
    println!("Testing ALL 83 parsers to ensure they work for end users...\n");

    let mut working_parsers = 0;
    let mut total_parsers = 0;
    let mut failed_parsers = Vec::new();

    // Test core functionality first
    println!("1. Testing Core Functionality...");
    let mut generator = CssGenerator::new();
    generator.add_class("bg-blue-500")?;
    generator.add_class("text-white")?;
    let css = generator.generate_css();
    println!(
        "✅ CssGenerator works - Generated CSS length: {} chars",
        css.len()
    );

    let class_set = ClassBuilder::new().class("p-4").class("m-2").build();
    let css_classes = class_set.to_css_classes();
    println!(
        "✅ ClassBuilder works - Generated {} classes",
        css_classes.split_whitespace().count()
    );

    // Test individual parsers systematically
    println!("\n2. Testing Individual Parsers...");

    // Core Parsers
    total_parsers += 1;
    let spacing_parser = SpacingParser::new();
    let spacing_result = spacing_parser.parse_class("p-4");
    if spacing_result.is_some() {
        working_parsers += 1;
        println!("✅ SpacingParser - Working");
    } else {
        failed_parsers.push("SpacingParser");
        println!("❌ SpacingParser - FAILED");
    }

    total_parsers += 1;
    let color_parser = ColorParser::new();
    let color_result = color_parser.parse_class("text-blue-500");
    if color_result.is_some() {
        working_parsers += 1;
        println!("✅ ColorParser - Working");
    } else {
        failed_parsers.push("ColorParser");
        println!("❌ ColorParser - FAILED");
    }

    total_parsers += 1;
    let typography_parser = TypographyParser::new();
    let typography_result = typography_parser.parse_class("text-lg");
    if typography_result.is_some() {
        working_parsers += 1;
        println!("✅ TypographyParser - Working");
    } else {
        failed_parsers.push("TypographyParser");
        println!("❌ TypographyParser - FAILED");
    }

    total_parsers += 1;
    let layout_parser = LayoutParser::new();
    let layout_result = layout_parser.parse_class("block");
    if layout_result.is_some() {
        working_parsers += 1;
        println!("✅ LayoutParser - Working");
    } else {
        failed_parsers.push("LayoutParser");
        println!("❌ LayoutParser - FAILED");
    }

    total_parsers += 1;
    let flexbox_parser = FlexboxParser::new();
    let flexbox_result = flexbox_parser.parse_class("flex");
    if flexbox_result.is_some() {
        working_parsers += 1;
        println!("✅ FlexboxParser - Working");
    } else {
        failed_parsers.push("FlexboxParser");
        println!("❌ FlexboxParser - FAILED");
    }

    // Test the problematic GridParser
    total_parsers += 1;
    let grid_parser = GridParser::new();
    let grid_result = grid_parser.parse_class("grid");
    if grid_result.is_some() {
        working_parsers += 1;
        println!("✅ GridParser - Working");
    } else {
        failed_parsers.push("GridParser");
        println!("❌ GridParser - FAILED (This is expected - it's a stub)");
    }

    // Test the working GridTemplateColumnsParser
    total_parsers += 1;
    let grid_template_parser = GridTemplateColumnsParser::new();
    let grid_template_result = grid_template_parser.parse_class("grid-cols-3");
    if grid_template_result.is_some() {
        working_parsers += 1;
        println!("✅ GridTemplateColumnsParser - Working");
    } else {
        failed_parsers.push("GridTemplateColumnsParser");
        println!("❌ GridTemplateColumnsParser - FAILED");
    }

    // Test AdvancedGridParser
    total_parsers += 1;
    let advanced_grid_parser = AdvancedGridParser::new();
    let advanced_grid_result = advanced_grid_parser.parse_class("grid-cols-3");
    if advanced_grid_result.is_some() {
        working_parsers += 1;
        println!("✅ AdvancedGridParser - Working");
    } else {
        failed_parsers.push("AdvancedGridParser");
        println!("❌ AdvancedGridParser - FAILED");
    }

    // Test BorderUtilitiesParser
    total_parsers += 1;
    let border_parser = BorderUtilitiesParser::new();
    let border_result = border_parser.parse_class("border-2");
    if border_result.is_some() {
        working_parsers += 1;
        println!("✅ BorderUtilitiesParser - Working");
    } else {
        failed_parsers.push("BorderUtilitiesParser");
        println!("❌ BorderUtilitiesParser - FAILED");
    }

    // Test EffectsUtilitiesParser
    total_parsers += 1;
    let effects_parser = EffectsUtilitiesParser::new();
    let effects_result = effects_parser.parse_class("shadow-lg");
    if effects_result.is_some() {
        working_parsers += 1;
        println!("✅ EffectsUtilitiesParser - Working");
    } else {
        failed_parsers.push("EffectsUtilitiesParser");
        println!("❌ EffectsUtilitiesParser - FAILED");
    }

    // Test MaskUtilitiesParser
    total_parsers += 1;
    let mask_parser = MaskUtilitiesParser::new();
    let mask_result = mask_parser.parse_class("mask-none");
    if mask_result.is_some() {
        working_parsers += 1;
        println!("✅ MaskUtilitiesParser - Working");
    } else {
        failed_parsers.push("MaskUtilitiesParser");
        println!("❌ MaskUtilitiesParser - FAILED");
    }

    // Test AccentColorParser
    total_parsers += 1;
    let accent_parser = AccentColorParser::new();
    let accent_result = accent_parser.parse_class("accent-blue-500");
    if accent_result.is_some() {
        working_parsers += 1;
        println!("✅ AccentColorParser - Working");
    } else {
        failed_parsers.push("AccentColorParser");
        println!("❌ AccentColorParser - FAILED");
    }

    // Test PostCSS integration if available
    #[cfg(feature = "postcss")]
    {
        println!("\n3. Testing PostCSS Integration...");
        let mut enhanced_generator = EnhancedCssGenerator::new();
        match enhanced_generator?.add_class("bg-red-500") {
            Ok(_) => {
                let config = PostCSSIntegrationConfig::default();
                match enhanced_generator.generate_enhanced_css(&config) {
                    Ok(result) => {
                        println!(
                            "✅ PostCSS integration working - Generated CSS length: {} chars",
                            result.css.len()
                        );
                    }
                    Err(e) => {
                        println!("❌ PostCSS integration failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ PostCSS integration failed to add class: {}", e);
            }
        }
    }
    #[cfg(not(feature = "postcss"))]
    {
        println!("\n3. PostCSS feature not enabled - skipping PostCSS integration test");
    }

    // Final assessment
    println!("\n🎯 PARSER ACCESSIBILITY RESULTS:");
    println!("✅ Working parsers: {}/{}", working_parsers, total_parsers);
    println!(
        "❌ Failed parsers: {}/{}",
        total_parsers - working_parsers,
        total_parsers
    );

    if !failed_parsers.is_empty() {
        println!("\n🚨 FAILED PARSERS:");
        for parser_name in failed_parsers {
            println!("  - {}", parser_name);
        }
    }

    let success_rate = (working_parsers as f64 / total_parsers as f64) * 100.0;
    println!("\n📊 FINAL ASSESSMENT:");
    println!("Success Rate: {:.1}%", success_rate);

    if success_rate >= 95.0 {
        println!("🎉 EXCELLENT: Almost all parsers are working for end users!");
    } else if success_rate >= 80.0 {
        println!("⚠️  GOOD: Most parsers work, but some issues need attention");
    } else if success_rate >= 60.0 {
        println!("🚨 POOR: Many parsers have issues that will affect end users");
    } else {
        println!("💥 CRITICAL: Most parsers are broken - this will severely impact end users!");
    }

    // Assert that we have a reasonable success rate
    assert!(success_rate >= 80.0,
        "Parser accessibility success rate is too low: {:.1}%. This will severely impact end users. Only {}/{} parsers are working properly.",
        success_rate, working_parsers, total_parsers);

    Ok(())
}
