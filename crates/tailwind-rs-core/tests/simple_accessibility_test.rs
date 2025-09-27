//! Simple test to verify core functionality and PostCSS integration works
//! This test ensures that end users can actually access the main functionality

use tailwind_rs_core::*;

#[cfg(feature = "postcss")]
use tailwind_rs_core::postcss_integration::*;

#[test]
fn test_core_functionality() -> Result<()> {
    println!("🧪 Testing Core Functionality and PostCSS Integration");

    // Test 1: Verify CssGenerator is accessible and works
    println!("\n1. Testing CssGenerator accessibility...");
    let mut generator = CssGenerator::new();

    // Test basic classes
    generator.add_class("bg-blue-500")?;
    generator.add_class("text-white")?;
    generator.add_class("px-4")?;
    generator.add_class("py-2")?;
    generator.add_class("rounded-lg")?;
    generator.add_class("hover:bg-blue-600")?;

    let css = generator.generate_css();
    println!(
        "✅ CssGenerator works - Generated CSS length: {} chars",
        css.len()
    );
    assert!(!css.is_empty(), "CssGenerator should generate CSS");

    // Test 2: Verify ClassBuilder is accessible and works
    println!("\n2. Testing ClassBuilder accessibility...");
    let class_builder = ClassBuilder::new();
    let class_set = class_builder
        .class("bg-red-500")
        .class("text-white")
        .class("p-4")
        .class("rounded")
        .build();

    let css_classes = class_set.to_css_classes();
    println!(
        "✅ ClassBuilder works - Generated {} classes",
        css_classes.len()
    );
    assert!(
        !css_classes.is_empty(),
        "ClassBuilder should generate classes"
    );

    // Test 3: Verify key parsers are accessible
    println!("\n3. Testing key parser accessibility...");

    // Test AccentColorParser (our new 83rd parser)
    let accent_parser = AccentColorParser::new();
    let accent_result = accent_parser.parse_class("accent-blue-500");
    println!(
        "✅ AccentColorParser accessible: {:?}",
        accent_result.is_some()
    );
    assert!(
        accent_result.is_some(),
        "AccentColorParser should parse accent-blue-500"
    );

    // Test other key parsers
    let color_parser = ColorParser::new();
    let color_result = color_parser.parse_class("text-red-500");
    println!("✅ ColorParser accessible: {:?}", color_result.is_some());
    assert!(
        color_result.is_some(),
        "ColorParser should parse text-red-500"
    );

    let typography_parser = TypographyParser::new();
    let typography_result = typography_parser.parse_class("text-lg");
    println!(
        "✅ TypographyParser accessible: {:?}",
        typography_result.is_some()
    );
    assert!(
        typography_result.is_some(),
        "TypographyParser should parse text-lg"
    );

    let layout_parser = LayoutParser::new();
    let layout_result = layout_parser.parse_class("flex");
    println!("✅ LayoutParser accessible: {:?}", layout_result.is_some());
    assert!(layout_result.is_some(), "LayoutParser should parse flex");

    let flexbox_parser = FlexboxParser::new();
    let flexbox_result = flexbox_parser.parse_class("justify-center");
    println!(
        "✅ FlexboxParser accessible: {:?}",
        flexbox_result.is_some()
    );
    assert!(
        flexbox_result.is_some(),
        "FlexboxParser should parse justify-center"
    );

    let grid_parser = GridTemplateColumnsParser::new();
    let grid_result = grid_parser.parse_class("grid-cols-3");
    println!(
        "✅ GridTemplateColumnsParser accessible: {:?}",
        grid_result.is_some()
    );
    assert!(
        grid_result.is_some(),
        "GridTemplateColumnsParser should parse grid-cols-3"
    );

    let border_parser = BorderUtilitiesParser::new();
    let border_result = border_parser.parse_class("border-2");
    println!(
        "✅ BorderUtilitiesParser accessible: {:?}",
        border_result.is_some()
    );
    assert!(
        border_result.is_some(),
        "BorderUtilitiesParser should parse border-2"
    );

    let effects_parser = EffectsUtilitiesParser::new();
    let effects_result = effects_parser.parse_class("shadow-lg");
    println!(
        "✅ EffectsUtilitiesParser accessible: {:?}",
        effects_result.is_some()
    );
    assert!(
        effects_result.is_some(),
        "EffectsUtilitiesParser should parse shadow-lg"
    );

    let mask_parser = MaskUtilitiesParser::new();
    let mask_result = mask_parser.parse_class("mask-none");
    println!(
        "✅ MaskUtilitiesParser accessible: {:?}",
        mask_result.is_some()
    );
    assert!(
        mask_result.is_some(),
        "MaskUtilitiesParser should parse mask-none"
    );

    // Test 4: Verify PostCSS integration (if feature enabled)
    #[cfg(feature = "postcss")]
    {
        println!("\n4. Testing PostCSS integration...");

        let postcss_config = PostCSSIntegrationConfig::default();
        let mut enhanced_generator = EnhancedCssGenerator::new(postcss_config);

        // Add some classes
        enhanced_generator.add_class("bg-gradient-to-r")?;
        enhanced_generator.add_class("from-blue-500")?;
        enhanced_generator.add_class("to-purple-600")?;
        enhanced_generator.add_class("transform")?;
        enhanced_generator.add_class("rotate-45")?;

        // Generate enhanced CSS
        let enhanced_result = enhanced_generator.generate_enhanced_css()?;
        println!(
            "✅ PostCSS integration works - Enhanced CSS length: {} chars",
            enhanced_result.css.len()
        );
        assert!(
            !enhanced_result.css.is_empty(),
            "PostCSS should generate enhanced CSS"
        );

        if let Some(metrics) = enhanced_result.metrics {
            println!("✅ PostCSS metrics available: {:?}", metrics);
        }
    }

    #[cfg(not(feature = "postcss"))]
    {
        println!("\n4. PostCSS feature not enabled - skipping PostCSS tests");
    }

    // Test 5: Verify parser count by checking a sample of parsers
    println!("\n5. Verifying parser accessibility...");

    // Test a sample of parsers individually
    let mut accessible_count = 0;
    let total_parsers = 10;

    // Test SpacingParser
    let spacing_parser = SpacingParser::new();
    let patterns = spacing_parser.get_supported_patterns();
    let priority = spacing_parser.get_priority();
    let category = spacing_parser.get_category();
    if !patterns.is_empty() && priority > 0 {
        accessible_count += 1;
        println!(
            "  ✅ SpacingParser - {} patterns, priority: {}, category: {:?}",
            patterns.len(),
            priority,
            category
        );
    }

    // Test ColorParser
    let color_parser = ColorParser::new();
    let patterns = color_parser.get_supported_patterns();
    let priority = color_parser.get_priority();
    let category = color_parser.get_category();
    if !patterns.is_empty() && priority > 0 {
        accessible_count += 1;
        println!(
            "  ✅ ColorParser - {} patterns, priority: {}, category: {:?}",
            patterns.len(),
            priority,
            category
        );
    }

    // Test TypographyParser
    let typography_parser = TypographyParser::new();
    let patterns = typography_parser.get_supported_patterns();
    let priority = typography_parser.get_priority();
    let category = typography_parser.get_category();
    if !patterns.is_empty() && priority > 0 {
        accessible_count += 1;
        println!(
            "  ✅ TypographyParser - {} patterns, priority: {}, category: {:?}",
            patterns.len(),
            priority,
            category
        );
    }

    // Test LayoutParser
    let layout_parser = LayoutParser::new();
    let patterns = layout_parser.get_supported_patterns();
    let priority = layout_parser.get_priority();
    let category = layout_parser.get_category();
    if !patterns.is_empty() && priority > 0 {
        accessible_count += 1;
        println!(
            "  ✅ LayoutParser - {} patterns, priority: {}, category: {:?}",
            patterns.len(),
            priority,
            category
        );
    }

    // Test FlexboxParser
    let flexbox_parser = FlexboxParser::new();
    let patterns = flexbox_parser.get_supported_patterns();
    let priority = flexbox_parser.get_priority();
    let category = flexbox_parser.get_category();
    if !patterns.is_empty() && priority > 0 {
        accessible_count += 1;
        println!(
            "  ✅ FlexboxParser - {} patterns, priority: {}, category: {:?}",
            patterns.len(),
            priority,
            category
        );
    }

    // Test GridParser
    let grid_parser = GridParser::new();
    let patterns = grid_parser.get_supported_patterns();
    let priority = grid_parser.get_priority();
    let category = grid_parser.get_category();
    if !patterns.is_empty() && priority > 0 {
        accessible_count += 1;
        println!(
            "  ✅ GridParser - {} patterns, priority: {}, category: {:?}",
            patterns.len(),
            priority,
            category
        );
    }

    // Test BorderUtilitiesParser
    let border_parser = BorderUtilitiesParser::new();
    let patterns = border_parser.get_supported_patterns();
    let priority = border_parser.get_priority();
    let category = border_parser.get_category();
    if !patterns.is_empty() && priority > 0 {
        accessible_count += 1;
        println!(
            "  ✅ BorderUtilitiesParser - {} patterns, priority: {}, category: {:?}",
            patterns.len(),
            priority,
            category
        );
    }

    // Test EffectsUtilitiesParser
    let effects_parser = EffectsUtilitiesParser::new();
    let patterns = effects_parser.get_supported_patterns();
    let priority = effects_parser.get_priority();
    let category = effects_parser.get_category();
    if !patterns.is_empty() && priority > 0 {
        accessible_count += 1;
        println!(
            "  ✅ EffectsUtilitiesParser - {} patterns, priority: {}, category: {:?}",
            patterns.len(),
            priority,
            category
        );
    }

    // Test MaskUtilitiesParser
    let mask_parser = MaskUtilitiesParser::new();
    let patterns = mask_parser.get_supported_patterns();
    let priority = mask_parser.get_priority();
    let category = mask_parser.get_category();
    if !patterns.is_empty() && priority > 0 {
        accessible_count += 1;
        println!(
            "  ✅ MaskUtilitiesParser - {} patterns, priority: {}, category: {:?}",
            patterns.len(),
            priority,
            category
        );
    }

    // Test AccentColorParser
    let accent_parser = AccentColorParser::new();
    let patterns = accent_parser.get_supported_patterns();
    let priority = accent_parser.get_priority();
    let category = accent_parser.get_category();
    if !patterns.is_empty() && priority > 0 {
        accessible_count += 1;
        println!(
            "  ✅ AccentColorParser - {} patterns, priority: {}, category: {:?}",
            patterns.len(),
            priority,
            category
        );
    }

    println!("\n🎯 PARSER ACCESSIBILITY RESULTS:");
    println!(
        "✅ Accessible parsers: {}/{}",
        accessible_count, total_parsers
    );

    assert!(
        accessible_count >= 8,
        "At least 8 sample parsers should be accessible"
    );

    if accessible_count >= 10 {
        println!("🎉 SUCCESS: All sample parsers are accessible to end users!");
    } else {
        println!("⚠️  WARNING: Some parsers may not be accessible");
    }

    println!("\n🎉 CORE FUNCTIONALITY VERIFICATION COMPLETE!");
    println!("✅ CssGenerator: Working");
    println!("✅ ClassBuilder: Working");
    println!("✅ Key Parsers: Accessible");
    println!(
        "✅ PostCSS Integration: {}",
        if cfg!(feature = "postcss") {
            "Working"
        } else {
            "Not enabled"
        }
    );

    Ok(())
}
