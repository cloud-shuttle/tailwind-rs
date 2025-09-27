//! Tailwind-RS Yew Demo Application
//!
//! This demo showcases the integration of Tailwind-RS with Yew,
//! demonstrating various Tailwind CSS classes and components.

use yew::prelude::*;
use tailwind_rs_yew::*;

#[function_component]
fn Header() -> Html {
    html! {
        <header class="bg-white shadow-lg">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center py-6">
                    <div class="flex items-center">
                        <h1 class="text-2xl font-bold text-gray-900">
                            {"Tailwind-RS Demo"}
                        </h1>
                    </div>
                    <nav class="hidden md:flex space-x-8">
                        <a href="#" class="text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium">
                            {"Home"}
                        </a>
                        <a href="#" class="text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium">
                            {"About"}
                        </a>
                        <a href="#" class="text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium">
                            {"Contact"}
                        </a>
                    </nav>
                </div>
            </div>
        </header>
    }
}

#[function_component]
fn Hero() -> Html {
    html! {
        <section class="bg-gradient-to-r from-blue-600 to-purple-600 text-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
                <div class="text-center">
                    <h2 class="text-4xl font-extrabold sm:text-5xl md:text-6xl">
                        {"Welcome to Tailwind-RS"}
                    </h2>
                    <p class="mt-3 max-w-md mx-auto text-base sm:text-lg md:mt-5 md:text-xl md:max-w-3xl">
                        {"A production-ready Rust implementation of Tailwind CSS with full framework integration."}
                    </p>
                    <div class="mt-5 max-w-md mx-auto sm:flex sm:justify-center md:mt-8">
                        <div class="rounded-md shadow">
                            <a href="#" class="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-blue-600 bg-white hover:bg-gray-50 md:py-4 md:text-lg md:px-10">
                                {"Get Started"}
                            </a>
                        </div>
                        <div class="mt-3 rounded-md shadow sm:mt-0 sm:ml-3">
                            <a href="#" class="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-500 hover:bg-blue-400 md:py-4 md:text-lg md:px-10">
                                {"Learn More"}
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct FeatureCardProps {
    title: String,
    description: String,
    icon: String,
}

#[function_component]
fn FeatureCard(props: &FeatureCardProps) -> Html {
    html! {
        <div class="bg-white overflow-hidden shadow-lg rounded-lg">
            <div class="p-6">
                <div class="flex items-center">
                    <div class="flex-shrink-0">
                        <div class="w-8 h-8 bg-blue-500 rounded-md flex items-center justify-center">
                            <span class="text-white font-bold">{&props.icon}</span>
                        </div>
                    </div>
                    <div class="ml-4">
                        <h3 class="text-lg font-medium text-gray-900">{&props.title}</h3>
                    </div>
                </div>
                <p class="mt-2 text-gray-600">{&props.description}</p>
            </div>
        </div>
    }
}

#[function_component]
fn Features() -> Html {
    html! {
        <section class="py-12 bg-gray-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-3xl font-extrabold text-gray-900">
                        {"Key Features"}
                    </h2>
                    <p class="mt-4 text-lg text-gray-600">
                        {"Tailwind-RS provides everything you need for modern web development."}
                    </p>
                </div>
                <div class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
                    <FeatureCard
                        title="Framework Integration".to_string()
                        description="Full support for Leptos, Yew, and Dioxus with zero overhead.".to_string()
                        icon="🚀".to_string()
                    />
                    <FeatureCard
                        title="Performance Optimized".to_string()
                        description="Built for speed with advanced optimization strategies.".to_string()
                        icon="⚡".to_string()
                    />
                    <FeatureCard
                        title="Type Safe".to_string()
                        description="Compile-time safety with Rust's type system.".to_string()
                        icon="🛡️".to_string()
                    />
                    <FeatureCard
                        title="Production Ready".to_string()
                        description="Battle-tested in production environments.".to_string()
                        icon="🏭".to_string()
                    />
                    <FeatureCard
                        title="Modern CSS".to_string()
                        description="Full Tailwind CSS class support with new parsers.".to_string()
                        icon="🎨".to_string()
                    />
                    <FeatureCard
                        title="Developer Experience".to_string()
                        description="Excellent tooling and documentation.".to_string()
                        icon="👨‍💻".to_string()
                    />
                </div>
            </div>
        </section>
    }
}

#[function_component]
fn InteractiveDemo() -> Html {
    let counter = use_state(|| 0);
    let is_hovered = use_state(|| false);
    
    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    
    let onmouseenter = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(true))
    };
    
    let onmouseleave = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(false))
    };
    
    let button_class = if *is_hovered {
        "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition-colors duration-200 transform scale-105"
    } else {
        "bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded transition-colors duration-200"
    };
    
    html! {
        <section class="py-12 bg-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h2 class="text-3xl font-extrabold text-gray-900">
                        {"Interactive Demo"}
                    </h2>
                    <p class="mt-4 text-lg text-gray-600">
                        {"Try out the interactive features powered by Tailwind-RS and Yew."}
                    </p>
                </div>
                <div class="mt-12 flex justify-center">
                    <div class="bg-gray-50 rounded-lg p-8 max-w-md w-full">
                        <div class="text-center">
                            <div class="text-6xl font-bold text-blue-600 mb-4">
                                {*counter}
                            </div>
                            <button
                                class={button_class}
                                onclick={increment}
                                onmouseenter={onmouseenter}
                                onmouseleave={onmouseleave}
                            >
                                {"Click me!"}
                            </button>
                            <p class="mt-4 text-sm text-gray-600">
                                {"This button uses Tailwind-RS classes for styling and Yew for reactivity."}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[function_component]
fn Footer() -> Html {
    html! {
        <footer class="bg-gray-800 text-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    <div>
                        <h3 class="text-lg font-semibold mb-4">{"Tailwind-RS"}</h3>
                        <p class="text-gray-400">
                            {"A production-ready Rust implementation of Tailwind CSS."}
                        </p>
                    </div>
                    <div>
                        <h3 class="text-lg font-semibold mb-4">{"Links"}</h3>
                        <ul class="space-y-2">
                            <li><a href="#" class="text-gray-400 hover:text-white">{"Documentation"}</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white">{"GitHub"}</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white">{"Examples"}</a></li>
                        </ul>
                    </div>
                    <div>
                        <h3 class="text-lg font-semibold mb-4">{"Community"}</h3>
                        <ul class="space-y-2">
                            <li><a href="#" class="text-gray-400 hover:text-white">{"Discord"}</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white">{"Twitter"}</a></li>
                            <li><a href="#" class="text-gray-400 hover:text-white">{"Reddit"}</a></li>
                        </ul>
                    </div>
                </div>
                <div class="mt-8 pt-8 border-t border-gray-700 text-center text-gray-400">
                    <p>{"© 2025 Tailwind-RS. Built with Rust and Yew."}</p>
                </div>
            </div>
        </footer>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div class="min-h-screen bg-gray-50">
            <Header />
            <Hero />
            <Features />
            <InteractiveDemo />
            <Footer />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
