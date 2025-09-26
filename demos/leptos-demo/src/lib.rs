use leptos::mount::mount_to_body;
use leptos::prelude::*;

mod css_generator;
use css_generator::DemoCssGenerator;

/// Simple demo that actually uses Tailwind-RS core functionality
#[component]
fn App() -> impl IntoView {
    let (dynamic_classes, set_dynamic_classes) =
        signal("bg-blue-500 text-white p-4 rounded-lg".to_string());
    let (generated_css, set_generated_css) = signal(String::new());

    // Actually use Tailwind-RS core functionality
    let update_css = move || {
        let mut generator = DemoCssGenerator::new();
        let classes = vec![
            "bg-blue-500",
            "text-white",
            "p-4",
            "rounded-lg",
            "bg-red-500",
            "text-black",
            "p-2",
            "rounded",
            "bg-green-500",
            "text-white",
            "p-6",
            "rounded-xl",
        ];

        for class in classes {
            let _ = generator.generator.add_class(class);
        }

        let css = generator.generator.generate_css();
        set_generated_css.set(css);
    };

    // Update CSS when component mounts
    update_css();

    view! {
        <div class="min-h-screen bg-gray-100" data-testid="app">
            <div class="container mx-auto px-4 py-8">
                <h1 class="text-4xl font-bold text-center text-gray-800 mb-8">
                    "Tailwind-RS Demo - Actually Using Core Models!"
                </h1>

                <div class="bg-white rounded-lg shadow-lg p-6 mb-8">
                    <h2 class="text-2xl font-semibold text-gray-800 mb-4">"Dynamic Class Preview"</h2>
                    <div class=format!("w-full h-32 rounded-lg border-2 border-dashed flex items-center justify-center text-gray-500 {}", dynamic_classes.get())>
                        "Preview Area"
                    </div>

                    <div class="mt-4">
                        <label class="block text-sm font-medium text-gray-700 mb-2">
                            "Enter Tailwind classes:"
                        </label>
                        <input
                            type="text"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="Enter Tailwind classes"
                            prop:value=dynamic_classes
                            on:input=move |ev| set_dynamic_classes.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                <div class="bg-white rounded-lg shadow-lg p-6 mb-8">
                    <h2 class="text-2xl font-semibold text-gray-800 mb-4">"Generated CSS (from Tailwind-RS Core)"</h2>
                    <pre class="bg-gray-100 p-4 rounded text-sm overflow-x-auto">
                        {generated_css.get()}
                    </pre>
                </div>

                <div class="bg-white rounded-lg shadow-lg p-6">
                    <h2 class="text-2xl font-semibold text-gray-800 mb-4">"Current Classes"</h2>
                    <code class="text-sm text-gray-700 bg-gray-100 px-2 py-1 rounded">
                        {dynamic_classes.get()}
                    </code>
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
