use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    let mut generator = CssGenerator::new();

    // Test header/navbar classes from Header.tsx
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
    println!(
        "  📊 Coverage: {:.1}%",
        (working_count as f32 / (working_count + broken_count) as f32) * 100.0
    );

    println!("\n🎨 Generated CSS:");
    println!("==================");
    let css = generator.generate_css();
    println!("{}", css);

    // Write CSS to file
    std::fs::write("header-demo.css", &css).expect("Failed to write CSS file");
    println!("\n✅ CSS written to header-demo.css");

    // Create a simple HTML demo
    let html = format!(
        "<!DOCTYPE html>\n\
        <html lang=\"en\">\n\
        <head>\n\
            <meta charset=\"UTF-8\">\n\
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\
            <title>Tailwind-RS Header Demo</title>\n\
            <style>{}</style>\n\
        </head>\n\
        <body class=\"bg-gray-100 dark:bg-gray-900\">\n\
            <div class=\"min-h-screen\">\n\
                <!-- Header -->\n\
                <header class=\"pointer-events-none relative z-50 flex flex-none flex-col\">\n\
                    <div class=\"top-0 z-10 h-16 pt-6\">\n\
                        <div class=\"container mx-auto px-4\">\n\
                            <div class=\"relative flex gap-4\">\n\
                                <!-- Logo/Avatar -->\n\
                                <div class=\"flex flex-1\">\n\
                                    <div class=\"h-10 w-10 rounded-full bg-white/90 p-0.5 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm dark:bg-zinc-800/90 dark:ring-white/10\">\n\
                                        <div class=\"h-9 w-9 rounded-full bg-zinc-100 object-cover dark:bg-zinc-800\"></div>\n\
                                    </div>\n\
                                </div>\n\
                                \n\
                                <!-- Desktop Navigation -->\n\
                                <div class=\"flex flex-1 justify-end md:justify-center\">\n\
                                    <nav class=\"pointer-events-auto hidden md:block\">\n\
                                        <ul class=\"flex rounded-full bg-white/90 px-3 text-sm font-medium text-zinc-800 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm dark:bg-zinc-800/90 dark:text-zinc-200 dark:ring-white/10\">\n\
                                            <li><a href=\"#\" class=\"relative block px-3 py-2 transition text-teal-500 dark:text-teal-400\">About</a></li>\n\
                                            <li><a href=\"#\" class=\"relative block px-3 py-2 transition hover:text-teal-500 dark:hover:text-teal-400\">Articles</a></li>\n\
                                            <li><a href=\"#\" class=\"relative block px-3 py-2 transition hover:text-teal-500 dark:hover:text-teal-400\">Projects</a></li>\n\
                                            <li><a href=\"#\" class=\"relative block px-3 py-2 transition hover:text-teal-500 dark:hover:text-teal-400\">Speaking</a></li>\n\
                                            <li><a href=\"#\" class=\"relative block px-3 py-2 transition hover:text-teal-500 dark:hover:text-teal-400\">Uses</a></li>\n\
                                        </ul>\n\
                                    </nav>\n\
                                </div>\n\
                                \n\
                                <!-- Theme Toggle -->\n\
                                <div class=\"flex justify-end md:flex-1\">\n\
                                    <div class=\"pointer-events-auto\">\n\
                                        <button class=\"group rounded-full bg-white/90 px-3 py-2 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm transition dark:bg-zinc-800/90 dark:ring-white/10 dark:hover:ring-white/20\">\n\
                                            <svg class=\"h-6 w-6 fill-zinc-100 stroke-zinc-500 transition group-hover:fill-zinc-200 group-hover:stroke-zinc-700 dark:hidden\">\n\
                                                <circle cx=\"12\" cy=\"12\" r=\"4\"/>\n\
                                                <path d=\"M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 6.34l-1.41 1.41M19.07 19.07l-1.41 1.41\"/>\n\
                                            </svg>\n\
                                            <svg class=\"hidden h-6 w-6 fill-zinc-700 stroke-zinc-500 transition dark:block\">\n\
                                                <path d=\"M17.25 16.22a6.937 6.937 0 0 1-9.47-9.47 7.451 7.451 0 1 0 9.47 9.47ZM12.75 7C17 7 17 2.75 17 2.75S17 7 21.25 7C17 7 17 11.25 17 11.25S17 7 12.75 7Z\"/>\n\
                                            </svg>\n\
                                        </button>\n\
                                    </div>\n\
                                </div>\n\
                            </div>\n\
                        </div>\n\
                    </div>\n\
                </header>\n\
                \n\
                <!-- Main Content -->\n\
                <main class=\"container mx-auto px-4 py-8\">\n\
                    <h1 class=\"text-4xl font-bold text-zinc-800 dark:text-zinc-100 mb-4\">Header/Navbar Demo</h1>\n\
                    <p class=\"text-lg text-zinc-600 dark:text-zinc-400 mb-8\">\n\
                        This demo shows a header/navbar built with Tailwind-RS classes. \n\
                        The header includes navigation, theme toggle, and responsive design.\n\
                    </p>\n\
                    \n\
                    <div class=\"grid grid-cols-1 md:grid-cols-2 gap-6\">\n\
                        <div class=\"bg-white dark:bg-zinc-800 p-6 rounded-lg shadow-lg ring-1 ring-zinc-900/5 dark:ring-zinc-700/50\">\n\
                            <h2 class=\"text-xl font-semibold text-zinc-800 dark:text-zinc-100 mb-2\">Features</h2>\n\
                            <ul class=\"text-zinc-600 dark:text-zinc-400 space-y-1\">\n\
                                <li>✅ Responsive navigation</li>\n\
                                <li>✅ Dark mode support</li>\n\
                                <li>✅ Backdrop blur effects</li>\n\
                                <li>✅ Hover states</li>\n\
                                <li>✅ Theme toggle</li>\n\
                            </ul>\n\
                        </div>\n\
                        \n\
                        <div class=\"bg-white dark:bg-zinc-800 p-6 rounded-lg shadow-lg ring-1 ring-zinc-900/5 dark:ring-zinc-700/50\">\n\
                            <h2 class=\"text-xl font-semibold text-zinc-800 dark:text-zinc-100 mb-2\">Coverage</h2>\n\
                            <p class=\"text-zinc-600 dark:text-zinc-400\">\n\
                                Working classes: {}<br>\n\
                                Broken classes: {}<br>\n\
                                Coverage: {:.1}%\n\
                            </p>\n\
                        </div>\n\
                    </div>\n\
                </main>\n\
            </div>\n\
        </body>\n\
        </html>",
        css, working_count, broken_count, (working_count as f32 / (working_count + broken_count) as f32) * 100.0
    );

    std::fs::write("header-demo.html", &html).expect("Failed to write HTML file");
    println!("✅ HTML demo written to header-demo.html");
    println!("🌐 Open header-demo.html in your browser to see the header/navbar demo!");
}
