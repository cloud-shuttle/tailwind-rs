extern crate tailwind_rs_core;

use tailwind_rs_core::css_generator::CssGenerator;
use std::collections::HashMap;

fn main() {
    let mut generator = CssGenerator::new();

    // REAL FAILURE DATA FROM SSR DEMO - These are the classes that actually failed in production
    let real_world_failures = vec![
        // ===== GRADIENT OPACITY FAILURES (HIGH PRIORITY - should work) =====
        "from-green-500/20", "to-emerald-600/20", "dark:from-green-900/30", "dark:to-emerald-900/30",
        "to-cyan-600/20", "dark:to-cyan-900/30", "to-pink-600/20", "dark:to-pink-900/30",
        "border-purple-500/50", "shadow-purple-500/50", "shadow-blue-500/50", "shadow-green-500/50",

        // ===== CONTAINER & LAYOUT FAILURES =====
        "container", "border-2", "mx-auto", "mb-12", "mb-8", "mb-4", "px-4", "py-8", "py-16",

        // ===== GRADIENT DIRECTION FAILURES (should definitely work) =====
        "from-slate-900", "to-slate-900", "dark:from-gray-900", "dark:to-gray-900",
        "to-pink-500", "from-green-400", "from-pink-400", "to-pink-500", "via-pink-500", "via-cyan-500",
        "from-green-400", "via-emerald-500", "to-emerald-500", "to-cyan-500",

        // ===== TRANSFORM & ANIMATION FAILURES =====
        "transform", "transition-all", "rounded-2xl", "rounded-xl", "rounded-full", "rounded",
        "hover:scale-105", "hover:scale-110", "hover:-rotate-3", "tracking-wide", "italic",
        "flex-wrap", "drop-shadow-lg", "drop-shadow-xl", "drop-shadow-2xl",

        // ===== HOVER STATE FAILURES =====
        "hover:to-pink-700", "hover:from-gray-600", "hover:to-gray-800",

        // ===== OUTLINE FAILURES =====
        "outline-none", "focus:outline-none",

        // ===== BORDER & SHADOW WITH OPACITY FAILURES =====
        "border-white/30", "dark:border-gray-600/30", "border-green-500/50", "border-blue-500/50", "border-purple-500/50",
        "dark:border-gray-700/50", "dark:border-gray-600/50", "border-white/20", "dark:border-gray-700/30",
        "border-white/10", "border-black/10", "shadow-blue-500/25", "shadow-purple-500/25",
        "hover:shadow-blue-500/25", "hover:shadow-red-500/25",

        // ===== BACKGROUND WITH OPACITY FAILURES =====
        "bg-white/10", "dark:bg-gray-800/20", "bg-white/5", "bg-black/5",
        "dark:bg-gray-800/50", "dark:bg-gray-900/50",

        // ===== TEXT COLOR WITH OPACITY FAILURES =====
        "text-blue-300", "dark:text-blue-200", "dark:text-gray-300", "dark:text-gray-400",
        "text-transparent",

        // ===== CUSTOM/ARBITRARY CLASSES (may not be supported) =====
        "shadow-neon-blue", "shadow-neon-purple", "shadow-neon-green",
        "text-neon-blue", "text-neon-purple", "text-neon-green",
        "border", // basic border without width
    ];

    // Classes that SHOULD work (baseline test)
    let baseline_success_classes = vec![
        "min-h-screen", "text-6xl", "font-black", "bg-gradient-to-br", "bg-gradient-to-r",
        "text-purple-400", "text-cyan-400", "font-bold", "shadow-lg", "mb-4",
        "bg-clip-text", "text-transparent", "animate-pulse", "text-4xl", "text-center",
        "mb-8", "text-white", "drop-shadow-2xl", "text-2xl", "font-semibold",
        "text-lg", "text-gray-300", "space-x-4", "text-3xl", "font-medium",
    ];

    let mut results = TestResults::new();

    println!("🧪 COMPREHENSIVE INTEGRATION TEST - Real World Failures");
    println!("Testing classes that FAILED in the SSR demo");
    println!("{}", "=".repeat(80));

    // Test real-world failures first
    println!("\n🚨 TESTING REAL-WORLD FAILURES ({} classes):", real_world_failures.len());
    for class in &real_world_failures {
        test_class(&mut generator, class, &mut results);
    }

    // Test baseline success classes
    println!("\n✅ TESTING BASELINE SUCCESS CLASSES ({} classes):", baseline_success_classes.len());
    for class in &baseline_success_classes {
        test_class(&mut generator, class, &mut results);
    }

    // Analyze results
    results.analyze_and_report();

    // Provide actionable next steps based on failure patterns
    provide_actionable_recommendations(&results);
}

