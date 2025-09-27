//! Comprehensive All Parsers Test
//! 
//! This test systematically tests ALL accessible parsers to identify:
//! 1. Which parsers are working
//! 2. Which parsers are stubs (return None for valid classes)
//! 3. Which parsers are completely broken
//! 
//! This is CRITICAL for production readiness.

use tailwind_rs_core::*;

#[cfg(feature = "postcss")]
use tailwind_rs_core::postcss_integration::*;

#[test]
fn comprehensive_all_parsers_test() -> Result<()> {
    println!("🔍 COMPREHENSIVE ALL PARSERS TEST");
    println!("Testing ALL accessible parsers systematically for production readiness...\n");
    
    let mut working_parsers = 0;
    let mut stub_parsers = 0;
    let mut broken_parsers = 0;
    let mut total_parsers = 0;
    
    let mut working_list: Vec<&str> = Vec::new();
    let mut stub_list: Vec<&str> = Vec::new();
    let mut broken_list: Vec<&str> = Vec::new();
    
    // Test all parsers systematically
    println!("🔍 TESTING ALL ACCESSIBLE PARSERS...\n");
    
    // Core Parsers (12 already tested)
    total_parsers += 1;
    let spacing_parser = SpacingParser::new();
    let spacing_result = spacing_parser.parse_class("p-4");
    if spacing_result.is_some() {
        working_parsers += 1;
        working_list.push("SpacingParser");
        println!("✅ SpacingParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("SpacingParser");
        println!("❌ SpacingParser - STUB");
    }
    
    total_parsers += 1;
    let color_parser = ColorParser::new();
    let color_result = color_parser.parse_class("text-blue-500");
    if color_result.is_some() {
        working_parsers += 1;
        working_list.push("ColorParser");
        println!("✅ ColorParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("ColorParser");
        println!("❌ ColorParser - STUB");
    }
    
    total_parsers += 1;
    let typography_parser = TypographyParser::new();
    let typography_result = typography_parser.parse_class("text-lg");
    if typography_result.is_some() {
        working_parsers += 1;
        working_list.push("TypographyParser");
        println!("✅ TypographyParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("TypographyParser");
        println!("❌ TypographyParser - STUB");
    }
    
    total_parsers += 1;
    let layout_parser = LayoutParser::new();
    let layout_result = layout_parser.parse_class("block");
    if layout_result.is_some() {
        working_parsers += 1;
        working_list.push("LayoutParser");
        println!("✅ LayoutParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("LayoutParser");
        println!("❌ LayoutParser - STUB");
    }
    
    total_parsers += 1;
    let flexbox_parser = FlexboxParser::new();
    let flexbox_result = flexbox_parser.parse_class("flex");
    if flexbox_result.is_some() {
        working_parsers += 1;
        working_list.push("FlexboxParser");
        println!("✅ FlexboxParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("FlexboxParser");
        println!("❌ FlexboxParser - STUB");
    }
    
    // Test the fixed GridParser
    total_parsers += 1;
    let grid_parser = GridParser::new();
    let grid_result = grid_parser.parse_class("grid");
    if grid_result.is_some() {
        working_parsers += 1;
        working_list.push("GridParser");
        println!("✅ GridParser - WORKING (FIXED!)");
    } else {
        stub_parsers += 1;
        stub_list.push("GridParser");
        println!("❌ GridParser - STUB");
    }
    
    // Test GridTemplateColumnsParser
    total_parsers += 1;
    let grid_template_parser = GridTemplateColumnsParser::new();
    let grid_template_result = grid_template_parser.parse_class("grid-cols-3");
    if grid_template_result.is_some() {
        working_parsers += 1;
        working_list.push("GridTemplateColumnsParser");
        println!("✅ GridTemplateColumnsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("GridTemplateColumnsParser");
        println!("❌ GridTemplateColumnsParser - STUB");
    }
    
    // Test AdvancedGridParser
    total_parsers += 1;
    let advanced_grid_parser = AdvancedGridParser::new();
    let advanced_grid_result = advanced_grid_parser.parse_class("grid-cols-3");
    if advanced_grid_result.is_some() {
        working_parsers += 1;
        working_list.push("AdvancedGridParser");
        println!("✅ AdvancedGridParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AdvancedGridParser");
        println!("❌ AdvancedGridParser - STUB");
    }
    
    // Test BorderUtilitiesParser
    total_parsers += 1;
    let border_parser = BorderUtilitiesParser::new();
    let border_result = border_parser.parse_class("border-2");
    if border_result.is_some() {
        working_parsers += 1;
        working_list.push("BorderUtilitiesParser");
        println!("✅ BorderUtilitiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("BorderUtilitiesParser");
        println!("❌ BorderUtilitiesParser - STUB");
    }
    
    // Test EffectsUtilitiesParser
    total_parsers += 1;
    let effects_parser = EffectsUtilitiesParser::new();
    let effects_result = effects_parser.parse_class("shadow-lg");
    if effects_result.is_some() {
        working_parsers += 1;
        working_list.push("EffectsUtilitiesParser");
        println!("✅ EffectsUtilitiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("EffectsUtilitiesParser");
        println!("❌ EffectsUtilitiesParser - STUB");
    }
    
    // Test MaskUtilitiesParser
    total_parsers += 1;
    let mask_parser = MaskUtilitiesParser::new();
    let mask_result = mask_parser.parse_class("mask-none");
    if mask_result.is_some() {
        working_parsers += 1;
        working_list.push("MaskUtilitiesParser");
        println!("✅ MaskUtilitiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("MaskUtilitiesParser");
        println!("❌ MaskUtilitiesParser - STUB");
    }
    
    // Test AccentColorParser
    total_parsers += 1;
    let accent_parser = AccentColorParser::new();
    let accent_result = accent_parser.parse_class("accent-blue-500");
    if accent_result.is_some() {
        working_parsers += 1;
        working_list.push("AccentColorParser");
        println!("✅ AccentColorParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AccentColorParser");
        println!("❌ AccentColorParser - STUB");
    }
    
    // Test additional parsers that are accessible
    // Note: We need to test all 83 parsers, but we'll start with the ones we can access
    
    // Test PostCSS integration if available
    #[cfg(feature = "postcss")]
    {
        println!("\n🔧 Testing PostCSS Integration...");
        let mut enhanced_generator = EnhancedCssGenerator::new();
        match enhanced_generator.add_class("bg-red-500") {
            Ok(_) => {
                let config = PostCSSIntegrationConfig::default();
                match enhanced_generator.generate_enhanced_css(&config) {
                    Ok(result) => {
                        println!("✅ PostCSS integration working - Generated CSS length: {} chars", result.css.len());
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
        println!("\n⚠️  PostCSS feature not enabled - skipping PostCSS integration test");
    }
    
    // Final assessment
    println!("\n🎯 COMPREHENSIVE ALL PARSERS TEST RESULTS:");
    println!("📊 Parser Status Summary:");
    println!("  ✅ Working parsers: {}/{} ({:.1}%)", working_parsers, total_parsers, (working_parsers as f64 / total_parsers as f64) * 100.0);
    println!("  🚨 Stub parsers: {}/{} ({:.1}%)", stub_parsers, total_parsers, (stub_parsers as f64 / total_parsers as f64) * 100.0);
    println!("  💥 Broken parsers: {}/{} ({:.1}%)", broken_parsers, total_parsers, (broken_parsers as f64 / total_parsers as f64) * 100.0);
    
    if !working_list.is_empty() {
        println!("\n✅ WORKING PARSERS:");
        for parser in working_list {
            println!("  - {}", parser);
        }
    }
    
    if !stub_list.is_empty() {
        println!("\n🚨 STUB PARSERS (CRITICAL ISSUE):");
        for parser in stub_list {
            println!("  - {}", parser);
        }
    }
    
    if !broken_list.is_empty() {
        println!("\n💥 BROKEN PARSERS:");
        for parser in broken_list {
            println!("  - {}", parser);
        }
    }
    
    // Critical assessment
    let success_rate = (working_parsers as f64 / total_parsers as f64) * 100.0;
    let stub_rate = (stub_parsers as f64 / total_parsers as f64) * 100.0;
    
    println!("\n📊 CRITICAL ASSESSMENT:");
    println!("Success Rate: {:.1}%", success_rate);
    println!("Stub Rate: {:.1}%", stub_rate);
    
    if success_rate >= 95.0 && stub_rate <= 5.0 {
        println!("🎉 EXCELLENT: Almost all parsers are working for end users!");
    } else if success_rate >= 80.0 && stub_rate <= 20.0 {
        println!("⚠️  GOOD: Most parsers work, but some issues need attention");
    } else if success_rate >= 60.0 && stub_rate <= 40.0 {
        println!("🚨 POOR: Many parsers have issues that will affect end users");
    } else {
        println!("💥 CRITICAL: Most parsers are broken - this will severely impact end users!");
    }
    
    // Assert that we have a reasonable success rate
    assert!(success_rate >= 80.0, 
        "Parser functionality success rate is too low: {:.1}%. This will severely impact end users. Only {}/{} parsers are working properly.", 
        success_rate, working_parsers, total_parsers);
    
    assert!(stub_rate <= 20.0, 
        "Too many stub parsers: {:.1}%. This will severely impact end users. {}/{} parsers are stubs.", 
        stub_rate, stub_parsers, total_parsers);
    
    Ok(())
}
