#!/bin/bash

echo "🚀 Building Tailwind-RS v0.15.0 Leptos Demo..."

# Create CSS output directory
mkdir -p css-output

# Generate CSS files using Tailwind-RS v0.15.0
echo "📦 Generating CSS with Tailwind-RS v0.15.0..."

# Create a simple CSS generation script
cat > generate_css.rs << 'EOF'
use tailwind_rs_core::*;
use tailwind_rs_postcss::*;
use tailwind_rs_scanner::*;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Generating comprehensive CSS with Tailwind-RS v0.15.0...");
    
    // Create CSS generator
    let mut generator = CssGenerator::new();
    let mut scanner = ClassScanner::new();
    let mut postcss_engine = PostCssEngine::new();
    
    // Scan for classes in source files
    let source_dir = Path::new("src");
    let classes = scanner.scan_directory(source_dir)?;
    
    // Add classes to the generator
    for class in &classes {
        if let Err(e) = generator.add_class(class) {
            eprintln!("Warning: Failed to add class '{}': {}", class, e);
        }
    }

    // Add comprehensive utility classes for v0.15.0 features
    let comprehensive_classes = vec![
        // Layout & Display
        "block", "inline-block", "flex", "grid", "hidden", "list-item",
        
        // Spacing
        "p-0", "p-1", "p-2", "p-4", "p-6", "p-8",
        "m-0", "m-1", "m-2", "m-4", "m-6", "m-8",
        "px-2", "px-4", "px-6", "py-2", "py-4", "py-6",
        "mx-auto", "my-2", "my-4",
        
        // Colors
        "bg-white", "bg-gray-100", "bg-gray-800", "bg-gray-900",
        "bg-blue-500", "bg-blue-600", "bg-red-500", "bg-red-600",
        "text-white", "text-gray-800", "text-gray-600", "text-gray-400",
        "text-blue-600", "text-blue-400",
        
        // Typography
        "text-4xl", "text-2xl", "text-xl", "text-sm", "text-6xl",
        "font-bold", "font-semibold", "font-medium",
        "text-center", "text-left", "text-right",
        
        // Borders & Effects
        "rounded-lg", "rounded-2xl", "shadow-lg", "shadow-xl",
        "border", "border-gray-300", "border-gray-600",
        
        // Flexbox
        "flex", "flex-col", "flex-row", "items-center", "justify-center",
        "justify-between", "gap-2", "gap-4",
        
        // Sizing
        "w-full", "w-1/2", "w-1/3", "h-screen", "min-h-screen",
        "max-w-md", "max-w-lg", "max-w-xl",
        
        // Interactive
        "hover:bg-blue-600", "hover:bg-red-600", "hover:bg-gray-600",
        "transition-colors", "cursor-pointer",
        
        // Dark mode
        "dark:bg-gray-800", "dark:bg-gray-700", "dark:text-white",
        "dark:text-gray-300", "dark:text-gray-400", "dark:border-gray-600",
        
        // Responsive
        "sm:text-lg", "md:text-xl", "lg:text-2xl",
        "sm:p-4", "md:p-6", "lg:p-8",
        
        // v0.15.0 New Features - Filter Utilities
        "blur-sm", "blur-md", "blur-lg", "brightness-50", "brightness-75",
        "contrast-50", "contrast-75", "grayscale", "hue-rotate-90",
        "invert", "saturate-50", "saturate-75", "sepia",
        "backdrop-blur-sm", "backdrop-blur-md", "backdrop-brightness-50",
        
        // v0.15.0 New Features - Table Utilities
        "table-auto", "table-fixed", "border-collapse", "border-separate",
        "border-spacing-0", "border-spacing-1", "border-spacing-2",
        "caption-top", "caption-bottom",
        
        // v0.15.0 New Features - SVG Utilities
        "fill-none", "fill-current", "fill-red-500", "fill-blue-600",
        "stroke-none", "stroke-current", "stroke-red-500", "stroke-blue-600",
        "stroke-width-1", "stroke-width-2", "stroke-width-4",
        
        // v0.15.0 New Features - Accessibility
        "forced-color-adjust-auto", "forced-color-adjust-none",
        
        // v0.15.0 New Features - Enhanced Transforms
        "transform", "transform-cpu", "transform-gpu",
        "scale-75", "scale-90", "scale-95", "scale-100", "scale-105", "scale-110", "scale-125",
        "rotate-0", "rotate-1", "rotate-2", "rotate-3", "rotate-6", "rotate-12", "rotate-45", "rotate-90", "rotate-180",
        "skew-x-0", "skew-x-1", "skew-x-2", "skew-x-3", "skew-x-6", "skew-x-12",
        "skew-y-0", "skew-y-1", "skew-y-2", "skew-y-3", "skew-y-6", "skew-y-12",
        "perspective-1000", "perspective-1500", "perspective-2000",
        "backface-visible", "backface-hidden",
        "transform-style-preserve-3d", "transform-style-flat",
        
        // v0.15.0 New Features - Touch Actions
        "touch-auto", "touch-none", "touch-pan-x", "touch-pan-left", "touch-pan-right",
        "touch-pan-y", "touch-pan-up", "touch-pan-down", "touch-pinch-zoom",
        "touch-manipulation",
        
        // v0.15.0 New Features - Enhanced Borders
        "rounded-t", "rounded-r", "rounded-b", "rounded-l",
        "rounded-tl", "rounded-tr", "rounded-br", "rounded-bl",
        "rounded-tl-lg", "rounded-tr-lg", "rounded-br-lg", "rounded-bl-lg",
        
        // v0.15.0 New Features - Background Utilities
        "bg-gradient-to-r", "bg-gradient-to-l", "bg-gradient-to-t", "bg-gradient-to-b",
        "bg-gradient-to-tr", "bg-gradient-to-tl", "bg-gradient-to-br", "bg-gradient-to-bl",
        "bg-size-auto", "bg-size-cover", "bg-size-contain",
        "bg-position-center", "bg-position-top", "bg-position-bottom", "bg-position-left", "bg-position-right",
    ];
    
    for class in comprehensive_classes {
        if let Err(e) = generator.add_class(&class) {
            eprintln!("Warning: Failed to add comprehensive class '{}': {}", class, e);
        }
    }

    // Generate CSS
    let css = generator.generate_css();
    
    // Process with PostCSS for optimization
    let processed_css = postcss_engine.process_css(&css)?;
    
    // Save CSS files
    fs::write("comprehensive-styles.css", &processed_css)?;
    fs::write("custom-styles.css", &processed_css)?;
    fs::write("generated-styles.css", &processed_css)?;
    
    println!("✅ CSS files generated successfully!");
    println!("   - comprehensive-styles.css");
    println!("   - custom-styles.css");
    println!("   - generated-styles.css");
    
    Ok(())
}
EOF

# Run the CSS generation
rustc generate_css.rs --extern tailwind_rs_core --extern tailwind_rs_postcss --extern tailwind_rs_scanner
./generate_css

# Clean up
rm generate_css.rs generate_css

echo "✅ CSS generation complete!"

# Build the WASM module
echo "🔨 Building WASM module..."
wasm-pack build --target web --out-dir pkg --dev

echo "🎉 Build complete! CSS files generated and WASM module built."
echo "📁 Generated files:"
echo "   - comprehensive-styles.css"
echo "   - custom-styles.css" 
echo "   - generated-styles.css"
echo "   - pkg/ (WASM module)"
