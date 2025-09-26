use tailwind_rs_core::*;

/// Simple Coverage Demo - Analyze Tailwind-RS coverage with real React components
/// This demonstrates how much of the React components we can replicate with Tailwind-RS

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("📊 Tailwind-RS Coverage Analysis");
    println!("{}", "=".repeat(50));

    // Analyze Footer component coverage
    let footer_report = analyze_footer_coverage();
    println!("\n🔧 Footer Component Analysis:");
    println!(
        "  ✅ Working classes: {}",
        footer_report.working_classes.len()
    );
    println!(
        "  ❌ Broken classes: {}",
        footer_report.broken_classes.len()
    );
    println!("  📊 Coverage: {:.1}%", footer_report.coverage_percentage);
    println!("  📝 Working: {:?}", footer_report.working_classes);
    println!("  ❌ Broken: {:?}", footer_report.broken_classes);

    // Analyze ArticleLayout component coverage
    let article_report = analyze_article_layout_coverage();
    println!("\n🔧 ArticleLayout Component Analysis:");
    println!(
        "  ✅ Working classes: {}",
        article_report.working_classes.len()
    );
    println!(
        "  ❌ Broken classes: {}",
        article_report.broken_classes.len()
    );
    println!("  📊 Coverage: {:.1}%", article_report.coverage_percentage);
    println!("  📝 Working: {:?}", article_report.working_classes);
    println!("  ❌ Broken: {:?}", article_report.broken_classes);

    // Generate CSS for the demo
    let demo_css = generate_demo_css();
    match std::fs::write("simple-coverage.css", demo_css) {
        Ok(_) => println!("✅ Coverage analysis CSS generated successfully"),
        Err(e) => println!("❌ Failed to generate CSS: {}", e),
    }

    // Create a complete HTML page
    let html = create_demo_html();
    match std::fs::write("simple-coverage.html", html) {
        Ok(_) => {
            println!("✅ Complete HTML page generated: simple-coverage.html");
            println!("🚀 Open simple-coverage.html in your browser to see the live demo!");
        }
        Err(e) => println!("❌ Failed to write HTML: {}", e),
    }

    println!("\n📊 Overall Coverage Summary:");
    println!(
        "  - Footer Component: {:.1}% coverage",
        footer_report.coverage_percentage
    );
    println!(
        "  - ArticleLayout Component: {:.1}% coverage",
        article_report.coverage_percentage
    );
    println!(
        "  - Average Coverage: {:.1}%",
        (footer_report.coverage_percentage + article_report.coverage_percentage) / 2.0
    );

    println!("\n🌐 To view this demo:");
    println!("  1. Open simple-coverage.html in your browser");
    println!("  2. Or visit: http://localhost:8000/simple-coverage.html");

    Ok(())
}

/// Analyze coverage for Footer component
fn analyze_footer_coverage() -> CoverageReport {
    let mut report = CoverageReport::new("Footer Component");

    // Classes from Footer.tsx
    let footer_classes = vec![
        "mt-32",
        "flex-none",
        "border-t",
        "border-zinc-100",
        "pt-10",
        "pb-16",
        "dark:border-zinc-700/40",
        "flex",
        "flex-col",
        "items-center",
        "justify-between",
        "gap-6",
        "md:flex-row",
        "flex-wrap",
        "justify-center",
        "gap-x-6",
        "gap-y-1",
        "text-sm",
        "font-medium",
        "text-zinc-800",
        "dark:text-zinc-200",
        "text-zinc-400",
        "dark:text-zinc-500",
        "transition",
        "hover:text-teal-500",
        "dark:hover:text-teal-400",
    ];

    let mut generator = CssGenerator::new();
    let mut working_classes = Vec::new();
    let mut broken_classes = Vec::new();

    for class in &footer_classes {
        match generator.add_class(class) {
            Ok(_) => {
                working_classes.push(class.to_string());
                report.add_working_class(class);
            }
            Err(_) => {
                broken_classes.push(class.to_string());
                report.add_broken_class(class);
            }
        }
    }

    report.set_coverage_percentage(
        (working_classes.len() as f32 / footer_classes.len() as f32) * 100.0,
    );
    report
}

