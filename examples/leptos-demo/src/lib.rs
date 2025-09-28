//! Tailwind-RS Leptos Demo Application
//!
//! This demo showcases the integration of Tailwind-RS with Leptos v0.8.9,
//! demonstrating API contracts, new parsers, and modern web development features.

use leptos::prelude::*;
use tailwind_rs_core::api_contracts::{ClassBuilderContract, ApiVersion, ClassBuilderInput, ApiContract};
use tailwind_rs_core::css_generator::CssGenerator;

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="bg-white shadow-lg">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center py-6">
                    <div class="flex items-center">
                        <h1 class="text-2xl font-bold text-gray-900">
                            "Tailwind-RS Demo"
                        </h1>
                    </div>
                    <nav class="hidden md:flex space-x-8">
                        <a href="#" class="text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium">
                            "Home"
                        </a>
                        <a href="#" class="text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium">
                            "About"
                        </a>
                        <a href="#" class="text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium">
                            "Contact"
                        </a>
                    </nav>
                </div>
            </div>
        </header>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="bg-gradient-to-r from-blue-600 to-purple-600 text-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
                <div class="text-center">
                    <h2 class="text-4xl font-extrabold sm:text-5xl md:text-6xl">
                        "Welcome to Tailwind-RS"
                    </h2>
                    <p class="mt-3 max-w-md mx-auto text-base sm:text-lg md:mt-5 md:text-xl md:max-w-3xl">
                        "A production-ready Rust implementation of Tailwind CSS with full framework integration."
                    </p>
                    <div class="mt-5 max-w-md mx-auto sm:flex sm:justify-center md:mt-8">
                        <div class="rounded-md shadow">
                            <a href="#" class="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-blue-600 bg-white hover:bg-gray-50 md:py-4 md:text-lg md:px-10">
                                "Get Started"
                            </a>
                        </div>
                        <div class="mt-3 rounded-md shadow sm:mt-0 sm:ml-3">
                            <a href="#" class="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-500 hover:bg-blue-400 md:py-4 md:text-lg md:px-10">
                                "Learn More"
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(title: &'static str, description: &'static str, icon: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white overflow-hidden shadow-lg rounded-lg">
            <div class="p-6">
                <div class="flex items-center">
                    <div class="flex-shrink-0">
                        <div class="w-8 h-8 bg-blue-500 rounded-md flex items-center justify-center">
                            <span class="text-white font-bold">{icon}</span>
                        </div>
                    </div>
                    <div class="ml-4">
                        <h3 class="text-lg font-medium text-gray-900">{title}</h3>
                    </div>
                </div>
                <p class="mt-2 text-gray-600">{description}</p>
            </div>
        </div>
    }
}

