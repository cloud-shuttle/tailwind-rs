extern crate tailwind_rs_core;

use tailwind_rs_core::css_generator::CssGenerator;
use std::collections::HashMap;

fn main() {
    let generator = CssGenerator::new();

    // REAL FAILURE DATA FROM SSR DEMO - These are the classes that actually failed in production
    let real_world_failures = vec![
        // Gradient opacity failures (HIGH PRIORITY - these should work)
        "from-green-500/20", "to-emerald-600/20", "dark:from-green-900/30", "dark:to-emerald-900/30",
        "to-cyan-600/20", "dark:to-cyan-900/30", "to-pink-600/20", "dark:to-pink-900/30",
        "border-purple-500/50", "shadow-purple-500/50", "shadow-blue-500/50", "shadow-green-500/50",

        // Container and layout failures
        "container", "border-2",

        // Basic gradient direction failures (these should definitely work)
        "from-slate-900", "to-slate-900", "dark:from-gray-900", "dark:to-gray-900",
        "to-pink-500", "from-green-400", "from-pink-400", "to-pink-500", "via-pink-500", "via-cyan-500",
        "from-green-400", "via-emerald-500", "to-emerald-500", "to-cyan-500",

        // Hover state failures
        "hover:to-pink-700", "hover:from-gray-600", "hover:to-gray-800",

        // Outline failures
        "outline-none", "focus:outline-none",

        // Custom/arbitrary classes that may not be supported (lower priority)
        "shadow-neon-blue", "shadow-neon-purple", "shadow-neon-green",
        "text-neon-blue", "text-neon-purple", "text-neon-green",
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
        test_class(&generator, class, &mut results);
    }

    // Test baseline success classes
    println!("\n✅ TESTING BASELINE SUCCESS CLASSES ({} classes):", baseline_success_classes.len());
    for class in &baseline_success_classes {
        test_class(&generator, class, &mut results);
    }

    // Analyze results
    results.analyze_and_report();

    // Provide actionable next steps based on failure patterns
    provide_actionable_recommendations(&results);
}

fn test_class(generator: &CssGenerator, class: &str, results: &mut TestResults) {
    match generator.class_to_css_rule(class) {
        Ok(rule) => {
            results.success_count += 1;
            results.successful_classes.push(class.to_string());
            println!("✅ {} -> {} properties", class, rule.properties.len());
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
    if class.contains("/20") || class.contains("/30") || class.contains("/50") {
        "OPACITY_SUFFIX".to_string()
    } else if class.starts_with("from-") || class.starts_with("to-") || class.starts_with("via-") ||
              class.starts_with("dark:from-") || class.starts_with("dark:to-") || class.starts_with("dark:via-") {
        "GRADIENT_DIRECTION".to_string()
    } else if class.starts_with("hover:") {
        "HOVER_STATE".to_string()
    } else if class.starts_with("focus:") {
        "FOCUS_STATE".to_string()
    } else if class.starts_with("border-") {
        "BORDER_WIDTH".to_string()
    } else if class.starts_with("outline-") {
        "OUTLINE".to_string()
    } else if class.starts_with("shadow-") && !class.contains("-500") {
        "CUSTOM_SHADOW".to_string()
    } else if class.starts_with("text-neon") {
        "CUSTOM_TEXT".to_string()
    } else if error.contains("container") || class == "container" {
        "CONTAINER".to_string()
    } else {
        "OTHER".to_string()
    }
}

fn provide_actionable_recommendations(results: &TestResults) {
    println!("\n🎯 ACTIONABLE RECOMMENDATIONS:");

    // Priority 1: Opacity suffixes (should be highest priority)
    if let Some(opacity_classes) = results.failures_by_category.get("OPACITY_SUFFIX") {
        println!("\n🚨 PRIORITY 1 - OPACITY SUFFIXES ({} failures)");
        println!("   These are CRITICAL for modern Tailwind usage!");
        println!("   💡 Implement opacity suffix parsing in color parsers");
        println!("   📝 Classes: from-green-500/20, border-purple-500/50, shadow-blue-500/50");
        println!("   🔧 Parser: ColorParser, ShadowParser, BorderParser");
    }

    // Priority 2: Basic gradient directions (should work)
    if let Some(gradient_classes) = results.failures_by_category.get("GRADIENT_DIRECTION") {
        println!("\n⚡ PRIORITY 2 - GRADIENT DIRECTIONS ({} failures)");
        println!("   Basic gradient classes should definitely work!");
        println!("   💡 Check GradientParser trie registration");
        println!("   📝 Classes: from-slate-900, to-pink-500, via-cyan-500");
        println!("   🔧 Parser: GradientParser");
    }

    // Priority 3: Hover/focus states
    if let Some(hover_classes) = results.failures_by_category.get("HOVER_STATE") {
        println!("\n🎨 PRIORITY 3 - HOVER STATES ({} failures)");
        println!("   Interactive states are core to Tailwind");
        println!("   💡 Check variant parser integration");
        println!("   📝 Classes: hover:to-pink-700, hover:from-gray-600");
        println!("   🔧 Parser: Variant system, base parsers");
    }

    // Priority 4: Container and layout
    if let Some(container_classes) = results.failures_by_category.get("CONTAINER") {
        println!("\n🏗️ PRIORITY 4 - CONTAINER & LAYOUT ({} failures)");
        println!("   Container queries are modern CSS");
        println!("   💡 Implement ContainerParser");
        println!("   📝 Classes: container");
        println!("   🔧 Parser: ContainerParser (new)");
    }

    // Priority 5: Border widths
    if let Some(border_classes) = results.failures_by_category.get("BORDER_WIDTH") {
        println!("\n🔲 PRIORITY 5 - BORDER WIDTHS ({} failures)");
        println!("   Basic border utilities");
        println!("   💡 Check BorderParser width support");
        println!("   📝 Classes: border-2");
        println!("   🔧 Parser: BorderParser");
    }

    println!("\n📋 IMPLEMENTATION CHECKLIST:");
    println!("□ Add opacity suffix parsing to ColorParser (/20, /30, /50, etc.)");
    println!("□ Fix GradientParser trie registration for direction classes");
    println!("□ Implement ContainerParser for container queries");
    println!("□ Enhance BorderParser for width variations");
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