fn test_class(generator: &mut CssGenerator, class: &str, results: &mut TestResults) {
    match generator.class_to_css_rule(class) {
        Ok(rule) => {
            // Validate that CSS properties contain actual values, not CSS variables
            let mut css_value_issues = Vec::new();

            for property in &rule.properties {
                // Check for CSS variable issues - these should be resolved to actual values
                // Allow CSS variables for gradient stops (they're expected to set --tw-gradient-* variables)
                let is_gradient_stop = class.starts_with("from-") || class.starts_with("to-") || class.starts_with("via-") ||
                                      class.starts_with("dark:from-") || class.starts_with("dark:to-") || class.starts_with("dark:via-") ||
                                      class.starts_with("hover:from-") || class.starts_with("hover:to-") || class.starts_with("hover:via-");

                if property.value.contains("var(--color-") {
                    css_value_issues.push(format!("CSS variable instead of color: {}", property.value));
                }
                if property.value.contains("var(--radius-") {
                    css_value_issues.push(format!("CSS variable instead of border-radius: {}", property.value));
                }
                if property.value.contains("var(--text-") {
                    css_value_issues.push(format!("CSS variable instead of font-size: {}", property.value));
                }
                if property.value.contains("var(--font-") {
                    css_value_issues.push(format!("CSS variable instead of font-family: {}", property.value));
                }
                // Allow --tw-gradient- variables for gradient stops AND gradient directions
                let is_gradient_direction = class.starts_with("bg-gradient-to-");
                if property.value.contains("var(--tw-gradient-") && !is_gradient_stop && !is_gradient_direction {
                    css_value_issues.push(format!("CSS variable instead of gradient: {}", property.value));
                }
                if property.value.contains("var(--tw-translate-") {
                    css_value_issues.push(format!("CSS variable instead of transform: {}", property.value));
                }
                if property.value.contains("var(--tw-rotate-") {
                    css_value_issues.push(format!("CSS variable instead of transform: {}", property.value));
                }
                if property.value.contains("var(--tw-scale-") {
                    css_value_issues.push(format!("CSS variable instead of transform: {}", property.value));
                }
                if property.value.contains("var(--tw-skew-") {
                    css_value_issues.push(format!("CSS variable instead of transform: {}", property.value));
                }
                // Catch any remaining CSS variables (should be minimal)
                // Allow CSS variables for gradient directions and gradient stops
                let is_gradient_related = is_gradient_stop || is_gradient_direction;
                if property.value.contains("var(--") && !is_gradient_related {
                    css_value_issues.push(format!("Generic CSS variable found: {}", property.value));
                }
            }

            if css_value_issues.is_empty() {
                results.success_count += 1;
                results.successful_classes.push(class.to_string());
                println!("✅ {} -> {} properties", class, rule.properties.len());
            } else {
                // CSS value issues - treat as failure
                results.fail_count += 1;
                let error_msg = format!("CSS value issues: {}", css_value_issues.join(", "));
                results.failed_classes.push((class.to_string(), error_msg.clone()));

                let category = "CSS_VALUE_ISSUES";
                results.failures_by_category.entry(category.to_string()).or_insert(Vec::new()).push(class.to_string());

                println!("⚠️  {} -> CSS value issues: {}", class, css_value_issues.join(", "));
            }
        }
        Err(e) => {
            results.fail_count += 1;
            results.failed_classes.push((class.to_string(), e.to_string()));

            // Categorize the failure
            let category = categorize_failure(class, &e.to_string());
            results.failures_by_category.entry(category.clone()).or_insert(Vec::new()).push(class.to_string());

            println!("❌ {} -> {} [{}]", class, e, category);
        }
    }
}

