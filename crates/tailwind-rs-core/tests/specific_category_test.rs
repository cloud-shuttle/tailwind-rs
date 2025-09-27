//! Specific Category Parser Test
//! 
//! This test focuses on specific parser categories to systematically test
//! the remaining 43 parsers by category (color, spacing, typography, etc.)

use tailwind_rs_core::*;

#[cfg(feature = "postcss")]
use tailwind_rs_core::postcss_integration::*;

#[test]
fn specific_category_test() -> Result<()> {
    println!("🔍 SPECIFIC CATEGORY PARSER TEST");
    println!("Testing specific parser categories for remaining 43 parsers...\n");
    
    let mut working_parsers = 0;
    let mut stub_parsers = 0;
    let mut broken_parsers = 0;
    let mut total_parsers = 0;
    
    let mut working_list: Vec<&str> = Vec::new();
    let mut stub_list: Vec<&str> = Vec::new();
    let mut broken_list: Vec<&str> = Vec::new();
    
    // Test specific parser categories systematically
    println!("🔍 TESTING SPECIFIC PARSER CATEGORIES...\n");
    
    // ===== COLOR PARSERS =====
    println!("🎨 COLOR PARSERS:");
    
    total_parsers += 1;
    let color_parser = ColorParser::new();
    let color_result = color_parser.parse_class("text-blue-500");
    if color_result.is_some() {
        working_parsers += 1;
        working_list.push("ColorParser");
        println!("  ✅ ColorParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("ColorParser");
        println!("  ❌ ColorParser - STUB");
    }
    
    total_parsers += 1;
    let advanced_color_parser = AdvancedColorParser::new();
    let advanced_color_result = advanced_color_parser.parse_class("bg-green-500");
    if advanced_color_result.is_some() {
        working_parsers += 1;
        working_list.push("AdvancedColorParser");
        println!("  ✅ AdvancedColorParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AdvancedColorParser");
        println!("  ❌ AdvancedColorParser - STUB");
    }
    
    // ===== SPACING PARSERS =====
    println!("\n📏 SPACING PARSERS:");
    
    total_parsers += 1;
    let spacing_parser = SpacingParser::new();
    let spacing_result = spacing_parser.parse_class("p-4");
    if spacing_result.is_some() {
        working_parsers += 1;
        working_list.push("SpacingParser");
        println!("  ✅ SpacingParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("SpacingParser");
        println!("  ❌ SpacingParser - STUB");
    }
    
    total_parsers += 1;
    let advanced_spacing_parser = AdvancedSpacingParser::new();
    let advanced_spacing_result = advanced_spacing_parser.parse_class("space-y-4");
    if advanced_spacing_result.is_some() {
        working_parsers += 1;
        working_list.push("AdvancedSpacingParser");
        println!("  ✅ AdvancedSpacingParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AdvancedSpacingParser");
        println!("  ❌ AdvancedSpacingParser - STUB");
    }
    
    // ===== TYPOGRAPHY PARSERS =====
    println!("\n📝 TYPOGRAPHY PARSERS:");
    
    total_parsers += 1;
    let typography_parser = TypographyParser::new();
    let typography_result = typography_parser.parse_class("text-lg");
    if typography_result.is_some() {
        working_parsers += 1;
        working_list.push("TypographyParser");
        println!("  ✅ TypographyParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("TypographyParser");
        println!("  ❌ TypographyParser - STUB");
    }
    
    // ===== LAYOUT PARSERS =====
    println!("\n🏗️ LAYOUT PARSERS:");
    
    total_parsers += 1;
    let layout_parser = LayoutParser::new();
    let layout_result = layout_parser.parse_class("block");
    if layout_result.is_some() {
        working_parsers += 1;
        working_list.push("LayoutParser");
        println!("  ✅ LayoutParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("LayoutParser");
        println!("  ❌ LayoutParser - STUB");
    }
    
    total_parsers += 1;
    let positioning_parser = PositioningParser::new();
    let positioning_result = positioning_parser.parse_class("relative");
    if positioning_result.is_some() {
        working_parsers += 1;
        working_list.push("PositioningParser");
        println!("  ✅ PositioningParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("PositioningParser");
        println!("  ❌ PositioningParser - STUB");
    }
    
    total_parsers += 1;
    let sizing_parser = SizingParser::new();
    let sizing_result = sizing_parser.parse_class("w-full");
    if sizing_result.is_some() {
        working_parsers += 1;
        working_list.push("SizingParser");
        println!("  ✅ SizingParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("SizingParser");
        println!("  ❌ SizingParser - STUB");
    }
    
    // ===== TRANSFORM PARSERS =====
    println!("\n🔄 TRANSFORM PARSERS:");
    
    total_parsers += 1;
    let transform_parser = TransformParser::new();
    let transform_result = transform_parser.parse_class("rotate-45");
    if transform_result.is_some() {
        working_parsers += 1;
        working_list.push("TransformParser");
        println!("  ✅ TransformParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("TransformParser");
        println!("  ❌ TransformParser - STUB");
    }
    
    total_parsers += 1;
    let fractional_transforms_parser = FractionalTransformsParser::new();
    let fractional_transforms_result = fractional_transforms_parser.parse_class("translate-x-1/2");
    if fractional_transforms_result.is_some() {
        working_parsers += 1;
        working_list.push("FractionalTransformsParser");
        println!("  ✅ FractionalTransformsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("FractionalTransformsParser");
        println!("  ❌ FractionalTransformsParser - STUB");
    }
    
    // ===== ANIMATION PARSERS =====
    println!("\n🎬 ANIMATION PARSERS:");
    
    total_parsers += 1;
    let animation_parser = AnimationParser::new();
    let animation_result = animation_parser.parse_class("animate-bounce");
    if animation_result.is_some() {
        working_parsers += 1;
        working_list.push("AnimationParser");
        println!("  ✅ AnimationParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AnimationParser");
        println!("  ❌ AnimationParser - STUB");
    }
    
    total_parsers += 1;
    let transition_parser = TransitionParser::new();
    let transition_result = transition_parser.parse_class("transition-all");
    if transition_result.is_some() {
        working_parsers += 1;
        working_list.push("TransitionParser");
        println!("  ✅ TransitionParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("TransitionParser");
        println!("  ❌ TransitionParser - STUB");
    }
    
    total_parsers += 1;
    let transition_properties_parser = TransitionPropertiesParser::new();
    let transition_properties_result = transition_properties_parser.parse_class("ease-linear");
    if transition_properties_result.is_some() {
        working_parsers += 1;
        working_list.push("TransitionPropertiesParser");
        println!("  ✅ TransitionPropertiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("TransitionPropertiesParser");
        println!("  ❌ TransitionPropertiesParser - STUB");
    }
    
    // ===== SVG PARSERS =====
    println!("\n🎨 SVG PARSERS:");
    
    total_parsers += 1;
    let svg_parser = SvgParser::new();
    let svg_result = svg_parser.parse_class("fill-current");
    if svg_result.is_some() {
        working_parsers += 1;
        working_list.push("SvgParser");
        println!("  ✅ SvgParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("SvgParser");
        println!("  ❌ SvgParser - STUB");
    }
    
    // ===== PROSE PARSERS =====
    println!("\n📖 PROSE PARSERS:");
    
    total_parsers += 1;
    let prose_parser = ProseParser::new();
    let prose_result = prose_parser.parse_class("prose");
    if prose_result.is_some() {
        working_parsers += 1;
        working_list.push("ProseParser");
        println!("  ✅ ProseParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("ProseParser");
        println!("  ❌ ProseParser - STUB");
    }
    
    // ===== DIVIDE PARSERS =====
    println!("\n➗ DIVIDE PARSERS:");
    
    total_parsers += 1;
    let divide_parser = DivideParser::new();
    let divide_result = divide_parser.parse_class("divide-y");
    if divide_result.is_some() {
        working_parsers += 1;
        working_list.push("DivideParser");
        println!("  ✅ DivideParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("DivideParser");
        println!("  ❌ DivideParser - STUB");
    }
    
    // ===== GRADIENT PARSERS =====
    println!("\n🌈 GRADIENT PARSERS:");
    
    total_parsers += 1;
    let gradient_parser = GradientParser::new();
    let gradient_result = gradient_parser.parse_class("bg-gradient-to-r");
    if gradient_result.is_some() {
        working_parsers += 1;
        working_list.push("GradientParser");
        println!("  ✅ GradientParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("GradientParser");
        println!("  ❌ GradientParser - STUB");
    }
    
    // ===== OBJECT FIT PARSERS =====
    println!("\n🖼️ OBJECT FIT PARSERS:");
    
    total_parsers += 1;
    let object_fit_parser = ObjectFitParser::new();
    let object_fit_result = object_fit_parser.parse_class("object-cover");
    if object_fit_result.is_some() {
        working_parsers += 1;
        working_list.push("ObjectFitParser");
        println!("  ✅ ObjectFitParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("ObjectFitParser");
        println!("  ❌ ObjectFitParser - STUB");
    }
    
    // ===== ARBITRARY PARSERS =====
    println!("\n🔧 ARBITRARY PARSERS:");
    
    total_parsers += 1;
    let arbitrary_parser = ArbitraryParser::new();
    let arbitrary_result = arbitrary_parser.parse_class("w-[100px]");
    if arbitrary_result.is_some() {
        working_parsers += 1;
        working_list.push("ArbitraryParser");
        println!("  ✅ ArbitraryParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("ArbitraryParser");
        println!("  ❌ ArbitraryParser - STUB");
    }
    
    total_parsers += 1;
    let data_attribute_parser = DataAttributeParser::new();
    let data_attribute_result = data_attribute_parser.parse_class("data-hover:bg-black/5");
    if data_attribute_result.is_some() {
        working_parsers += 1;
        working_list.push("DataAttributeParser");
        println!("  ✅ DataAttributeParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("DataAttributeParser");
        println!("  ❌ DataAttributeParser - STUB");
    }
    
    // ===== BACKGROUND PARSERS =====
    println!("\n🎨 BACKGROUND PARSERS:");
    
    total_parsers += 1;
    let background_parser = BackgroundParser::new();
    let background_result = background_parser.parse_class("bg-red-500");
    if background_result.is_some() {
        working_parsers += 1;
        working_list.push("BackgroundParser");
        println!("  ✅ BackgroundParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("BackgroundParser");
        println!("  ❌ BackgroundParser - STUB");
    }
    
    total_parsers += 1;
    let background_properties_parser = BackgroundPropertiesParser::new();
    let background_properties_result = background_properties_parser.parse_class("bg-no-repeat");
    if background_properties_result.is_some() {
        working_parsers += 1;
        working_list.push("BackgroundPropertiesParser");
        println!("  ✅ BackgroundPropertiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("BackgroundPropertiesParser");
        println!("  ❌ BackgroundPropertiesParser - STUB");
    }
    
    // ===== ASPECT RATIO PARSERS =====
    println!("\n📐 ASPECT RATIO PARSERS:");
    
    total_parsers += 1;
    let aspect_ratio_parser = AspectRatioParser::new();
    let aspect_ratio_result = aspect_ratio_parser.parse_class("aspect-square");
    if aspect_ratio_result.is_some() {
        working_parsers += 1;
        working_list.push("AspectRatioParser");
        println!("  ✅ AspectRatioParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AspectRatioParser");
        println!("  ❌ AspectRatioParser - STUB");
    }
    
    // ===== COLUMNS PARSERS =====
    println!("\n📰 COLUMNS PARSERS:");
    
    total_parsers += 1;
    let columns_parser = ColumnsParser::new();
    let columns_result = columns_parser.parse_class("columns-2");
    if columns_result.is_some() {
        working_parsers += 1;
        working_list.push("ColumnsParser");
        println!("  ✅ ColumnsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("ColumnsParser");
        println!("  ❌ ColumnsParser - STUB");
    }
    
    // ===== BREAK CONTROL PARSERS =====
    println!("\n⏸️ BREAK CONTROL PARSERS:");
    
    total_parsers += 1;
    let break_control_parser = BreakControlParser::new();
    let break_control_result = break_control_parser.parse_class("break-after-auto");
    if break_control_result.is_some() {
        working_parsers += 1;
        working_list.push("BreakControlParser");
        println!("  ✅ BreakControlParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("BreakControlParser");
        println!("  ❌ BreakControlParser - STUB");
    }
    
    // ===== BOX UTILITIES PARSERS =====
    println!("\n📦 BOX UTILITIES PARSERS:");
    
    total_parsers += 1;
    let box_utilities_parser = BoxUtilitiesParser::new();
    let box_utilities_result = box_utilities_parser.parse_class("box-border");
    if box_utilities_result.is_some() {
        working_parsers += 1;
        working_list.push("BoxUtilitiesParser");
        println!("  ✅ BoxUtilitiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("BoxUtilitiesParser");
        println!("  ❌ BoxUtilitiesParser - STUB");
    }
    
    // ===== LAYOUT UTILITIES PARSERS =====
    println!("\n🏗️ LAYOUT UTILITIES PARSERS:");
    
    total_parsers += 1;
    let layout_utilities_parser = LayoutUtilitiesParser::new();
    let layout_utilities_result = layout_utilities_parser.parse_class("float-right");
    if layout_utilities_result.is_some() {
        working_parsers += 1;
        working_list.push("LayoutUtilitiesParser");
        println!("  ✅ LayoutUtilitiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("LayoutUtilitiesParser");
        println!("  ❌ LayoutUtilitiesParser - STUB");
    }
    
    // ===== OVERFLOW PARSERS =====
    println!("\n🌊 OVERFLOW PARSERS:");
    
    total_parsers += 1;
    let overflow_parser = OverflowParser::new();
    let overflow_result = overflow_parser.parse_class("overflow-hidden");
    if overflow_result.is_some() {
        working_parsers += 1;
        working_list.push("OverflowParser");
        println!("  ✅ OverflowParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("OverflowParser");
        println!("  ❌ OverflowParser - STUB");
    }
    
    total_parsers += 1;
    let overscroll_parser = OverscrollParser::new();
    let overscroll_result = overscroll_parser.parse_class("overscroll-auto");
    if overscroll_result.is_some() {
        working_parsers += 1;
        working_list.push("OverscrollParser");
        println!("  ✅ OverscrollParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("OverscrollParser");
        println!("  ❌ OverscrollParser - STUB");
    }
    
    // ===== POSITION PARSERS =====
    println!("\n📍 POSITION PARSERS:");
    
    total_parsers += 1;
    let position_parser = PositionParser::new();
    let position_result = position_parser.parse_class("relative");
    if position_result.is_some() {
        working_parsers += 1;
        working_list.push("PositionParser");
        println!("  ✅ PositionParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("PositionParser");
        println!("  ❌ PositionParser - STUB");
    }
    
    total_parsers += 1;
    let inset_parser = InsetParser::new();
    let inset_result = inset_parser.parse_class("inset-0");
    if inset_result.is_some() {
        working_parsers += 1;
        working_list.push("InsetParser");
        println!("  ✅ InsetParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("InsetParser");
        println!("  ❌ InsetParser - STUB");
    }
    
    // ===== VISIBILITY PARSERS =====
    println!("\n👁️ VISIBILITY PARSERS:");
    
    total_parsers += 1;
    let visibility_parser = VisibilityParser::new();
    let visibility_result = visibility_parser.parse_class("visible");
    if visibility_result.is_some() {
        working_parsers += 1;
        working_list.push("VisibilityParser");
        println!("  ✅ VisibilityParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("VisibilityParser");
        println!("  ❌ VisibilityParser - STUB");
    }
    
    total_parsers += 1;
    let z_index_parser = ZIndexParser::new();
    let z_index_result = z_index_parser.parse_class("z-10");
    if z_index_result.is_some() {
        working_parsers += 1;
        working_list.push("ZIndexParser");
        println!("  ✅ ZIndexParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("ZIndexParser");
        println!("  ❌ ZIndexParser - STUB");
    }
    
    // ===== FILTER PARSERS =====
    println!("\n🔍 FILTER PARSERS:");
    
    total_parsers += 1;
    let filter_utilities_parser = FilterUtilitiesParser::new();
    let filter_utilities_result = filter_utilities_parser.parse_class("blur-sm");
    if filter_utilities_result.is_some() {
        working_parsers += 1;
        working_list.push("FilterUtilitiesParser");
        println!("  ✅ FilterUtilitiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("FilterUtilitiesParser");
        println!("  ❌ FilterUtilitiesParser - STUB");
    }
    
    total_parsers += 1;
    let backdrop_filter_utilities_parser = BackdropFilterUtilitiesParser::new();
    let backdrop_filter_utilities_result = backdrop_filter_utilities_parser.parse_class("backdrop-blur-sm");
    if backdrop_filter_utilities_result.is_some() {
        working_parsers += 1;
        working_list.push("BackdropFilterUtilitiesParser");
        println!("  ✅ BackdropFilterUtilitiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("BackdropFilterUtilitiesParser");
        println!("  ❌ BackdropFilterUtilitiesParser - STUB");
    }
    
    // ===== ACCESSIBILITY PARSERS =====
    println!("\n♿ ACCESSIBILITY PARSERS:");
    
    total_parsers += 1;
    let accessibility_parser = AccessibilityParser::new();
    let accessibility_result = accessibility_parser.parse_class("forced-color-adjust-auto");
    if accessibility_result.is_some() {
        working_parsers += 1;
        working_list.push("AccessibilityParser");
        println!("  ✅ AccessibilityParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AccessibilityParser");
        println!("  ❌ AccessibilityParser - STUB");
    }
    
    // ===== TABLE PARSERS =====
    println!("\n📊 TABLE PARSERS:");
    
    total_parsers += 1;
    let table_parser = TableParser::new();
    let table_result = table_parser.parse_class("table-auto");
    if table_result.is_some() {
        working_parsers += 1;
        working_list.push("TableParser");
        println!("  ✅ TableParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("TableParser");
        println!("  ❌ TableParser - STUB");
    }
    
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
    println!("\n🎯 SPECIFIC CATEGORY PARSER TEST RESULTS:");
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