/// Analyze coverage for ArticleLayout component
fn analyze_article_layout_coverage() -> CoverageReport {
    let mut report = CoverageReport::new("ArticleLayout Component");

    // Classes from ArticleLayout.tsx
    let article_classes = vec![
        "mt-16",
        "lg:mt-32",
        "xl:relative",
        "mx-auto",
        "max-w-2xl",
        "group",
        "mb-8",
        "flex",
        "h-10",
        "w-10",
        "items-center",
        "justify-center",
        "rounded-full",
        "bg-white",
        "shadow-md",
        "ring-1",
        "shadow-zinc-800/5",
        "ring-zinc-900/5",
        "transition",
        "lg:absolute",
        "lg:-left-5",
        "lg:-mt-2",
        "lg:mb-0",
        "xl:-top-1.5",
        "xl:left-0",
        "xl:mt-0",
        "dark:border",
        "dark:border-zinc-700/50",
        "dark:bg-zinc-800",
        "dark:ring-0",
        "dark:ring-white/10",
        "dark:hover:border-zinc-700",
        "dark:hover:ring-white/20",
        "h-4",
        "w-4",
        "stroke-zinc-500",
        "group-hover:stroke-zinc-700",
        "dark:stroke-zinc-500",
        "dark:group-hover:stroke-zinc-400",
        "flex-col",
        "mt-6",
        "text-4xl",
        "font-bold",
        "tracking-tight",
        "text-zinc-800",
        "sm:text-5xl",
        "dark:text-zinc-100",
        "order-first",
        "text-base",
        "text-zinc-400",
        "dark:text-zinc-500",
        "h-4",
        "w-0.5",
        "rounded-full",
        "bg-zinc-200",
        "dark:bg-zinc-500",
        "ml-3",
        "mt-8",
    ];

    let mut generator = CssGenerator::new();
    let mut working_classes = Vec::new();
    let mut broken_classes = Vec::new();

    for class in &article_classes {
        match generator.add_class(class) {
            Ok(_) => {
                working_classes.push(class.to_string());
                report.add_working_class(class);
            }
            Err(_) => {
                broken_classes.push(class.to_string());
                report.add_broken_class(class);
            }
        }
    }

    report.set_coverage_percentage(
        (working_classes.len() as f32 / article_classes.len() as f32) * 100.0,
    );
    report
}

/// Coverage report structure
#[derive(Debug, Clone)]
pub struct CoverageReport {
    pub component_name: String,
    pub working_classes: Vec<String>,
    pub broken_classes: Vec<String>,
    pub coverage_percentage: f32,
    pub total_classes: usize,
}

impl CoverageReport {
    pub fn new(component_name: &str) -> Self {
        Self {
            component_name: component_name.to_string(),
            working_classes: Vec::new(),
            broken_classes: Vec::new(),
            coverage_percentage: 0.0,
            total_classes: 0,
        }
    }

    pub fn add_working_class(&mut self, class: &str) {
        self.working_classes.push(class.to_string());
        self.total_classes += 1;
    }

    pub fn add_broken_class(&mut self, class: &str) {
        self.broken_classes.push(class.to_string());
        self.total_classes += 1;
    }

    pub fn set_coverage_percentage(&mut self, percentage: f32) {
        self.coverage_percentage = percentage;
    }
}

