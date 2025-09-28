use tailwind_rs_core::css_generator::CssGenerator;
use std::collections::HashMap;

/// Comprehensive parser debugging tool
/// Tests parsers in isolation and through the full delegation pipeline
fn main() {
    println!("🔍 Tailwind-RS Parser Debugging Tool");
    println!("=====================================");

    // Test 1: Known Working Classes (Baseline)
    println!("\n📊 Phase 1: Testing Known Working Classes (Baseline)");
    test_working_classes();

    // Test 2: Suspected Broken Classes
    println!("\n🚨 Phase 2: Testing Suspected Broken Classes");
    test_broken_classes();

    // Test 3: Parser Isolation Tests
    println!("\n🔬 Phase 3: Parser Isolation Tests");
    test_parser_isolation();

    // Test 4: Delegation Flow Analysis
    println!("\n🔗 Phase 4: Delegation Flow Analysis");
    test_delegation_flow();

    println!("\n✅ Debugging complete. Check output above for issues.");
}

/// Test classes we know work (baseline)
fn test_working_classes() {
    let mut generator = CssGenerator::new();
    let working_classes = vec![
        "text-green-300",
        "bg-gradient-to-br",
        "from-purple-600",
        "to-blue-600",
        "text-white",
        "bg-black/30",
        "rounded-lg",
    ];

    println!("Testing {} known working classes:", working_classes.len());

    for class in working_classes {
        match generator.add_class(class) {
            Ok(_) => {
                println!("  ✅ {} - Added successfully", class);
            },
            Err(e) => {
                println!("  ❌ {} - Failed: {}", class, e);
            }
        }
    }

    let css = generator.generate_css();
    println!("Generated CSS length: {} chars", css.len());
    if css.len() < 500 {
        println!("Generated CSS:\n{}", css);
    } else {
        println!("Generated CSS (first 500 chars):\n{}", &css[..500]);
    }
}

/// Test classes we suspect are broken
fn test_broken_classes() {
    let mut generator = CssGenerator::new();
    let broken_classes = vec![
        "animate-spin",
        "scale-105",
        "rotate-45",
        "blur-sm",
        "backdrop-blur-lg",
        "hover:bg-blue-600",
        "md:text-lg",
        "focus:ring-2",
        "transition-all",
        "duration-300",
    ];

    println!("Testing {} suspected broken classes:", broken_classes.len());

    for class in broken_classes {
        match generator.add_class(class) {
            Ok(_) => {
                println!("  ✅ {} - Added successfully", class);
            },
            Err(e) => {
                println!("  ❌ {} - Failed: {}", class, e);
            }
        }
    }

    let css = generator.generate_css();
    println!("Generated CSS length: {} chars", css.len());
    if css.len() < 500 {
        println!("Generated CSS:\n{}", css);
    } else {
        println!("Generated CSS (first 500 chars):\n{}", &css[..500]);
    }
}

/// Test parsers in complete isolation
fn test_parser_isolation() {
    println!("Testing parsers in isolation...");

    // Test AnimationParser directly
    test_animation_parser();

    // Test TransformParser directly
    test_transform_parser();

    // Test FilterParser directly
    test_filter_parser();

    // Test BackgroundParser directly
    test_background_parser();
}

fn test_animation_parser() {
    println!("  🎬 Testing AnimationParser isolation:");

    let parser = tailwind_rs_core::css_generator::parsers::AnimationParser::new();

    let test_classes = vec![
        "animate-spin",
        "animate-bounce",
        "animate-ping",
        "animate-pulse",
        "animate-none",
        "invalid-animation-class",
    ];

    for class in test_classes {
        let result = parser.parse_class(class);
        match result {
            Some(properties) => {
                println!("    ✅ {} -> {} properties", class, properties.len());
                for prop in &properties {
                    println!("      - {}: {}", prop.name, prop.value);
                }
            },
            None => {
                println!("    ⚪ {} -> None (not recognized)", class);
            }
        }
    }
}