#[derive(Debug)]
struct TestResults {
    success_count: usize,
    fail_count: usize,
    successful_classes: Vec<String>,
    failed_classes: Vec<(String, String)>,
    failures_by_category: HashMap<String, Vec<String>>,
}

impl TestResults {
    fn new() -> Self {
        Self {
            success_count: 0,
            fail_count: 0,
            successful_classes: Vec::new(),
            failed_classes: Vec::new(),
            failures_by_category: HashMap::new(),
        }
    }

    fn analyze_and_report(&self) {
        let total_classes = self.success_count + self.fail_count;

        println!("\n{}", "=".repeat(80));
        println!("📊 COMPREHENSIVE RESULTS:");
        println!("✅ Successful: {} classes", self.success_count);
        println!("❌ Failed: {} classes", self.fail_count);
        println!("📈 Success Rate: {:.1}%", (self.success_count as f64 / total_classes as f64) * 100.0);
        println!("🎯 Total Classes Tested: {}", total_classes);

        if !self.failures_by_category.is_empty() {
            println!("\n🔍 FAILURES BY CATEGORY:");
            for (category, classes) in &self.failures_by_category {
                println!("  📁 {}: {} classes", category, classes.len());
                for class in classes.iter().take(3) { // Show first 3 examples
                    println!("    • {}", class);
                }
                if classes.len() > 3 {
                    println!("    • ... and {} more", classes.len() - 3);
                }
            }
        }
    }
}

fn categorize_failure(class: &str, error: &str) -> String {
    // Categorize failures based on class patterns and error types

    // Opacity suffixes (highest priority)
    if class.contains("/20") || class.contains("/30") || class.contains("/50") || class.contains("/5") || class.contains("/10") {
        "OPACITY_SUFFIX".to_string()
    }
    // Gradient stops
    else if class.starts_with("from-") || class.starts_with("to-") || class.starts_with("via-") ||
              class.starts_with("dark:from-") || class.starts_with("dark:to-") || class.starts_with("dark:via-") {
        "GRADIENT_STOP".to_string()
    }
    // Hover/focus states
    else if class.starts_with("hover:") {
        "HOVER_STATE".to_string()
    } else if class.starts_with("focus:") {
        "FOCUS_STATE".to_string()
    }
    // Transform & animation
    else if class == "transform" || class.starts_with("hover:scale-") || class.starts_with("hover:-rotate-") {
        "TRANSFORM".to_string()
    } else if class == "transition-all" {
        "TRANSITION".to_string()
    } else if class.starts_with("rounded-") || class == "rounded" {
        "BORDER_RADIUS".to_string()
    } else if class == "tracking-wide" {
        "LETTER_SPACING".to_string()
    } else if class == "italic" {
        "FONT_STYLE".to_string()
    } else if class == "flex-wrap" {
        "FLEXBOX".to_string()
    }
    // Border & shadow with opacity
    else if class.starts_with("border-") && (class.contains("/") || class == "border") {
        "BORDER_OPACITY".to_string()
    } else if class.starts_with("border-") {
        "BORDER_WIDTH".to_string()
    } else if class.starts_with("shadow-") && class.contains("/") {
        "SHADOW_OPACITY".to_string()
    }
    // Background with opacity
    else if class.starts_with("bg-") && class.contains("/") {
        "BACKGROUND_OPACITY".to_string()
    }
    // Text with opacity or special colors
    else if class.starts_with("text-") && (class.contains("/") || class.contains("transparent") || class.starts_with("text-neon")) {
        if class.starts_with("text-neon") {
            "CUSTOM_TEXT".to_string()
        } else {
            "TEXT_OPACITY".to_string()
        }
    }
    // Drop shadows
    else if class.starts_with("drop-shadow-") {
        "DROP_SHADOW".to_string()
    }
    // Outline
    else if class.starts_with("outline-") {
        "OUTLINE".to_string()
    }
    // Container & layout
    else if error.contains("container") || class == "container" {
        "CONTAINER".to_string()
    }
    // Spacing
    else if class.starts_with("m") || class.starts_with("p") {
        "SPACING".to_string()
    }
    // Custom shadows (neon, etc.)
    else if class.starts_with("shadow-") && !class.contains("/") {
        "CUSTOM_SHADOW".to_string()
    }
    // Everything else
    else {
        "OTHER".to_string()
    }
}

