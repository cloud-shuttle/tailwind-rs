use std::fs;

use tailwind_rs_core::CssGenerator;

fn main() {
    println!("🎨 Generating comprehensive CSS for Tailwind-RS demo...");

    let mut generator = CssGenerator::new();

    // Test with a simple class first
    println!("Testing with a simple class...");
    let _ = generator.add_class("bg-blue-500");
    let simple_css = generator.generate_css();
    println!("Simple CSS generated: {} characters", simple_css.len());

    // Reset generator for comprehensive test
    let mut generator = CssGenerator::new();

    // Add comprehensive demo classes
    let demo_classes = vec![
        // Layout
        "container", "mx-auto", "flex", "grid", "block", "inline", "hidden",
        "min-h-screen", "items-center", "justify-center", "max-w-7xl", "space-y-6",
        "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3", "gap-6",

        // Spacing
        "p-4", "m-2", "px-8", "py-6", "mt-8", "mb-4", "mb-12", "mb-6", "mb-8",

        // Colors - Comprehensive palette
        // Reds
        "bg-red-50", "bg-red-100", "bg-red-200", "bg-red-300", "bg-red-400", "bg-red-500", "bg-red-600", "bg-red-700", "bg-red-800", "bg-red-900",
        "text-red-50", "text-red-100", "text-red-200", "text-red-300", "text-red-400", "text-red-500", "text-red-600", "text-red-700", "text-red-800", "text-red-900",

        // Blues
        "bg-blue-50", "bg-blue-100", "bg-blue-200", "bg-blue-300", "bg-blue-400", "bg-blue-500", "bg-blue-600", "bg-blue-700", "bg-blue-800", "bg-blue-900",
        "text-blue-50", "text-blue-100", "text-blue-200", "text-blue-300", "text-blue-400", "text-blue-500", "text-blue-600", "text-blue-700", "text-blue-800", "text-blue-900",

        // Greens
        "bg-green-50", "bg-green-100", "bg-green-200", "bg-green-300", "bg-green-400", "bg-green-500", "bg-green-600", "bg-green-700", "bg-green-800", "bg-green-900",
        "text-green-50", "text-green-100", "text-green-200", "text-green-300", "text-green-400", "text-green-500", "text-green-600", "text-green-700", "text-green-800", "text-green-900",

        // Purples
        "bg-purple-50", "bg-purple-100", "bg-purple-200", "bg-purple-300", "bg-purple-400", "bg-purple-500", "bg-purple-600", "bg-purple-700", "bg-purple-800", "bg-purple-900",
        "text-purple-50", "text-purple-100", "text-purple-200", "text-purple-300", "text-purple-400", "text-purple-500", "text-purple-600", "text-purple-700", "text-purple-800", "text-purple-900",

        // Pinks
        "bg-pink-50", "bg-pink-100", "bg-pink-200", "bg-pink-300", "bg-pink-400", "bg-pink-500", "bg-pink-600", "bg-pink-700", "bg-pink-800", "bg-pink-900",
        "text-pink-50", "text-pink-100", "text-pink-200", "text-pink-300", "text-pink-400", "text-pink-500", "text-pink-600", "text-pink-700", "text-pink-800", "text-pink-900",

        // Cyans
        "bg-cyan-50", "bg-cyan-100", "bg-cyan-200", "bg-cyan-300", "bg-cyan-400", "bg-cyan-500", "bg-cyan-600", "bg-cyan-700", "bg-cyan-800", "bg-cyan-900",
        "text-cyan-50", "text-cyan-100", "text-cyan-200", "text-cyan-300", "text-cyan-400", "text-cyan-500", "text-cyan-600", "text-cyan-700", "text-cyan-800", "text-cyan-900",

        // Teals
        "bg-teal-50", "bg-teal-100", "bg-teal-200", "bg-teal-300", "bg-teal-400", "bg-teal-500", "bg-teal-600", "bg-teal-700", "bg-teal-800", "bg-teal-900",
        "text-teal-50", "text-teal-100", "text-teal-200", "text-teal-300", "text-teal-400", "text-teal-500", "text-teal-600", "text-teal-700", "text-teal-800", "text-teal-900",

        // Emeralds
        "bg-emerald-50", "bg-emerald-100", "bg-emerald-200", "bg-emerald-300", "bg-emerald-400", "bg-emerald-500", "bg-emerald-600", "bg-emerald-700", "bg-emerald-800", "bg-emerald-900",
        "text-emerald-50", "text-emerald-100", "text-emerald-200", "text-emerald-300", "text-emerald-400", "text-emerald-500", "text-emerald-600", "text-emerald-700", "text-emerald-800", "text-emerald-900",

        // Yellows/Oranges
        "bg-yellow-50", "bg-yellow-100", "bg-yellow-200", "bg-yellow-300", "bg-yellow-400", "bg-yellow-500", "bg-yellow-600", "bg-yellow-700", "bg-yellow-800", "bg-yellow-900",
        "bg-orange-50", "bg-orange-100", "bg-orange-200", "bg-orange-300", "bg-orange-400", "bg-orange-500", "bg-orange-600", "bg-orange-700", "bg-orange-800", "bg-orange-900",
        "text-yellow-50", "text-yellow-100", "text-yellow-200", "text-yellow-300", "text-yellow-400", "text-yellow-500", "text-yellow-600", "text-yellow-700", "text-yellow-800", "text-yellow-900",
        "text-orange-50", "text-orange-100", "text-orange-200", "text-orange-300", "text-orange-400", "text-orange-500", "text-orange-600", "text-orange-700", "text-orange-800", "text-orange-900",

        // Indigos
        "bg-indigo-50", "bg-indigo-100", "bg-indigo-200", "bg-indigo-300", "bg-indigo-400", "bg-indigo-500", "bg-indigo-600", "bg-indigo-700", "bg-indigo-800", "bg-indigo-900",
        "text-indigo-50", "text-indigo-100", "text-indigo-200", "text-indigo-300", "text-indigo-400", "text-indigo-500", "text-indigo-600", "text-indigo-700", "text-indigo-800", "text-indigo-900",

        // Slate/Zinc/Neutral/Stone
        "bg-slate-50", "bg-slate-100", "bg-slate-200", "bg-slate-300", "bg-slate-400", "bg-slate-500", "bg-slate-600", "bg-slate-700", "bg-slate-800", "bg-slate-900",
        "bg-zinc-50", "bg-zinc-100", "bg-zinc-200", "bg-zinc-300", "bg-zinc-400", "bg-zinc-500", "bg-zinc-600", "bg-zinc-700", "bg-zinc-800", "bg-zinc-900",
        "bg-neutral-50", "bg-neutral-100", "bg-neutral-200", "bg-neutral-300", "bg-neutral-400", "bg-neutral-500", "bg-neutral-600", "bg-neutral-700", "bg-neutral-800", "bg-neutral-900",
        "bg-stone-50", "bg-stone-100", "bg-stone-200", "bg-stone-300", "bg-stone-400", "bg-stone-500", "bg-stone-600", "bg-stone-700", "bg-stone-800", "bg-stone-900",
        "text-slate-50", "text-slate-100", "text-slate-200", "text-slate-300", "text-slate-400", "text-slate-500", "text-slate-600", "text-slate-700", "text-slate-800", "text-slate-900",
        "text-zinc-50", "text-zinc-100", "text-zinc-200", "text-zinc-300", "text-zinc-400", "text-zinc-500", "text-zinc-600", "text-zinc-700", "text-zinc-800", "text-zinc-900",
        "text-neutral-50", "text-neutral-100", "text-neutral-200", "text-neutral-300", "text-neutral-400", "text-neutral-500", "text-neutral-600", "text-neutral-700", "text-neutral-800", "text-neutral-900",
        "text-stone-50", "text-stone-100", "text-stone-200", "text-stone-300", "text-stone-400", "text-stone-500", "text-stone-600", "text-stone-700", "text-stone-800", "text-stone-900",

        // Border colors
        "border-gray-300", "border-gray-700", "border-white/20", "border-green-500/50", "border-blue-500/50", "border-purple-500/20", "border-pink-500/20", "border-cyan-500/20", "border-2",

        // Special background colors used in demo
        "bg-white/10", "bg-gray-800/20",

        // Background gradients - ALL TYPES and DIRECTIONS used in demo
        "bg-gradient-to-r", "bg-gradient-to-br", "bg-gradient-to-bl", "bg-gradient-to-tr",
        "bg-gradient-to-l", "bg-gradient-to-t", "bg-gradient-to-b", "bg-gradient-to-tl",
        "bg-conic", "bg-radial",

        // Gradient stops - ALL COLORS used in demo
        "from-blue-400", "via-purple-500", "to-pink-500",
        "from-slate-900", "via-indigo-900", "to-purple-900",
        "from-gray-900", "to-indigo-900",
        "from-green-400", "to-emerald-500",
        "from-cyan-400", "via-blue-500", "to-indigo-600",
        "from-emerald-400", "via-teal-500", "to-cyan-600",
        "from-purple-500", "via-pink-500", "to-red-500",
        "from-purple-900/30", "to-pink-900/30",
        "from-green-500/20", "to-emerald-600/20",
        "from-purple-500/20", "to-pink-600/20",
        "from-blue-500/20", "to-purple-600/20",
        "from-green-400", "to-emerald-500",
        "from-red-500", "via-pink-500", "to-red-500",
        "from-orange-500", "via-red-500", "to-pink-600",
        "from-teal-500", "via-cyan-500", "to-blue-600",

        // Dark mode variants
        "dark:from-gray-900", "dark:via-purple-900", "dark:to-indigo-900",
        "dark:bg-gray-800/20", "dark:border-gray-700/30",
        "dark:from-purple-900/30", "dark:to-pink-900/30",
        "dark:from-green-900/30", "dark:to-emerald-900/30",
        "dark:from-purple-900/30", "dark:to-pink-900/30",
        "dark:from-blue-900/30", "dark:via-purple-900", "dark:to-cyan-900/30",

        // Typography
        "text-lg", "font-bold", "text-center", "leading-relaxed",
        "text-6xl", "font-black", "text-4xl", "text-2xl", "text-sm", "font-semibold",
        "font-mono", "text-3em", "text-1.5em", "text-yellow", "font-weight-bold",
        "bg-clip-text", "text-transparent", "animate-pulse",

        // Effects (shadows)
        "shadow-sm", "shadow", "shadow-md", "shadow-lg", "shadow-xl", "shadow-2xl",
        "shadow-none", "shadow-inner", "backdrop-blur-lg",
        "shadow-blue-500/25", "shadow-purple-500/25", "shadow-orange-500/25", "shadow-teal-500/25",

        // Transforms and animations
        "transform", "hover:scale-105", "hover:scale-110", "hover:rotate-3", "hover:-rotate-3", "hover:rotate-2",
        "transition-all", "duration-300", "duration-500",

        // Borders and rounded
        "rounded-xl", "rounded-2xl", "rounded-full", "border", "border-white/20",

        // Positioning
        "relative", "absolute", "inline-block",

        // Responsive variants
        "md:flex", "lg:grid", "sm:text-center", "md:grid-cols-2", "lg:grid-cols-3",

        // Hover states
        "hover:bg-blue-600", "hover:shadow-lg", "hover:shadow-xl",
        "hover:from-purple-400", "hover:via-pink-400", "hover:to-red-400",
        "hover:from-orange-400", "hover:via-red-400", "hover:to-pink-500",
        "hover:from-teal-400", "hover:via-cyan-400", "hover:to-blue-500",
        "hover:from-red-600", "hover:to-red-600",
        "hover:from-pink-700", "hover:to-pink-700",
        "hover:from-gray-600", "hover:from-gray-800",

        // Opacity
        "opacity-90", "opacity-80",

        // Dark mode
        "dark:bg-gray-800/20", "dark:text-white", "dark:text-gray-200",
    ];

    println!("Adding {} demo classes...", demo_classes.len());
    let mut success_count = 0;
    let mut fail_count = 0;

    for class in demo_classes {
        match generator.add_class(class) {
            Ok(_) => {
                success_count += 1;
                if success_count <= 5 { // Show first 5 successes
                    println!("✅ Added: {}", class);
                }
            }
            Err(e) => {
                fail_count += 1;
                if fail_count <= 5 { // Show first 5 failures
                    println!("❌ Failed to add '{}': {}", class, e);
                }
            }
        }
    }

    println!("Results: {} successful, {} failed", success_count, fail_count);

    println!("Generator rules count: {}", generator.rules.len());
    println!("Generator custom properties count: {}", generator.custom_properties.as_ref().map(|m| m.len()).unwrap_or(0));

    // Try manually adding a simple rule to test CSS generation
    use tailwind_rs_core::css_generator::types::{CssProperty, CssRule};
    let test_rule = CssRule {
        selector: ".test-bg-blue".to_string(),
        properties: vec![
            CssProperty {
                name: "background-color".to_string(),
                value: "#3b82f6".to_string(),
                important: false,
            }
        ],
        media_query: None,
        specificity: 1,
    };
    generator.rules.insert("test-bg-blue".to_string(), test_rule);
    println!("Added manual test rule, rules count now: {}", generator.rules.len());

    println!("About to call generator.generate_css()...");
    let css = generator.generate_css();
    let css_len = css.len();
    println!("Called generator.generate_css(), got {} characters", css_len);
    println!("Comprehensive CSS length: {} characters", css_len);

    // Show first 500 characters for debugging
    if css_len > 0 {
        println!("First 500 characters of CSS:");
        println!("{}", &css[..std::cmp::min(500, css_len)]);
    } else {
        println!("CSS is empty! Checking a few rules manually...");
        // Check if we can manually generate CSS for a simple class
        if let Some(rule) = generator.rules.get("bg-blue-500") {
            println!("Found bg-blue-500 rule: {:?}", rule);
        } else {
            println!("bg-blue-500 rule not found");
        }

        // Try generating CSS for just one rule
        if let Some((selector, rule)) = generator.rules.iter().next() {
            let mut manual_css = format!("{} {{\n", selector);
            for prop in &rule.properties {
                manual_css.push_str(&format!("  {}: {};\n", prop.name, prop.value));
            }
            manual_css.push_str("}\n");
            println!("Manual CSS for first rule: {}", manual_css);
        }
    }

    // Ensure the assets directory exists
    fs::create_dir_all("assets").unwrap_or_default();

    // Write the CSS to the assets file
    fs::write("assets/generated.css", css).expect("Failed to write CSS file");

    println!("✅ Generated CSS file with {} characters", css_len);
    println!("📁 CSS saved to: assets/generated.css");
}
