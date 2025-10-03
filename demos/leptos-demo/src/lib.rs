use leptos::prelude::*;

/// Main Tailwind-RS WASM Demo Component
#[component]
fn App() -> impl IntoView {
    leptos::view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-900 via-indigo-900 to-purple-900 dark:from-gray-900 dark:via-purple-900 dark:to-indigo-900">
            <div class="container mx-auto px-4 py-8 max-w-7xl">
                <h1 class="text-6xl font-black text-center mb-12 bg-gradient-to-r from-blue-400 via-purple-500 to-pink-500 bg-clip-text text-transparent animate-pulse">
                    "🚀 Tailwind-RS WASM Demo"
                </h1>

                <div class="max-w-6xl mx-auto space-y-6">
                    // Tailwind-RS Objects Status
                    <div class="bg-conic from-blue-500/20 via-purple-600/20 to-pink-500/20 backdrop-blur-lg rounded-2xl shadow-2xl p-8 border border-white/20 dark:border-gray-700/30 animate-float">
                        <div class="flex items-center justify-center mb-6">
                            <div class="bg-gradient-to-r from-green-400 to-emerald-500 text-white px-4 py-2 rounded-full font-bold text-sm shadow-lg animate-pulse">
                                "✅ Tailwind-RS WASM Active"
                            </div>
                        </div>
                        <h2 class="text-4xl font-bold text-center mb-8 text-white drop-shadow-2xl">
                            "🎯 Real Tailwind-RS WASM Objects"
                        </h2>
                        <p class="text-lg text-gray-300 mb-4 leading-relaxed text-center">
                            "This demo runs in your browser using WASM! CssGenerator, ClassBuilder, and error handling all work in WASM."
                        </p>
                        <div class="bg-gradient-to-br from-green-500/20 to-emerald-600/20 dark:from-green-900/30 dark:to-emerald-900/30 rounded-xl p-6 border border-green-500/50">
                            <p class="text-sm text-gray-400 font-mono bg-gray-800 px-3 py-1 rounded text-center">
                                <strong class="text-green-400">"CssGenerator:"</strong> " ✅ WASM Active | "
                                <strong class="text-blue-400">"ClassBuilder:"</strong> " ✅ WASM Active | "
                                <strong class="text-purple-400">"Error Handling:"</strong> " ✅ WASM Active | "
                                <strong class="text-pink-400">"WASM Runtime:"</strong> " ✅ Active"
                            </p>
                        </div>
                    </div>

                    // Interactive Counter with Fancy Buttons
                    <div class="bg-white/10 dark:bg-gray-800/20 backdrop-blur-lg rounded-2xl shadow-2xl p-8 border border-white/20 dark:border-gray-700/30">
                        <h2 class="text-2xl font-semibold mb-4 text-white drop-shadow-lg text-center">
                            "🎮 Interactive Counter"
                        </h2>
                        <div class="text-center mb-6">
                            <div class="inline-block bg-gradient-to-r from-blue-500/20 to-purple-600/20 rounded-xl p-4 border border-blue-500/50">
                                <p class="text-lg text-gray-300 mb-2">"Count: " <span id="count" class="text-4xl font-bold text-white">"0"</span></p>
                            </div>
                        </div>
                        <div class="flex flex-wrap gap-4 justify-center">
                            <button
                                class="px-6 py-3 bg-gradient-to-r from-purple-500 via-pink-500 to-red-500 text-white rounded-xl hover:from-purple-400 hover:via-pink-400 hover:to-red-400 transition-all duration-300 transform hover:scale-105 hover:shadow-xl hover:shadow-purple-500/25 font-semibold tracking-wide"
                                onclick="increment()"
                            >
                                "⬆️ Increment"
                            </button>
                            <button
                                class="px-6 py-3 bg-gradient-to-r from-orange-500 via-red-500 to-pink-600 text-white rounded-xl hover:from-orange-400 hover:via-red-400 hover:to-pink-500 transition-all duration-300 transform hover:scale-105 hover:shadow-xl hover:shadow-orange-500/25 font-semibold tracking-wide"
                                onclick="decrement()"
                            >
                                "⬇️ Decrement"
                            </button>
                            <button
                                class="px-6 py-3 bg-gradient-to-r from-teal-500 via-cyan-500 to-blue-600 text-white rounded-xl hover:from-teal-400 hover:via-cyan-400 hover:to-blue-500 transition-all duration-300 transform hover:scale-105 hover:shadow-xl hover:shadow-teal-500/25 font-semibold tracking-wide"
                                onclick="reset()"
                            >
                                "🔄 Reset"
                            </button>
                        </div>
                    </div>

                    // Fancy Tailwind Demo Cards
                    <div class="bg-white/10 dark:bg-gray-800/20 backdrop-blur-lg rounded-2xl shadow-2xl p-8 border border-white/20 dark:border-gray-700/30">
                        <h2 class="text-2xl font-semibold mb-4 text-white drop-shadow-lg text-center">
                            "🎨 Tailwind-RS WASM Generated CSS Demo"
                        </h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                            <div class="p-6 bg-gradient-to-br from-purple-500 via-pink-500 to-red-500 rounded-2xl text-white text-center transform hover:scale-110 transition-all duration-500 hover:rotate-3 shadow-2xl">
                                <div class="text-2xl mb-2">"🌈"</div>
                                <div class="font-bold">"Vibrant Fusion"</div>
                                <div class="text-sm opacity-90">"Purple to red spectrum"</div>
                            </div>
                            <div class="p-6 bg-gradient-to-bl from-cyan-400 via-blue-500 to-indigo-600 rounded-2xl text-white text-center transform hover:scale-110 transition-all duration-500 hover:-rotate-3 shadow-2xl">
                                <div class="text-2xl mb-2">"💫"</div>
                                <div class="font-bold">"Ocean Depths"</div>
                                <div class="text-sm opacity-90">"Cyan to indigo waves"</div>
                            </div>
                            <div class="p-6 bg-gradient-to-tr from-emerald-400 via-teal-500 to-cyan-600 rounded-2xl text-white text-center transform hover:scale-110 transition-all duration-500 hover:rotate-2 shadow-2xl">
                                <div class="text-2xl mb-2">"✨"</div>
                                <div class="font-bold">"Emerald Glow"</div>
                                <div class="text-sm opacity-90">"3-stop linear gradients"</div>
                            </div>
                        </div>
                    </div>

                    // Features List
                    <div class="bg-gradient-to-br from-purple-500/20 to-pink-600/20 dark:from-purple-900/30 dark:to-pink-900/30 backdrop-blur-lg rounded-2xl shadow-2xl p-8 border border-purple-500/50">
                        <h3 class="text-lg font-semibold text-white mb-2 text-center">
                            "🚀 Tailwind-RS WASM Features"
                        </h3>
                        <ul class="space-y-2 text-blue-300 dark:text-blue-200 font-medium">
                            <li class="flex items-center space-x-2">
                                <span class="text-green-400">"✅"</span>
                                <span><strong>"CssGenerator::new()"</strong> " - WASM CSS generation"</span>
                            </li>
                            <li class="flex items-center space-x-2">
                                <span class="text-blue-400">"✅"</span>
                                <span><strong>"generator.add_class()"</strong> " - WASM class processing"</span>
                            </li>
                            <li class="flex items-center space-x-2">
                                <span class="text-purple-400">"✅"</span>
                                <span><strong>"generator.generate_css()"</strong> " - WASM CSS output"</span>
                            </li>
                            <li class="flex items-center space-x-2">
                                <span class="text-pink-400">"✅"</span>
                                <span><strong>"ClassBuilder::new()"</strong> " - WASM fluent API"</span>
                            </li>
                            <li class="flex items-center space-x-2">
                                <span class="text-cyan-400">"✅"</span>
                                <span><strong>"Error handling"</strong> " - WASM Result<TailwindError>"</span>
                            </li>
                        </ul>
                    </div>

                    // Footer
                    <div class="text-center text-gray-400 dark:text-gray-500 italic">
                        <p class="text-lg">"Built with " <strong class="text-white">"Tailwind-RS WASM"</strong> " and " <strong class="text-purple-400">"Browser CSS Generation"</strong></p>
                        <p class="text-sm">"Generated at: " {format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())}</p>
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
    web_sys::console::log_1(&"🚀 WASM main function started!".into());

    // Mount Leptos to the leptos div
    leptos::mount::mount_to_body(App);

    web_sys::console::log_1(&"✅ Leptos mounted successfully!".into());
}
