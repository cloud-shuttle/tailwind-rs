//! Debug Stub Parsers Test
//!
//! This test debugs why the 9 "stub" parsers are not working.

use tailwind_rs_core::*;

#[test]
fn debug_stub_parsers() -> Result<()> {
    println!("🔍 DEBUG STUB PARSERS");
    println!("Debugging the 9 'stub' parsers...\n");

    // Test AdvancedColorParser
    println!("🎨 Testing AdvancedColorParser:");
    let advanced_color_parser = AdvancedColorParser::new();
    let test_classes = vec![
        "bg-green-500",
        "bg-green-500/50",
        "border-red-500",
        "border-red-500/25",
    ];
    for class in test_classes {
        let result = advanced_color_parser.parse_class(class);
        println!("  {} -> {:?}", class, result);
    }

    // Test AdvancedSpacingParser
    println!("\n📏 Testing AdvancedSpacingParser:");
    let advanced_spacing_parser = AdvancedSpacingParser::new();
    let test_classes = vec!["space-y-4", "space-x-2", "gap-4", "gap-x-2", "gap-y-4"];
    for class in test_classes {
        let result = advanced_spacing_parser.parse_class(class);
        println!("  {} -> {:?}", class, result);
    }

    // Test FractionalTransformsParser
    println!("\n🔄 Testing FractionalTransformsParser:");
    let fractional_transforms_parser = FractionalTransformsParser::new();
    let test_classes = vec![
        "translate-x-1/2",
        "translate-y-1/2",
        "-translate-x-1/2",
        "-translate-y-1/2",
        "translate-x-1/3",
        "translate-y-2/3",
    ];
    for class in test_classes {
        let result = fractional_transforms_parser.parse_class(class);
        println!("  {} -> {:?}", class, result);
    }

    // Test TransitionPropertiesParser
    println!("\n🎬 Testing TransitionPropertiesParser:");
    let transition_properties_parser = TransitionPropertiesParser::new();
    let test_classes = vec!["ease-linear", "duration-300", "delay-100", "scale-95"];
    for class in test_classes {
        let result = transition_properties_parser.parse_class(class);
        println!("  {} -> {:?}", class, result);
    }

    // Test DataAttributeParser
    println!("\n🔧 Testing DataAttributeParser:");
    let data_attribute_parser = DataAttributeParser::new();
    let test_classes = vec![
        "data-hover:bg-black/5",
        "data-active:scale-95",
        "data-open:opacity-100",
        "data-closed:opacity-0",
    ];
    for class in test_classes {
        let result = data_attribute_parser.parse_class(class);
        println!("  {} -> {:?}", class, result);
    }

    // Test BackgroundPropertiesParser
    println!("\n🎨 Testing BackgroundPropertiesParser:");
    let background_properties_parser = BackgroundPropertiesParser::new();
    let test_classes = vec!["bg-no-repeat", "bg-repeat", "bg-fixed", "bg-clip-text"];
    for class in test_classes {
        let result = background_properties_parser.parse_class(class);
        println!("  {} -> {:?}", class, result);
    }

    // Test BreakControlParser
    println!("\n⏸️ Testing BreakControlParser:");
    let break_control_parser = BreakControlParser::new();
    let test_classes = vec![
        "break-after-auto",
        "break-before-column",
        "break-inside-avoid",
    ];
    for class in test_classes {
        let result = break_control_parser.parse_class(class);
        println!("  {} -> {:?}", class, result);
    }

    // Test LayoutUtilitiesParser
    println!("\n🏗️ Testing LayoutUtilitiesParser:");
    let layout_utilities_parser = LayoutUtilitiesParser::new();
    let test_classes = vec!["float-right", "clear-both", "isolate", "object-center"];
    for class in test_classes {
        let result = layout_utilities_parser.parse_class(class);
        println!("  {} -> {:?}", class, result);
    }

    // Test AccessibilityParser
    println!("\n♿ Testing AccessibilityParser:");
    let accessibility_parser = AccessibilityParser::new();
    let test_classes = vec!["forced-color-adjust-auto", "forced-color-adjust-none"];
    for class in test_classes {
        let result = accessibility_parser.parse_class(class);
        println!("  {} -> {:?}", class, result);
    }

    Ok(())
}