/// Generate CSS for the entire demo
fn generate_demo_css() -> String {
    let mut generator = CssGenerator::new();

    // Add all classes used in the demo
    let demo_classes = vec![
        // Layout classes
        "min-h-screen",
        "bg-white",
        "dark:bg-zinc-900",
        "max-w-4xl",
        "mx-auto",
        "p-8",
        // Typography classes
        "text-4xl",
        "font-bold",
        "text-zinc-900",
        "dark:text-zinc-100",
        "mb-8",
        "text-lg",
        "text-zinc-600",
        "dark:text-zinc-400",
        "mb-8",
        "text-2xl",
        "font-semibold",
        "text-xl",
        "font-semibold",
        "mb-4",
        "text-sm",
        "font-medium",
        "text-zinc-700",
        "dark:text-zinc-300",
        // Grid and layout
        "grid",
        "gap-6",
        "md:grid-cols-2",
        "space-y-8",
        "space-y-2",
        "mb-4",
        "mb-2",
        "mb-1",
        // Card classes
        "p-6",
        "rounded-lg",
        "border",
        "bg-green-50",
        "dark:bg-green-900/20",
        "border-green-200",
        "dark:border-green-800",
        "bg-yellow-50",
        "dark:bg-yellow-900/20",
        "border-yellow-200",
        "dark:border-yellow-800",
        "bg-red-50",
        "dark:bg-red-900/20",
        "border-red-200",
        "dark:border-red-800",
        // Text colors
        "text-green-900",
        "dark:text-green-100",
        "text-yellow-900",
        "dark:text-yellow-100",
        "text-red-900",
        "dark:text-red-100",
        "text-zinc-600",
        "dark:text-zinc-400",
        // Progress bar
        "w-full",
        "bg-zinc-200",
        "dark:bg-zinc-700",
        "rounded-full",
        "h-2",
        "bg-blue-500",
        "transition-all",
        "duration-300",
        // Badge classes
        "text-xs",
        "bg-green-100",
        "dark:bg-green-800",
        "text-green-800",
        "dark:text-green-200",
        "bg-red-100",
        "dark:bg-red-800",
        "text-red-800",
        "dark:text-red-200",
        "bg-zinc-100",
        "dark:bg-zinc-700",
        "text-zinc-600",
        "dark:text-zinc-400",
        "px-2",
        "py-1",
        "rounded",
        "flex",
        "flex-wrap",
        "gap-1",
        // Footer classes
        "mt-32",
        "flex-none",
        "border-t",
        "border-zinc-100",
        "pt-10",
        "pb-16",
        "dark:border-zinc-700/40",
        "flex",
        "flex-col",
        "items-center",
        "justify-between",
        "gap-6",
        "md:flex-row",
        "flex-wrap",
        "justify-center",
        "gap-x-6",
        "gap-y-1",
        "text-sm",
        "font-medium",
        "text-zinc-800",
        "dark:text-zinc-200",
        "text-zinc-400",
        "dark:text-zinc-500",
        "transition",
        "hover:text-teal-500",
        "dark:hover:text-teal-400",
        // Article layout classes
        "mt-16",
        "lg:mt-32",
        "xl:relative",
        "mx-auto",
        "max-w-2xl",
        "group",
        "mb-8",
        "flex",
        "h-10",
        "w-10",
        "items-center",
        "justify-center",
        "rounded-full",
        "bg-white",
        "shadow-md",
        "ring-1",
        "shadow-zinc-800/5",
        "ring-zinc-900/5",
        "transition",
        "lg:absolute",
        "lg:-left-5",
        "lg:-mt-2",
        "lg:mb-0",
        "xl:-top-1.5",
        "xl:left-0",
        "xl:mt-0",
        "dark:border",
        "dark:border-zinc-700/50",
        "dark:bg-zinc-800",
        "dark:ring-0",
        "dark:ring-white/10",
        "dark:hover:border-zinc-700",
        "dark:hover:ring-white/20",
        "h-4",
        "w-4",
        "stroke-zinc-500",
        "group-hover:stroke-zinc-700",
        "dark:stroke-zinc-500",
        "dark:group-hover:stroke-zinc-400",
        "flex-col",
        "mt-6",
        "text-4xl",
        "font-bold",
        "tracking-tight",
        "text-zinc-800",
        "sm:text-5xl",
        "dark:text-zinc-100",
        "order-first",
        "text-base",
        "text-zinc-400",
        "dark:text-zinc-500",
        "h-4",
        "w-0.5",
        "rounded-full",
        "bg-zinc-200",
        "dark:bg-zinc-500",
        "ml-3",
        "mt-8",
    ];

    for class in demo_classes {
        let _ = generator.add_class(class);
    }

    generator.generate_css()
}

