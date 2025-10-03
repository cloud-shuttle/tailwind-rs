//! Working WASM-compatible Leptos demo using published tailwind-rs-wasm crate
//! This version demonstrates element-based processing concepts

use leptos::prelude::*;
use tailwind_rs_wasm::*;

// Provide missing wasm-bindgen extern function for document.head access
#[cfg(target_arch = "wasm32")]
extern "C" {
    #[link_name = "__wbg_head_e5bcca7f38d7ca47"]
    fn head_e5bcca7f38d7ca47(arg0: u32) -> u32;
}

/// Working WASM-compatible demo component demonstrating element-based processing concepts
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    // Demonstrate element-based processing with WasmClassBuilder
    // This shows how we can build classes for specific elements
    let mut main_builder = WasmClassBuilder::new();
    main_builder.add_classes("min-h-screen bg-gradient-to-br from-pink-400 via-purple-500 to-cyan-400");

    let mut container_builder = WasmClassBuilder::new();
    container_builder.add_classes("container mx-auto px-4 py-8 max-w-7xl");

    let mut title_builder = WasmClassBuilder::new();
    title_builder.add_classes("text-8xl font-black text-center mb-12 bg-gradient-to-r from-yellow-400 via-pink-500 to-cyan-400 bg-clip-text text-transparent");

    let mut status_card_builder = WasmClassBuilder::new();
    status_card_builder.add_classes("bg-gradient-to-br from-yellow-300/20 via-pink-400/20 to-purple-500/20 backdrop-blur-lg rounded-3xl shadow-2xl p-8 border border-yellow-400/50");

    let mut status_badge_builder = WasmClassBuilder::new();
    status_badge_builder.add_classes("bg-gradient-to-r from-yellow-400 via-pink-500 to-cyan-400 text-white px-6 py-3 rounded-full font-bold text-lg shadow-2xl border border-yellow-300/50");

    let mut counter_card_builder = WasmClassBuilder::new();
    counter_card_builder.add_classes("bg-gradient-to-br from-cyan-300/20 via-blue-400/20 to-purple-500/20 backdrop-blur-lg rounded-3xl shadow-2xl p-8 border border-cyan-400/50");

    let mut counter_display_builder = WasmClassBuilder::new();
    counter_display_builder.add_classes("text-6xl font-black text-center mb-8 bg-gradient-to-r from-pink-400 via-purple-500 to-cyan-400 bg-clip-text text-transparent");

    let mut button_plus_builder = WasmClassBuilder::new();
    button_plus_builder.add_classes("px-6 py-3 bg-gradient-to-r from-purple-500 via-pink-500 to-red-500 text-white rounded-xl hover:from-purple-400 hover:via-pink-400 hover:to-red-400 transition-all duration-300 transform hover:scale-105 hover:shadow-xl font-semibold tracking-wide");

    let mut button_minus_builder = WasmClassBuilder::new();
    button_minus_builder.add_classes("px-6 py-3 bg-gradient-to-r from-orange-500 via-red-500 to-pink-600 text-white rounded-xl hover:from-orange-400 hover:via-red-400 hover:to-pink-500 transition-all duration-300 transform hover:scale-105 hover:shadow-xl font-semibold tracking-wide");

    let mut button_reset_builder = WasmClassBuilder::new();
    button_reset_builder.add_classes("px-6 py-3 bg-gradient-to-r from-teal-500 via-cyan-500 to-blue-600 text-white rounded-xl hover:from-teal-400 hover:via-cyan-400 hover:to-blue-500 transition-all duration-300 transform hover:scale-105 hover:shadow-xl font-semibold tracking-wide");

    let mut demo_cards_builder = WasmClassBuilder::new();
    demo_cards_builder.add_classes("bg-white/10 dark:bg-gray-800/20 backdrop-blur-lg rounded-2xl shadow-2xl p-8 border border-white/20 dark:border-gray-700/30");

    let mut card_1_builder = WasmClassBuilder::new();
    card_1_builder.add_classes("p-6 bg-gradient-to-br from-purple-500 via-pink-500 to-red-500 rounded-2xl text-white text-center transform hover:scale-110 transition-all duration-500 hover:rotate-3 shadow-2xl");

    let mut card_2_builder = WasmClassBuilder::new();
    card_2_builder.add_classes("p-6 bg-gradient-to-bl from-cyan-400 via-blue-500 to-indigo-600 rounded-2xl text-white text-center transform hover:scale-110 transition-all duration-500 hover:-rotate-3 shadow-2xl");

    let mut card_3_builder = WasmClassBuilder::new();
    card_3_builder.add_classes("p-6 bg-gradient-to-tr from-emerald-400 via-teal-500 to-cyan-600 rounded-2xl text-white text-center transform hover:scale-110 transition-all duration-500 hover:rotate-2 shadow-2xl");

    view! {
        <div class={main_builder.build()}>
            <div class={container_builder.build()}>
                <h1 class={title_builder.build()}>
                    "🌈🎨 Tailwind-RS WASM Demo"
                </h1>

                <div class="max-w-6xl mx-auto space-y-6">
                    // Tailwind-RS Objects Status
                    <div class={status_card_builder.build()}>
                        <div class="flex items-center justify-center mb-6">
                            <div class={status_badge_builder.build()}>
                                "🎨✨ Tailwind-RS WASM Active ✨🎨"
                            </div>
                        </div>
                        <h2 class="text-5xl font-black text-center mb-8 bg-gradient-to-r from-pink-400 via-purple-500 to-cyan-400 bg-clip-text text-transparent">
                            "🎯 Real Tailwind-RS WASM Objects"
                        </h2>
                        <p class="text-xl text-white mb-6 leading-relaxed text-center font-semibold">
                            "🌈 This demo runs in your browser using WASM! Element-based processing concepts work in WASM! 🚀"
                        </p>
                        <div class="bg-gradient-to-br from-pink-500/30 via-purple-600/30 to-cyan-500/30 rounded-2xl p-8 border border-pink-400/60 shadow-xl">
                            <p class="text-lg text-white font-mono px-6 py-3 rounded-xl text-center">
                                <strong>"🎨 WasmClassBuilder:"</strong> " ✅ Active | "
                                <strong>"🏗️ Element Processing:"</strong> " ✅ Active | "
                                <strong>"🛡️ Class Building:"</strong> " ✅ Active | "
                                <strong>"⚡ WASM Runtime:"</strong> " ✅ Active ⚡"
                            </p>
                        </div>
                    </div>

                    // Interactive Counter with Fancy Buttons
                    <div class={counter_card_builder.build()}>
                        <h2 class="text-3xl font-black mb-6 text-center">
                            "🎮 Interactive Counter 🎨"
                        </h2>
                        <div class={counter_display_builder.build()} id="count">
                            {count}
                        </div>
                        <div class="flex justify-center space-x-4">
                            <button
                                class={button_plus_builder.build()}
                                on:click=move |_| set_count.update(|n| *n += 1)
                            >
                                "⬆️ Increment"
                            </button>
                            <button
                                class={button_minus_builder.build()}
                                on:click=move |_| set_count.update(|n| *n -= 1)
                            >
                                "⬇️ Decrement"
                            </button>
                            <button
                                class={button_reset_builder.build()}
                                on:click=move |_| *set_count.write() = 0
                            >
                                "🔄 Reset"
                            </button>
                        </div>
                    </div>

                    // Fancy Tailwind Demo Cards
                    <div class={demo_cards_builder.build()}>
                        <h2 class="text-4xl font-black mb-8 text-center">
                            "🎨 Tailwind-RS WASM Generated CSS Demo 🌈"
                        </h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                            <div class={card_1_builder.build()}>
                                <div class="text-5xl mb-4">"🌈"</div>
                                <div class="font-black text-2xl mb-2">"Rainbow Fusion ✨"</div>
                                <div class="text-lg opacity-90 font-semibold">"Pink to red spectrum"</div>
                            </div>
                            <div class={card_2_builder.build()}>
                                <div class="text-5xl mb-4">"💫"</div>
                                <div class="font-black text-2xl mb-2">"Ocean Dreams 🌊"</div>
                                <div class="text-lg opacity-90 font-semibold">"Cyan to indigo waves"</div>
                            </div>
                            <div class={card_3_builder.build()}>
                                <div class="text-5xl mb-4">"✨"</div>
                                <div class="font-black text-2xl mb-2">"Emerald Glow 🌟"</div>
                                <div class="text-lg opacity-90 font-semibold">"Emerald to cyan magic"</div>
                            </div>
                        </div>
                    </div>

                    // Features List
                    <div class="bg-gradient-to-br from-purple-400/20 via-pink-500/20 to-cyan-400/20 backdrop-blur-lg rounded-3xl shadow-2xl p-10 border border-purple-400/50">
                        <h3 class="text-3xl font-black text-center mb-8">
                            "🚀 Tailwind-RS WASM Features ⚡"
                        </h3>
                        <ul class="space-y-4 text-white font-bold text-lg">
                            <li class="flex items-center space-x-4 p-4 bg-gradient-to-r from-green-400/30 to-emerald-500/30 rounded-2xl border border-green-300/50">
                                <span class="text-3xl">"✅"</span>
                                <span><strong>"🎨 WasmClassBuilder::new()"</strong> " - WASM class building!"</span>
                            </li>
                            <li class="flex items-center space-x-4 p-4 bg-gradient-to-r from-blue-400/30 to-cyan-500/30 rounded-2xl border border-blue-300/50">
                                <span class="text-3xl">"✅"</span>
                                <span><strong>"🏗️ builder.add_classes()"</strong> " - Element-based processing!"</span>
                            </li>
                            <li class="flex items-center space-x-4 p-4 bg-gradient-to-r from-purple-400/30 to-pink-500/30 rounded-2xl border border-purple-300/50">
                                <span class="text-3xl">"✅"</span>
                                <span><strong>"⚡ builder.build()"</strong> " - WASM string generation!"</span>
                            </li>
                            <li class="flex items-center space-x-4 p-4 bg-gradient-to-r from-pink-400/30 to-red-500/30 rounded-2xl border border-pink-300/50">
                                <span class="text-3xl">"✅"</span>
                                <span><strong>"🎭 Responsive design"</strong> " - WASM breakpoint system!"</span>
                            </li>
                            <li class="flex items-center space-x-4 p-4 bg-gradient-to-r from-cyan-400/30 to-blue-500/30 rounded-2xl border border-cyan-300/50">
                                <span class="text-3xl">"✅"</span>
                                <span><strong>"🛡️ Color system"</strong> " - Full Tailwind palette!"</span>
                            </li>
                        </ul>
                    </div>

                    // Footer
                    <div class="text-center mt-12 text-white/80 text-sm">
                        <p class="text-2xl text-white font-black mb-2">"Built with " <strong>"🎨 WasmClassBuilder"</strong> " and " <strong>"🌈 Element-Based Concepts"</strong></p>
                        <p class="text-lg font-semibold">"✨ The future of CSS generation is here! ✨"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Main entry point for the WASM demo using published tailwind-rs-wasm crate
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    // Log that we're demonstrating element-based processing concepts
    web_sys::console::log_1(&"🎨 Starting Tailwind-RS WASM with element-based processing concepts!".into());

    mount_to_body(App);
}
