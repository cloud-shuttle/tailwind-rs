use leptos::prelude::*;
use leptos::mount::mount_to_body;
use leptos::control_flow::{Show, For};
use leptos::html::ElementChild;
use leptos::prelude::event_target_value;
use web_sys::console;

/// Simple class builder for demo purposes
#[derive(Debug, Clone)]
pub struct ClassBuilder {
    classes: Vec<String>,
}

impl ClassBuilder {
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }
    
    pub fn class(mut self, class: &str) -> Self {
        self.classes.push(class.to_string());
        self
    }
    
    pub fn build(self) -> String {
        self.classes.join(" ")
    }
}

/// Main demo application
#[component]
pub fn App() -> impl IntoView {
    // Initialize logging for WASM
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logging");

    // Set up meta tags
    leptos_meta::provide_meta_context();
    
    // Create reactive state for the demo
    let (current_page, set_current_page) = signal(Page::Home);
    let (theme, set_theme) = signal(Theme::Light);
    let (dynamic_classes, set_dynamic_classes) = signal("bg-blue-500 text-white p-4 rounded-lg".to_string());

    // Provide theme context
    provide_context(ThemeContext { theme, set_theme });

    view! {
        <div class=get_app_classes(theme)>
            <Header 
                current_page=current_page
                set_current_page=set_current_page
                theme=theme
                set_theme=set_theme
            />
            
            <main class="flex-1 container mx-auto px-4 py-8">
                <Show
                    when=move || current_page.get() == Page::Home
                    fallback=|| view! { <div></div> }
                >
                    <HomePage />
                </Show>
                
                <Show
                    when=move || current_page.get() == Page::Components
                    fallback=|| view! { <div></div> }
                >
                    <ComponentsPage />
                </Show>
                
                <Show
                    when=move || current_page.get() == Page::Dynamic
                    fallback=|| view! { <div></div> }
                >
                    <DynamicPage 
                        dynamic_classes=dynamic_classes
                        set_dynamic_classes=set_dynamic_classes
                    />
                </Show>
            </main>
            
            <Footer />
        </div>
    }
}

fn get_app_classes(theme: ReadSignal<Theme>) -> String {
    ClassBuilder::new()
        .class("min-h-screen flex flex-col")
        .class(match theme.get() {
            Theme::Light => "bg-gray-50 text-gray-900",
            Theme::Dark => "bg-gray-900 text-gray-100",
        })
        .build()
}

/// Demo pages
#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Home,
    Components,
    Dynamic,
}

impl Page {
    pub fn title(&self) -> &'static str {
        match self {
            Page::Home => "Home",
            Page::Components => "Components",
            Page::Dynamic => "Dynamic Styling",
        }
    }
}

/// Theme context
#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub struct ThemeContext {
    pub theme: ReadSignal<Theme>,
    pub set_theme: WriteSignal<Theme>,
}

/// Header component with navigation and theme toggle
#[component]
pub fn Header(
    current_page: ReadSignal<Page>,
    set_current_page: WriteSignal<Page>,
    theme: ReadSignal<Theme>,
    set_theme: WriteSignal<Theme>,
) -> impl IntoView {
    let header_classes = ClassBuilder::new()
        .class("bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700")
        .class("sticky top-0 z-50")
        .build();

    let nav_classes = ClassBuilder::new()
        .class("container mx-auto px-4 py-4")
        .class("flex items-center justify-between")
        .build();

    let logo_classes = ClassBuilder::new()
        .class("text-2xl font-bold")
        .class("bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent")
        .build();

    let nav_item_classes = |is_active: bool| {
        ClassBuilder::new()
            .class("px-3 py-2 rounded-md text-sm font-medium transition-colors")
            .class(if is_active {
                "bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300"
            } else {
                "text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-gray-100"
            })
            .build()
    };

    let toggle_theme = move |_| {
        set_theme.update(|theme| {
            *theme = match *theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
        });
    };

    view! {
        <header class=header_classes>
            <nav class=nav_classes>
                <div class="flex items-center space-x-8">
                    <h1 class=logo_classes>
                        "🎨 Tailwind-RS Demo"
                    </h1>
                    
                    <div class="flex space-x-1">
                        <button
                            class=nav_item_classes(current_page.get() == Page::Home)
                            on:click=move |_| set_current_page.set(Page::Home)
                        >
                            "Home"
                        </button>
                        <button
                            class=nav_item_classes(current_page.get() == Page::Components)
                            on:click=move |_| set_current_page.set(Page::Components)
                        >
                            "Components"
                        </button>
                        <button
                            class=nav_item_classes(current_page.get() == Page::Dynamic)
                            on:click=move |_| set_current_page.set(Page::Dynamic)
                        >
                            "Dynamic"
                        </button>
                    </div>
                </div>
                
                <button
                    class="p-2 rounded-md bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                    on:click=toggle_theme
                >
                    <Show
                        when=move || theme.get() == Theme::Light
                        fallback=|| view! { <span class="text-yellow-500">"🌙"</span> }
                    >
                        <span class="text-yellow-500">"☀️"</span>
                    </Show>
                </button>
            </nav>
        </header>
    }
}

