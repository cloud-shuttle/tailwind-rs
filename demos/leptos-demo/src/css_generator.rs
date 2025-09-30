use tailwind_rs_core::css_generator::CssGenerator;
use std::result::Result as StdResult;

/// CSS Generator for Tailwind-RS Core (generates actual CSS)
pub struct DemoCssGenerator {
    generator: CssGenerator,
}

impl DemoCssGenerator {
    pub fn new() -> Self {
        Self {
            generator: CssGenerator::new(),
        }
    }

    pub fn add_class(&mut self, class: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.generator.add_class(class);
        Ok(())
    }

    pub fn generate_css(&self) -> String {
        // Generate actual CSS using Tailwind-RS core
        self.generator.generate_css()
    }

    /// Generate comprehensive classes for the Leptos demo
    pub fn generate_demo_css(&mut self) -> StdResult<String, Box<dyn std::error::Error>> {
        // Add comprehensive utility classes for the demo
        let comprehensive_classes = self.get_comprehensive_classes();

        // Group classes by gradient type
        let mut regular_classes = Vec::new();
        let mut gradient_stop_classes = Vec::new();
        let mut gradient_direction_classes = Vec::new();

        for class in comprehensive_classes {
            // Check if this is a gradient stop class (starts with from-, via-, or to-)
            let is_gradient_stop = class.starts_with("from-") || class.starts_with("via-") || class.starts_with("to-") ||
                                  class.contains(":from-") || class.contains(":via-") || class.contains(":to-");

            if is_gradient_stop {
                gradient_stop_classes.push(class);
            } else if class.contains("bg-gradient") {
                gradient_direction_classes.push(class);
            } else {
                regular_classes.push(class);
            }
        }

        // Process regular classes individually
        for class in regular_classes {
            let _ = self.generator.add_class(&class);
        }

        // Process all gradient classes individually - the CSS variable system handles the connections
        for class in &gradient_stop_classes {
            let _ = self.generator.add_class(class);
        }

        // Process gradient directions individually - they use CSS variables set by gradient stops
        for class in gradient_direction_classes {
            self.generator.add_class(&class)?;
        }

        // Generate actual CSS
        let css = self.generator.generate_css();

        Ok(css)
    }

