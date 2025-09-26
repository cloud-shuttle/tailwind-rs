use tailwind_rs_core::*;

/// Analyze Tailwind CSS 4.1 coverage
fn main() {
    println!("🔍 Tailwind CSS 4.1 Coverage Analysis");
    println!("=====================================");

    let generator = CssGenerator::new();

    // Core Tailwind CSS 4.1 features
    let core_features = vec![
        // Layout & Display
        "block",
        "inline",
        "flex",
        "grid",
        "hidden",
        "table",
        "contents",
        // Spacing
        "p-4",
        "m-2",
        "px-6",
        "py-3",
        "pt-8",
        "pb-4",
        "pl-2",
        "pr-6",
        "mx-auto",
        "my-4",
        "mt-6",
        "mb-8",
        "ml-2",
        "mr-4",
        // Sizing
        "w-full",
        "h-screen",
        "max-w-4xl",
        "min-h-screen",
        "size-12",
        "size-6",
        // Typography
        "text-lg",
        "text-sm",
        "font-bold",
        "font-medium",
        "tracking-tight",
        "text-center",
        "text-left",
        "text-right",
        "leading-relaxed",
        // Colors
        "bg-blue-500",
        "text-white",
        "border-gray-200",
        "ring-blue-500",
        "bg-white/90",
        "text-black/50",
        "border-black/5",
        // States
        "hover:bg-blue-600",
        "focus:ring-2",
        "active:scale-95",
        "group-hover:opacity-100",
        "dark:bg-zinc-800",
        "dark:text-white",
        // Responsive
        "sm:text-lg",
        "md:flex-row",
        "lg:grid-cols-3",
        "xl:relative",
        // Advanced
        "backdrop-blur-sm",
        "drop-shadow-lg",
        "transform",
        "transition-all",
        "data-hover:bg-black/5",
        "data-closed:opacity-0",
        // Arbitrary values
        "w-[100px]",
        "h-[50px]",
        "bg-[url(/image.png)]",
        "top-[4px]",
        // Complex features
        "bg-gradient-to-r",
        "from-blue-500",
        "to-purple-600",
        "divide-y",
        "divide-gray-200",
        "object-cover",
        "origin-top",
    ];

    let mut working = 0;
    let mut broken = 0;

    for class in &core_features {
        match generator.class_to_properties(class) {
            Ok(_) => {
                println!("  ✅ {}", class);
                working += 1;
            }
            Err(_) => {
                println!("  ❌ {}", class);
                broken += 1;
            }
        }
    }

    let total = core_features.len();
    let coverage = (working as f32 / total as f32) * 100.0;

    println!("\n📊 Coverage Results:");
    println!("===================");
    println!("  ✅ Working: {}/{}", working, total);
    println!("  ❌ Broken: {}/{}", broken, total);
    println!("  📈 Coverage: {:.1}%", coverage);

    if coverage >= 95.0 {
        println!("\n🎉 Excellent coverage! Ready for release!");
    } else if coverage >= 90.0 {
        println!("\n🚀 Good coverage! Consider release candidate.");
    } else {
        println!("\n⚠️  Coverage needs improvement before release.");
    }
}
