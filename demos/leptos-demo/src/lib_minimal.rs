//! Minimal WASM-compatible Leptos demo
//! This version avoids problematic dependencies that cause WASM_BIGINT issues

use leptos::prelude::*;
use leptos::attr::global::ClassAttribute;

/// Simple WASM-compatible demo component
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (name, set_name) = signal("Tailwind-RS".to_string());

    view! {
        <div class="min-h-screen bg-gray-100 dark:bg-gray-900">
            <div class="container mx-auto px-4 py-8">
                <h1 class="text-4xl font-bold text-center mb-8 text-gray-800 dark:text-white">
                    "Tailwind-RS Leptos Demo"
                </h1>
                
                <div class="max-w-2xl mx-auto space-y-6">
                    // Basic Counter Demo
                    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
                        <h2 class="text-2xl font-semibold mb-4 text-gray-800 dark:text-white">
                            "Counter Demo"
                        </h2>
                        <p class="text-lg mb-4 text-gray-600 dark:text-gray-300">
                            "Count: " {move || count.get()}
                        </p>
                        <div class="space-x-4">
                            <button
                                class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                                on:click=move |_| set_count.update(|c| *c += 1)
                            >
                                "Increment"
                            </button>
                            <button
                                class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition-colors"
                                on:click=move |_| set_count.update(|c| *c -= 1)
                            >
                                "Decrement"
                            </button>
                            <button
                                class="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600 transition-colors"
                                on:click=move |_| set_count.set(0)
                            >
                                "Reset"
                            </button>
                        </div>
                    </div>

                    // Name Input Demo
                    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
                        <h2 class="text-2xl font-semibold mb-4 text-gray-800 dark:text-white">
                            "Name Input Demo"
                        </h2>
                        <p class="text-lg mb-4 text-gray-600 dark:text-gray-300">
                            "Hello, " {move || name.get()} "!"
                        </p>
                        <input
                            type="text"
                            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
                            placeholder="Enter your name"
                            prop:value=name
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                        />
                    </div>

                    // Tailwind Classes Demo
                    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
                        <h2 class="text-2xl font-semibold mb-4 text-gray-800 dark:text-white">
                            "Tailwind Classes Demo"
                        </h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                            <div class="p-4 bg-gradient-to-r from-purple-400 to-pink-400 rounded-lg text-white text-center">
                                "Gradient Card"
                            </div>
                            <div class="p-4 bg-blue-500 rounded-lg text-white text-center transform hover:scale-105 transition-transform">
                                "Hover Effect"
                            </div>
                            <div class="p-4 bg-green-500 rounded-lg text-white text-center shadow-lg">
                                "Shadow Card"
                            </div>
                        </div>
                    </div>

                    <div class="text-center text-gray-600 dark:text-gray-400">
                        <p>"Built with Leptos v0.8.8 and Tailwind-RS"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Main entry point for the WASM demo
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
