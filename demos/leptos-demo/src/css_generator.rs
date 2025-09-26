use tailwind_rs_core::*;
use tailwind_rs_postcss::*;
use tailwind_rs_scanner::*;
use std::fs;
use std::path::Path;

/// CSS Generator for Tailwind-RS v0.15.0
pub struct DemoCssGenerator {
    generator: CssGenerator,
    scanner: ClassScanner,
    postcss_engine: PostCSSEngine,
}

impl DemoCssGenerator {
    pub fn new() -> Self {
        Self {
            generator: CssGenerator::new(),
            scanner: ClassScanner::new(),
            postcss_engine: PostCSSEngine::new(),
        }
    }

    /// Generate comprehensive CSS for the Leptos demo
    pub fn generate_demo_css(&mut self) -> std::result::Result<String, Box<dyn std::error::Error>> {
        // Scan for classes in the source files
        let source_dir = Path::new("src");
        let classes = self.scanner.scan_directory(source_dir)?;
        
        // Add classes to the generator
        for class in &classes {
            if let Err(e) = self.generator.add_class(class) {
                eprintln!("Warning: Failed to add class '{}': {}", class, e);
            }
        }

        // Add comprehensive utility classes for v0.15.0 features
        let comprehensive_classes = self.get_comprehensive_classes();
        for class in comprehensive_classes {
            if let Err(e) = self.generator.add_class(&class) {
                eprintln!("Warning: Failed to add comprehensive class '{}': {}", class, e);
            }
        }

        // Generate CSS
        let css = self.generator.generate_css();
        
        // Process with PostCSS for optimization
        let processed_css = self.postcss_engine.process_css(&css)?;
        
        Ok(processed_css)
    }