/// Footer component
#[component]
pub fn Footer() -> impl IntoView {
    let footer_classes = ClassBuilder::new()
        .class("bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700")
        .class("py-8 mt-auto")
        .build();

    let container_classes = ClassBuilder::new()
        .class("container mx-auto px-4 text-center")
        .class("text-gray-600 dark:text-gray-400")
        .build();

    view! {
        <footer class=footer_classes>
            <div class=container_classes>
                <p>"Built with ❤️ using Tailwind-RS and Leptos"</p>
                <p class="mt-2 text-sm">
                    "Rust-native Tailwind CSS implementation with type safety and performance"
                </p>
            </div>
        </footer>
    }
}

/// Home page showcasing the demo
#[component]
pub fn HomePage() -> impl IntoView {
    let hero_classes = ClassBuilder::new()
        .class("text-center py-16")
        .build();

    let title_classes = ClassBuilder::new()
        .class("text-5xl font-bold mb-6")
        .class("bg-gradient-to-r from-blue-600 via-purple-600 to-pink-600 bg-clip-text text-transparent")
        .build();

    let subtitle_classes = ClassBuilder::new()
        .class("text-xl text-gray-600 dark:text-gray-300 mb-8 max-w-3xl mx-auto")
        .build();

    let feature_grid_classes = ClassBuilder::new()
        .class("grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 mt-16")
        .build();

    view! {
        <div class=hero_classes>
            <h1 class=title_classes>
                "Tailwind-RS Demo"
            </h1>
            <p class=subtitle_classes>
                "Experience the power of Rust-native Tailwind CSS with type safety, 
                performance, and seamless integration with Leptos, Yew, and Dioxus."
            </p>
            
            <div class="flex justify-center space-x-4">
                <button class="bg-blue-500 hover:bg-blue-600 text-white px-6 py-3 rounded-lg font-medium transition-colors">
                    "Get Started"
                </button>
                <button class="border-2 border-blue-600 text-blue-600 hover:bg-blue-600 hover:text-white px-6 py-3 rounded-lg font-medium transition-colors">
                    "View Documentation"
                </button>
            </div>
            
            <div class=feature_grid_classes>
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 overflow-hidden">
                    <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700">
                        <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
                            "🚀 Performance"
                        </h3>
                    </div>
                    <div class="px-6 py-4">
                        <p class="text-gray-600 dark:text-gray-300">
                            "Optimized for speed with tree-shaking, minimal bundles, and efficient runtime class generation."
                        </p>
                    </div>
                </div>
                
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 overflow-hidden">
                    <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700">
                        <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
                            "🛡️ Type Safety"
                        </h3>
                    </div>
                    <div class="px-6 py-4">
                        <p class="text-gray-600 dark:text-gray-300">
                            "Compile-time validation ensures you never use invalid classes or break your design system."
                        </p>
                    </div>
                </div>
                
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 overflow-hidden">
                    <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700">
                        <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
                            "🎨 Dynamic Styling"
                        </h3>
                    </div>
                    <div class="px-6 py-4">
                        <p class="text-gray-600 dark:text-gray-300">
                            "Generate classes at runtime with reactive signals and state management integration."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Components showcase page
#[component]
pub fn ComponentsPage() -> impl IntoView {
    let container_classes = ClassBuilder::new()
        .class("max-w-6xl mx-auto")
        .build();

    let section_classes = ClassBuilder::new()
        .class("mb-12")
        .build();

    let button_grid_classes = ClassBuilder::new()
        .class("grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4")
        .build();

    view! {
        <div class=container_classes>
            <div class=section_classes.clone()>
                <h2 class="text-3xl font-bold mb-6 text-gray-900 dark:text-gray-100">
                    "Button Components"
                </h2>
                <div class=button_grid_classes>
                    <button class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors">
                        "Primary"
                    </button>
                    <button class="bg-gray-200 hover:bg-gray-300 text-gray-900 px-4 py-2 rounded-lg font-medium transition-colors dark:bg-gray-700 dark:hover:bg-gray-600 dark:text-gray-100">
                        "Secondary"
                    </button>
                    <button class="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg font-medium transition-colors">
                        "Success"
                    </button>
                    <button class="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg font-medium transition-colors">
                        "Danger"
                    </button>
                    <button class="border-2 border-blue-600 text-blue-600 hover:bg-blue-600 hover:text-white px-4 py-2 rounded-lg font-medium transition-colors">
                        "Outline"
                    </button>
                </div>
                
                <div class="mt-6 grid grid-cols-3 gap-4">
                    <button class="bg-blue-600 hover:bg-blue-700 text-white px-3 py-1.5 rounded-lg text-sm font-medium transition-colors">
                        "Small"
                    </button>
                    <button class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors">
                        "Medium"
                    </button>
                    <button class="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg text-lg font-medium transition-colors">
                        "Large"
                    </button>
                </div>
            </div>
            
            <div class=section_classes.clone()>
                <h2 class="text-3xl font-bold mb-6 text-gray-900 dark:text-gray-100">
                    "Card Components"
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 overflow-hidden">
                        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700">
                            <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
                                "Basic Card"
                            </h3>
                        </div>
                        <div class="px-6 py-4">
                            <p class="text-gray-600 dark:text-gray-300">
                                "This is a basic card component with a title and content."
                            </p>
                        </div>
                    </div>
                    
                    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 overflow-hidden">
                        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700">
                            <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
                                "Feature Card"
                            </h3>
                        </div>
                        <div class="px-6 py-4">
                            <p class="text-gray-600 dark:text-gray-300 mb-4">
                                "Cards can contain any content and automatically handle dark mode."
                            </p>
                            <button class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors">
                                "Learn More"
                            </button>
                        </div>
                    </div>
                    
                    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 overflow-hidden">
                        <div class="px-6 py-4">
                            <h3 class="text-lg font-semibold mb-2 text-gray-900 dark:text-gray-100">
                                "Card without Title Prop"
                            </h3>
                            <p class="text-gray-600 dark:text-gray-300">
                                "Cards can also be created without using the title prop."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Dynamic styling page
#[component]
pub fn DynamicPage(
    dynamic_classes: ReadSignal<String>,
    set_dynamic_classes: WriteSignal<String>,
) -> impl IntoView {
    let container_classes = ClassBuilder::new()
        .class("max-w-4xl mx-auto")
        .build();

    let preview_classes = move || {
        ClassBuilder::new()
            .class("w-full h-32 rounded-lg border-2 border-dashed border-gray-300 dark:border-gray-600")
            .class("flex items-center justify-center text-center")
            .class(&dynamic_classes.get())
            .build()
    };

    let input_classes = ClassBuilder::new()
        .class("w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md")
        .class("bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100")
        .class("focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent")
        .class("font-mono text-sm")
        .build();


    view! {
        <div class=container_classes>
            <h2 class="text-3xl font-bold mb-6 text-gray-900 dark:text-gray-100">
                "Dynamic Class Generation"
            </h2>
            
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 overflow-hidden">
                <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-700">
                    <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
                        "Live Preview"
                    </h3>
                </div>
                <div class="px-6 py-4">
                    <div class="mb-6">
                        <div class=preview_classes()>
                            <span class="text-lg font-medium">
                                "Dynamic Styling Preview"
                            </span>
                        </div>
                    </div>
                    
                    <div class="mb-6">
                        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                            "CSS Classes"
                        </label>
                        <input
                            class=input_classes
                            value=move || dynamic_classes.get()
                            on:input=move |ev| set_dynamic_classes.set(event_target_value(&ev))
                            placeholder="Enter Tailwind classes (e.g., bg-blue-500 text-white p-4 rounded-lg)"
                        />
                    </div>
                    
                    <div class="mb-6">
                        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                            "Preset Styles"
                        </label>
                        <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
                            <button
                                class="px-3 py-2 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-md transition-colors"
                                on:click=move |_| set_dynamic_classes.set("bg-blue-500 text-white p-4 rounded-lg".to_string())
                            >
                                "Blue Card"
                            </button>
                            <button
                                class="px-3 py-2 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-md transition-colors"
                                on:click=move |_| set_dynamic_classes.set("bg-gradient-to-r from-purple-500 to-pink-500 text-white p-6 rounded-xl".to_string())
                            >
                                "Gradient"
                            </button>
                            <button
                                class="px-3 py-2 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-md transition-colors"
                                on:click=move |_| set_dynamic_classes.set("bg-gray-100 dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600 p-4 rounded".to_string())
                            >
                                "Bordered"
                            </button>
                            <button
                                class="px-3 py-2 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-md transition-colors"
                                on:click=move |_| set_dynamic_classes.set("bg-yellow-400 text-black p-4 rounded-full shadow-lg".to_string())
                            >
                                "Yellow Circle"
                            </button>
                        </div>
                    </div>
                    
                    <div class="bg-gray-50 dark:bg-gray-800 p-4 rounded-lg">
                        <h4 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                            "Generated Classes:"
                        </h4>
                        <code class="text-sm text-gray-600 dark:text-gray-400 font-mono">
                            {move || dynamic_classes.get()}
                        </code>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Entry point for WASM
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    console::log_1(&"🚀 Starting Tailwind-RS Leptos Demo".into());
    
    mount_to_body(|| {
        view! { <App /> }
    });
}