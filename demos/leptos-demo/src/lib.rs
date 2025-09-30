use leptos::prelude::*;

pub mod css_generator;
use css_generator::DemoCssGenerator;

#[cfg(target_arch = "wasm32")]
use web_sys::window;

// Include the generated CSS as a string
const GENERATED_CSS: &str = include_str!("../assets/generated.css");

/// Spectacular modern demo showcasing Tailwind-RS capabilities
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 text-white overflow-x-hidden" data-testid="app">
            <div class="absolute inset-0 bg-gradient-to-br from-purple-900/20 via-transparent to-cyan-900/20"></div>

            <div class="relative z-10 container mx-auto px-4 py-8">
                // Navigation
                <nav class="flex justify-between items-center mb-16 pt-8">
                    <div class="text-2xl font-bold bg-gradient-to-r from-cyan-400 to-purple-400 bg-clip-text text-transparent">
                        "Tailwind-RS"
                    </div>
                    <div class="hidden md:flex space-x-8">
                        <a href="#features" class="hover:text-cyan-400 transition-colors">"Features"</a>
                        <a href="#components" class="hover:text-cyan-400 transition-colors">"Components"</a>
                        <a href="#animations" class="hover:text-cyan-400 transition-colors">"Animations"</a>
                    </div>
                </nav>

                // Hero Section
                <div class="text-center mb-24 relative">
                    <div class="absolute inset-0 flex items-center justify-center">
                        <div class="w-96 h-96 bg-gradient-to-r from-purple-500/30 to-cyan-500/30 rounded-full blur-3xl animate-pulse"></div>
                    </div>
                    <div class="relative z-10">
                        <h1 class="text-7xl md:text-8xl font-black mb-8 bg-gradient-to-r from-cyan-300 via-purple-300 to-pink-300 bg-clip-text text-transparent leading-tight">
                            "TAILWIND"
                            <br/>
                            <span class="text-5xl md:text-6xl bg-gradient-to-r from-orange-400 to-red-400 bg-clip-text text-transparent">"RS"</span>
                        </h1>
                        <p class="text-xl md:text-2xl opacity-90 max-w-4xl mx-auto mb-12 leading-relaxed">
                            "The most advanced WebAssembly-powered CSS generation engine. Zero runtime overhead, 100% Tailwind CSS compatibility, and lightning-fast compilation."
                        </p>
                        <div class="flex flex-col sm:flex-row justify-center gap-6 mb-16">
                            <button class="group relative bg-gradient-to-r from-purple-600 to-cyan-600 hover:from-purple-500 hover:to-cyan-500 rounded-full px-12 py-4 font-bold text-lg transition-all duration-300 hover:scale-105 hover:shadow-2xl hover:shadow-purple-500/25 overflow-hidden">
                                <span class="relative z-10">"Get Started"</span>
                                <div class="absolute inset-0 bg-gradient-to-r from-cyan-600 to-purple-600 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                            </button>
                            <button class="border-2 border-white/20 hover:border-white/40 backdrop-blur-sm rounded-full px-12 py-4 font-semibold text-lg transition-all duration-300 hover:scale-105 hover:bg-white/10">
                                "View Source"
                            </button>
                        </div>

                        // Stats
                        <div class="grid grid-cols-2 md:grid-cols-4 gap-8 max-w-2xl mx-auto">
                            <div class="text-center">
                                <div class="text-3xl font-bold text-cyan-400 mb-2">"100%"</div>
                                <div class="text-sm opacity-80">"Tailwind Compatible"</div>
                            </div>
                            <div class="text-center">
                                <div class="text-3xl font-bold text-purple-400 mb-2">"<1ms"</div>
                                <div class="text-sm opacity-80">"CSS Generation"</div>
                            </div>
                            <div class="text-center">
                                <div class="text-3xl font-bold text-pink-400 mb-2">"0KB"</div>
                                <div class="text-sm opacity-80">"Runtime Overhead"</div>
                            </div>
                            <div class="text-center">
                                <div class="text-3xl font-bold text-orange-400 mb-2">"∞"</div>
                                <div class="text-sm opacity-80">"Possibilities"</div>
                            </div>
                        </div>
                    </div>
                </div>

                // Interactive Components Showcase
                <section id="components" class="mb-32">
                    <h2 class="text-5xl font-bold text-center mb-16 bg-gradient-to-r from-cyan-400 via-purple-400 to-pink-400 bg-clip-text text-transparent">
                        "🎨 Component Gallery"
                    </h2>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        // Card 1 - Gradient Card
                        <div class="group relative bg-gradient-to-br from-purple-500/20 via-pink-500/20 to-cyan-500/20 backdrop-blur-xl rounded-3xl p-8 border border-white/10 hover:border-white/30 transition-all duration-500 hover:scale-105 hover:-rotate-1 hover:shadow-2xl hover:shadow-purple-500/20 overflow-hidden">
                            <div class="absolute inset-0 bg-gradient-to-br from-purple-600/10 to-cyan-600/10 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
                            <div class="relative z-10">
                                <div class="w-16 h-16 bg-gradient-to-br from-purple-400 to-pink-400 rounded-2xl mb-6 flex items-center justify-center text-2xl">
                                    "🎭"
                                </div>
                                <h3 class="text-2xl font-bold mb-4">"Gradient Magic"</h3>
                                <p class="opacity-80 text-sm leading-relaxed mb-6">
                                    "Beautiful gradient backgrounds with backdrop blur effects and smooth hover animations."
                                </p>
                                <div class="flex flex-wrap gap-2">
                                    <span class="px-3 py-1 bg-purple-500/20 rounded-full text-xs font-medium">"backdrop-blur"</span>
                                    <span class="px-3 py-1 bg-pink-500/20 rounded-full text-xs font-medium">"gradients"</span>
                                </div>
                            </div>
                        </div>

                        // Card 2 - Interactive Button
                        <div class="group bg-gradient-to-r from-cyan-500/20 via-blue-500/20 to-purple-500/20 backdrop-blur-xl rounded-3xl p-8 border border-white/10 hover:border-cyan-400/30 transition-all duration-500 hover:scale-105 hover:shadow-2xl hover:shadow-cyan-500/20">
                            <div class="text-center">
                                <div class="w-16 h-16 bg-gradient-to-br from-cyan-400 to-blue-400 rounded-2xl mb-6 mx-auto flex items-center justify-center text-2xl group-hover:animate-bounce">
                                    "⚡"
                                </div>
                                <h3 class="text-2xl font-bold mb-4">"Interactive Elements"</h3>
                                <p class="opacity-80 text-sm leading-relaxed mb-6">
                                    "Hover effects, transforms, and smooth transitions powered by pure CSS."
                                </p>
                                <button class="w-full bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 rounded-xl py-3 font-semibold transition-all duration-300 hover:scale-105 hover:shadow-lg">
                                    "Click Me!"
                                </button>
                            </div>
                        </div>

                        // Card 3 - Typography Showcase
                        <div class="bg-gradient-to-br from-pink-500/20 via-purple-500/20 to-indigo-500/20 backdrop-blur-xl rounded-3xl p-8 border border-white/10">
                            <div class="space-y-4">
                                <h4 class="text-sm font-bold text-pink-400 uppercase tracking-wider">"Typography Scale"</h4>
                                <div class="space-y-2">
                                    <p class="text-xs opacity-60">"Extra Small"</p>
                                    <p class="text-sm opacity-70">"Small Text"</p>
                                    <p class="text-base opacity-80">"Base Size"</p>
                                    <p class="text-lg opacity-90">"Large Text"</p>
                                    <p class="text-xl font-semibold">"Extra Large"</p>
                                    <p class="text-2xl font-bold text-purple-300">"Display Size"</p>
                                </div>
                            </div>
                        </div>

                        // Card 4 - Color Palette
                        <div class="bg-black/20 backdrop-blur-xl rounded-3xl p-8 border border-white/10">
                            <h4 class="text-lg font-bold mb-6 text-center">"Color Harmony"</h4>
                            <div class="grid grid-cols-4 gap-3 mb-6">
                                <div class="aspect-square bg-gradient-to-br from-red-500 to-pink-500 rounded-lg"></div>
                                <div class="aspect-square bg-gradient-to-br from-yellow-500 to-orange-500 rounded-lg"></div>
                                <div class="aspect-square bg-gradient-to-br from-green-500 to-emerald-500 rounded-lg"></div>
                                <div class="aspect-square bg-gradient-to-br from-blue-500 to-cyan-500 rounded-lg"></div>
                                <div class="aspect-square bg-gradient-to-br from-purple-500 to-violet-500 rounded-lg"></div>
                                <div class="aspect-square bg-gradient-to-br from-pink-500 to-rose-500 rounded-lg"></div>
                                <div class="aspect-square bg-gradient-to-br from-indigo-500 to-blue-500 rounded-lg"></div>
                                <div class="aspect-square bg-gradient-to-br from-teal-500 to-cyan-500 rounded-lg"></div>
                            </div>
                            <p class="text-sm opacity-80 text-center">
                                "Full spectrum of Tailwind colors with opacity variants"
                            </p>
                        </div>

                        // Card 5 - Animation Gallery
                        <div class="bg-gradient-to-br from-orange-500/20 via-red-500/20 to-pink-500/20 backdrop-blur-xl rounded-3xl p-8 border border-white/10">
                            <div class="text-center">
                                <div class="w-16 h-16 bg-gradient-to-br from-orange-400 to-pink-400 rounded-2xl mb-6 mx-auto flex items-center justify-center">
                                    <div class="w-8 h-8 bg-white rounded-full animate-ping"></div>
                                </div>
                                <h3 class="text-xl font-bold mb-4">"Animation Engine"</h3>
                                <div class="flex justify-center space-x-4">
                                    <div class="w-3 h-3 bg-cyan-400 rounded-full animate-bounce" style="animation-delay: 0s"></div>
                                    <div class="w-3 h-3 bg-purple-400 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                                    <div class="w-3 h-3 bg-pink-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
                                </div>
                            </div>
                        </div>

                        // Card 6 - Layout System
                        <div class="bg-gradient-to-br from-green-500/20 via-teal-500/20 to-cyan-500/20 backdrop-blur-xl rounded-3xl p-8 border border-white/10">
                            <h4 class="text-lg font-bold mb-6">"📐 Layout System"</h4>
                            <div class="grid grid-cols-3 gap-2 mb-4">
                                <div class="aspect-square bg-gradient-to-br from-green-400 to-teal-400 rounded-lg flex items-center justify-center font-bold text-white text-xs">"1"</div>
                                <div class="aspect-square bg-gradient-to-br from-teal-400 to-cyan-400 rounded-lg flex items-center justify-center font-bold text-white text-xs">"2"</div>
                                <div class="aspect-square bg-gradient-to-br from-cyan-400 to-blue-400 rounded-lg flex items-center justify-center font-bold text-white text-xs">"3"</div>
                            </div>
                            <div class="flex items-center gap-3">
                                <div class="w-8 h-8 bg-green-500 rounded-full flex-shrink-0"></div>
                                <div class="flex-1">
                                    <div class="font-semibold text-sm">"CSS Grid & Flexbox"</div>
                                    <div class="text-xs opacity-70">"Modern layout systems"</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Advanced Features Section
                <section id="features" class="mb-32">
                    <div class="text-center mb-16">
                        <h2 class="text-5xl font-bold mb-8 bg-gradient-to-r from-green-400 via-blue-400 to-purple-400 bg-clip-text text-transparent">
                            "🚀 Advanced Features"
                        </h2>
                        <p class="text-xl opacity-80 max-w-3xl mx-auto">
                            "Experience the full power of Tailwind-RS with cutting-edge CSS features and WebAssembly performance"
                        </p>
                    </div>

                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                        // Left Column - Effects & Filters
                        <div class="space-y-8">
                            <div class="bg-gradient-to-r from-purple-500/10 via-pink-500/10 to-cyan-500/10 backdrop-blur-xl rounded-3xl p-8 border border-white/10">
                                <h3 class="text-3xl font-bold mb-6 text-purple-300">"✨ Visual Effects"</h3>
                                <div class="grid grid-cols-2 gap-6">
                                    <div class="text-center">
                                        <div class="w-20 h-20 bg-gradient-to-br from-purple-400 to-pink-400 rounded-2xl mx-auto mb-4 shadow-2xl shadow-purple-500/30"></div>
                                        <div class="font-semibold mb-2">"Box Shadow"</div>
                                        <div class="text-sm opacity-70">"Layered shadows with color"</div>
                                    </div>
                                    <div class="text-center">
                                        <div class="w-20 h-20 bg-gradient-to-br from-cyan-400 to-blue-400 rounded-2xl mx-auto mb-4 backdrop-blur-sm border border-white/20"></div>
                                        <div class="font-semibold mb-2">"Backdrop Blur"</div>
                                        <div class="text-sm opacity-70">"Frosted glass effects"</div>
                                    </div>
                                </div>
                            </div>

                            <div class="bg-gradient-to-r from-cyan-500/10 via-blue-500/10 to-indigo-500/10 backdrop-blur-xl rounded-3xl p-8 border border-white/10">
                                <h3 class="text-3xl font-bold mb-6 text-cyan-300">"🎭 Hover Interactions"</h3>
                                <div class="space-y-4">
                                    <div class="group flex items-center gap-4 p-4 bg-white/5 rounded-xl hover:bg-white/10 transition-all duration-300 cursor-pointer">
                                        <div class="w-10 h-10 bg-gradient-to-br from-cyan-400 to-blue-400 rounded-xl group-hover:rotate-12 transition-transform duration-300"></div>
                                        <div>
                                            <div class="font-semibold group-hover:text-cyan-300 transition-colors">"Scale & Rotate"</div>
                                            <div class="text-sm opacity-70">"Smooth transform effects"</div>
                                        </div>
                                    </div>
                                    <div class="group flex items-center gap-4 p-4 bg-white/5 rounded-xl hover:bg-white/10 transition-all duration-300 cursor-pointer">
                                        <div class="w-10 h-10 bg-gradient-to-br from-purple-400 to-pink-400 rounded-xl group-hover:shadow-2xl group-hover:shadow-purple-500/50 transition-all duration-300"></div>
                                        <div>
                                            <div class="font-semibold group-hover:text-purple-300 transition-colors">"Glow Effect"</div>
                                            <div class="text-sm opacity-70">"Colored shadow animations"</div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Right Column - Responsive & Advanced
                        <div class="space-y-8">
                            <div class="bg-gradient-to-r from-pink-500/10 via-purple-500/10 to-indigo-500/10 backdrop-blur-xl rounded-3xl p-8 border border-white/10">
                                <h3 class="text-3xl font-bold mb-6 text-pink-300">"📱 Responsive Design"</h3>
                                <div class="space-y-4">
                                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                                        <div class="aspect-square bg-gradient-to-br from-red-500/50 to-pink-500/50 rounded-xl flex items-center justify-center font-bold text-white text-sm">
                                            "Mobile"
                                        </div>
                                        <div class="aspect-square bg-gradient-to-br from-yellow-500/50 to-orange-500/50 rounded-xl flex items-center justify-center font-bold text-white text-sm">
                                            "Tablet"
                                        </div>
                                        <div class="aspect-square bg-gradient-to-br from-green-500/50 to-teal-500/50 rounded-xl flex items-center justify-center font-bold text-white text-sm">
                                            "Desktop"
                                        </div>
                                    </div>
                                    <p class="text-sm opacity-80">
                                        "Seamless responsive breakpoints from mobile to desktop with fluid layouts"
                                    </p>
                                </div>
                            </div>

                            <div class="bg-gradient-to-r from-orange-500/10 via-red-500/10 to-pink-500/10 backdrop-blur-xl rounded-3xl p-8 border border-white/10">
                                <h3 class="text-3xl font-bold mb-6 text-orange-300">"⚡ Performance"</h3>
                                <div class="space-y-6">
                                    <div class="flex items-center justify-between">
                                        <span class="font-semibold">"CSS Generation"</span>
                                        <div class="flex items-center gap-2">
                                            <div class="w-24 h-2 bg-white/20 rounded-full overflow-hidden">
                                                <div class="w-full h-full bg-gradient-to-r from-green-400 to-green-500 rounded-full animate-pulse"></div>
                                            </div>
                                            <span class="text-green-400 font-bold">"<1ms"</span>
                                        </div>
                                    </div>
                                    <div class="flex items-center justify-between">
                                        <span class="font-semibold">"Bundle Size"</span>
                                        <div class="flex items-center gap-2">
                                            <div class="w-24 h-2 bg-white/20 rounded-full overflow-hidden">
                                                <div class="w-full h-full bg-gradient-to-r from-blue-400 to-blue-500 rounded-full"></div>
                                            </div>
                                            <span class="text-blue-400 font-bold">"0KB"</span>
                                        </div>
                                    </div>
                                    <div class="flex items-center justify-between">
                                        <span class="font-semibold">"Compatibility"</span>
                                        <div class="flex items-center gap-2">
                                            <div class="w-24 h-2 bg-white/20 rounded-full overflow-hidden">
                                                <div class="w-full h-full bg-gradient-to-r from-purple-400 to-purple-500 rounded-full"></div>
                                            </div>
                                            <span class="text-purple-400 font-bold">"100%"</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Call to Action
                <div class="text-center mb-16">
                    <div class="bg-gradient-to-r from-purple-600/20 via-pink-600/20 to-cyan-600/20 backdrop-blur-xl rounded-3xl p-12 border border-white/20 max-w-4xl mx-auto">
                        <h2 class="text-4xl font-bold mb-6 bg-gradient-to-r from-cyan-300 to-purple-300 bg-clip-text text-transparent">
                            "Ready to Experience the Future?"
                        </h2>
                        <p class="text-xl opacity-90 mb-8">
                            "Join thousands of developers using Tailwind-RS for lightning-fast, zero-overhead CSS generation"
                        </p>
                        <div class="flex flex-col sm:flex-row justify-center gap-6">
                            <button class="bg-gradient-to-r from-purple-600 to-cyan-600 hover:from-purple-500 hover:to-cyan-500 rounded-full px-12 py-4 font-bold text-lg transition-all duration-300 hover:scale-105 hover:shadow-2xl hover:shadow-purple-500/25">
                                "Start Building →"
                            </button>
                            <a href="https://github.com/peterhanssens/tailwind-rs" class="border-2 border-white/30 hover:border-white/50 rounded-full px-12 py-4 font-semibold text-lg transition-all duration-300 hover:scale-105 hover:bg-white/10 flex items-center justify-center gap-3">
                                <span>"View on GitHub"</span>
                                <span>"⭐"</span>
                            </a>
                        </div>
                    </div>
                </div>

                // Footer
                <footer class="text-center py-12 border-t border-white/10">
                    <div class="flex flex-col md:flex-row justify-between items-center gap-6">
                        <div class="text-2xl font-bold bg-gradient-to-r from-cyan-400 to-purple-400 bg-clip-text text-transparent">
                            "Tailwind-RS"
                        </div>
                        <div class="flex gap-6 text-sm opacity-80">
                            <a href="#" class="hover:text-cyan-400 transition-colors">"Documentation"</a>
                            <a href="#" class="hover:text-cyan-400 transition-colors">"Examples"</a>
                            <a href="#" class="hover:text-cyan-400 transition-colors">"GitHub"</a>
                        </div>
                        <div class="text-sm opacity-60">
                            "Built with ❤️ using Rust & WebAssembly"
                        </div>
                    </div>
                </footer>
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
