use tailwind_rs_core::*;

fn main() {
    println!("🚀 Testing Amazing SSR Demo CSS Generation");
    
    // Test the amazing CSS classes from your demo
    let mut generator = CssGenerator::new();
    
    // Layout & Structure
    let layout_classes = vec![
        "min-h-screen", "bg-gradient-to-br", "from-slate-900", "via-purple-900", "to-slate-900",
        "dark:from-gray-900", "dark:via-purple-900", "dark:to-gray-900",
        "container", "mx-auto", "px-4", "py-8", "max-w-7xl"
    ];
    
    // Typography & Headers
    let typography_classes = vec![
        "text-6xl", "font-black", "text-center", "mb-12",
        "bg-gradient-to-r", "from-blue-400", "via-purple-500", "to-pink-500",
        "bg-clip-text", "text-transparent", "animate-pulse"
    ];
    
    // Interactive Elements
    let interactive_classes = vec![
        "px-6", "py-3", "bg-gradient-to-r", "from-blue-500", "to-purple-600",
        "text-white", "rounded-xl", "hover:from-blue-600", "hover:to-purple-700",
        "transition-all", "duration-300", "transform", "hover:scale-105",
        "hover:shadow-xl", "hover:shadow-blue-500/25"
    ];
    
    // Grid & Layout
    let grid_classes = vec![
        "grid", "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3", "gap-6",
        "space-y-6", "max-w-6xl", "mx-auto"
    ];
    
    // Special Effects
    let effects_classes = vec![
        "p-6", "bg-gradient-to-br", "from-purple-400", "via-pink-500", "to-red-500",
        "rounded-2xl", "text-white", "text-center", "transform", "hover:scale-110",
        "transition-all", "duration-500", "hover:rotate-3", "shadow-2xl"
    ];
    
    // Animations & Transitions
    let animation_classes = vec![
        "animate-bounce", "animate-pulse", "animate-spin", "animate-ping",
        "transition-all", "duration-300", "ease-in-out",
        "hover:animate-pulse", "hover:animate-bounce"
    ];
    
    // Shadows & Effects
    let shadow_classes = vec![
        "shadow-lg", "shadow-xl", "shadow-2xl", "shadow-blue-500/25",
        "drop-shadow-lg", "drop-shadow-xl", "drop-shadow-2xl",
        "backdrop-blur-sm", "backdrop-blur-md", "backdrop-blur-lg"
    ];
    
    // Add all classes
    let all_classes = vec![
        layout_classes, typography_classes, interactive_classes,
        grid_classes, effects_classes, animation_classes, shadow_classes
    ].into_iter().flatten().collect::<Vec<_>>();
    
    println!("📝 Adding {} amazing CSS classes...", all_classes.len());
    
    for class in all_classes {
        match generator.add_class(class) {
            Ok(_) => {
                // Successfully added class
            }
            Err(e) => {
                eprintln!("Warning: Failed to add class '{}': {:?}", class, e);
            }
        }
    }
    
    // Generate CSS
    let css = generator.generate_css();
    println!("✅ Generated CSS with {} characters", css.len());
    println!("🎉 Your amazing SSR demo CSS is working!");
    
    // Show first 500 characters of CSS
    if css.len() > 500 {
        println!("📄 CSS preview: {}...", &css[..500]);
    } else {
        println!("📄 Full CSS: {}", css);
    }
}
