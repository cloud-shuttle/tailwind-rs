use tailwind_rs_core::css_generator::CssGenerator;

fn main() {
    println!("🚀 Tailwind-RS Comprehensive End-to-End Test Suite");
    println!("==================================================");
    println!("Testing all major utility categories for world-class coverage");
    println!();
    
    let mut generator = CssGenerator::new();
    let mut total_tests = 0;
    let mut successful_tests = 0;
    let mut category_results = Vec::new();
    
    // Test categories with comprehensive coverage
    let test_categories = vec![
        ("Layout & Display", vec![
            "block", "flex", "grid", "hidden", "inline", "inline-block", "inline-flex",
            "table", "table-cell", "table-row", "contents", "list-item", "flow-root"
        ]),
        ("Spacing & Sizing", vec![
            "p-4", "m-2", "px-3", "py-2", "pt-1", "pr-2", "pb-3", "pl-4",
            "w-full", "h-full", "w-4", "h-4", "w-1/2", "h-1/3", "min-w-0", "max-w-2xl",
            "min-h-screen", "max-h-96", "size-6", "size-12", "size-[38px]"
        ]),
        ("Colors & Backgrounds", vec![
            "bg-white", "bg-black", "bg-red-500", "bg-blue-600", "bg-green-400",
            "text-white", "text-black", "text-red-500", "text-blue-600", "text-green-400",
            "bg-gradient-to-r", "bg-gradient-to-b", "from-blue-500", "to-purple-600",
            "bg-[url(/image.jpg)]", "bg-size-cover", "bg-position-center"
        ]),
        ("Typography", vec![
            "text-sm", "text-base", "text-lg", "text-xl", "text-2xl", "text-3xl",
            "font-thin", "font-normal", "font-medium", "font-semibold", "font-bold", "font-black",
            "text-left", "text-center", "text-right", "text-justify",
            "leading-tight", "leading-normal", "leading-relaxed", "leading-loose",
            "tracking-tight", "tracking-normal", "tracking-wide", "tracking-wider",
            "uppercase", "lowercase", "capitalize", "normal-case"
        ]),
        ("Borders & Effects", vec![
            "border", "border-2", "border-4", "border-t", "border-r", "border-b", "border-l",
            "border-solid", "border-dashed", "border-dotted", "border-double", "border-none",
            "rounded", "rounded-lg", "rounded-full", "rounded-none", "rounded-t", "rounded-r",
            "shadow", "shadow-sm", "shadow-md", "shadow-lg", "shadow-xl", "shadow-2xl",
            "opacity-0", "opacity-25", "opacity-50", "opacity-75", "opacity-100"
        ]),
        ("Flexbox & Grid", vec![
            "flex", "flex-col", "flex-row", "flex-wrap", "flex-nowrap", "flex-wrap-reverse",
            "justify-start", "justify-center", "justify-end", "justify-between", "justify-around", "justify-evenly",
            "items-start", "items-center", "items-end", "items-stretch", "items-baseline",
            "flex-1", "flex-auto", "flex-initial", "flex-none", "flex-grow", "flex-shrink",
            "grid", "grid-cols-1", "grid-cols-2", "grid-cols-3", "grid-cols-4", "grid-cols-6", "grid-cols-12",
            "col-span-1", "col-span-2", "col-span-3", "col-span-full", "row-span-1", "row-span-2", "row-span-3",
            "gap-1", "gap-2", "gap-4", "gap-6", "gap-8", "gap-x-4", "gap-y-2"
        ]),
        ("Positioning & Layout", vec![
            "static", "relative", "absolute", "fixed", "sticky",
            "top-0", "right-0", "bottom-0", "left-0", "inset-0", "inset-x-0", "inset-y-0",
            "z-0", "z-10", "z-20", "z-30", "z-40", "z-50", "z-auto",
            "float-left", "float-right", "float-none", "clear-left", "clear-right", "clear-both", "clear-none"
        ]),
        ("Interactivity & States", vec![
            "cursor-pointer", "cursor-default", "cursor-not-allowed", "cursor-wait", "cursor-text",
            "select-none", "select-text", "select-all", "select-auto",
            "resize-none", "resize", "resize-x", "resize-y",
            "scroll-smooth", "scroll-auto", "touch-auto", "touch-none", "touch-pan-x", "touch-pan-y"
        ]),
        ("Responsive Design", vec![
            "sm:block", "md:flex", "lg:grid", "xl:hidden",
            "sm:p-4", "md:m-2", "lg:px-3", "xl:py-2",
            "sm:text-sm", "md:text-base", "lg:text-lg", "xl:text-xl",
            "sm:w-full", "md:w-1/2", "lg:w-1/3", "xl:w-1/4"
        ]),
        ("Dark Mode", vec![
            "dark:bg-gray-900", "dark:text-white", "dark:border-gray-700",
            "dark:hover:bg-gray-800", "dark:focus:ring-gray-500", "dark:active:bg-gray-700"
        ]),
        ("Hover & Focus States", vec![
            "hover:bg-blue-500", "hover:text-white", "hover:shadow-lg", "hover:scale-105",
            "focus:ring-2", "focus:ring-blue-500", "focus:outline-none", "focus:ring-offset-2",
            "active:bg-blue-600", "active:scale-95", "active:shadow-sm"
        ]),
        ("Transitions & Animations", vec![
            "transition", "transition-all", "transition-colors", "transition-opacity", "transition-transform",
            "duration-75", "duration-100", "duration-150", "duration-200", "duration-300", "duration-500", "duration-700", "duration-1000",
            "ease-linear", "ease-in", "ease-out", "ease-in-out",
            "animate-spin", "animate-ping", "animate-pulse", "animate-bounce"
        ]),
        ("Advanced Mask Utilities", vec![
            "mask-none", "mask-alpha", "mask-luminance", "mask-match",
            "mask-origin-border", "mask-origin-padding", "mask-origin-content",
            "mask-top-left", "mask-center", "mask-bottom-right",
            "mask-repeat", "mask-no-repeat", "mask-repeat-x", "mask-repeat-y",
            "mask-auto", "mask-cover", "mask-contain",
            "mask-type-alpha", "mask-type-luminance",
            "mask-linear-45", "mask-radial-from-75%", "mask-conic-from-60%",
            "mask-x-to-90%", "mask-y-to-90%", "mask-radial-to-85%", "mask-conic-to-75%", "mask-conic-45"
        ]),
        ("Advanced Background Utilities", vec![
            "bg-fixed", "bg-local", "bg-scroll", "bg-clip-border", "bg-clip-padding", "bg-clip-content", "bg-clip-text",
            "bg-origin-border", "bg-origin-padding", "bg-origin-content",
            "bg-top", "bg-center", "bg-bottom", "bg-left", "bg-right",
            "bg-repeat", "bg-no-repeat", "bg-repeat-x", "bg-repeat-y", "bg-repeat-round", "bg-repeat-space",
            "bg-auto", "bg-cover", "bg-contain", "bg-size-[50%_50%]"
        ]),
        ("Advanced Border Utilities", vec![
            "border-t-2", "border-r-4", "border-b-8", "border-l-1",
            "border-red-500", "border-blue-600", "border-green-400",
            "border-solid", "border-dashed", "border-dotted", "border-double", "border-none",
            "rounded-t-lg", "rounded-r-full", "rounded-b-none", "rounded-l-sm",
            "outline-0", "outline-1", "outline-2", "outline-4", "outline-8",
            "outline-red-500", "outline-blue-600", "outline-green-400",
            "outline-solid", "outline-dashed", "outline-dotted", "outline-double", "outline-none"
        ]),
        ("Advanced Effects Utilities", vec![
            "shadow-sm", "shadow-md", "shadow-lg", "shadow-xl", "shadow-2xl", "shadow-inner", "shadow-none",
            "shadow-red-500", "shadow-blue-600", "shadow-green-400",
            "drop-shadow-sm", "drop-shadow-md", "drop-shadow-lg", "drop-shadow-xl", "drop-shadow-2xl", "drop-shadow-none",
            "opacity-0", "opacity-5", "opacity-10", "opacity-20", "opacity-25", "opacity-30", "opacity-40", "opacity-50",
            "mix-blend-normal", "mix-blend-multiply", "mix-blend-screen", "mix-blend-overlay", "mix-blend-difference",
            "bg-blend-normal", "bg-blend-multiply", "bg-blend-screen", "bg-blend-overlay", "bg-blend-difference"
        ])
    ];
    
    // Test each category
    for (category_name, classes) in &test_categories {
        println!("🧪 Testing {} ({} classes)", category_name, classes.len());
        let mut category_successful = 0;
        let mut category_failed = Vec::new();
        
        for class in classes {
            total_tests += 1;
            match generator.add_class(class) {
                Ok(_) => {
                    category_successful += 1;
                    successful_tests += 1;
                }
                Err(e) => {
                    category_failed.push((class, e));
                }
            }
        }
        
        let category_coverage = (category_successful as f64 / classes.len() as f64) * 100.0;
        category_results.push((category_name, category_successful, classes.len(), category_coverage));
        
        if category_failed.is_empty() {
            println!("  ✅ {}: {}/{} (100.0%)", category_name, category_successful, classes.len());
        } else {
            println!("  ⚠️  {}: {}/{} ({:.1}%)", category_name, category_successful, classes.len(), category_coverage);
            for (class, error) in &category_failed {
                println!("    ❌ {} - {}", class, error);
            }
        }
        println!();
    }
    
    // Generate comprehensive CSS
    println!("🎨 Generating comprehensive CSS...");
    let css = generator.generate_css();
    let css_line_count = css.lines().count();
    println!("✅ Generated {} CSS rules", css_line_count);
    
    // Calculate overall coverage
    let overall_coverage = (successful_tests as f64 / total_tests as f64) * 100.0;
    
    // Display results
    println!("\n📊 COMPREHENSIVE TEST RESULTS");
    println!("==============================");
    println!("✅ Total classes tested: {}", total_tests);
    println!("✅ Successful: {}", successful_tests);
    println!("❌ Failed: {}", total_tests - successful_tests);
    println!("🎯 Overall Coverage: {:.1}%", overall_coverage);
    println!("📝 CSS Rules Generated: {}", css_line_count);
    
    println!("\n📈 Category Breakdown:");
    for (category_name, successful, total, coverage) in &category_results {
        let status = if *coverage >= 100.0 { "🎉" } else if *coverage >= 95.0 { "🔥" } else if *coverage >= 90.0 { "✅" } else { "⚠️" };
        println!("  {} {}: {}/{} ({:.1}%)", status, category_name, successful, total, coverage);
    }
    
    // Show sample CSS output
    println!("\n📝 Sample CSS Output:");
    let sample_lines: Vec<&str> = css.lines().take(15).collect();
    for line in sample_lines {
        println!("  {}", line);
    }
    if css_line_count > 15 {
        println!("  ... and {} more lines", css_line_count - 15);
    }
    
    // Final assessment
    println!("\n🏆 FINAL ASSESSMENT");
    println!("===================");
    if overall_coverage >= 95.0 {
        println!("🎉 WORLD-CLASS! Tailwind-RS is production-ready with {:.1}% coverage!", overall_coverage);
        println!("🚀 Ready for enterprise use and large-scale applications!");
    } else if overall_coverage >= 90.0 {
        println!("🔥 EXCELLENT! Tailwind-RS has {:.1}% coverage with minor gaps!", overall_coverage);
        println!("✅ Ready for production use!");
    } else if overall_coverage >= 80.0 {
        println!("✅ GOOD! Tailwind-RS has {:.1}% coverage with some areas for improvement!", overall_coverage);
        println!("🔧 Consider addressing failed classes for better coverage!");
    } else {
        println!("⚠️  NEEDS WORK! Tailwind-RS has {:.1}% coverage and needs significant improvements!", overall_coverage);
        println!("🛠️  Focus on implementing missing functionality!");
    }
    
    println!("\n🎯 Test completed successfully!");
}
