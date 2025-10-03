//! Standalone demo of element-based processing functionality
//! This demonstrates the new CssGenerator::process_element_classes() API

use tailwind_rs_core::{CssGenerator, ElementContext, GradientContext, ShadowContext, TransformContext, VariantContext};

fn main() {
    println!("🎨 TAILWIND-RS ELEMENT-BASED PROCESSING DEMO");
    println!("============================================");

    let mut generator = CssGenerator::new();

    // Example 1: Interactive Button with Hover Effects
    println!("\n📱 Example 1: Interactive Button");
    println!("-------------------------------");
    let button_classes = vec![
        "px-6", "py-3",
        "bg-gradient-to-r", "from-purple-500", "via-pink-500", "to-red-500",
        "text-white", "rounded-xl", "font-semibold", "tracking-wide",
        "hover:from-purple-400", "hover:via-pink-400", "hover:to-red-400",
        "hover:scale-105", "hover:shadow-xl", "hover:shadow-purple-500/25",
        "transition-all", "duration-300", "transform"
    ];

    let button_css = generator.process_element_classes(&button_classes);
    println!("Classes: {:?}", button_classes);
    println!("Generated CSS:\n{}", button_css);

    // Example 2: Responsive Card with Multiple Effects
    println!("\n🎴 Example 2: Responsive Card");
    println!("----------------------------");
    let card_classes = vec![
        "p-6", "bg-gradient-to-br", "from-blue-500", "via-purple-600", "to-pink-500",
        "rounded-2xl", "text-white", "shadow-2xl", "backdrop-blur-lg",
        "md:scale-110", "md:rotate-3", "md:shadow-blue-500/30",
        "hover:shadow-3xl", "hover:-rotate-1",
        "dark:border", "dark:border-white/20"
    ];

    let card_css = generator.process_element_classes(&card_classes);
    println!("Classes: {:?}", card_classes);
    println!("Generated CSS:\n{}", card_css);

    // Example 3: Complex Variant Combinations
    println!("\n🔗 Example 3: Complex Variant Combinations");
    println!("-----------------------------------------");
    let complex_classes = vec![
        "bg-gray-100", "text-gray-800",
        "dark:bg-gray-800", "dark:text-gray-100",
        "md:dark:hover:bg-purple-600", "md:dark:hover:text-white",
        "md:dark:hover:scale-110", "md:dark:hover:shadow-lg"
    ];

    let complex_css = generator.process_element_classes(&complex_classes);
    println!("Classes: {:?}", complex_classes);
    println!("Generated CSS:\n{}", complex_css);

    // Example 4: Gradient with Opacity Stops
    println!("\n🌈 Example 4: Gradient with Opacity Stops");
    println!("----------------------------------------");
    let gradient_classes = vec![
        "bg-conic", "from-purple-500/20", "via-pink-600/30", "to-red-500/40",
        "hover:from-purple-400/30", "hover:via-pink-500/40", "hover:to-red-400/50"
    ];

    let gradient_css = generator.process_element_classes(&gradient_classes);
    println!("Classes: {:?}", gradient_classes);
    println!("Generated CSS:\n{}", gradient_css);

    println!("\n🎉 ELEMENT-BASED PROCESSING DEMO COMPLETE!");
    println!("==========================================");
    println!("✅ Gradients: State-aware multi-stop processing");
    println!("✅ Shadows: Proper box-shadow generation");
    println!("✅ Transforms: Combined scale/rotate/translate");
    println!("✅ Variants: Complex hover/responsive/dark combinations");
    println!("✅ Media Queries: Responsive breakpoint support");
    println!("✅ CSS Variables: Proper Tailwind variable usage");
}
