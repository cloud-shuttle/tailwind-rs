//! Tailwind-RS Dioxus Demo Application
//!
//! This demo showcases the integration of Tailwind-RS with Dioxus,
//! demonstrating various Tailwind CSS classes and components.

use dioxus::prelude::*;
use tailwind_rs_dioxus::*;

#[component]
fn Header() -> Element {
    rsx! {
        header { class: "bg-white shadow-lg",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between items-center py-6",
                    div { class: "flex items-center",
                        h1 { class: "text-2xl font-bold text-gray-900",
                            "Tailwind-RS Demo"
                        }
                    }
                    nav { class: "hidden md:flex space-x-8",
                        a { href: "#", class: "text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium",
                            "Home"
                        }
                        a { href: "#", class: "text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium",
                            "About"
                        }
                        a { href: "#", class: "text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium",
                            "Contact"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "bg-gradient-to-r from-blue-600 to-purple-600 text-white",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24",
                div { class: "text-center",
                    h2 { class: "text-4xl font-extrabold sm:text-5xl md:text-6xl",
                        "Welcome to Tailwind-RS"
                    }
                    p { class: "mt-3 max-w-md mx-auto text-base sm:text-lg md:mt-5 md:text-xl md:max-w-3xl",
                        "A production-ready Rust implementation of Tailwind CSS with full framework integration."
                    }
                    div { class: "mt-5 max-w-md mx-auto sm:flex sm:justify-center md:mt-8",
                        div { class: "rounded-md shadow",
                            a { href: "#", class: "w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-blue-600 bg-white hover:bg-gray-50 md:py-4 md:text-lg md:px-10",
                                "Get Started"
                            }
                        }
                        div { class: "mt-3 rounded-md shadow sm:mt-0 sm:ml-3",
                            a { href: "#", class: "w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-500 hover:bg-blue-400 md:py-4 md:text-lg md:px-10",
                                "Learn More"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FeatureCard(title: String, description: String, icon: String) -> Element {
    rsx! {
        div { class: "bg-white overflow-hidden shadow-lg rounded-lg",
            div { class: "p-6",
                div { class: "flex items-center",
                    div { class: "flex-shrink-0",
                        div { class: "w-8 h-8 bg-blue-500 rounded-md flex items-center justify-center",
                            span { class: "text-white font-bold", "{icon}" }
                        }
                    }
                    div { class: "ml-4",
                        h3 { class: "text-lg font-medium text-gray-900", "{title}" }
                    }
                }
                p { class: "mt-2 text-gray-600", "{description}" }
            }
        }
    }
}

#[component]
fn Features() -> Element {
    rsx! {
        section { class: "py-12 bg-gray-50",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "text-center",
                    h2 { class: "text-3xl font-extrabold text-gray-900",
                        "Key Features"
                    }
                    p { class: "mt-4 text-lg text-gray-600",
                        "Tailwind-RS provides everything you need for modern web development."
                    }
                }
                div { class: "mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3",
                    FeatureCard {
                        title: "Framework Integration".to_string(),
                        description: "Full support for Leptos, Yew, and Dioxus with zero overhead.".to_string(),
                        icon: "🚀".to_string()
                    }
                    FeatureCard {
                        title: "Performance Optimized".to_string(),
                        description: "Built for speed with advanced optimization strategies.".to_string(),
                        icon: "⚡".to_string()
                    }
                    FeatureCard {
                        title: "Type Safe".to_string(),
                        description: "Compile-time safety with Rust's type system.".to_string(),
                        icon: "🛡️".to_string()
                    }
                    FeatureCard {
                        title: "Production Ready".to_string(),
                        description: "Battle-tested in production environments.".to_string(),
                        icon: "🏭".to_string()
                    }
                    FeatureCard {
                        title: "Modern CSS".to_string(),
                        description: "Full Tailwind CSS class support with new parsers.".to_string(),
                        icon: "🎨".to_string()
                    }
                    FeatureCard {
                        title: "Developer Experience".to_string(),
                        description: "Excellent tooling and documentation.".to_string(),
                        icon: "👨‍💻".to_string()
                    }
                }
            }
        }
    }
}

#[component]
fn InteractiveDemo() -> Element {
    let mut counter = use_signal(|| 0);
    let mut is_hovered = use_signal(|| false);
    
    let button_class = if is_hovered() {
        "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition-colors duration-200 transform scale-105"
    } else {
        "bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded transition-colors duration-200"
    };
    
    rsx! {
        section { class: "py-12 bg-white",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "text-center",
                    h2 { class: "text-3xl font-extrabold text-gray-900",
                        "Interactive Demo"
                    }
                    p { class: "mt-4 text-lg text-gray-600",
                        "Try out the interactive features powered by Tailwind-RS and Dioxus."
                    }
                }
                div { class: "mt-12 flex justify-center",
                    div { class: "bg-gray-50 rounded-lg p-8 max-w-md w-full",
                        div { class: "text-center",
                            div { class: "text-6xl font-bold text-blue-600 mb-4",
                                "{counter()}"
                            }
                            button {
                                class: button_class,
                                onclick: move |_| counter.set(counter() + 1),
                                onmouseenter: move |_| is_hovered.set(true),
                                onmouseleave: move |_| is_hovered.set(false),
                                "Click me!"
                            }
                            p { class: "mt-4 text-sm text-gray-600",
                                "This button uses Tailwind-RS classes for styling and Dioxus for reactivity."
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "bg-gray-800 text-white",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                    div {
                        h3 { class: "text-lg font-semibold mb-4", "Tailwind-RS" }
                        p { class: "text-gray-400",
                            "A production-ready Rust implementation of Tailwind CSS."
                        }
                    }
                    div {
                        h3 { class: "text-lg font-semibold mb-4", "Links" }
                        ul { class: "space-y-2",
                            li { a { href: "#", class: "text-gray-400 hover:text-white", "Documentation" } }
                            li { a { href: "#", class: "text-gray-400 hover:text-white", "GitHub" } }
                            li { a { href: "#", class: "text-gray-400 hover:text-white", "Examples" } }
                        }
                    }
                    div {
                        h3 { class: "text-lg font-semibold mb-4", "Community" }
                        ul { class: "space-y-2",
                            li { a { href: "#", class: "text-gray-400 hover:text-white", "Discord" } }
                            li { a { href: "#", class: "text-gray-400 hover:text-white", "Twitter" } }
                            li { a { href: "#", class: "text-gray-400 hover:text-white", "Reddit" } }
                        }
                    }
                }
                div { class: "mt-8 pt-8 border-t border-gray-700 text-center text-gray-400",
                    p { "© 2025 Tailwind-RS. Built with Rust and Dioxus." }
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-50",
            Header {}
            Hero {}
            Features {}
            InteractiveDemo {}
            Footer {}
        }
    }
}

fn main() {
    dioxus::launch(App);
}
