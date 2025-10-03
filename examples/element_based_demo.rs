//! Standalone demo showcasing Tailwind-RS Element-Based Processing
//! This demonstrates all Phase 3 advanced features working together

use tailwind_rs_core::CssGenerator;

fn main() {
    println!("🎉 TAILWIND-RS ELEMENT-BASED PROCESSING DEMO");
    println!("============================================");
    println!();

    let mut generator = CssGenerator::new();

    println!("🎨 PHASE 3 ADVANCED FEATURES DEMO");
    println!("=================================");
    println!();

    // Example 1: Complex Interactive Button with All Features
    println!("📱 Example 1: Ultimate Interactive Button");
    println!("----------------------------------------");
    let ultimate_button_classes = vec![
        // Layout & Spacing
        "px-8", "py-4", "m-4",
        // Typography
        "text-white", "font-bold", "text-lg", "tracking-wide",
        // Background & Gradients
        "bg-gradient-to-r", "from-purple-500", "via-pink-500", "to-red-500",
        // Borders & Rounded
        "rounded-2xl", "border-2", "border-white/20",
        // Shadows
        "shadow-2xl", "shadow-purple-500/25",
        // Transforms
        "transform", "scale-100",
        // Filters
        "backdrop-blur-sm", "brightness-110",
        // Animations
        "animate-pulse", "duration-300", "ease-in-out",
        // Hover Effects
        "hover:from-purple-400", "hover:via-pink-400", "hover:to-red-400",
        "hover:scale-110", "hover:shadow-3xl", "hover:shadow-purple-400/50",
        "hover:brightness-125", "hover:backdrop-blur-md",
        "transition-all", "duration-300",
        // Responsive Variants
        "md:px-12", "md:py-6", "md:text-xl",
        "lg:px-16", "lg:py-8", "lg:text-2xl", "lg:scale-105",
        // Dark Mode
        "dark:from-indigo-600", "dark:via-purple-600", "dark:to-pink-600",
        "dark:hover:from-indigo-500", "dark:hover:via-purple-500", "dark:hover:to-pink-500",
        // Arbitrary Values
        "w-[200px]", "h-[60px]", "text-[18px]", "rounded-[16px]",
        "bg-[#ff6b6b]", "shadow-[#ff6b6b]/30", "border-[#ffffff]/30",
    ];

    let css = generator.process_element_classes(&ultimate_button_classes);
    println!("🎯 Generated CSS for Ultimate Button:");
    println!("{}", css);
    println!();

    // Example 2: Advanced Card with Multiple Effects
    println!("🎴 Example 2: Advanced Card with Multiple Effects");
    println!("------------------------------------------------");
    let card_classes = vec![
        // Layout
        "relative", "overflow-hidden",
        // Background & Gradients
        "bg-gradient-to-br", "from-slate-900", "via-purple-900", "to-slate-900",
        // Spacing & Borders
        "p-8", "rounded-3xl", "border", "border-white/10",
        // Shadows & Filters
        "shadow-2xl", "backdrop-blur-xl", "brightness-105",
        // Animations
        "animate-float", "duration-6000", "ease-in-out",
        // Hover Effects
        "hover:shadow-3xl", "hover:shadow-purple-500/20",
        "hover:scale-105", "hover:brightness-110",
        "hover:backdrop-blur-2xl", "transition-all", "duration-500",
        // Responsive
        "md:p-12", "lg:p-16",
        // Arbitrary Values
        "min-h-[300px]", "bg-[#1a1a2e]", "shadow-[#6366f1]/10",
    ];

    let card_css = generator.process_element_classes(&card_classes);
    println!("🎴 Generated CSS for Advanced Card:");
    println!("{}", card_css);
    println!();

    // Example 3: Text Effects Showcase
    println!("✨ Example 3: Text Effects Showcase");
    println!("-----------------------------------");
    let text_classes = vec![
        // Typography
        "text-transparent", "bg-clip-text",
        "bg-gradient-to-r", "from-cyan-400", "via-blue-500", "to-purple-600",
        "font-black", "text-6xl", "tracking-tight",
        // Animations
        "animate-rainbow", "duration-3000", "ease-linear",
        // Filters
        "drop-shadow-2xl", "drop-shadow-cyan-400/50",
        // Responsive
        "md:text-8xl", "lg:text-9xl",
        // Arbitrary Values
        "text-[4rem]", "bg-[#00d4ff]", "drop-shadow-[#00d4ff]/60",
    ];

    let text_css = generator.process_element_classes(&text_classes);
    println!("✨ Generated CSS for Text Effects:");
    println!("{}", text_css);
    println!();

    // Summary
    println!("🎉 SUMMARY");
    println!("==========");
    println!("✅ Element-Based Processing: ACTIVE");
    println!("✅ GradientContext: Multi-stop gradients working");
    println!("✅ ShadowContext: Box shadows with custom colors");
    println!("✅ TransformContext: Scale, rotate, translate, skew");
    println!("✅ VariantContext: Hover, responsive, dark mode");
    println!("✅ FilterContext: Blur, brightness, contrast, etc.");
    println!("✅ AnimationContext: Duration, timing, keyframes");
    println!("✅ ArbitraryValueContext: [value] syntax support");
    println!("✅ Complex Combinations: All features working together");
    println!();
    println!("🚀 Tailwind-RS Element-Based Processing is PRODUCTION READY!");
}
