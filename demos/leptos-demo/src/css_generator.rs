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

    /// Get comprehensive classes for the impressive demo - ONLY classes that are actually implemented
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

            // Backgrounds and gradients (WORKING!)
            "bg-gradient-to-br".to_string(),
            "from-purple-600".to_string(),
            "to-blue-600".to_string(),
            "bg-white/10".to_string(),
            "bg-black/30".to_string(),
            "backdrop-blur-sm".to_string(),
            "bg-white".to_string(),
            "bg-gray-100".to_string(),

            // Typography (WORKING!)
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

            // Responsive classes
            "md:grid-cols-3".to_string(),
            "sm:flex-row".to_string(),
            "lg:grid-cols-2".to_string(),
            "md:grid-cols-4".to_string(),

            // Borders and shadows
            "rounded-lg".to_string(),
            "rounded-xl".to_string(),
            "border".to_string(),
            "border-2".to_string(),
            "border-white/20".to_string(),
            "border-white/30".to_string(),

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
        ]
    }

}

impl Default for DemoCssGenerator {
    fn default() -> Self {
        Self::new()
    }
}