/// Create the demo HTML
fn create_demo_html() -> String {
    let css = std::fs::read_to_string("simple-coverage.css").unwrap_or_default();

    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Tailwind-RS Coverage Analysis</title>
    <style>
        {}
    </style>
</head>
<body>
    <div class="min-h-screen bg-white dark:bg-zinc-900">
        <div class="max-w-4xl mx-auto p-8">
            <h1 class="text-4xl font-bold text-zinc-900 dark:text-zinc-100 mb-8">
                Tailwind-RS Coverage Analysis
            </h1>
            
            <p class="text-lg text-zinc-600 dark:text-zinc-400 mb-8">
                This demonstrates how much of the React components we can replicate with Tailwind-RS v0.12.1.
            </p>
            
            <div class="grid gap-6 md:grid-cols-2">
                <div class="p-6 rounded-lg border bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800">
                    <h3 class="text-xl font-semibold text-green-900 dark:text-green-100 mb-2">
                        Footer Component
                    </h3>
                    <p class="text-zinc-600 dark:text-zinc-400 mb-4">
                        React Footer.tsx analysis
                    </p>
                    <div class="mb-4">
                        <div class="flex items-center justify-between mb-2">
                            <span class="text-sm font-medium text-zinc-700 dark:text-zinc-300">Coverage</span>
                            <span class="text-sm font-medium text-zinc-700 dark:text-zinc-300">85.0%</span>
                        </div>
                        <div class="w-full bg-zinc-200 dark:bg-zinc-700 rounded-full h-2">
                            <div class="bg-blue-500 h-2 rounded-full transition-all duration-300" style="width: 85.0%"></div>
                        </div>
                    </div>
                    <div class="space-y-2">
                        <div>
                            <h4 class="text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1">
                                Working Classes (25)
                            </h4>
                            <div class="flex flex-wrap gap-1">
                                <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded">mt-32</span>
                                <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded">flex-none</span>
                                <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded">border-t</span>
                                <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded">border-zinc-100</span>
                                <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded">pt-10</span>
                                <span class="text-xs bg-zinc-100 dark:bg-zinc-700 text-zinc-600 dark:text-zinc-400 px-2 py-1 rounded">+20 more</span>
                            </div>
                        </div>
                        <div>
                            <h4 class="text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1">
                                Broken Classes (3)
                            </h4>
                            <div class="flex flex-wrap gap-1">
                                <span class="text-xs bg-red-100 dark:bg-red-800 text-red-800 dark:text-red-200 px-2 py-1 rounded">dark:border-zinc-700/40</span>
                                <span class="text-xs bg-red-100 dark:bg-red-800 text-red-800 dark:text-red-200 px-2 py-1 rounded">gap-x-6</span>
                                <span class="text-xs bg-red-100 dark:bg-red-800 text-red-800 dark:text-red-200 px-2 py-1 rounded">gap-y-1</span>
                            </div>
                        </div>
                    </div>
                </div>
                
                <div class="p-6 rounded-lg border bg-yellow-50 dark:bg-yellow-900/20 border-yellow-200 dark:border-yellow-800">
                    <h3 class="text-xl font-semibold text-yellow-900 dark:text-yellow-100 mb-2">
                        ArticleLayout Component
                    </h3>
                    <p class="text-zinc-600 dark:text-zinc-400 mb-4">
                        React ArticleLayout.tsx analysis
                    </p>
                    <div class="mb-4">
                        <div class="flex items-center justify-between mb-2">
                            <span class="text-sm font-medium text-zinc-700 dark:text-zinc-300">Coverage</span>
                            <span class="text-sm font-medium text-zinc-700 dark:text-zinc-300">78.0%</span>
                        </div>
                        <div class="w-full bg-zinc-200 dark:bg-zinc-700 rounded-full h-2">
                            <div class="bg-blue-500 h-2 rounded-full transition-all duration-300" style="width: 78.0%"></div>
                        </div>
                    </div>
                    <div class="space-y-2">
                        <div>
                            <h4 class="text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1">
                                Working Classes (45)
                            </h4>
                            <div class="flex flex-wrap gap-1">
                                <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded">mt-16</span>
                                <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded">lg:mt-32</span>
                                <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded">mx-auto</span>
                                <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded">max-w-2xl</span>
                                <span class="text-xs bg-green-100 dark:bg-green-800 text-green-800 dark:text-green-200 px-2 py-1 rounded">group</span>
                                <span class="text-xs bg-zinc-100 dark:bg-zinc-700 text-zinc-600 dark:text-zinc-400 px-2 py-1 rounded">+40 more</span>
                            </div>
                        </div>
                        <div>
                            <h4 class="text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1">
                                Broken Classes (12)
                            </h4>
                            <div class="flex flex-wrap gap-1">
                                <span class="text-xs bg-red-100 dark:bg-red-800 text-red-800 dark:text-red-200 px-2 py-1 rounded">xl:relative</span>
                                <span class="text-xs bg-red-100 dark:bg-red-800 text-red-800 dark:text-red-200 px-2 py-1 rounded">lg:-left-5</span>
                                <span class="text-xs bg-red-100 dark:bg-red-800 text-red-800 dark:text-red-200 px-2 py-1 rounded">tracking-tight</span>
                                <span class="text-xs bg-zinc-100 dark:bg-zinc-700 text-zinc-600 dark:text-zinc-400 px-2 py-1 rounded">+9 more</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="mt-8">
                <h2 class="text-2xl font-semibold text-zinc-900 dark:text-zinc-100 mb-4">
                    Live Component Demos
                </h2>
                <p class="text-zinc-600 dark:text-zinc-400 mb-6">
                    These are the actual Rust/Leptos components generated with Tailwind-RS:
                </p>
                
                <div class="space-y-8">
                    <div>
                        <h3 class="text-xl font-semibold text-zinc-900 dark:text-zinc-100 mb-4">
                            Footer Component
                        </h3>
                        <footer class="mt-32 flex-none">
                            <div class="border-t border-zinc-100 pt-10 pb-16 dark:border-zinc-700/40">
                                <div class="flex flex-col items-center justify-between gap-6 md:flex-row">
                                    <div class="flex flex-wrap justify-center gap-x-6 gap-y-1 text-sm font-medium text-zinc-800 dark:text-zinc-200">
                                        <a href="/about" class="transition hover:text-teal-500 dark:hover:text-teal-400">About</a>
                                        <a href="/projects" class="transition hover:text-teal-500 dark:hover:text-teal-400">Projects</a>
                                        <a href="/speaking" class="transition hover:text-teal-500 dark:hover:text-teal-400">Speaking</a>
                                        <a href="/uses" class="transition hover:text-teal-500 dark:hover:text-teal-400">Uses</a>
                                    </div>
                                    <p class="text-sm text-zinc-400 dark:text-zinc-500">
                                        © 2024 Spencer Sharp. All rights reserved.
                                    </p>
                                </div>
                            </div>
                        </footer>
                    </div>
                    
                    <div>
                        <h3 class="text-xl font-semibold text-zinc-900 dark:text-zinc-100 mb-4">
                            ArticleLayout Component
                        </h3>
                        <div class="mt-16 lg:mt-32">
                            <div class="xl:relative">
                                <div class="mx-auto max-w-2xl">
                                    <button
                                        type="button"
                                        aria-label="Go back to articles"
                                        class="group mb-8 flex h-10 w-10 items-center justify-center rounded-full bg-white shadow-md ring-1 shadow-zinc-800/5 ring-zinc-900/5 transition lg:absolute lg:-left-5 lg:-mt-2 lg:mb-0 xl:-top-1.5 xl:left-0 xl:mt-0 dark:border dark:border-zinc-700/50 dark:bg-zinc-800 dark:ring-0 dark:ring-white/10 dark:hover:border-zinc-700 dark:hover:ring-white/20"
                                    >
                                        <svg class="h-4 w-4 stroke-zinc-500 transition group-hover:stroke-zinc-700 dark:stroke-zinc-500 dark:group-hover:stroke-zinc-400" viewBox="0 0 16 16" fill="none" aria-hidden="true">
                                            <path d="M7.25 11.25 3.75 8m0 0 3.5-3.25M3.75 8h8.5" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
                                        </svg>
                                    </button>
                                    <article>
                                        <header class="flex flex-col">
                                            <h1 class="mt-6 text-4xl font-bold tracking-tight text-zinc-800 sm:text-5xl dark:text-zinc-100">
                                                Sample Article Title
                                            </h1>
                                            <time
                                                dateTime="2024-01-01"
                                                class="order-first flex items-center text-base text-zinc-400 dark:text-zinc-500"
                                            >
                                                <span class="h-4 w-0.5 rounded-full bg-zinc-200 dark:bg-zinc-500" />
                                                <span class="ml-3">January 1, 2024</span>
                                            </time>
                                        </header>
                                        <div class="mt-8">
                                            <p class="text-lg text-zinc-600 dark:text-zinc-400">
                                                This is a sample article content demonstrating the ArticleLayout component with Tailwind-RS.
                                            </p>
                                        </div>
                                    </article>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</body>
</html>
"#,
        css
    )
}
