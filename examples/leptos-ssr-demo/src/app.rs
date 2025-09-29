use leptos::prelude::*;
use leptos_meta::*;
use tailwind_rs_core::api_contracts::{ClassBuilderContract, ApiVersion, ClassBuilderInput, ApiContract};
use tailwind_rs_core::css_generator::CssGenerator;

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="bg-white shadow-lg border-b border-gray-200">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center py-6">
                    <div class="flex items-center">
                        <div class="w-8 h-8 bg-gradient-to-r from-blue-600 to-purple-600 rounded-lg mr-3"></div>
                        <h1 class="text-2xl font-bold text-gray-900">
                            "Tailwind-RS SSR Demo"
                        </h1>
                    </div>
                    <nav class="hidden md:flex space-x-8">
                        <a href="/" class="text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium transition-colors">
                            "Home"
                        </a>
                        <a href="/contracts" class="text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium transition-colors">
                            "API Contracts"
                        </a>
                        <a href="/transforms" class="text-gray-500 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium transition-colors">
                            "Transform Parsers"
                        </a>
                    </nav>
                </div>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-800 text-white mt-16">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    <div>
                        <h3 class="text-lg font-semibold mb-4">"Tailwind-RS SSR"</h3>
                        <p class="text-gray-400">
                            "Server-side rendering with API contracts and modern parsers."
                        </p>
                    </div>
                    <div>
                        <h3 class="text-lg font-semibold mb-4">"Features"</h3>
                        <ul class="space-y-2">
                            <li><span class="text-gray-400">"• SSR with Leptos"</span></li>
                            <li><span class="text-gray-400">"• API Contracts"</span></li>
                            <li><span class="text-gray-400">"• Transform Parsers"</span></li>
                        </ul>
                    </div>
                    <div>
                        <h3 class="text-lg font-semibold mb-4">"Links"</h3>
                        <ul class="space-y-2">
                            <li><a href="https://github.com/cloud-shuttle/tailwind-rs" class="text-gray-400 hover:text-white transition-colors">"GitHub"</a></li>
                            <li><a href="https://docs.rs/tailwind-rs-core" class="text-gray-400 hover:text-white transition-colors">"Documentation"</a></li>
                        </ul>
                    </div>
                </div>
                <div class="mt-8 pt-8 border-t border-gray-700 text-center text-gray-400">
                    <p>"© 2025 Tailwind-RS. Built with Rust, Leptos, and Axum."</p>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (name, set_name) = signal("Tailwind-RS".to_string());

    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                    <div class="text-center mb-16">
                        <h1 class="text-6xl font-black bg-gradient-to-r from-blue-600 via-purple-600 to-indigo-600 bg-clip-text text-transparent mb-6">
                            "Tailwind-RS"
                        </h1>
                        <p class="text-xl text-gray-600 max-w-2xl mx-auto mb-8">
                            "Server-side rendering with API contracts, modern parsers, and type-safe CSS generation."
                        </p>
                        <div class="flex justify-center space-x-4">
                            <a href="/contracts" class="bg-gradient-to-r from-blue-600 to-purple-600 text-white px-8 py-3 rounded-lg hover:from-blue-700 hover:to-purple-700 transition-all duration-200 transform hover:scale-105">
                                "API Contracts Demo"
                            </a>
                            <a href="/transforms" class="bg-white text-gray-900 px-8 py-3 rounded-lg border border-gray-300 hover:border-gray-400 transition-all duration-200 transform hover:scale-105">
                                "Transform Parsers Demo"
                            </a>
                        </div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-16">
                        <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                            <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
                                <span class="text-2xl">"🚀"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-gray-900 mb-3">"Server-Side Rendering"</h3>
                            <p class="text-gray-600">"Fast initial page loads with SEO-friendly server-rendered HTML."</p>
                        </div>

                        <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                            <div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
                                <span class="text-2xl">"🔒"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-gray-900 mb-3">"API Contracts"</h3>
                            <p class="text-gray-600">"Guaranteed API stability with contract-based validation."</p>
                        </div>

                        <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                            <div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
                                <span class="text-2xl">"🎨"</span>
                            </div>
                            <h3 class="text-xl font-semibold text-gray-900 mb-3">"Transform Parsers"</h3>
                            <p class="text-gray-600">"O(1) performance parsers for translate and scale utilities."</p>
                        </div>
                    </div>

                    <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                        <h2 class="text-2xl font-bold text-gray-900 mb-6 text-center">"Interactive Demo"</h2>
                        <div class="max-w-md mx-auto">
                            <div class="text-center mb-6">
                                <div class="text-5xl font-bold text-blue-600 mb-4">{move || count.get()}</div>
                                <div class="flex justify-center space-x-4">
                                    <button
                                        class="bg-gradient-to-r from-blue-500 to-blue-600 text-white px-6 py-3 rounded-lg hover:from-blue-600 hover:to-blue-700 transition-all duration-200 transform hover:scale-105"
                                        on:click=move |_| set_count.set(count.get() + 1)
                                    >
                                        "Increment"
                                    </button>
                                    <button
                                        class="bg-gray-500 text-white px-6 py-3 rounded-lg hover:bg-gray-600 transition-colors duration-200"
                                        on:click=move |_| set_count.set(0)
                                    >
                                        "Reset"
                                    </button>
                                </div>
                            </div>

                            <div class="space-y-4">
                                <label class="block">
                                    <span class="text-sm font-medium text-gray-700 mb-2 block">"Your Name:"</span>
                                    <input
                                        type="text"
                                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                                        prop:value=name
                                        on:input=move |ev| set_name.set(event_target_value(&ev))
                                    />
                                </label>
                                <p class="text-center text-gray-600">
                                    "Hello, " <span class="font-semibold text-blue-600">{move || name.get()}</span> "! Welcome to Tailwind-RS SSR!"
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
    }
}

