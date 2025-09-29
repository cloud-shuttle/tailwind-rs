use leptos::prelude::*;

pub mod css_generator;
use css_generator::DemoCssGenerator;

#[cfg(target_arch = "wasm32")]
use web_sys::window;

// Include the generated CSS as a string
const GENERATED_CSS: &str = include_str!("../assets/generated.css");

/// Spectacular demo showcasing Tailwind-RS capabilities
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-purple-600 via-pink-500 to-blue-600 text-white" data-testid="app">
            <div class="container mx-auto px-4 py-16">
                // Hero Section
                <div class="text-center mb-20">
                    <h1 class="text-6xl font-bold mb-6 bg-gradient-to-r from-yellow-300 via-pink-300 to-cyan-300 bg-clip-text text-transparent">
                        "🚀 Tailwind-RS v4.1"
                    </h1>
                    <p class="text-xl opacity-90 max-w-2xl mx-auto">
                        "The most advanced WASM-powered CSS generation engine with 100% Tailwind compatibility"
                    </p>
                    <div class="mt-8 flex justify-center gap-4">
                        <button class="bg-white/20 hover:bg-white/30 backdrop-blur-sm border border-white/30 rounded-full px-8 py-3 font-semibold transition-all duration-300 hover:scale-105">
                            "Get Started"
                        </button>
                        <button class="bg-gradient-to-r from-pink-500 to-purple-600 hover:from-pink-600 hover:to-purple-700 rounded-full px-8 py-3 font-semibold transition-all duration-300 hover:scale-105">
                            "View Docs"
                        </button>
                    </div>
                </div>

                // Feature Showcase Grid
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-20">
                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20 hover:border-white/40 transition-all duration-300 hover:scale-105 hover:shadow-2xl">
                        <div class="text-4xl mb-4">"⚡"</div>
                        <h3 class="text-2xl font-bold mb-4">"Lightning Fast"</h3>
                        <p class="opacity-80 text-sm leading-relaxed">
                            "Compiled WebAssembly engine delivers sub-millisecond CSS generation with zero runtime overhead."
                        </p>
                    </div>

                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20 hover:border-white/40 transition-all duration-300 hover:scale-105 hover:shadow-2xl">
                        <div class="text-4xl mb-4">"🎨"</div>
                        <h3 class="text-2xl font-bold mb-4">"100% Compatible"</h3>
                        <p class="opacity-80 text-sm leading-relaxed">
                            "Full Tailwind CSS v4.1 support including advanced features, variants, and responsive design."
                        </p>
                    </div>

                    <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 border border-white/20 hover:border-white/40 transition-all duration-300 hover:scale-105 hover:shadow-2xl">
                        <div class="text-4xl mb-4">"🔮"</div>
                        <h3 class="text-2xl font-bold mb-4">"Future Proof"</h3>
                        <p class="opacity-80 text-sm leading-relaxed">
                            "Built with modern Rust architecture supporting plugins, themes, and custom utilities."
                        </p>
                    </div>
                </div>

                // Interactive Color Showcase
                <div class="bg-black/20 backdrop-blur-xl rounded-3xl p-12 border border-white/10 mb-20">
                    <h2 class="text-4xl font-bold mb-12 text-center bg-gradient-to-r from-cyan-300 to-blue-300 bg-clip-text text-transparent">
                        "🎨 Advanced Color System"
                    </h2>

                    <div class="grid grid-cols-2 md:grid-cols-4 gap-6 mb-12">
                        <div class="bg-gradient-to-br from-red-500 to-pink-500 rounded-xl p-6 text-center hover:scale-105 transition-transform">
                            <div class="text-white font-bold text-lg mb-2">"Reds"</div>
                            <div class="text-red-100 text-sm">"500 shades"</div>
                        </div>
                        <div class="bg-gradient-to-br from-blue-500 to-cyan-500 rounded-xl p-6 text-center hover:scale-105 transition-transform">
                            <div class="text-white font-bold text-lg mb-2">"Blues"</div>
                            <div class="text-blue-100 text-sm">"Ocean depths"</div>
                        </div>
                        <div class="bg-gradient-to-br from-green-500 to-emerald-500 rounded-xl p-6 text-center hover:scale-105 transition-transform">
                            <div class="text-white font-bold text-lg mb-2">"Greens"</div>
                            <div class="text-green-100 text-sm">"Nature's palette"</div>
                        </div>
                        <div class="bg-gradient-to-br from-purple-500 to-violet-500 rounded-xl p-6 text-center hover:scale-105 transition-transform">
                            <div class="text-white font-bold text-lg mb-2">"Purples"</div>
                            <div class="text-purple-100 text-sm">"Royal elegance"</div>
                        </div>
                    </div>

                    // Text Colors Showcase
                    <div class="text-center">
                        <div class="inline-flex flex-wrap gap-4 justify-center">
                            <span class="text-red-400 font-semibold">"Red"</span>
                            <span class="text-blue-400 font-semibold">"Blue"</span>
                            <span class="text-green-400 font-semibold">"Green"</span>
                            <span class="text-yellow-400 font-semibold">"Yellow"</span>
                            <span class="text-purple-400 font-semibold">"Purple"</span>
                            <span class="text-pink-400 font-semibold">"Pink"</span>
                            <span class="text-cyan-400 font-semibold">"Cyan"</span>
                            <span class="text-orange-400 font-semibold">"Orange"</span>
                        </div>
                    </div>
                </div>

                // Typography & Layout Demo
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 mb-20">
                    // Typography Section
                    <div class="bg-white/5 backdrop-blur-sm rounded-2xl p-8 border border-white/10">
                        <h3 class="text-2xl font-bold mb-6 text-cyan-300">"📝 Typography Scale"</h3>
                        <div class="space-y-4">
                            <p class="text-xs opacity-80">"Extra Small - The quick brown fox jumps over the lazy dog"</p>
                            <p class="text-sm opacity-85">"Small - The quick brown fox jumps over the lazy dog"</p>
                            <p class="text-base opacity-90">"Base - The quick brown fox jumps over the lazy dog"</p>
                            <p class="text-lg opacity-95">"Large - The quick brown fox jumps over the lazy dog"</p>
                            <p class="text-xl font-semibold">"Extra Large - The quick brown fox jumps over the lazy dog"</p>
                            <p class="text-2xl font-bold text-purple-300">"2XL - The quick brown fox jumps over the lazy dog"</p>
                        </div>
                    </div>

                    // Layout Section
                    <div class="bg-white/5 backdrop-blur-sm rounded-2xl p-8 border border-white/10">
                        <h3 class="text-2xl font-bold mb-6 text-pink-300">"📐 Layout System"</h3>
                        <div class="space-y-4">
                            <div class="flex items-center gap-4 p-4 bg-white/10 rounded-lg">
                                <div class="w-8 h-8 bg-blue-500 rounded-full flex-shrink-0"></div>
                                <div class="flex-1">
                                    <div class="font-semibold">"Flexbox Layout"</div>
                                    <div class="text-sm opacity-70">"Responsive alignment system"</div>
                                </div>
                            </div>
                            <div class="grid grid-cols-3 gap-2">
                                <div class="aspect-square bg-gradient-to-br from-purple-400 to-pink-400 rounded-lg flex items-center justify-center font-bold text-white">"1"</div>
                                <div class="aspect-square bg-gradient-to-br from-blue-400 to-cyan-400 rounded-lg flex items-center justify-center font-bold text-white">"2"</div>
                                <div class="aspect-square bg-gradient-to-br from-green-400 to-emerald-400 rounded-lg flex items-center justify-center font-bold text-white">"3"</div>
                            </div>
                        </div>
                    </div>
                </div>

                // Status Footer
                <div class="bg-gradient-to-r from-black/40 via-purple-900/20 to-black/40 backdrop-blur-xl rounded-3xl p-12 border border-white/20">
                    <div class="text-center mb-8">
                        <h2 class="text-3xl font-bold mb-4">"✅ Fully Functional Tailwind-RS"</h2>
                        <p class="text-lg opacity-80">"All major features working perfectly"</p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        <div class="text-center">
                            <div class="text-4xl mb-4 text-green-400">"🎨"</div>
                            <h4 class="font-bold text-lg mb-2">"Colors & Gradients"</h4>
                            <ul class="text-sm opacity-80 space-y-1">
                                <li>"✅ RGB & HSL colors"</li>
                                <li>"✅ Opacity variants"</li>
                                <li>"✅ Gradient backgrounds"</li>
                                <li>"✅ Backdrop filters"</li>
                            </ul>
                        </div>

                        <div class="text-center">
                            <div class="text-4xl mb-4 text-blue-400">"📱"</div>
                            <h4 class="font-bold text-lg mb-2">"Responsive & Interactive"</h4>
                            <ul class="text-sm opacity-80 space-y-1">
                                <li>"✅ Responsive breakpoints"</li>
                                <li>"✅ Hover/Focus states"</li>
                                <li>"✅ Transform animations"</li>
                                <li>"✅ Transition effects"</li>
                            </ul>
                        </div>

                        <div class="text-center">
                            <div class="text-4xl mb-4 text-purple-400">"🏗️"</div>
                            <h4 class="font-bold text-lg mb-2">"Layout & Typography"</h4>
                            <ul class="text-sm opacity-80 space-y-1">
                                <li>"✅ Grid & Flexbox"</li>
                                <li>"✅ Spacing system"</li>
                                <li>"✅ Font scales"</li>
                                <li>"✅ Border radius"</li>
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
