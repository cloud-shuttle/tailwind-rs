//! Final 6 Parsers Test
//! 
//! This test identifies and tests the remaining 6 parsers to reach 100% coverage.

use tailwind_rs_core::*;

#[test]
fn final_6_parsers_test() -> Result<()> {
    println!("🔍 FINAL 6 PARSERS TEST");
    println!("Testing remaining 6 parsers to reach 100% coverage...\n");
    
    let mut working_parsers = 0;
    let mut stub_parsers = 0;
    let mut broken_parsers = 0;
    let mut total_parsers = 0;
    
    let mut working_list: Vec<&str> = Vec::new();
    let mut stub_list: Vec<&str> = Vec::new();
    let mut broken_list: Vec<&str> = Vec::new();
    
    println!("🔍 TESTING REMAINING 6 PARSERS...\n");
    
    // Test parsers that might not have been tested yet
    // Based on the mod.rs file, let's test some that might be missing
    
    // ===== SHADOW PARSERS =====
    println!("🌫️ SHADOW PARSERS:");
    
    total_parsers += 1;
    let shadow_parser = ShadowParser::new();
    let shadow_result = shadow_parser.parse_class("shadow-lg");
    if shadow_result.is_some() {
        working_parsers += 1;
        working_list.push("ShadowParser");
        println!("  ✅ ShadowParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("ShadowParser");
        println!("  ❌ ShadowParser - STUB");
    }
    
    // ===== RING PARSERS =====
    println!("\n💍 RING PARSERS:");
    
    total_parsers += 1;
    let ring_parser = RingParser::new();
    let ring_result = ring_parser.parse_class("ring-4");
    if ring_result.is_some() {
        working_parsers += 1;
        working_list.push("RingParser");
        println!("  ✅ RingParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("RingParser");
        println!("  ❌ RingParser - STUB");
    }
    
    // ===== MASK UTILITIES PARSERS =====
    println!("\n🎭 MASK UTILITIES PARSERS:");
    
    total_parsers += 1;
    let mask_utilities_parser = MaskUtilitiesParser::new();
    let mask_utilities_result = mask_utilities_parser.parse_class("mask-none");
    if mask_utilities_result.is_some() {
        working_parsers += 1;
        working_list.push("MaskUtilitiesParser");
        println!("  ✅ MaskUtilitiesParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("MaskUtilitiesParser");
        println!("  ❌ MaskUtilitiesParser - STUB");
    }
    
    // ===== EFFECTS PARSERS =====
    println!("\n✨ EFFECTS PARSERS:");
    
    total_parsers += 1;
    let effects_parser = EffectsParser::new();
    let effects_result = effects_parser.parse_class("opacity-50");
    if effects_result.is_some() {
        working_parsers += 1;
        working_list.push("EffectsParser");
        println!("  ✅ EffectsParser - WORKING");
    } else {
        stub_parsers += 1;
        stub_list.push("EffectsParser");
        println!("  ❌ EffectsParser - STUB");
    }
    
    // ===== INTERACTIVE PARSERS =====
    println!("\n🎮 INTERACTIVE PARSERS:");
    
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
    
    // ===== GAP PARSERS =====
    println!("\n📏 GAP PARSERS:");
    
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
    
    // Final assessment
    println!("\n🎯 FINAL 6 PARSERS TEST RESULTS:");
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