    /// Get comprehensive classes for v0.15.0 features
    fn get_comprehensive_classes(&self) -> Vec<String> {
        vec![
            // Layout & Display
            "block".to_string(), "inline-block".to_string(), "flex".to_string(), "grid".to_string(), "hidden".to_string(), "list-item".to_string(),
            
            // Spacing
            "p-0".to_string(), "p-1".to_string(), "p-2".to_string(), "p-4".to_string(), "p-6".to_string(), "p-8".to_string(),
            "m-0".to_string(), "m-1".to_string(), "m-2".to_string(), "m-4".to_string(), "m-6".to_string(), "m-8".to_string(),
            "px-2".to_string(), "px-4".to_string(), "px-6".to_string(), "py-2".to_string(), "py-4".to_string(), "py-6".to_string(),
            "mx-auto".to_string(), "my-2".to_string(), "my-4".to_string(),
            
            // Colors
            "bg-white".to_string(), "bg-gray-100".to_string(), "bg-gray-800".to_string(), "bg-gray-900".to_string(),
            "bg-blue-500".to_string(), "bg-blue-600".to_string(), "bg-red-500".to_string(), "bg-red-600".to_string(),
            "text-white".to_string(), "text-gray-800".to_string(), "text-gray-600".to_string(), "text-gray-400".to_string(),
            "text-blue-600".to_string(), "text-blue-400".to_string(),
            
            // Typography
            "text-4xl".to_string(), "text-2xl".to_string(), "text-xl".to_string(), "text-sm".to_string(), "text-6xl".to_string(),
            "font-bold".to_string(), "font-semibold".to_string(), "font-medium".to_string(),
            "text-center".to_string(), "text-left".to_string(), "text-right".to_string(),
            
            // Borders & Effects
            "rounded-lg".to_string(), "rounded-2xl".to_string(), "shadow-lg".to_string(), "shadow-xl".to_string(),
            "border".to_string(), "border-gray-300".to_string(), "border-gray-600".to_string(),
            
            // Flexbox
            "flex".to_string(), "flex-col".to_string(), "flex-row".to_string(), "items-center".to_string(), "justify-center".to_string(),
            "justify-between".to_string(), "gap-2".to_string(), "gap-4".to_string(),
            
            // Sizing
            "w-full".to_string(), "w-1/2".to_string(), "w-1/3".to_string(), "h-screen".to_string(), "min-h-screen".to_string(),
            "max-w-md".to_string(), "max-w-lg".to_string(), "max-w-xl".to_string(),
            
            // Interactive
            "hover:bg-blue-600".to_string(), "hover:bg-red-600".to_string(), "hover:bg-gray-600".to_string(),
            "transition-colors".to_string(), "cursor-pointer".to_string(),
            
            // Dark mode
            "dark:bg-gray-800".to_string(), "dark:bg-gray-700".to_string(), "dark:text-white".to_string(),
            "dark:text-gray-300".to_string(), "dark:text-gray-400".to_string(), "dark:border-gray-600".to_string(),
            
            // Responsive
            "sm:text-lg".to_string(), "md:text-xl".to_string(), "lg:text-2xl".to_string(),
            "sm:p-4".to_string(), "md:p-6".to_string(), "lg:p-8".to_string(),
            
            // v0.15.0 New Features - Filter Utilities
            "blur-sm".to_string(), "blur-md".to_string(), "blur-lg".to_string(), "brightness-50".to_string(), "brightness-75".to_string(),
            "contrast-50".to_string(), "contrast-75".to_string(), "grayscale".to_string(), "hue-rotate-90".to_string(),
            "invert".to_string(), "saturate-50".to_string(), "saturate-75".to_string(), "sepia".to_string(),
            "backdrop-blur-sm".to_string(), "backdrop-blur-md".to_string(), "backdrop-brightness-50".to_string(),
            
            // v0.15.0 New Features - Table Utilities
            "table-auto".to_string(), "table-fixed".to_string(), "border-collapse".to_string(), "border-separate".to_string(),
            "border-spacing-0".to_string(), "border-spacing-1".to_string(), "border-spacing-2".to_string(),
            "caption-top".to_string(), "caption-bottom".to_string(),
            
            // v0.15.0 New Features - SVG Utilities
            "fill-none".to_string(), "fill-current".to_string(), "fill-red-500".to_string(), "fill-blue-600".to_string(),
            "stroke-none".to_string(), "stroke-current".to_string(), "stroke-red-500".to_string(), "stroke-blue-600".to_string(),
            "stroke-width-1".to_string(), "stroke-width-2".to_string(), "stroke-width-4".to_string(),
            
            // v0.15.0 New Features - Accessibility
            "forced-color-adjust-auto".to_string(), "forced-color-adjust-none".to_string(),
            
            // v0.15.0 New Features - Enhanced Transforms
            "transform".to_string(), "transform-cpu".to_string(), "transform-gpu".to_string(),
            "scale-75".to_string(), "scale-90".to_string(), "scale-95".to_string(), "scale-100".to_string(), "scale-105".to_string(), "scale-110".to_string(), "scale-125".to_string(),
            "rotate-0".to_string(), "rotate-1".to_string(), "rotate-2".to_string(), "rotate-3".to_string(), "rotate-6".to_string(), "rotate-12".to_string(), "rotate-45".to_string(), "rotate-90".to_string(), "rotate-180".to_string(),
            "skew-x-0".to_string(), "skew-x-1".to_string(), "skew-x-2".to_string(), "skew-x-3".to_string(), "skew-x-6".to_string(), "skew-x-12".to_string(),
            "skew-y-0".to_string(), "skew-y-1".to_string(), "skew-y-2".to_string(), "skew-y-3".to_string(), "skew-y-6".to_string(), "skew-y-12".to_string(),
            "perspective-1000".to_string(), "perspective-1500".to_string(), "perspective-2000".to_string(),
            "backface-visible".to_string(), "backface-hidden".to_string(),
            "transform-style-preserve-3d".to_string(), "transform-style-flat".to_string(),
            
            // v0.15.0 New Features - Touch Actions
            "touch-auto".to_string(), "touch-none".to_string(), "touch-pan-x".to_string(), "touch-pan-left".to_string(), "touch-pan-right".to_string(),
            "touch-pan-y".to_string(), "touch-pan-up".to_string(), "touch-pan-down".to_string(), "touch-pinch-zoom".to_string(),
            "touch-manipulation".to_string(),
            
            // v0.15.0 New Features - Enhanced Borders
            "rounded-t".to_string(), "rounded-r".to_string(), "rounded-b".to_string(), "rounded-l".to_string(),
            "rounded-tl".to_string(), "rounded-tr".to_string(), "rounded-br".to_string(), "rounded-bl".to_string(),
            "rounded-tl-lg".to_string(), "rounded-tr-lg".to_string(), "rounded-br-lg".to_string(), "rounded-bl-lg".to_string(),
            
            // v0.15.0 New Features - Background Utilities
            "bg-gradient-to-r".to_string(), "bg-gradient-to-l".to_string(), "bg-gradient-to-t".to_string(), "bg-gradient-to-b".to_string(),
            "bg-gradient-to-tr".to_string(), "bg-gradient-to-tl".to_string(), "bg-gradient-to-br".to_string(), "bg-gradient-to-bl".to_string(),
            "bg-size-auto".to_string(), "bg-size-cover".to_string(), "bg-size-contain".to_string(),
            "bg-position-center".to_string(), "bg-position-top".to_string(), "bg-position-bottom".to_string(), "bg-position-left".to_string(), "bg-position-right".to_string(),
        ]
    }

    /// Save CSS to file
    pub fn save_css_to_file(&mut self, file_path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let css = self.generate_demo_css()?;
        fs::write(file_path, css)?;
        println!("✅ CSS generated and saved to: {}", file_path);
        Ok(())
    }

    /// Generate multiple CSS files for different purposes
    pub fn generate_all_css_files(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Generate comprehensive styles
        self.save_css_to_file("comprehensive-styles.css")?;
        
        // Generate custom styles
        self.save_css_to_file("custom-styles.css")?;
        
        // Generate main styles
        self.save_css_to_file("generated-styles.css")?;
        
        Ok(())
    }
}

impl Default for DemoCssGenerator {
    fn default() -> Self {
        Self::new()
    }
}
