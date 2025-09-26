use leptos::prelude::*;
use leptos::mount::mount_to_body;
use leptos::attr::global::ClassAttribute;

mod memory_analysis;
mod performance_testing;
mod advanced_signal_management;
mod css_generator;
mod css_validation;
mod comprehensive_validation;
use memory_analysis::{MemoryAnalysisDemo, PerformanceTestDemo};
use performance_testing::PerformanceTestingDemo;
use advanced_signal_management::{AdvancedSignalManagementDemo, BatchedUpdatesDemo};
use css_generator::DemoCssGenerator;

/// Simple WASM-compatible demo component
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (name, set_name) = signal("Tailwind-RS".to_string());

    // Generate CSS on component mount
    let mut css_generator = DemoCssGenerator::new();
    let _ = css_generator.generate_all_css_files();

    view! {
        <Html lang="en">
            <Head>
                <Title text="Peter Hanssens - Data Engineer & Community Leader"/>
                <Meta charset="utf-8"/>
                <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <Meta name="description" content="Interactive demo showcasing Tailwind-RS v0.15.0 capabilities with Leptos"/>
                
                // Load generated CSS files
                <Link rel="stylesheet" href="/comprehensive-styles.css"/>
                <Link rel="stylesheet" href="/custom-styles.css"/>
                <Link rel="stylesheet" href="/generated-styles.css"/>
            </Head>
            <Body>
                <div class="min-h-screen bg-gray-100 dark:bg-gray-900">
            <div class="container mx-auto px-4 py-8">
                <h1 class="text-4xl font-bold text-center text-gray-800 dark:text-white mb-8">
                    "Tailwind-RS Leptos Demo"
                </h1>
                
                <div class="max-w-md mx-auto bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
                    <h2 class="text-2xl font-semibold text-gray-800 dark:text-white mb-4">
                        "Interactive Counter"
                    </h2>
                    
                    <div class="text-center mb-4">
                        <span class="text-6xl font-bold text-blue-600 dark:text-blue-400">
                            {move || count.get()}
                        </span>
                    </div>
                    
                    <div class="flex gap-2 justify-center mb-4">
                        <button
                            class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
                            on:click=move |_| set_count.update(|c| *c += 1)
                        >
                            "Increment"
                        </button>
                        <button
                            class="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors"
                            on:click=move |_| set_count.update(|c| *c -= 1)
                        >
                            "Decrement"
                        </button>
                        <button
                            class="px-4 py-2 bg-gray-500 hover:bg-gray-600 text-white rounded-lg transition-colors"
                            on:click=move |_| set_count.set(0)
                        >
                            "Reset"
                        </button>
                    </div>
                    
                    <div class="mt-6">
                        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                            "Name:"
                        </label>
                        <input
                            type="text"
                            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                            prop:value=move || name.get()
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                        />
                        <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
                            "Hello, " {move || name.get()} "!"
                        </p>
                    </div>
                </div>
                
                // Memory Analysis Demo
                <div class="mt-8">
                    <MemoryAnalysisDemo />
                </div>
                
                // Performance Test Demo
                <div class="mt-8">
                    <PerformanceTestDemo />
                </div>
                
                // Comprehensive Performance Testing Demo
                <div class="mt-8">
                    <PerformanceTestingDemo />
                </div>
                
                // Advanced Signal Management Demo
                <div class="mt-8">
                    <AdvancedSignalManagementDemo />
                </div>
                
                // Batched Updates Demo
                <div class="mt-8">
                    <BatchedUpdatesDemo />
                </div>
                
                <div class="mt-8 text-center">
                    <p class="text-gray-600 dark:text-gray-400">
                        "Built with Leptos v0.8.8, Tailwind-RS v0.15.0, and Signal Management"
                    </p>
                </div>
            </div>
        </div>
            </Body>
        </Html>
    }
}

/// Main entry point for the WASM demo
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}