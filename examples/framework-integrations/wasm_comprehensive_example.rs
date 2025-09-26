//! Comprehensive WASM Integration Example
//! 
//! This example demonstrates the full capabilities of Tailwind-RS with WASM,
//! including direct browser integration, performance optimization, and advanced styling.

use wasm_bindgen::prelude::*;
use web_sys::*;
use tailwind_rs_wasm::*;

/// Main WASM Application
#[wasm_bindgen]
pub struct TailwindApp {
    counter: i32,
    theme: String,
    generator: CssGenerator,
}

#[wasm_bindgen]
impl TailwindApp {
    /// Create a new TailwindApp instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            counter: 0,
            theme: "light".to_string(),
            generator: CssGenerator::new(),
        }
    }
    
    /// Initialize the app and render to DOM
    pub fn init(&mut self) -> Result<(), JsValue> {
        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        
        // Clear existing content
        body.set_inner_html("");
        
        // Create main container
        let container = document.create_element("div")?;
        container.set_class_name(&self.get_container_classes());
        
        // Create header
        let header = self.create_header(&document)?;
        container.append_child(&header)?;
        
        // Create main content
        let main_content = self.create_main_content(&document)?;
        container.append_child(&main_content)?;
        
        // Create footer
        let footer = self.create_footer(&document)?;
        container.append_child(&footer)?;
        
        body.append_child(&container)?;
        
        // Generate and inject CSS
        self.inject_css(&document)?;
        
        Ok(())
    }
    
    /// Get container classes
    fn get_container_classes(&self) -> String {
        ClassBuilder::new()
            .class("min-h-screen")
            .class("bg-gradient-to-br")
            .class("from-indigo-50")
            .class("to-purple-100")
            .class("dark:from-gray-900")
            .class("dark:to-gray-800")
            .class("transition-colors")
            .class("duration-300")
            .build()
            .to_string()
    }
    
    /// Create header element
    fn create_header(&self, document: &Document) -> Result<Element, JsValue> {
        let header = document.create_element("header")?;
        header.set_class_name("container mx-auto px-4 py-8");
        
        let title = document.create_element("h1")?;
        title.set_class_name("text-4xl font-bold text-center text-gray-800 dark:text-white mb-4");
        title.set_text_content(Some("Tailwind-RS + WASM"));
        
        let subtitle = document.create_element("p")?;
        subtitle.set_class_name("text-lg text-center text-gray-600 dark:text-gray-300");
        subtitle.set_text_content(Some("Direct browser integration with optimized performance"));
        
        header.append_child(&title)?;
        header.append_child(&subtitle)?;
        
        Ok(header)
    }
    
    /// Create main content area
    fn create_main_content(&self, document: &Document) -> Result<Element, JsValue> {
        let main = document.create_element("main")?;
        main.set_class_name("container mx-auto px-4 py-8");
        
        let content_wrapper = document.create_element("div")?;
        content_wrapper.set_class_name("max-w-4xl mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6");
        
        // Create feature cards
        let cards = vec![
            ("Performance", "Optimized WASM compilation for maximum speed"),
            ("Styling", "Complete Tailwind CSS utility coverage"),
            ("Integration", "Seamless browser API integration"),
            ("Responsive", "Mobile-first responsive design"),
            ("Dark Mode", "Built-in dark mode support"),
            ("Animations", "Smooth transitions and transforms"),
        ];
        
        for (title, description) in cards {
            let card = self.create_feature_card(document, title, description)?;
            content_wrapper.append_child(&card)?;
        }
        
        // Create interactive section
        let interactive_section = self.create_interactive_section(document)?;
        content_wrapper.append_child(&interactive_section)?;
        
        main.append_child(&content_wrapper)?;
        Ok(main)
    }
    
    /// Create feature card
    fn create_feature_card(&self, document: &Document, title: &str, description: &str) -> Result<Element, JsValue> {
        let card = document.create_element("div")?;
        card.set_class_name(&ClassBuilder::new()
            .class("bg-white")
            .class("dark:bg-gray-800")
            .class("rounded-xl")
            .class("shadow-lg")
            .class("p-6")
            .class("border")
            .class("border-gray-200")
            .class("dark:border-gray-700")
            .class("transition-all")
            .class("duration-300")
            .class("hover:shadow-xl")
            .class("hover:scale-105")
            .build()
            .to_string());
        
        let title_elem = document.create_element("h3")?;
        title_elem.set_class_name("text-xl font-bold text-gray-800 dark:text-white mb-2");
        title_elem.set_text_content(Some(title));
        
        let desc_elem = document.create_element("p")?;
        desc_elem.set_class_name("text-gray-600 dark:text-gray-300");
        desc_elem.set_text_content(Some(description));
        
        card.append_child(&title_elem)?;
        card.append_child(&desc_elem)?;
        
        Ok(card)
    }
    
    /// Create interactive section
    fn create_interactive_section(&self, document: &Document) -> Result<Element, JsValue> {
        let section = document.create_element("div")?;
        section.set_class_name("col-span-full bg-gradient-to-r from-blue-50 to-purple-50 dark:from-blue-900/20 dark:to-purple-900/20 rounded-xl p-8");
        
        let title = document.create_element("h2")?;
        title.set_class_name("text-2xl font-bold text-center text-gray-800 dark:text-white mb-6");
        title.set_text_content(Some("Interactive Demo"));
        
        let counter_display = document.create_element("div")?;
        counter_display.set_id("counter-display");
        counter_display.set_class_name("text-6xl font-bold text-center text-blue-600 dark:text-blue-400 mb-6");
        counter_display.set_text_content(Some(&self.counter.to_string()));
        
        let button_container = document.create_element("div")?;
        button_container.set_class_name("flex justify-center space-x-4");
        
        // Increment button
        let inc_button = document.create_element("button")?;
        inc_button.set_class_name(&ClassBuilder::new()
            .class("px-6")
            .class("py-3")
            .class("bg-blue-600")
            .class("hover:bg-blue-700")
            .class("text-white")
            .class("font-semibold")
            .class("rounded-lg")
            .class("transition-colors")
            .class("duration-200")
            .class("transform")
            .class("hover:scale-105")
            .class("active:scale-95")
            .class("shadow-md")
            .class("hover:shadow-lg")
            .build()
            .to_string());
        inc_button.set_text_content(Some("Increment"));
        
        // Decrement button
        let dec_button = document.create_element("button")?;
        dec_button.set_class_name(&ClassBuilder::new()
            .class("px-6")
            .class("py-3")
            .class("bg-red-600")
            .class("hover:bg-red-700")
            .class("text-white")
            .class("font-semibold")
            .class("rounded-lg")
            .class("transition-colors")
            .class("duration-200")
            .class("transform")
            .class("hover:scale-105")
            .class("active:scale-95")
            .class("shadow-md")
            .class("hover:shadow-lg")
            .build()
            .to_string());
        dec_button.set_text_content(Some("Decrement"));
        
        // Theme toggle button
        let theme_button = document.create_element("button")?;
        theme_button.set_class_name(&ClassBuilder::new()
            .class("px-4")
            .class("py-2")
            .class("bg-gray-200")
            .class("dark:bg-gray-700")
            .class("hover:bg-gray-300")
            .class("dark:hover:bg-gray-600")
            .class("text-gray-800")
            .class("dark:text-white")
            .class("rounded-md")
            .class("transition-colors")
            .class("duration-200")
            .class("text-sm")
            .class("font-medium")
            .build()
            .to_string());
        theme_button.set_text_content(Some(&format!("Toggle Theme ({})", self.theme)));
        
        button_container.append_child(&inc_button)?;
        button_container.append_child(&dec_button)?;
        button_container.append_child(&theme_button)?;
        
        section.append_child(&title)?;
        section.append_child(&counter_display)?;
        section.append_child(&button_container)?;
        
        Ok(section)
    }
    
    /// Create footer
    fn create_footer(&self, document: &Document) -> Result<Element, JsValue> {
        let footer = document.create_element("footer")?;
        footer.set_class_name("container mx-auto px-4 py-8 text-center");
        
        let footer_text = document.create_element("p")?;
        footer_text.set_class_name("text-gray-600 dark:text-gray-400");
        footer_text.set_text_content(Some("Built with Tailwind-RS and WASM for maximum performance"));
        
        footer.append_child(&footer_text)?;
        Ok(footer)
    }
    
    /// Inject generated CSS into the document
    fn inject_css(&mut self, document: &Document) -> Result<(), JsValue> {
        // Add some sample classes to the generator
        self.generator.add_class("min-h-screen")?;
        self.generator.add_class("bg-gradient-to-br")?;
        self.generator.add_class("from-indigo-50")?;
        self.generator.add_class("to-purple-100")?;
        self.generator.add_class("dark:from-gray-900")?;
        self.generator.add_class("dark:to-gray-800")?;
        self.generator.add_class("transition-colors")?;
        self.generator.add_class("duration-300")?;
        self.generator.add_class("container")?;
        self.generator.add_class("mx-auto")?;
        self.generator.add_class("px-4")?;
        self.generator.add_class("py-8")?;
        self.generator.add_class("text-4xl")?;
        self.generator.add_class("font-bold")?;
        self.generator.add_class("text-center")?;
        self.generator.add_class("text-gray-800")?;
        self.generator.add_class("dark:text-white")?;
        self.generator.add_class("mb-4")?;
        self.generator.add_class("text-lg")?;
        self.generator.add_class("text-gray-600")?;
        self.generator.add_class("dark:text-gray-300")?;
        self.generator.add_class("max-w-4xl")?;
        self.generator.add_class("grid")?;
        self.generator.add_class("grid-cols-1")?;
        self.generator.add_class("md:grid-cols-2")?;
        self.generator.add_class("lg:grid-cols-3")?;
        self.generator.add_class("gap-6")?;
        self.generator.add_class("bg-white")?;
        self.generator.add_class("dark:bg-gray-800")?;
        self.generator.add_class("rounded-xl")?;
        self.generator.add_class("shadow-lg")?;
        self.generator.add_class("p-6")?;
        self.generator.add_class("border")?;
        self.generator.add_class("border-gray-200")?;
        self.generator.add_class("dark:border-gray-700")?;
        self.generator.add_class("transition-all")?;
        self.generator.add_class("duration-300")?;
        self.generator.add_class("hover:shadow-xl")?;
        self.generator.add_class("hover:scale-105")?;
        self.generator.add_class("text-xl")?;
        self.generator.add_class("mb-2")?;
        self.generator.add_class("col-span-full")?;
        self.generator.add_class("bg-gradient-to-r")?;
        self.generator.add_class("from-blue-50")?;
        self.generator.add_class("to-purple-50")?;
        self.generator.add_class("dark:from-blue-900/20")?;
        self.generator.add_class("dark:to-purple-900/20")?;
        self.generator.add_class("p-8")?;
        self.generator.add_class("text-2xl")?;
        self.generator.add_class("mb-6")?;
        self.generator.add_class("text-6xl")?;
        self.generator.add_class("text-blue-600")?;
        self.generator.add_class("dark:text-blue-400")?;
        self.generator.add_class("flex")?;
        self.generator.add_class("justify-center")?;
        self.generator.add_class("space-x-4")?;
        self.generator.add_class("px-6")?;
        self.generator.add_class("py-3")?;
        self.generator.add_class("bg-blue-600")?;
        self.generator.add_class("hover:bg-blue-700")?;
        self.generator.add_class("text-white")?;
        self.generator.add_class("font-semibold")?;
        self.generator.add_class("rounded-lg")?;
        self.generator.add_class("transition-colors")?;
        self.generator.add_class("duration-200")?;
        self.generator.add_class("transform")?;
        self.generator.add_class("hover:scale-105")?;
        self.generator.add_class("active:scale-95")?;
        self.generator.add_class("shadow-md")?;
        self.generator.add_class("hover:shadow-lg")?;
        self.generator.add_class("bg-red-600")?;
        self.generator.add_class("hover:bg-red-700")?;
        self.generator.add_class("px-4")?;
        self.generator.add_class("py-2")?;
        self.generator.add_class("bg-gray-200")?;
        self.generator.add_class("dark:bg-gray-700")?;
        self.generator.add_class("hover:bg-gray-300")?;
        self.generator.add_class("dark:hover:bg-gray-600")?;
        self.generator.add_class("text-gray-800")?;
        self.generator.add_class("dark:text-white")?;
        self.generator.add_class("rounded-md")?;
        self.generator.add_class("text-sm")?;
        self.generator.add_class("font-medium")?;
        self.generator.add_class("text-gray-600")?;
        self.generator.add_class("dark:text-gray-400")?;
        
        // Generate CSS
        let css = self.generator.generate_css();
        
        // Create style element
        let style = document.create_element("style")?;
        style.set_text_content(Some(&css));
        
        // Append to head
        let head = document.head().unwrap();
        head.append_child(&style)?;
        
        Ok(())
    }
    
    /// Increment counter
    #[wasm_bindgen]
    pub fn increment(&mut self) {
        self.counter += 1;
        self.update_counter_display();
    }
    
    /// Decrement counter
    #[wasm_bindgen]
    pub fn decrement(&mut self) {
        self.counter -= 1;
        self.update_counter_display();
    }
    
    /// Toggle theme
    #[wasm_bindgen]
    pub fn toggle_theme(&mut self) {
        self.theme = if self.theme == "light" { "dark" } else { "light" };
        self.update_theme();
    }
    
    /// Update counter display
    fn update_counter_display(&self) {
        let document = window().unwrap().document().unwrap();
        if let Some(element) = document.get_element_by_id("counter-display") {
            element.set_text_content(Some(&self.counter.to_string()));
        }
    }
    
    /// Update theme
    fn update_theme(&self) {
        let document = window().unwrap().document().unwrap();
        let html = document.document_element().unwrap();
        
        if self.theme == "dark" {
            html.set_class_name("dark");
        } else {
            html.set_class_name("");
        }
    }
}

/// Initialize the WASM app
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    
    let mut app = TailwindApp::new();
    if let Err(e) = app.init() {
        web_sys::console::error_1(&e);
    }
}

/// Export functions for JavaScript interaction
#[wasm_bindgen]
pub fn create_app() -> TailwindApp {
    TailwindApp::new()
}

#[wasm_bindgen]
pub fn init_app(app: &mut TailwindApp) -> Result<(), JsValue> {
    app.init()
}