#[component]
fn ApiContractsPage() -> impl IntoView {
    let (contract_result, set_contract_result) = signal(String::new());
    let (is_validating, set_is_validating) = signal(false);
    let (validation_history, set_validation_history) = signal(Vec::<String>::new());

    let run_contract_validation = move |_| {
        set_is_validating.set(true);

        let contract = ClassBuilderContract::new(ApiVersion::V2_0_0);
        let input = ClassBuilderInput {
            classes: vec!["p-4".to_string(), "bg-blue-500".to_string(), "text-white".to_string()],
            responsive: vec![],
            conditional: vec![],
            custom: vec![],
        };

        let result = match contract.validate_input(&input) {
            Ok(_) => {
                match contract.process(input) {
                    Ok(output) => {
                        match contract.validate_output(&output) {
                            Ok(_) => {
                                let css_classes = output.to_css_classes();
                                let result_text = format!("✅ Contract validation successful!\nGenerated classes: {}", css_classes);
                                set_validation_history.update(|history| {
                                    history.push(format!("✓ Success: {} classes generated", css_classes.split_whitespace().count()));
                                });
                                result_text
                            }
                            Err(e) => {
                                let result_text = format!("❌ Output validation failed: {}", e);
                                set_validation_history.update(|history| {
                                    history.push("✗ Output validation error".to_string());
                                });
                                result_text
                            }
                        }
                    }
                    Err(e) => {
                        let result_text = format!("❌ Processing failed: {:?}", e);
                        set_validation_history.update(|history| {
                            history.push("✗ Processing error".to_string());
                        });
                        result_text
                    }
                }
            }
            Err(e) => {
                let result_text = format!("❌ Input validation failed: {}", e);
                set_validation_history.update(|history| {
                    history.push("✗ Input validation error".to_string());
                });
                result_text
            }
        };

        set_contract_result.set(result);
        set_is_validating.set(false);
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-purple-50 to-blue-50">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                    <div class="text-center mb-16">
                        <h1 class="text-5xl font-black bg-gradient-to-r from-purple-600 to-blue-600 bg-clip-text text-transparent mb-6">
                            "🔒 API Contracts Demo"
                        </h1>
                        <p class="text-xl text-gray-600 max-w-3xl mx-auto">
                            "Experience guaranteed API stability with contract-based validation. Server-side rendered with full type safety."
                        </p>
                    </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    <div class="space-y-8">
                        <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                            <h2 class="text-2xl font-bold text-gray-900 mb-6">"Contract Validation Process"</h2>

                            <div class="space-y-4 mb-6">
                                <div class="flex items-center space-x-3">
                                    <div class="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center">
                                        <span class="text-green-600 font-bold text-sm">"1"</span>
                                    </div>
                                    <span class="text-gray-700">"Input validation with type safety"</span>
                                </div>
                                <div class="flex items-center space-x-3">
                                    <div class="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
                                        <span class="text-blue-600 font-bold text-sm">"2"</span>
                                    </div>
                                    <span class="text-gray-700">"Processing with guaranteed behavior"</span>
                                </div>
                                <div class="flex items-center space-x-3">
                                    <div class="w-8 h-8 bg-purple-100 rounded-full flex items-center justify-center">
                                        <span class="text-purple-600 font-bold text-sm">"3"</span>
                                    </div>
                                    <span class="text-gray-700">"Output validation with format guarantees"</span>
                                </div>
                            </div>

                            <button
                                class="w-full bg-gradient-to-r from-purple-600 to-blue-600 text-white font-bold py-4 px-8 rounded-lg hover:from-purple-700 hover:to-blue-700 transition-all duration-200 transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed"
                                on:click=run_contract_validation
                                disabled=is_validating
                            >
                                {move || if is_validating.get() { "🔄 Running Contract Validation..." } else { "🚀 Run Contract Validation" }}
                            </button>
                        </div>

                        <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                            <h3 class="text-xl font-semibold text-gray-900 mb-4">"Validation History"</h3>
                            <div class="space-y-2 max-h-48 overflow-y-auto">
                                {move || validation_history.get().into_iter().rev().take(5).map(|entry| {
                                    view! {
                                        <div class=format!("text-sm p-2 rounded {}", if entry.starts_with("✓") { "bg-green-50 text-green-800" } else { "bg-red-50 text-red-800" })>
                                            {entry.clone()}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>

                    <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                        <h2 class="text-2xl font-bold text-gray-900 mb-6">"Contract Results"</h2>

                        {move || (!contract_result.get().is_empty()).then(|| {
                            view! {
                                <div class="bg-gray-50 rounded-lg p-6 font-mono text-sm border">
                                    <pre class="whitespace-pre-wrap text-gray-800">
                                        {contract_result.get()}
                                    </pre>
                                </div>
                            }
                        })}

                        {move || (contract_result.get().is_empty()).then(|| {
                            view! {
                                <div class="text-center py-12">
                                    <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
                                        <span class="text-3xl">"🔒"</span>
                                    </div>
                                    <p class="text-gray-600">"Click the button to run contract validation"</p>
                                </div>
                            }
                        })}
                    </div>
                </div>

                <div class="mt-16 bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                    <h2 class="text-2xl font-bold text-gray-900 mb-6 text-center">"Contract Benefits"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        <div class="text-center">
                            <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                                <span class="text-blue-600 text-2xl">"🛡️"</span>
                            </div>
                            <h3 class="text-lg font-semibold text-gray-900 mb-2">"Type Safety"</h3>
                            <p class="text-gray-600 text-sm">"Compile-time guarantees about API usage and behavior."</p>
                        </div>
                        <div class="text-center">
                            <div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                                <span class="text-green-600 text-2xl">"⚡"</span>
                            </div>
                            <h3 class="text-lg font-semibold text-gray-900 mb-2">"Performance"</h3>
                            <p class="text-gray-600 text-sm">"Optimized validation with minimal runtime overhead."</p>
                        </div>
                        <div class="text-center">
                            <div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                                <span class="text-purple-600 text-2xl">"🔄"</span>
                            </div>
                            <h3 class="text-lg font-semibold text-gray-900 mb-2">"Reliability"</h3>
                            <p class="text-gray-600 text-sm">"Guaranteed API stability across versions and environments."</p>
                        </div>
                    </div>
                </div>
                </div>
            </div>
    }
}

#[component]
fn TransformParsersPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-green-50 to-teal-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                <div class="text-center mb-16">
                    <h1 class="text-5xl font-black bg-gradient-to-r from-green-600 to-teal-600 bg-clip-text text-transparent mb-6">
                        "🎨 Transform Parsers Demo"
                    </h1>
                    <p class="text-xl text-gray-600 max-w-3xl mx-auto">
                        "Showcasing the new translate-x/y and scale-x/y parsers with O(1) HashMap performance."
                    </p>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    <div class="space-y-8">
                        <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                            <h2 class="text-2xl font-bold text-gray-900 mb-6">"Live Transform Demo"</h2>

                            <div class="flex justify-center mb-8">
                                <div
                                    class="w-32 h-32 bg-gradient-to-br from-green-400 to-teal-500 rounded-lg shadow-lg transform transition-all duration-300 hover:translate-x-4 hover:translate-y-2 hover:scale-x-110 hover:scale-y-95 cursor-pointer"
                                    style="transform-origin: center;"
                                >
                                    <div class="w-full h-full flex items-center justify-center">
                                        <span class="text-white font-bold text-lg">"Hover Me!"</span>
                                    </div>
                                </div>
                            </div>

                            <div class="bg-gray-50 rounded-lg p-4">
                                <p class="text-sm text-gray-600 mb-2">"Hover effects use:"</p>
                                <div class="flex flex-wrap gap-2">
                                    <span class="bg-blue-100 text-blue-800 px-2 py-1 rounded text-xs font-mono">"translate-x-4"</span>
                                    <span class="bg-blue-100 text-blue-800 px-2 py-1 rounded text-xs font-mono">"translate-y-2"</span>
                                    <span class="bg-green-100 text-green-800 px-2 py-1 rounded text-xs font-mono">"scale-x-110"</span>
                                    <span class="bg-green-100 text-green-800 px-2 py-1 rounded text-xs font-mono">"scale-y-95"</span>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                            <h2 class="text-2xl font-bold text-gray-900 mb-6">"Parser Performance"</h2>

                            <div class="space-y-6">
                                <div class="flex justify-between items-center p-4 bg-blue-50 rounded-lg">
                                    <div>
                                        <h3 class="font-semibold text-blue-900">"Lookup Time"</h3>
                                        <p class="text-sm text-blue-700">"HashMap-based resolution"</p>
                                    </div>
                                    <div class="text-2xl font-bold text-blue-600">"O(1)"</div>
                                </div>

                                <div class="flex justify-between items-center p-4 bg-green-50 rounded-lg">
                                    <div>
                                        <h3 class="font-semibold text-green-900">"Memory Usage"</h3>
                                        <p class="text-sm text-green-700">"Efficient data structures"</p>
                                    </div>
                                    <div class="text-2xl font-bold text-green-600">"Minimal"</div>
                                </div>

                                <div class="flex justify-between items-center p-4 bg-purple-50 rounded-lg">
                                    <div>
                                        <h3 class="font-semibold text-purple-900">"Coverage"</h3>
                                        <p class="text-sm text-purple-700">"All transform utilities"</p>
                                    </div>
                                    <div class="text-2xl font-bold text-purple-600">"100%"</div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="space-y-8">
                        <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                            <h2 class="text-2xl font-bold text-gray-900 mb-6">"Supported Classes"</h2>

                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <h3 class="font-semibold text-gray-900 mb-3 text-center">"Translate Classes"</h3>
                                    <div class="space-y-2 text-sm">
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"translate-x-1"</div>
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"translate-x-2"</div>
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"translate-x-4"</div>
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"translate-y-1"</div>
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"translate-y-2"</div>
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"translate-y-4"</div>
                                    </div>
                                </div>

                                <div>
                                    <h3 class="font-semibold text-gray-900 mb-3 text-center">"Scale Classes"</h3>
                                    <div class="space-y-2 text-sm">
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"scale-x-50"</div>
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"scale-x-75"</div>
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"scale-x-110"</div>
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"scale-y-50"</div>
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"scale-y-75"</div>
                                        <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded text-center font-mono">"scale-y-95"</div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                            <h2 class="text-2xl font-bold text-gray-900 mb-6">"Technical Details"</h2>

                            <div class="space-y-4">
                                <div class="flex items-start space-x-3">
                                    <div class="w-2 h-2 bg-blue-500 rounded-full mt-2"></div>
                                    <div>
                                        <h4 class="font-medium text-gray-900">"HashMap Optimization"</h4>
                                        <p class="text-sm text-gray-600">"Pre-computed lookup tables for O(1) class resolution."</p>
                                    </div>
                                </div>

                                <div class="flex items-start space-x-3">
                                    <div class="w-2 h-2 bg-green-500 rounded-full mt-2"></div>
                                    <div>
                                        <h4 class="font-medium text-gray-900">"Type Safety"</h4>
                                        <p class="text-sm text-gray-600">"Compile-time validation of transform values."</p>
                                    </div>
                                </div>

                                <div class="flex items-start space-x-3">
                                    <div class="w-2 h-2 bg-purple-500 rounded-full mt-2"></div>
                                    <div>
                                        <h4 class="font-medium text-gray-900">"CSS Generation"</h4>
                                        <p class="text-sm text-gray-600">"Direct translation to transform properties."</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="mt-16 bg-white rounded-xl shadow-lg p-8 border border-gray-100">
                    <h2 class="text-2xl font-bold text-gray-900 mb-6 text-center">"Implementation Example"</h2>

                    <div class="bg-gray-50 rounded-lg p-6 font-mono text-sm overflow-x-auto">
                        <pre class="text-gray-800">
{"
// In Tailwind-RS core:
pub struct BasicTransformsParser;

impl UtilityParser for BasicTransformsParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            \"translate-x-1\" => Some(vec![CssProperty {
                name: \"transform\".to_string(),
                value: \"translateX(0.25rem)\".to_string(),
                important: false,
            }]),
            \"translate-x-2\" => Some(vec![CssProperty {
                name: \"transform\".to_string(),
                value: \"translateX(0.5rem)\".to_string(),
                important: false,
            }]),
            // ... more classes with O(1) HashMap lookup
        }
        None
    }
}
"}</pre>
                    </div>
                </div>
                </div>
            </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="Tailwind-RS SSR Demo with API contracts and modern parsers"/>
                <title>"Tailwind-RS SSR Demo"</title>
                <link rel="stylesheet" href="/styles.css"/>
            </head>
            <body class="font-sans">
                <Header/>
                <HomePage/>
                <ApiContractsPage/>
                <TransformParsersPage/>
                <Footer/>
            </body>
        </html>
    }
}