fn provide_actionable_recommendations(results: &TestResults) {
    println!("\n🎯 ACTIONABLE RECOMMENDATIONS:");

    // Priority 1: Opacity suffixes (should be highest priority)
    if let Some(opacity_classes) = results.failures_by_category.get("OPACITY_SUFFIX") {
        println!("\n🚨 PRIORITY 1 - OPACITY SUFFIXES ({} failures)", opacity_classes.len());
        println!("   These are CRITICAL for modern Tailwind usage!");
        println!("   💡 Implement opacity suffix parsing in color parsers");
        println!("   📝 Classes: from-green-500/20, border-purple-500/50, shadow-blue-500/50");
        println!("   🔧 Parser: ColorParser, ShadowParser, BorderParser");
    }

    // Priority 2: Basic gradient directions (should work)
    if let Some(gradient_classes) = results.failures_by_category.get("GRADIENT_DIRECTION") {
        println!("\n⚡ PRIORITY 2 - GRADIENT DIRECTIONS ({} failures)", gradient_classes.len());
        println!("   Basic gradient classes should definitely work!");
        println!("   💡 Check GradientParser trie registration");
        println!("   📝 Classes: from-slate-900, to-pink-500, via-cyan-500");
        println!("   🔧 Parser: GradientParser");
    }

    // Priority 3: Transform & animation properties
    if let Some(transform_classes) = results.failures_by_category.get("TRANSFORM") {
        println!("\n🔄 PRIORITY 3 - TRANSFORM PROPERTIES ({} failures)", transform_classes.len());
        println!("   Transform utilities are essential for modern UI");
        println!("   💡 Implement TransformParser");
        println!("   📝 Classes: transform, hover:scale-105, hover:-rotate-3");
        println!("   🔧 Parser: TransformParser (new)");
    }

    if let Some(transition_classes) = results.failures_by_category.get("TRANSITION") {
        println!("\n🎭 PRIORITY 3 - TRANSITION PROPERTIES ({} failures)", transition_classes.len());
        println!("   Smooth transitions are core to good UX");
        println!("   💡 Implement TransitionParser");
        println!("   📝 Classes: transition-all");
        println!("   🔧 Parser: TransitionParser (new)");
    }

    if let Some(radius_classes) = results.failures_by_category.get("BORDER_RADIUS") {
        println!("\n🔘 PRIORITY 3 - BORDER RADIUS ({} failures)", radius_classes.len());
        println!("   Rounded corners are everywhere in modern design");
        println!("   💡 Enhance existing border parser or create BorderRadiusParser");
        println!("   📝 Classes: rounded-2xl, rounded-xl, rounded-full");
        println!("   🔧 Parser: BorderParser enhancement");
    }

    // Priority 4: Interactive states
    if let Some(hover_classes) = results.failures_by_category.get("HOVER_STATE") {
        println!("\n🎨 PRIORITY 4 - HOVER STATES ({} failures)", hover_classes.len());
        println!("   Interactive states are core to Tailwind");
        println!("   💡 Check variant parser integration");
        println!("   📝 Classes: hover:to-pink-700, hover:from-gray-600");
        println!("   🔧 Parser: Variant system, base parsers");
    }

    // Priority 5: Container and layout
    if let Some(container_classes) = results.failures_by_category.get("CONTAINER") {
        println!("\n🏗️ PRIORITY 5 - CONTAINER & LAYOUT ({} failures)", container_classes.len());
        println!("   Container queries are modern CSS");
        println!("   💡 Implement ContainerParser");
        println!("   📝 Classes: container");
        println!("   🔧 Parser: ContainerParser (new)");
    }

    // Priority 6: Typography enhancements
    if let Some(letter_classes) = results.failures_by_category.get("LETTER_SPACING") {
        println!("\n📝 PRIORITY 6 - LETTER SPACING ({} failures)", letter_classes.len());
        println!("   Typography controls are important for design");
        println!("   💡 Enhance TypographyParser");
        println!("   📝 Classes: tracking-wide");
        println!("   🔧 Parser: TypographyParser");
    }

    if let Some(font_classes) = results.failures_by_category.get("FONT_STYLE") {
        println!("\n🔤 PRIORITY 6 - FONT STYLE ({} failures)", font_classes.len());
        println!("   Font styling is basic typography");
        println!("   💡 Enhance TypographyParser");
        println!("   📝 Classes: italic");
        println!("   🔧 Parser: TypographyParser");
    }

    // Priority 7: Layout & spacing
    if let Some(flex_classes) = results.failures_by_category.get("FLEXBOX") {
        println!("\n📦 PRIORITY 7 - FLEXBOX PROPERTIES ({} failures)", flex_classes.len());
        println!("   Flexbox is fundamental to modern layouts");
        println!("   💡 Implement or enhance FlexboxParser");
        println!("   📝 Classes: flex-wrap");
        println!("   🔧 Parser: FlexboxParser");
    }

    if let Some(spacing_classes) = results.failures_by_category.get("SPACING") {
        println!("\n📏 PRIORITY 7 - SPACING UTILITIES ({} failures)", spacing_classes.len());
        println!("   Spacing is core to Tailwind");
        println!("   💡 Check SpacingParser implementation");
        println!("   📝 Classes: mb-4, px-4, py-8, mx-auto");
        println!("   🔧 Parser: SpacingParser");
    }

    // Priority 8: Border & shadow enhancements
    if let Some(border_opacity) = results.failures_by_category.get("BORDER_OPACITY") {
        println!("\n🔳 PRIORITY 8 - BORDER OPACITY ({} failures)", border_opacity.len());
        println!("   Border opacity is modern design feature");
        println!("   💡 Add opacity support to BorderParser");
        println!("   📝 Classes: border-white/30, border-gray-700/50");
        println!("   🔧 Parser: BorderParser");
    }

    if let Some(shadow_opacity) = results.failures_by_category.get("SHADOW_OPACITY") {
        println!("\n👤 PRIORITY 8 - SHADOW OPACITY ({} failures)", shadow_opacity.len());
        println!("   Shadow opacity creates depth");
        println!("   💡 Add opacity support to ShadowParser");
        println!("   📝 Classes: shadow-blue-500/25, shadow-purple-500/25");
        println!("   🔧 Parser: ShadowParser");
    }

    if let Some(drop_shadow) = results.failures_by_category.get("DROP_SHADOW") {
        println!("\n🌑 PRIORITY 8 - DROP SHADOW ({} failures)", drop_shadow.len());
        println!("   Drop shadows are image effects");
        println!("   💡 Enhance ShadowParser for drop-shadow variants");
        println!("   📝 Classes: drop-shadow-lg, drop-shadow-2xl");
        println!("   🔧 Parser: ShadowParser");
    }

    // Priority 9: Background & text opacity
    if let Some(bg_opacity) = results.failures_by_category.get("BACKGROUND_OPACITY") {
        println!("\n🎨 PRIORITY 9 - BACKGROUND OPACITY ({} failures)", bg_opacity.len());
        println!("   Background opacity is essential for overlays");
        println!("   💡 Add opacity support to BackgroundParser");
        println!("   📝 Classes: bg-white/10, bg-gray-800/50");
        println!("   🔧 Parser: BackgroundParser");
    }

    if let Some(text_opacity) = results.failures_by_category.get("TEXT_OPACITY") {
        println!("\n📝 PRIORITY 9 - TEXT OPACITY ({} failures)", text_opacity.len());
        println!("   Text opacity affects readability");
        println!("   💡 Add opacity support to TypographyParser");
        println!("   📝 Classes: text-transparent");
        println!("   🔧 Parser: TypographyParser");
    }

    // Priority 10: Outline & focus
    if let Some(outline_classes) = results.failures_by_category.get("OUTLINE") {
        println!("\n📋 PRIORITY 10 - OUTLINE UTILITIES ({} failures)", outline_classes.len());
        println!("   Outline is important for accessibility");
        println!("   💡 Implement OutlineParser");
        println!("   📝 Classes: outline-none");
        println!("   🔧 Parser: OutlineParser (new)");
    }

    if let Some(focus_classes) = results.failures_by_category.get("FOCUS_STATE") {
        println!("\n🎯 PRIORITY 10 - FOCUS STATES ({} failures)", focus_classes.len());
        println!("   Focus states are critical for accessibility");
        println!("   💡 Ensure focus variant works with all parsers");
        println!("   📝 Classes: focus:outline-none");
        println!("   🔧 Parser: Variant system");
    }

    // Priority 11: Custom/arbitrary classes (lowest priority)
    if let Some(custom_shadow) = results.failures_by_category.get("CUSTOM_SHADOW") {
        println!("\n👻 PRIORITY 11 - CUSTOM SHADOWS ({} failures)", custom_shadow.len());
        println!("   Custom shadows may be design-specific");
        println!("   💡 Consider if these should be supported");
        println!("   📝 Classes: shadow-neon-blue, shadow-neon-purple");
        println!("   🔧 Parser: May not need implementation");
    }

    if let Some(custom_text) = results.failures_by_category.get("CUSTOM_TEXT") {
        println!("\n🎨 PRIORITY 11 - CUSTOM TEXT COLORS ({} failures)", custom_text.len());
        println!("   Custom text colors may be design-specific");
        println!("   💡 Consider if these should be supported");
        println!("   📝 Classes: text-neon-blue, text-neon-green");
        println!("   🔧 Parser: May not need implementation");
    }

    println!("\n📋 IMPLEMENTATION CHECKLIST:");
    println!("□ Add opacity suffix parsing to ALL parsers (/20, /30, /50, /5, /10)");
    println!("□ Fix GradientParser trie registration for direction classes");
    println!("□ Implement TransformParser for transform, scale, rotate");
    println!("□ Implement TransitionParser for transition-all");
    println!("□ Enhance BorderParser for radius variations");
    println!("□ Implement ContainerParser for container queries");
    println!("□ Enhance TypographyParser for letter-spacing, font-style");
    println!("□ Implement or enhance FlexboxParser");
    println!("□ Ensure SpacingParser handles all spacing utilities");
    println!("□ Add opacity support to ShadowParser and BorderParser");
    println!("□ Add opacity support to BackgroundParser and TypographyParser");
    println!("□ Implement OutlineParser");
    println!("□ Verify variant system for hover/focus states");
    println!("□ Add integration tests for each parser category");
    println!("□ Run this test after each parser implementation");

    println!("\n🔧 MAXIMUM ROUTER TESTING:");
    println!("After implementing parsers, verify router coverage:");
    println!("• Each parser should handle its registered prefixes");
    println!("• Test with real class names from this failure list");
    println!("• Measure router efficiency (trie lookups should be fast)");
    println!("• Verify no conflicts between parser prefixes");

    println!("\n🧪 TEST IMPROVEMENT COMPLETE:");
    println!("This test now uses REAL failure data instead of synthetic examples!");
    println!("Run this test after each parser implementation to track progress.");
}