#[component]
fn Features() -> impl IntoView {
    view! {
        <section class="py-12 bg-gray-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-3xl font-extrabold text-gray-900">
                        "Key Features"
                    </h2>
                    <p class="mt-4 text-lg text-gray-600">
                        "Tailwind-RS provides everything you need for modern web development."
                    </p>
                </div>
                <div class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
                    <FeatureCard
                        title="Framework Integration"
                        description="Full support for Leptos, Yew, and Dioxus with zero overhead."
                        icon="🚀"
                    />
                    <FeatureCard
                        title="Performance Optimized"
                        description="Built for speed with advanced optimization strategies."
                        icon="⚡"
                    />
                    <FeatureCard
                        title="Type Safe"
                        description="Compile-time safety with Rust's type system."
                        icon="🛡️"
                    />
                    <FeatureCard
                        title="Production Ready"
                        description="Battle-tested in production environments."
                        icon="🏭"
                    />
                    <FeatureCard
                        title="Modern CSS"
                        description="Full Tailwind CSS class support with new parsers."
                        icon="🎨"
                    />
                    <FeatureCard
                        title="Developer Experience"
                        description="Excellent tooling and documentation."
                        icon="👨‍💻"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn ApiContractsDemo() -> impl IntoView {
    let (contract_result, set_contract_result) = signal(String::new());
    let (is_validating, set_is_validating) = signal(false);

    let run_contract_validation = move |_| {
        set_is_validating.set(true);

        // Demonstrate API contracts in action
        let contract = ClassBuilderContract::new(ApiVersion::V2_0_0);
        let input = ClassBuilderInput {
            classes: vec!["p-4".to_string(), "bg-blue-500".to_string(), "text-white".to_string()],
            responsive: vec![],
            conditional: vec![],
            custom: vec![],
        };

        // Validate input
        match contract.validate_input(&input) {
            Ok(_) => {
                // Process with contract
                match contract.process(input) {
                    Ok(output) => {
                        // Validate output
                        match contract.validate_output(&output) {
                            Ok(_) => {
                                set_contract_result.set(format!(
                                    "✅ Contract validation successful!\nClasses: {}",
                                    output.to_css_classes()
                                ));
                            }
                            Err(e) => {
                                set_contract_result.set(format!("❌ Output validation failed: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        set_contract_result.set(format!("❌ Processing failed: {:?}", e));
                    }
                }
            }
            Err(e) => {
                set_contract_result.set(format!("❌ Input validation failed: {}", e));
            }
        }

        set_is_validating.set(false);
    };

    view! {
        <section class="py-12 bg-gradient-to-br from-purple-50 to-blue-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-3xl font-extrabold text-gray-900">
                        "🔒 API Contracts Demo"
                    </h2>
                    <p class="mt-4 text-lg text-gray-600">
                        "Experience the power of contract-based validation for guaranteed API stability."
                    </p>
                </div>
                <div class="mt-12 flex justify-center">
                    <div class="bg-white rounded-xl shadow-xl p-8 max-w-2xl w-full">
                        <div class="text-center space-y-6">
                            <div class="bg-blue-50 rounded-lg p-4">
                                <h3 class="text-lg font-semibold text-blue-900 mb-2">
                                    "Contract Validation Process"
                                </h3>
                                <div class="text-sm text-blue-700 space-y-1">
                                    <p>"1. Input validation with type safety"</p>
                                    <p>"2. Processing with guaranteed behavior"</p>
                                    <p>"3. Output validation with format guarantees"</p>
                                </div>
                            </div>

                            <button
                                class="bg-gradient-to-r from-purple-600 to-blue-600 text-white font-bold py-3 px-8 rounded-lg hover:from-purple-700 hover:to-blue-700 transform hover:scale-105 transition-all duration-200 disabled:opacity-50"
                                on:click=run_contract_validation
                                disabled=is_validating
                            >
                                {move || if is_validating.get() { "🔄 Validating..." } else { "🚀 Run Contract Validation" }}
                            </button>

                            {move || (!contract_result.get().is_empty()).then(|| {
                                view! {
                                    <div class="bg-gray-50 rounded-lg p-4 font-mono text-sm">
                                        <pre class="whitespace-pre-wrap text-gray-800">
                                            {contract_result.get()}
                                        </pre>
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TransformParsersDemo() -> impl IntoView {
    let (transform_classes, set_transform_classes) = signal(vec![
        "translate-x-4".to_string(),
        "translate-y-2".to_string(),
        "scale-x-110".to_string(),
        "scale-y-95".to_string(),
    ]);

    let _add_transform_class = move |class: String| {
        set_transform_classes.update(|classes| classes.push(class));
    };

    view! {
        <section class="py-12 bg-gradient-to-br from-green-50 to-teal-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-3xl font-extrabold text-gray-900">
                        "🎨 New Transform Parsers Demo"
                    </h2>
                    <p class="mt-4 text-lg text-gray-600">
                        "Showcasing the latest translate-x/y and scale-x/y parsers with O(1) performance."
                    </p>
                </div>
                <div class="mt-12">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        // Transform Demo Box
                        <div class="bg-white rounded-xl shadow-lg p-8">
                            <h3 class="text-xl font-semibold text-gray-900 mb-6 text-center">
                                "Live Transform Demo"
                            </h3>
                            <div class="flex justify-center">
                                <div
                                    class="w-32 h-32 bg-gradient-to-br from-blue-400 to-purple-500 rounded-lg shadow-lg transform transition-all duration-300 hover:translate-x-4 hover:translate-y-2 hover:scale-x-110 hover:scale-y-95"
                                    style="transform-origin: center;"
                                >
                                    <div class="w-full h-full flex items-center justify-center">
                                        <span class="text-white font-bold text-lg">"Hover Me!"</span>
                                    </div>
                                </div>
                            </div>
                            <p class="mt-4 text-sm text-gray-600 text-center">
                                "Hover to see translate-x-4, translate-y-2, scale-x-110, scale-y-95 in action"
                            </p>
                        </div>

                        // Parser Performance Info
                        <div class="bg-white rounded-xl shadow-lg p-8">
                            <h3 class="text-xl font-semibold text-gray-900 mb-6 text-center">
                                "Parser Performance"
                            </h3>
                            <div class="space-y-4">
                                <div class="flex justify-between items-center">
                                    <span class="text-gray-700">"Lookup Time"</span>
                                    <span class="bg-green-100 text-green-800 px-3 py-1 rounded-full text-sm font-medium">
                                        "O(1) HashMap"
                                    </span>
                                </div>
                                <div class="flex justify-between items-center">
                                    <span class="text-gray-700">"Memory Usage"</span>
                                    <span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-sm font-medium">
                                        "Minimal"
                                    </span>
                                </div>
                                <div class="flex justify-between items-center">
                                    <span class="text-gray-700">"Coverage"</span>
                                    <span class="bg-purple-100 text-purple-800 px-3 py-1 rounded-full text-sm font-medium">
                                        "100% Complete"
                                    </span>
                                </div>
                            </div>

                            <div class="mt-6">
                                <h4 class="text-lg font-medium text-gray-900 mb-3">"Supported Classes"</h4>
                                <div class="grid grid-cols-2 gap-2 text-sm">
                                    {transform_classes.get().into_iter().map(|class| {
                                        view! {
                                            <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">
                                                {class}
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn InteractiveDemo() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (_is_hovered, _set_hovered) = signal(false);

    view! {
        <section class="py-12 bg-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-3xl font-extrabold text-gray-900">
                        "Interactive Demo"
                    </h2>
                    <p class="mt-4 text-lg text-gray-600">
                        "Try out the interactive features powered by Tailwind-RS and Leptos."
                    </p>
                </div>
                <div class="mt-12 flex justify-center">
                    <div class="bg-gray-50 rounded-lg p-8 max-w-md w-full">
                        <div class="text-center">
                            <div class="text-6xl font-bold text-blue-600 mb-4">
                                {move || count.get()}
                            </div>
                            <button
                                class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded transition-colors duration-200 transform hover:scale-105"
                                on:click=move |_| set_count.update(|c| *c += 1)
                            >
                                "Click me!"
                            </button>
                            <p class="mt-4 text-sm text-gray-600">
                                "This button uses Tailwind-RS classes for styling and Leptos for reactivity."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-800 text-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    <div>
                        <h3 class="text-lg font-semibold mb-4">"Tailwind-RS"</h3>
                        <p class="text-gray-400">
                            "A production-ready Rust implementation of Tailwind CSS."
                        </p>
                    </div>
                    <div>
                        <h3 class="text-lg font-semibold mb-4">"Links"</h3>
                        <ul class="space-y-2">
                            <li><a href="#" class="text-gray-400 hover:text-white">"Documentation"</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white">"GitHub"</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white">"Examples"</a></li>
                        </ul>
                    </div>
                    <div>
                        <h3 class="text-lg font-semibold mb-4">"Community"</h3>
                        <ul class="space-y-2">
                            <li><a href="#" class="text-gray-400 hover:text-white">"Discord"</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white">"Twitter"</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white">"Reddit"</a></li>
                        </ul>
                    </div>
                </div>
                <div class="mt-8 pt-8 border-t border-gray-700 text-center text-gray-400">
                    <p>"© 2025 Tailwind-RS. Built with Rust and Leptos."</p>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <Header />
            <Hero />
            <Features />
            <ApiContractsDemo />
            <TransformParsersDemo />
            <InteractiveDemo />
            <Footer />
        </div>
    }
}

use wasm_bindgen::prelude::*;

// Global CSS generator for the demo
static mut CSS_GENERATOR: Option<CssGenerator> = None;

// Initialize CSS generator with common Tailwind classes
fn init_css_generator() -> CssGenerator {
    let mut generator = CssGenerator::new();

    // Add common classes used throughout the demo
    let common_classes = vec![
        // Layout and spacing
        "min-h-screen", "container", "mx-auto", "px-4", "py-8", "py-12", "py-24", "flex", "justify-center", "items-center",
        "justify-between", "flex-col", "space-y-6", "space-y-8", "space-x-4", "space-x-8", "grid", "grid-cols-1",
        "md:grid-cols-2", "lg:grid-cols-3", "gap-8", "max-w-7xl", "max-w-2xl", "max-w-md", "w-full",

        // Colors and backgrounds
        "bg-white", "bg-gray-50", "bg-gray-100", "bg-gray-800", "bg-blue-500", "bg-blue-600", "bg-purple-600",
        "bg-green-100", "bg-green-200", "bg-green-500", "bg-red-500", "text-white", "text-gray-600", "text-gray-800",
        "text-gray-900", "text-blue-600", "text-blue-900", "text-green-800", "text-purple-800",

        // Borders and shadows
        "border", "border-transparent", "border-green-400", "border-green-600", "rounded", "rounded-lg", "rounded-xl",
        "rounded-md", "shadow-lg", "shadow-md",

        // Typography
        "text-4xl", "text-3xl", "text-2xl", "text-xl", "text-lg", "text-base", "text-sm", "font-bold", "font-semibold",
        "font-medium", "font-extrabold", "leading-tight", "text-center", "text-left",

        // Positioning and sizing
        "relative", "absolute", "top-0", "left-0", "w-32", "h-32", "w-8", "h-8", "w-full", "h-full",

        // Hover and transitions
        "hover:bg-blue-600", "hover:bg-blue-700", "hover:bg-purple-700", "hover:bg-gray-50", "hover:text-white",
        "hover:scale-105", "hover:translate-x-4", "hover:translate-y-2", "hover:scale-x-110", "hover:scale-y-95",
        "transition-colors", "transition-all", "duration-200", "duration-300",

        // Responsive utilities
        "sm:px-6", "sm:flex", "sm:justify-center", "sm:ml-3", "sm:mt-0", "sm:text-lg", "sm:text-5xl", "sm:text-lg",
        "md:flex", "md:grid-cols-3", "md:py-4", "md:text-lg", "md:px-10", "md:mt-8", "md:max-w-3xl", "lg:px-8", "lg:grid-cols-3",

        // Transform utilities (the new parsers)
        "transform", "translate-x-4", "translate-y-2", "scale-x-110", "scale-y-95",
    ];

    for class in common_classes {
        let _ = generator.add_class(class);
    }

    generator
}

// Inject CSS into the page
fn inject_css(css: &str) {
    use wasm_bindgen::JsCast;
    use leptos::web_sys::{window, Document, Element, HtmlElement, HtmlStyleElement};

    let document = window().unwrap().document().unwrap();

    // Create style element
    let style = document.create_element("style").unwrap();
    style.set_text_content(Some(css));

    // Insert at the beginning of head
    let head = document.head().unwrap();
    head.insert_before(&style, head.first_child().as_ref()).unwrap();
}

// This is the entry point for the web app
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize CSS generator and inject styles
    unsafe {
        CSS_GENERATOR = Some(init_css_generator());
        if let Some(generator) = &CSS_GENERATOR {
            let css = generator.generate_css();
            inject_css(&css);
        }
    }

    mount_to_body(App)
}
