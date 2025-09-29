use tailwind_rs_core::css_generator::CssGenerator;

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
    pub fn generate_demo_css(&mut self) -> std::result::Result<String, Box<dyn std::error::Error>> {
        // Add comprehensive utility classes for the demo
        let comprehensive_classes = self.get_comprehensive_classes();
        for class in comprehensive_classes {
            self.generator.add_class(&class);
        }

        // Generate actual CSS
        let css = self.generator.generate_css();

        Ok(css)
    }

    /// Get comprehensive classes for the impressive demo - ALL classes that should work
    fn get_comprehensive_classes(&self) -> Vec<String> {
        vec![
            // Core layout and positioning
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

            // Backgrounds and gradients
            "bg-gradient-to-br".to_string(),
            "from-purple-600".to_string(),
            "to-blue-600".to_string(),
            "bg-white/10".to_string(),
            "bg-black/30".to_string(),
            "bg-black/20".to_string(),
            "backdrop-blur-sm".to_string(),
            "bg-white".to_string(),
            "bg-gray-100".to_string(),

            // Typography
            "text-5xl".to_string(),
            "text-xl".to_string(),
            "text-2xl".to_string(),
            "text-3xl".to_string(),
            "text-lg".to_string(),
            "text-sm".to_string(),
            "font-bold".to_string(),
            "font-semibold".to_string(),
            "opacity-90".to_string(),
            "opacity-80".to_string(),
            "text-white".to_string(),
            "text-purple-600".to_string(),
            "text-green-300".to_string(),
            "text-blue-300".to_string(),

            // Responsive classes - base classes
            "md:grid-cols-3".to_string(),
            "sm:flex-row".to_string(),
            "lg:grid-cols-2".to_string(),
            "md:grid-cols-4".to_string(),
            // Also add the base responsive classes
            "grid-cols-1".to_string(),
            "grid-cols-2".to_string(),
            "grid-cols-3".to_string(),

            // Borders and shadows
            "rounded-lg".to_string(),
            "rounded-xl".to_string(),
            "border".to_string(),
            "border-2".to_string(),
            "border-white/20".to_string(),
            "border-white/30".to_string(),
            "border-white/10".to_string(),

            // Flexbox and alignment
            "flex".to_string(),
            "flex-col".to_string(),
            "items-center".to_string(),
            "justify-center".to_string(),
            "w-2".to_string(),
            "h-2".to_string(),
            "mr-2".to_string(),
            "mb-2".to_string(),
            "mb-4".to_string(),
            "mb-6".to_string(),

            // Spacing
            "p-6".to_string(),
            "p-8".to_string(),
            "px-6".to_string(),
            "py-3".to_string(),
            "py-4".to_string(),
            "px-4".to_string(),

            // Interactive states
            "hover:bg-gray-100".to_string(),
            "hover:bg-white/10".to_string(),

            // Grid columns
            "grid-cols-1".to_string(),
            "grid-cols-2".to_string(),
            "grid-cols-3".to_string(),
            "grid-cols-4".to_string(),

            // Font styling
            "font-mono".to_string(),

            // Additional spacing for the new demo
            "space-y-2".to_string(),

            // List styling
            "list-disc".to_string(),
            "list-inside".to_string(),

            // More comprehensive classes for the spectacular demo
            "min-h-screen".to_string(),
            "text-6xl".to_string(),
            "bg-gradient-to-r".to_string(),
            "from-yellow-300".to_string(),
            "via-pink-500".to_string(),
            "to-blue-600".to_string(),
            "bg-clip-text".to_string(),
            "text-transparent".to_string(),
            "max-w-2xl".to_string(),
            "rounded-full".to_string(),
            "px-8".to_string(),
            "py-3".to_string(),
            "transition-all".to_string(),
            "duration-300".to_string(),
            "hover:scale-105".to_string(),
            "backdrop-blur-lg".to_string(),
            "rounded-2xl".to_string(),
            "hover:border-white/40".to_string(),
            "hover:shadow-2xl".to_string(),
            "text-4xl".to_string(),
            "leading-relaxed".to_string(),
            "backdrop-blur-xl".to_string(),
            "rounded-3xl".to_string(),
            "bg-gradient-to-br".to_string(),
            "from-red-500".to_string(),
            "to-pink-500".to_string(),
            "from-blue-500".to_string(),
            "to-cyan-500".to_string(),
            "from-green-500".to_string(),
            "to-emerald-500".to_string(),
            "from-purple-500".to_string(),
            "to-violet-500".to_string(),
            "text-red-400".to_string(),
            "text-blue-400".to_string(),
            "text-green-400".to_string(),
            "text-yellow-400".to_string(),
            "text-pink-400".to_string(),
            "text-cyan-400".to_string(),
            "text-orange-400".to_string(),
            "text-red-100".to_string(),
            "text-blue-100".to_string(),
            "text-green-100".to_string(),
            "text-purple-100".to_string(),
            "inline-flex".to_string(),
            "flex-wrap".to_string(),
            "lg:grid-cols-2".to_string(),
            "text-xs".to_string(),
            "text-base".to_string(),
            "text-purple-300".to_string(),
            "aspect-square".to_string(),
            "flex-shrink-0".to_string(),
            "from-cyan-300".to_string(),
            "to-blue-300".to_string(),
            "from-pink-600".to_string(),
            "to-purple-600".to_string(),
            "hover:from-pink-600".to_string(),
            "hover:to-purple-700".to_string(),
            "bg-gradient-to-r".to_string(),
            "from-black/40".to_string(),
            "via-purple-900/20".to_string(),
            "to-black/40".to_string(),
        ]
    }

}

impl Default for DemoCssGenerator {
    fn default() -> Self {
        Self::new()
    }
}

