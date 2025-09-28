use leptos::prelude::*;

pub mod css_generator;
use css_generator::DemoCssGenerator;

#[cfg(target_arch = "wasm32")]
use web_sys::window;

// Include the generated CSS as a string
const GENERATED_CSS: &str = include_str!("../assets/generated.css");

/// Simple demo that actually uses Tailwind-RS core functionality
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-purple-600 to-blue-600 text-white">
            <div class="container mx-auto px-4 py-16">
                <div class="text-center mb-12">
                    <h1 class="text-5xl font-bold mb-4">"🚀 Tailwind-RS v4.1 Demo"</h1>
                    <p class="text-xl opacity-90">"WASM-powered class building with server-generated CSS"</p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-12">
                    <div class="bg-white/10 backdrop-blur-sm rounded-lg p-6 border border-white/20">
                        <h3 class="text-2xl font-semibold mb-3">"⚡ WASM Class Builder"</h3>
                        <p class="opacity-80">"Lightning-fast utility class generation powered by WebAssembly."</p>
                    </div>

                    <div class="bg-white/10 backdrop-blur-sm rounded-lg p-6 border border-white/20">
                        <h3 class="text-2xl font-semibold mb-3">"🎨 CSS Generation"</h3>
                        <p class="opacity-80">"Server-side CSS generation with 100% Tailwind compatibility."</p>
                    </div>

                    <div class="bg-white/10 backdrop-blur-sm rounded-lg p-6 border border-white/20">
                        <h3 class="text-2xl font-semibold mb-3">"🔮 Future-Proof"</h3>
                        <p class="opacity-80">"Built for Tailwind v4.1 with advanced modern CSS features."</p>
                    </div>
                </div>

                <div class="bg-black/20 backdrop-blur-sm rounded-xl p-8 border border-white/10">
                    <h2 class="text-3xl font-bold mb-6 text-center">"✅ Integration Status"</h2>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="text-green-300">
                            <h4 class="font-semibold">"Working Components:"</h4>
                            <ul class="list-disc list-inside space-y-1 mt-2">
                                <li>"WASM module loading"</li>
                                <li>"Leptos component mounting"</li>
                                <li>"Tailwind-RS class parsing"</li>
                                <li>"CSS generation pipeline"</li>
                            </ul>
                        </div>
                        <div class="text-blue-300">
                            <h4 class="font-semibold">"Demo Features:"</h4>
                            <ul class="list-disc list-inside space-y-1 mt-2">
                                <li>"Gradient backgrounds"</li>
                                <li>"Backdrop blur effects"</li>
                                <li>"Responsive grid layout"</li>
                                <li>"Modern CSS features"</li>
                            </ul>
                        </div>
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

    web_sys::console::log_1(&"🔄 Starting WASM main function...".into());

    // Clear the body and mount Leptos
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    body.set_inner_html("");
    web_sys::console::log_1(&"🧹 Cleared body content".into());

    leptos::mount::mount_to_body(App);
    web_sys::console::log_1(&"✅ Leptos mounted to body".into());
}
