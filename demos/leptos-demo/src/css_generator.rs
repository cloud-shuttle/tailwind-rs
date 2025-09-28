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

    /// Get comprehensive classes for the demo - ALL classes used in the Leptos component
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
            "text-center".to_string(),
            "relative".to_string(),
            "grid".to_string(),
            "gap-8".to_string(),

            // Advanced backgrounds and gradients (the missing pieces!)
            "bg-gradient-to-br".to_string(),
            "from-purple-600".to_string(),
            "to-blue-600".to_string(),
            "bg-white/10".to_string(),
            "backdrop-blur-sm".to_string(),
            "backdrop-blur-xl".to_string(),
            "bg-black/40".to_string(),
            "bg-black/50".to_string(),
            "bg-black/30".to_string(),
            "bg-black/20".to_string(),
            "border-white/10".to_string(),
            "border-white/20".to_string(),

            // Typography with opacity
            "text-5xl".to_string(),
            "text-xl".to_string(),
            "text-2xl".to_string(),
            "text-3xl".to_string(),
            "font-bold".to_string(),
            "opacity-90".to_string(),
            "text-white".to_string(),
            "text-white/90".to_string(),
            "text-white/80".to_string(),
            "text-cyan-400".to_string(),
            "text-purple-400".to_string(),
            "text-green-400".to_string(),

            // Responsive classes
            "md:grid-cols-3".to_string(),
            "md:grid-cols-2".to_string(),

            // Borders and shadows
            "rounded-lg".to_string(),
            "rounded-xl".to_string(),
            "rounded-3xl".to_string(),
            "border".to_string(),
            "shadow-lg".to_string(),
            "shadow-2xl".to_string(),

            // Flexbox and alignment
            "flex".to_string(),
            "items-center".to_string(),
            "justify-center".to_string(),

            // Spacing
            "p-6".to_string(),
            "p-8".to_string(),
            "space-y-6".to_string(),
            "space-y-4".to_string(),
            "space-y-1".to_string(),

            // List styling
            "list-disc".to_string(),
            "list-inside".to_string(),

            // Grid columns
            "grid-cols-1".to_string(),
            "grid-cols-2".to_string(),
            "grid-cols-3".to_string(),
        ]
    }

}

impl Default for DemoCssGenerator {
    fn default() -> Self {
        Self::new()
    }
}

