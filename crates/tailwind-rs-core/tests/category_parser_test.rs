//! Category Parser Test
//!
//! This test systematically tests ALL accessible parsers by category to identify:
//! 1. Which parsers are working
//! 2. Which parsers are stubs (return None for valid classes)
//! 3. Which parsers are completely broken
//!
//! This is CRITICAL for production readiness.

use tailwind_rs_core::*;

#[cfg(feature = "postcss")]
use tailwind_rs_core::postcss_integration::*;

#[test]
fn category_parser_test() -> Result<()> {
    println!("🔍 CATEGORY PARSER TEST");
    println!("Testing ALL accessible parsers by category for production readiness...\n");

    let mut working_parsers = 0;
    let mut stub_parsers = 0;
    let mut broken_parsers = 0;
    let mut total_parsers = 0;

    let mut working_list: Vec<&str> = Vec::new();
    let mut stub_list: Vec<&str> = Vec::new();
    let mut broken_list: Vec<&str> = Vec::new();

    // Test all parsers systematically by category
    println!("🔍 TESTING ALL ACCESSIBLE PARSERS BY CATEGORY...\n");

    // ===== CORE PARSERS =====
    println!("📦 CORE PARSERS:");

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

    // ===== FLEXBOX PARSERS =====
    println!("\n🔧 FLEXBOX PARSERS:");

    total_parsers += 1;
    let flexbox_parser = FlexboxParser::new();
    let flexbox_result = flexbox_parser.parse_class("flex");
    if flexbox_result.is_some() {
        working_parsers += 1;
        working_list.push("FlexboxParser");
        println!("  ✅ FlexboxParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("FlexboxParser");
        println!("  ❌ FlexboxParser - STUB");
    }

    // Test additional flexbox parsers
    total_parsers += 1;
    let flex_basis_parser = FlexBasisParser::new();
    let flex_basis_result = flex_basis_parser.parse_class("basis-1/2");
    if flex_basis_result.is_some() {
        working_parsers += 1;
        working_list.push("FlexBasisParser");
        println!("  ✅ FlexBasisParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("FlexBasisParser");
        println!("  ❌ FlexBasisParser - STUB");
    }

    total_parsers += 1;
    let flex_direction_parser = FlexDirectionParser::new();
    let flex_direction_result = flex_direction_parser.parse_class("flex-row");
    if flex_direction_result.is_some() {
        working_parsers += 1;
        working_list.push("FlexDirectionParser");
        println!("  ✅ FlexDirectionParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("FlexDirectionParser");
        println!("  ❌ FlexDirectionParser - STUB");
    }

    total_parsers += 1;
    let flex_wrap_parser = FlexWrapParser::new();
    let flex_wrap_result = flex_wrap_parser.parse_class("flex-wrap");
    if flex_wrap_result.is_some() {
        working_parsers += 1;
        working_list.push("FlexWrapParser");
        println!("  ✅ FlexWrapParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("FlexWrapParser");
        println!("  ❌ FlexWrapParser - STUB");
    }

    total_parsers += 1;
    let flex_grow_parser = FlexGrowParser::new();
    let flex_grow_result = flex_grow_parser.parse_class("grow");
    if flex_grow_result.is_some() {
        working_parsers += 1;
        working_list.push("FlexGrowParser");
        println!("  ✅ FlexGrowParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("FlexGrowParser");
        println!("  ❌ FlexGrowParser - STUB");
    }

    total_parsers += 1;
    let flex_shrink_parser = FlexShrinkParser::new();
    let flex_shrink_result = flex_shrink_parser.parse_class("shrink");
    if flex_shrink_result.is_some() {
        working_parsers += 1;
        working_list.push("FlexShrinkParser");
        println!("  ✅ FlexShrinkParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("FlexShrinkParser");
        println!("  ❌ FlexShrinkParser - STUB");
    }

    total_parsers += 1;
    let order_parser = OrderParser::new();
    let order_result = order_parser.parse_class("order-1");
    if order_result.is_some() {
        working_parsers += 1;
        working_list.push("OrderParser");
        println!("  ✅ OrderParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("OrderParser");
        println!("  ❌ OrderParser - STUB");
    }

    // ===== GRID PARSERS =====
    println!("\n📐 GRID PARSERS:");

    total_parsers += 1;
    let grid_parser = GridParser::new();
    let grid_result = grid_parser.parse_class("grid");
    if grid_result.is_some() {
        working_parsers += 1;
        working_list.push("GridParser");
        println!("  ✅ GridParser - WORKING (FIXED!)");
    } else {
        stub_parsers += 1;
        stub_list.push("GridParser");
        println!("  ❌ GridParser - STUB");
    }

    total_parsers += 1;
    let grid_template_parser = GridTemplateColumnsParser::new();
    let grid_template_result = grid_template_parser.parse_class("grid-cols-3");
    if grid_template_result.is_some() {
        working_parsers += 1;
        working_list.push("GridTemplateColumnsParser");
        println!("  ✅ GridTemplateColumnsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("GridTemplateColumnsParser");
        println!("  ❌ GridTemplateColumnsParser - STUB");
    }

    total_parsers += 1;
    let advanced_grid_parser = AdvancedGridParser::new();
    let advanced_grid_result = advanced_grid_parser.parse_class("grid-cols-3");
    if advanced_grid_result.is_some() {
        working_parsers += 1;
        working_list.push("AdvancedGridParser");
        println!("  ✅ AdvancedGridParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AdvancedGridParser");
        println!("  ❌ AdvancedGridParser - STUB");
    }

    // Test additional grid parsers
    total_parsers += 1;
    let grid_column_parser = GridColumnParser::new();
    let grid_column_result = grid_column_parser.parse_class("col-span-2");
    if grid_column_result.is_some() {
        working_parsers += 1;
        working_list.push("GridColumnParser");
        println!("  ✅ GridColumnParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("GridColumnParser");
        println!("  ❌ GridColumnParser - STUB");
    }

    total_parsers += 1;
    let grid_template_rows_parser = GridTemplateRowsParser::new();
    let grid_template_rows_result = grid_template_rows_parser.parse_class("grid-rows-3");
    if grid_template_rows_result.is_some() {
        working_parsers += 1;
        working_list.push("GridTemplateRowsParser");
        println!("  ✅ GridTemplateRowsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("GridTemplateRowsParser");
        println!("  ❌ GridTemplateRowsParser - STUB");
    }

    total_parsers += 1;
    let grid_row_parser = GridRowParser::new();
    let grid_row_result = grid_row_parser.parse_class("row-span-2");
    if grid_row_result.is_some() {
        working_parsers += 1;
        working_list.push("GridRowParser");
        println!("  ✅ GridRowParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("GridRowParser");
        println!("  ❌ GridRowParser - STUB");
    }

    total_parsers += 1;
    let grid_auto_flow_parser = GridAutoFlowParser::new();
    let grid_auto_flow_result = grid_auto_flow_parser.parse_class("grid-flow-row");
    if grid_auto_flow_result.is_some() {
        working_parsers += 1;
        working_list.push("GridAutoFlowParser");
        println!("  ✅ GridAutoFlowParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("GridAutoFlowParser");
        println!("  ❌ GridAutoFlowParser - STUB");
    }

    total_parsers += 1;
    let grid_auto_columns_parser = GridAutoColumnsParser::new();
    let grid_auto_columns_result = grid_auto_columns_parser.parse_class("auto-cols-auto");
    if grid_auto_columns_result.is_some() {
        working_parsers += 1;
        working_list.push("GridAutoColumnsParser");
        println!("  ✅ GridAutoColumnsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("GridAutoColumnsParser");
        println!("  ❌ GridAutoColumnsParser - STUB");
    }

    total_parsers += 1;
    let grid_auto_rows_parser = GridAutoRowsParser::new();
    let grid_auto_rows_result = grid_auto_rows_parser.parse_class("auto-rows-auto");
    if grid_auto_rows_result.is_some() {
        working_parsers += 1;
        working_list.push("GridAutoRowsParser");
        println!("  ✅ GridAutoRowsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("GridAutoRowsParser");
        println!("  ❌ GridAutoRowsParser - STUB");
    }

    total_parsers += 1;
    let gap_parser = GapParser::new();
    let gap_result = gap_parser.parse_class("gap-4");
    if gap_result.is_some() {
        working_parsers += 1;
        working_list.push("GapParser");
        println!("  ✅ GapParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("GapParser");
        println!("  ❌ GapParser - STUB");
    }

    // ===== ALIGNMENT PARSERS =====
    println!("\n🎯 ALIGNMENT PARSERS:");

    total_parsers += 1;
    let justify_content_parser = JustifyContentParser::new();
    let justify_content_result = justify_content_parser.parse_class("justify-center");
    if justify_content_result.is_some() {
        working_parsers += 1;
        working_list.push("JustifyContentParser");
        println!("  ✅ JustifyContentParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("JustifyContentParser");
        println!("  ❌ JustifyContentParser - STUB");
    }

    total_parsers += 1;
    let justify_items_parser = JustifyItemsParser::new();
    let justify_items_result = justify_items_parser.parse_class("justify-items-center");
    if justify_items_result.is_some() {
        working_parsers += 1;
        working_list.push("JustifyItemsParser");
        println!("  ✅ JustifyItemsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("JustifyItemsParser");
        println!("  ❌ JustifyItemsParser - STUB");
    }

    total_parsers += 1;
    let justify_self_parser = JustifySelfParser::new();
    let justify_self_result = justify_self_parser.parse_class("justify-self-center");
    if justify_self_result.is_some() {
        working_parsers += 1;
        working_list.push("JustifySelfParser");
        println!("  ✅ JustifySelfParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("JustifySelfParser");
        println!("  ❌ JustifySelfParser - STUB");
    }

    total_parsers += 1;
    let align_content_parser = AlignContentParser::new();
    let align_content_result = align_content_parser.parse_class("content-center");
    if align_content_result.is_some() {
        working_parsers += 1;
        working_list.push("AlignContentParser");
        println!("  ✅ AlignContentParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AlignContentParser");
        println!("  ❌ AlignContentParser - STUB");
    }

    total_parsers += 1;
    let align_items_parser = AlignItemsParser::new();
    let align_items_result = align_items_parser.parse_class("items-center");
    if align_items_result.is_some() {
        working_parsers += 1;
        working_list.push("AlignItemsParser");
        println!("  ✅ AlignItemsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AlignItemsParser");
        println!("  ❌ AlignItemsParser - STUB");
    }

    total_parsers += 1;
    let align_self_parser = AlignSelfParser::new();
    let align_self_result = align_self_parser.parse_class("self-center");
    if align_self_result.is_some() {
        working_parsers += 1;
        working_list.push("AlignSelfParser");
        println!("  ✅ AlignSelfParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AlignSelfParser");
        println!("  ❌ AlignSelfParser - STUB");
    }

    total_parsers += 1;
    let place_content_parser = PlaceContentParser::new();
    let place_content_result = place_content_parser.parse_class("place-content-center");
    if place_content_result.is_some() {
        working_parsers += 1;
        working_list.push("PlaceContentParser");
        println!("  ✅ PlaceContentParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("PlaceContentParser");
        println!("  ❌ PlaceContentParser - STUB");
    }

    total_parsers += 1;
    let place_items_parser = PlaceItemsParser::new();
    let place_items_result = place_items_parser.parse_class("place-items-center");
    if place_items_result.is_some() {
        working_parsers += 1;
        working_list.push("PlaceItemsParser");
        println!("  ✅ PlaceItemsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("PlaceItemsParser");
        println!("  ❌ PlaceItemsParser - STUB");
    }

    total_parsers += 1;
    let place_self_parser = PlaceSelfParser::new();
    let place_self_result = place_self_parser.parse_class("place-self-center");
    if place_self_result.is_some() {
        working_parsers += 1;
        working_list.push("PlaceSelfParser");
        println!("  ✅ PlaceSelfParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("PlaceSelfParser");
        println!("  ❌ PlaceSelfParser - STUB");
    }

    // ===== BORDER PARSERS =====
    println!("\n🖼️ BORDER PARSERS:");

    total_parsers += 1;
    let border_parser = BorderUtilitiesParser::new();
    let border_result = border_parser.parse_class("border-2");
    if border_result.is_some() {
        working_parsers += 1;
        working_list.push("BorderUtilitiesParser");
        println!("  ✅ BorderUtilitiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("BorderUtilitiesParser");
        println!("  ❌ BorderUtilitiesParser - STUB");
    }

    // Test additional border parsers
    total_parsers += 1;
    let border_parser_advanced = BorderParser::new();
    let border_advanced_result = border_parser_advanced.parse_class("border");
    if border_advanced_result.is_some() {
        working_parsers += 1;
        working_list.push("BorderParser");
        println!("  ✅ BorderParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("BorderParser");
        println!("  ❌ BorderParser - STUB");
    }

    total_parsers += 1;
    let advanced_border_parser = AdvancedBorderParser::new();
    let advanced_border_result = advanced_border_parser.parse_class("border-t");
    if advanced_border_result.is_some() {
        working_parsers += 1;
        working_list.push("AdvancedBorderParser");
        println!("  ✅ AdvancedBorderParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AdvancedBorderParser");
        println!("  ❌ AdvancedBorderParser - STUB");
    }

    total_parsers += 1;
    let ring_parser = RingParser::new();
    let ring_result = ring_parser.parse_class("ring-2");
    if ring_result.is_some() {
        working_parsers += 1;
        working_list.push("RingParser");
        println!("  ✅ RingParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("RingParser");
        println!("  ❌ RingParser - STUB");
    }

    // ===== EFFECTS PARSERS =====
    println!("\n✨ EFFECTS PARSERS:");

    total_parsers += 1;
    let effects_parser = EffectsUtilitiesParser::new();
    let effects_result = effects_parser.parse_class("shadow-lg");
    if effects_result.is_some() {
        working_parsers += 1;
        working_list.push("EffectsUtilitiesParser");
        println!("  ✅ EffectsUtilitiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("EffectsUtilitiesParser");
        println!("  ❌ EffectsUtilitiesParser - STUB");
    }

    // Test additional effects parsers
    total_parsers += 1;
    let effects_parser_advanced = EffectsParser::new();
    let effects_advanced_result = effects_parser_advanced.parse_class("shadow");
    if effects_advanced_result.is_some() {
        working_parsers += 1;
        working_list.push("EffectsParser");
        println!("  ✅ EffectsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("EffectsParser");
        println!("  ❌ EffectsParser - STUB");
    }

    total_parsers += 1;
    let shadow_parser = ShadowParser::new();
    let shadow_result = shadow_parser.parse_class("shadow-md");
    if shadow_result.is_some() {
        working_parsers += 1;
        working_list.push("ShadowParser");
        println!("  ✅ ShadowParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("ShadowParser");
        println!("  ❌ ShadowParser - STUB");
    }

    // ===== MASK PARSERS =====
    println!("\n🎭 MASK PARSERS:");

    total_parsers += 1;
    let mask_parser = MaskUtilitiesParser::new();
    let mask_result = mask_parser.parse_class("mask-none");
    if mask_result.is_some() {
        working_parsers += 1;
        working_list.push("MaskUtilitiesParser");
        println!("  ✅ MaskUtilitiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("MaskUtilitiesParser");
        println!("  ❌ MaskUtilitiesParser - STUB");
    }

    // ===== INTERACTIVE PARSERS =====
    println!("\n🎮 INTERACTIVE PARSERS:");

    total_parsers += 1;
    let accent_parser = AccentColorParser::new();
    let accent_result = accent_parser.parse_class("accent-blue-500");
    if accent_result.is_some() {
        working_parsers += 1;
        working_list.push("AccentColorParser");
        println!("  ✅ AccentColorParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("AccentColorParser");
        println!("  ❌ AccentColorParser - STUB");
    }

    // Test additional interactive parsers
    total_parsers += 1;
    let interactive_parser = InteractiveParser::new();
    let interactive_result = interactive_parser.parse_class("cursor-pointer");
    if interactive_result.is_some() {
        working_parsers += 1;
        working_list.push("InteractiveParser");
        println!("  ✅ InteractiveParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("InteractiveParser");
        println!("  ❌ InteractiveParser - STUB");
    }

    // Test PostCSS integration if available
    #[cfg(feature = "postcss")]
    {
        println!("\n🔧 Testing PostCSS Integration...");
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
        println!("\n⚠️  PostCSS feature not enabled - skipping PostCSS integration test");
    }

    // Final assessment
    println!("\n🎯 CATEGORY PARSER TEST RESULTS:");
    println!("📊 Parser Status Summary:");
    println!(
        "  ✅ Working parsers: {}/{} ({:.1}%)",
        working_parsers,
        total_parsers,
        (working_parsers as f64 / total_parsers as f64) * 100.0
    );
    println!(
        "  🚨 Stub parsers: {}/{} ({:.1}%)",
        stub_parsers,
        total_parsers,
        (stub_parsers as f64 / total_parsers as f64) * 100.0
    );
    println!(
        "  💥 Broken parsers: {}/{} ({:.1}%)",
        broken_parsers,
        total_parsers,
        (broken_parsers as f64 / total_parsers as f64) * 100.0
    );

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