fn test_transform_parser() {
    println!("  🔄 Testing Transform Parsers isolation:");

    // Test BasicTransformsParser
    let basic_parser = tailwind_rs_core::css_generator::parsers::BasicTransformsParser::new();
    let scale_parser = tailwind_rs_core::css_generator::parsers::ScaleParser::new();

    let test_classes = vec![
        "scale-105",
        "scale-x-110",
        "scale-y-90",
        "rotate-45",
        "rotate-90",
        "translate-x-4",
        "translate-y-2",
        "skew-x-12",
        "skew-y-6",
        "invalid-transform-class",
    ];

    println!("    BasicTransformsParser:");
    for class in &test_classes {
        let result = basic_parser.parse_class(class);
        match result {
            Some(properties) => {
                println!("      ✅ {} -> {} properties", class, properties.len());
            },
            None => {
                println!("      ⚪ {} -> None", class);
            }
        }
    }

    println!("    ScaleParser:");
    for class in &test_classes {
        let result = scale_parser.parse_class(class);
        match result {
            Some(properties) => {
                println!("      ✅ {} -> {} properties", class, properties.len());
            },
            None => {
                println!("      ⚪ {} -> None", class);
            }
        }
    }
}

fn test_filter_parser() {
    println!("  🌫️ Testing Filter Parsers isolation:");

    let filter_parser = tailwind_rs_core::css_generator::parsers::FilterUtilitiesParser::new();
    let backdrop_parser = tailwind_rs_core::css_generator::parsers::BackdropFilterUtilitiesParser::new();

    let test_classes = vec![
        "blur-sm",
        "blur-md",
        "brightness-110",
        "contrast-125",
        "grayscale",
        "invert",
        "sepia",
        "backdrop-blur-lg",
        "backdrop-brightness-90",
        "invalid-filter-class",
    ];

    println!("    FilterUtilitiesParser:");
    for class in &test_classes {
        let result = filter_parser.parse_class(class);
        match result {
            Some(properties) => {
                println!("      ✅ {} -> {} properties", class, properties.len());
            },
            None => {
                println!("      ⚪ {} -> None", class);
            }
        }
    }

    println!("    BackdropFilterUtilitiesParser:");
    for class in &test_classes {
        let result = backdrop_parser.parse_class(class);
        match result {
            Some(properties) => {
                println!("      ✅ {} -> {} properties", class, properties.len());
            },
            None => {
                println!("      ⚪ {} -> None", class);
            }
        }
    }
}

fn test_background_parser() {
    println!("  🎨 Testing Background Parsers isolation:");

    let bg_parser = tailwind_rs_core::css_generator::parsers::BackgroundParser::new();
    let bg_props_parser = tailwind_rs_core::css_generator::parsers::BackgroundPropertiesParser::new();

    let test_classes = vec![
        "bg-fixed",
        "bg-local",
        "bg-scroll",
        "bg-clip-border",
        "bg-clip-padding",
        "bg-clip-content",
        "bg-clip-text",
        "bg-origin-border",
        "bg-origin-padding",
        "bg-origin-content",
        "bg-repeat-round",
        "bg-repeat-space",
        "invalid-bg-class",
    ];

    println!("    BackgroundParser:");
    for class in &test_classes {
        let result = bg_parser.parse_class(class);
        match result {
            Some(properties) => {
                println!("      ✅ {} -> {} properties", class, properties.len());
            },
            None => {
                println!("      ⚪ {} -> None", class);
            }
        }
    }

    println!("    BackgroundPropertiesParser:");
    for class in &test_classes {
        let result = bg_props_parser.parse_class(class);
        match result {
            Some(properties) => {
                println!("      ✅ {} -> {} properties", class, properties.len());
            },
            None => {
                println!("      ⚪ {} -> None", class);
            }
        }
    }
}

/// Test the full delegation flow to see where it fails
fn test_delegation_flow() {
    println!("Testing full delegation flow...");

    // We'll need to add logging to the delegation logic to see this properly
    // For now, just test the end-to-end result

    let mut generator = CssGenerator::new();

    // Test a class that should work but might be failing
    let test_class = "animate-spin";

    println!("Testing delegation for: {}", test_class);

    match generator.add_class(test_class) {
        Ok(_) => {
            println!("  ✅ Successfully added to generator");
            let css = generator.generate_css();

            if css.contains("animation") {
                println!("  ✅ CSS contains animation properties");
                // Find the animation rule
                for line in css.lines() {
                    if line.contains("animation") {
                        println!("  📋 Generated rule: {}", line);
                    }
                }
            } else {
                println!("  ❌ CSS does not contain animation properties");
                println!("  📄 Full CSS: {}", css);
            }
        },
        Err(e) => {
            println!("  ❌ Failed to add to generator: {}", e);
        }
    }
}
