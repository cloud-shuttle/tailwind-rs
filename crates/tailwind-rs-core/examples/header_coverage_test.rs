use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();
    
    // Test key header/navbar classes from Header.tsx
    let header_classes = vec![
        // Basic layout
        "pointer-events-none",
        "relative",
        "z-50",
        "flex",
        "flex-none",
        "flex-col",
        "h-16",
        "pt-6",
        "top-0",
        "z-10",
        
        // Navigation
        "group",
        "flex",
        "items-center",
        "rounded-full",
        "bg-white/90",
        "px-4",
        "py-2",
        "text-sm",
        "font-medium",
        "text-zinc-800",
        "shadow-lg",
        "ring-1",
        "shadow-zinc-800/5",
        "ring-zinc-900/5",
        "backdrop-blur-sm",
        "dark:bg-zinc-800/90",
        "dark:text-zinc-200",
        "dark:ring-white/10",
        "dark:hover:ring-white/20",
        
        // Mobile navigation
        "ml-3",
        "h-auto",
        "w-2",
        "stroke-zinc-500",
        "group-hover:stroke-zinc-700",
        "dark:group-hover:stroke-zinc-400",
        
        // Backdrop
        "fixed",
        "inset-0",
        "z-50",
        "bg-zinc-800/40",
        "backdrop-blur-xs",
        
        // Panel
        "fixed",
        "inset-x-4",
        "top-8",
        "z-50",
        "origin-top",
        "rounded-3xl",
        "bg-white",
        "p-8",
        "ring-1",
        "ring-zinc-900/5",
        "dark:bg-zinc-900",
        "dark:ring-zinc-800",
        
        // Navigation items
        "flex",
        "flex-row-reverse",
        "items-center",
        "justify-between",
        "-m-1",
        "p-1",
        "h-6",
        "w-6",
        "text-zinc-500",
        "dark:text-zinc-400",
        "text-sm",
        "font-medium",
        "text-zinc-600",
        "dark:text-zinc-400",
        "mt-6",
        "-my-2",
        "divide-y",
        "divide-zinc-100",
        "text-base",
        "text-zinc-800",
        "dark:divide-zinc-100/5",
        "dark:text-zinc-300",
        
        // Desktop navigation
        "flex",
        "rounded-full",
        "bg-white/90",
        "px-3",
        "text-sm",
        "font-medium",
        "text-zinc-800",
        "shadow-lg",
        "ring-1",
        "shadow-zinc-800/5",
        "ring-zinc-900/5",
        "backdrop-blur-sm",
        "dark:bg-zinc-800/90",
        "dark:text-zinc-200",
        "dark:ring-white/10",
        
        // Nav items
        "relative",
        "block",
        "px-3",
        "py-2",
        "transition",
        "text-teal-500",
        "dark:text-teal-400",
        "hover:text-teal-500",
        "dark:hover:text-teal-400",
        "absolute",
        "inset-x-1",
        "-bottom-px",
        "h-px",
        "bg-linear-to-r",
        "from-teal-500/0",
        "via-teal-500/40",
        "to-teal-500/0",
        "dark:from-teal-400/0",
        "dark:via-teal-400/40",
        "dark:to-teal-400/0",
        
        // Theme toggle
        "group",
        "rounded-full",
        "bg-white/90",
        "px-3",
        "py-2",
        "shadow-lg",
        "ring-1",
        "shadow-zinc-800/5",
        "ring-zinc-900/5",
        "backdrop-blur-sm",
        "transition",
        "dark:bg-zinc-800/90",
        "dark:ring-white/10",
        "dark:hover:ring-white/20",
        "h-6",
        "w-6",
        "fill-zinc-100",
        "stroke-zinc-500",
        "transition",
        "group-hover:fill-zinc-200",
        "group-hover:stroke-zinc-700",
        "dark:hidden",
        "hidden",
        "h-6",
        "w-6",
        "fill-zinc-700",
        "stroke-zinc-500",
        "transition",
        "dark:block",
        
        // Avatar
        "h-10",
        "w-10",
        "rounded-full",
        "bg-white/90",
        "p-0.5",
        "shadow-lg",
        "ring-1",
        "shadow-zinc-800/5",
        "ring-zinc-900/5",
        "backdrop-blur-sm",
        "dark:bg-zinc-800/90",
        "dark:ring-white/10",
        "pointer-events-auto",
        "rounded-full",
        "bg-zinc-100",
        "object-cover",
        "dark:bg-zinc-800",
        "h-16",
        "w-16",
        "h-9",
        "w-9",
        
        // Container
        "flex",
        "gap-4",
        "flex-1",
        "justify-end",
        "md:justify-center",
        "md:hidden",
        "pointer-events-auto",
        "hidden",
        "md:block",
        "justify-end",
        "md:flex-1",
        "pointer-events-auto",
    ];
    
    println!("🔍 Testing Header/Navbar Classes");
    println!("================================");
    
    let mut working_count = 0;
    let mut broken_count = 0;
    
    for class in header_classes {
        match generator.add_class(class) {
            Ok(_) => {
                working_count += 1;
                println!("  ✅ {}", class);
            }
            Err(e) => {
                broken_count += 1;
                println!("  ❌ {} - Error: {}", class, e);
            }
        }
    }
    
    println!("\n📊 Header/Navbar Test Results:");
    println!("  ✅ Working classes: {}", working_count);
    println!("  ❌ Broken classes: {}", broken_count);
    println!("  📊 Coverage: {:.1}%", (working_count as f32 / (working_count + broken_count) as f32) * 100.0);
    
    println!("\n🎨 Generated CSS:");
    println!("==================");
    let css = generator.generate_css();
    println!("{}", css);
    
    // Write CSS to file
    std::fs::write("header-coverage-test.css", &css).expect("Failed to write CSS file");
    println!("\n✅ CSS written to header-coverage-test.css");
    
    println!("\n🌐 To test the header/navbar:");
    println!("1. Open header-coverage-test.css in a text editor");
    println!("2. Copy the CSS and use it in your HTML");
    println!("3. Test the header/navbar functionality!");
}