    /// Get comprehensive classes for the impressive demo - ALL classes that should work
    fn get_comprehensive_classes(&self) -> Vec<String> {
        vec![
            // ============ ANIMATIONS & TRANSITIONS ============
            "animate-spin".to_string(),
            "animate-ping".to_string(),
            "animate-pulse".to_string(),
            "animate-bounce".to_string(),
            "animate-float".to_string(),
            "animate-twinkle".to_string(),
            "animate-rainbow".to_string(),
            "animate-shimmer".to_string(),
            "animate-drift".to_string(),
            "animate-glow".to_string(),
            "transition-all".to_string(),
            "transition-colors".to_string(),
            "transition-transform".to_string(),
            "duration-300".to_string(),
            "duration-500".to_string(),
            "ease-in-out".to_string(),

            // ============ TRANSFORMS ============
            "transform".to_string(),
            "scale-105".to_string(),
            "scale-110".to_string(),
            "scale-x-105".to_string(),
            "scale-y-105".to_string(),
            "rotate-3".to_string(),
            "rotate-12".to_string(),
            "-rotate-3".to_string(),
            "-rotate-12".to_string(),
            "hover:scale-105".to_string(),
            "hover:scale-110".to_string(),
            "hover:rotate-12".to_string(),
            "hover:-rotate-12".to_string(),
            "group-hover:rotate-12".to_string(),

            // ============ ADVANCED BACKGROUNDS & GRADIENTS ============
            "bg-gradient-to-t".to_string(),
            "bg-gradient-to-tr".to_string(),
            "bg-gradient-to-r".to_string(),
            "bg-gradient-to-br".to_string(),
            "bg-gradient-to-b".to_string(),
            "bg-gradient-to-bl".to_string(),
            "bg-gradient-to-l".to_string(),
            "bg-gradient-to-tl".to_string(),

            // Gradient stops - comprehensive
            "from-black".to_string(),
            "from-white".to_string(),
            "from-red-500".to_string(),
            "from-yellow-300".to_string(),
            "from-yellow-400".to_string(),
            "from-green-400".to_string(),
            "from-green-500".to_string(),
            "from-blue-400".to_string(),
            "from-blue-500".to_string(),
            "from-blue-600".to_string(),
            "from-purple-400".to_string(),
            "from-purple-500".to_string(),
            "from-purple-600".to_string(),
            "from-purple-900".to_string(),
            "from-pink-400".to_string(),
            "from-pink-500".to_string(),
            "from-pink-600".to_string(),
            "from-cyan-300".to_string(),
            "from-cyan-400".to_string(),
            "from-cyan-500".to_string(),

            "via-transparent".to_string(),
            "via-black".to_string(),
            "via-white".to_string(),
            "via-red-500".to_string(),
            "via-yellow-400".to_string(),
            "via-green-500".to_string(),
            "via-blue-500".to_string(),
            "via-purple-500".to_string(),
            "from-slate-900".to_string(),
            "via-purple-900".to_string(),
            "to-slate-900".to_string(),
            "via-purple-900/20".to_string(),
            "via-pink-500".to_string(),
            "via-cyan-500".to_string(),

            "to-black".to_string(),
            "to-white".to_string(),
            "to-red-500".to_string(),
            "to-yellow-400".to_string(),
            "to-green-400".to_string(),
            "to-green-500".to_string(),
            "to-blue-300".to_string(),
            "to-blue-500".to_string(),
            "to-blue-600".to_string(),
            "to-purple-600".to_string(),
            "to-purple-700".to_string(),
            "to-pink-500".to_string(),
            "to-pink-600".to_string(),
            "to-cyan-500".to_string(),
            "to-cyan-600".to_string(),
            "to-emerald-500".to_string(),
            "to-violet-500".to_string(),

            // ============ BACKDROP & FILTERS ============
            "backdrop-blur-sm".to_string(),
            "backdrop-blur-md".to_string(),
            "backdrop-blur-lg".to_string(),
            "backdrop-blur-xl".to_string(),

            // ============ ADVANCED COLORS ============
            "text-red-50".to_string(),
            "text-red-100".to_string(),
            "text-red-200".to_string(),
            "text-red-300".to_string(),
            "text-red-400".to_string(),
            "text-red-500".to_string(),
            "text-red-600".to_string(),
            "text-red-700".to_string(),
            "text-red-800".to_string(),
            "text-red-900".to_string(),

            "text-blue-50".to_string(),
            "text-blue-100".to_string(),
            "text-blue-200".to_string(),
            "text-blue-300".to_string(),
            "text-blue-400".to_string(),
            "text-blue-500".to_string(),
            "text-blue-600".to_string(),
            "text-blue-700".to_string(),
            "text-blue-800".to_string(),
            "text-blue-900".to_string(),

            "text-green-50".to_string(),
            "text-green-100".to_string(),
            "text-green-200".to_string(),
            "text-green-300".to_string(),
            "text-green-400".to_string(),
            "text-green-500".to_string(),
            "text-green-600".to_string(),
            "text-green-700".to_string(),
            "text-green-800".to_string(),
            "text-green-900".to_string(),

            "text-purple-50".to_string(),
            "text-purple-100".to_string(),
            "text-purple-200".to_string(),
            "text-purple-300".to_string(),
            "text-purple-400".to_string(),
            "text-purple-500".to_string(),
            "text-purple-600".to_string(),
            "text-purple-700".to_string(),
            "text-purple-800".to_string(),
            "text-purple-900".to_string(),

            "text-pink-400".to_string(),
            "text-pink-500".to_string(),
            "text-cyan-400".to_string(),
            "text-cyan-500".to_string(),
            "text-orange-400".to_string(),
            "text-yellow-400".to_string(),

            // ============ ADVANCED LAYOUT & POSITIONING ============
            "absolute".to_string(),
            "relative".to_string(),
            "fixed".to_string(),
            "sticky".to_string(),
            "top-0".to_string(),
            "left-0".to_string(),
            "right-0".to_string(),
            "bottom-0".to_string(),
            "z-10".to_string(),
            "z-20".to_string(),
            "z-50".to_string(),

            // ============ ADVANCED FLEXBOX ============
            "flex-1".to_string(),
            "flex-shrink-0".to_string(),
            "flex-shrink".to_string(),
            "flex-grow".to_string(),
            "flex-grow-0".to_string(),
            "justify-start".to_string(),
            "justify-end".to_string(),
            "justify-center".to_string(),
            "justify-between".to_string(),
            "justify-around".to_string(),
            "justify-evenly".to_string(),
            "items-start".to_string(),
            "items-end".to_string(),
            "items-center".to_string(),
            "items-baseline".to_string(),
            "items-stretch".to_string(),
            "self-start".to_string(),
            "self-end".to_string(),
            "self-center".to_string(),
            "self-stretch".to_string(),

            // ============ ADVANCED GRID ============
            "grid-cols-1".to_string(),
            "grid-cols-2".to_string(),
            "grid-cols-3".to_string(),
            "grid-cols-4".to_string(),
            "grid-cols-5".to_string(),
            "grid-cols-6".to_string(),
            "grid-cols-12".to_string(),
            "col-span-1".to_string(),
            "col-span-2".to_string(),
            "col-span-3".to_string(),
            "col-span-full".to_string(),
            "row-span-1".to_string(),
            "row-span-2".to_string(),
            "row-span-3".to_string(),

            // ============ ADVANCED SPACING ============
            "space-x-4".to_string(),
            "space-x-6".to_string(),
            "space-x-8".to_string(),
            "space-y-4".to_string(),
            "space-y-6".to_string(),
            "space-y-8".to_string(),
            "space-y-2".to_string(),

            // ============ ADVANCED BORDERS & ROUNDED ============
            "rounded-none".to_string(),
            "rounded-sm".to_string(),
            "rounded".to_string(),
            "rounded-md".to_string(),
            "rounded-lg".to_string(),
            "rounded-xl".to_string(),
            "rounded-2xl".to_string(),
            "rounded-3xl".to_string(),
            "rounded-full".to_string(),
            "border-l-4".to_string(),
            "border-r-4".to_string(),
            "border-t-4".to_string(),
            "border-b-4".to_string(),

            // ============ ADVANCED SHADOWS ============
            "shadow-sm".to_string(),
            "shadow".to_string(),
            "shadow-md".to_string(),
            "shadow-lg".to_string(),
            "shadow-xl".to_string(),
            "shadow-2xl".to_string(),
            "shadow-inner".to_string(),
            "shadow-none".to_string(),
            "shadow-purple-500/25".to_string(),
            "shadow-cyan-500/25".to_string(),
            "shadow-pink-500/25".to_string(),
            "shadow-blue-500/25".to_string(),

            // ============ ADVANCED HOVER STATES ============
            "hover:shadow-xl".to_string(),
            "hover:shadow-2xl".to_string(),
            "hover:shadow-purple-500/25".to_string(),
            "hover:shadow-cyan-500/25".to_string(),
            "hover:shadow-pink-500/25".to_string(),
            "hover:shadow-blue-500/25".to_string(),
            "hover:shadow-red-500/25".to_string(),
            "hover:shadow-gray-500/25".to_string(),
            "hover:border-purple-400/30".to_string(),
            "hover:border-cyan-400/30".to_string(),
            "hover:border-pink-400/30".to_string(),
            "hover:-rotate-1".to_string(),

            // ============ RESPONSIVE BREAKPOINTS ============
            "sm:flex-row".to_string(),
            "sm:grid-cols-2".to_string(),
            "md:grid-cols-3".to_string(),
            "md:grid-cols-4".to_string(),
            "lg:grid-cols-2".to_string(),
            "lg:grid-cols-3".to_string(),
            "lg:grid-cols-4".to_string(),
            "xl:grid-cols-5".to_string(),

            // ============ ADVANCED TYPOGRAPHY ============
            "text-xs".to_string(),
            "text-sm".to_string(),
            "text-base".to_string(),
            "text-lg".to_string(),
            "text-xl".to_string(),
            "text-2xl".to_string(),
            "text-3xl".to_string(),
            "text-4xl".to_string(),
            "text-5xl".to_string(),
            "text-6xl".to_string(),
            "text-7xl".to_string(),
            "text-8xl".to_string(),
            "font-thin".to_string(),
            "font-light".to_string(),
            "font-normal".to_string(),
            "font-medium".to_string(),
            "font-semibold".to_string(),
            "font-bold".to_string(),
            "font-extrabold".to_string(),
            "font-black".to_string(),
            "tracking-tight".to_string(),
            "tracking-normal".to_string(),
            "tracking-wide".to_string(),
            "tracking-wider".to_string(),
            "tracking-widest".to_string(),
            "leading-none".to_string(),
            "leading-tight".to_string(),
            "leading-snug".to_string(),
            "leading-normal".to_string(),
            "leading-relaxed".to_string(),
            "leading-loose".to_string(),

            // ============ ADVANCED UTILITIES ============
            "overflow-hidden".to_string(),
            "overflow-x-hidden".to_string(),
            "overflow-y-hidden".to_string(),
            "cursor-pointer".to_string(),
            "cursor-not-allowed".to_string(),
            "select-none".to_string(),
            "pointer-events-none".to_string(),
            "opacity-0".to_string(),
            "opacity-25".to_string(),
            "opacity-50".to_string(),
            "opacity-75".to_string(),
            "opacity-100".to_string(),

            // ============ EXISTING CLASSES (PRESERVED) ============
            "min-h-screen".to_string(),
            "container".to_string(),
            "mx-auto".to_string(),
            "px-4".to_string(),
            "py-8".to_string(),
            "py-16".to_string(),
            "mb-8".to_string(),
            "mb-4".to_string(),
            "mb-16".to_string(),
            "mb-12".to_string(),
            "mb-6".to_string(),
            "text-center".to_string(),
            "grid".to_string(),
            "gap-8".to_string(),
            "gap-6".to_string(),
            "bg-white/10".to_string(),
            "bg-black/30".to_string(),
            "bg-black/20".to_string(),
            "bg-white".to_string(),
            "bg-gray-100".to_string(),
            "text-white".to_string(),
            "flex".to_string(),
            "flex-col".to_string(),
            "w-2".to_string(),
            "h-2".to_string(),
            "mr-2".to_string(),
            "mb-2".to_string(),
            "p-6".to_string(),
            "p-8".to_string(),
            "px-6".to_string(),
            "py-3".to_string(),
            "py-4".to_string(),
            "px-4".to_string(),
            "px-8".to_string(),
            "hover:bg-gray-100".to_string(),
            "hover:bg-white/10".to_string(),
            "font-mono".to_string(),
            "list-disc".to_string(),
            "list-inside".to_string(),
            "bg-clip-text".to_string(),
            "text-transparent".to_string(),
            "max-w-2xl".to_string(),
            "inline-flex".to_string(),
            "flex-wrap".to_string(),
            "aspect-square".to_string(),
            "flex-shrink-0".to_string(),
        ]
    }

}

impl Default for DemoCssGenerator {
    fn default() -> Self {
        Self::new()
    }
}

