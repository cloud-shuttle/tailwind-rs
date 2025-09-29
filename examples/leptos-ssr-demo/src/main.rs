use axum::{response::{Html, Response}, routing::get, Router, body::Body};
use leptos::prelude::*;
use tailwind_rs_core::css_generator::CssGenerator;
use tailwind_rs_leptos_ssr_demo::app::*;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Generate CSS for the application
    let css = generate_app_css();

    println!("🚀 Starting Tailwind-RS SSR Demo server...");
    println!("📱 Open http://127.0.0.1:3001 in your browser");
    println!("⚡ Server-Side Rendering with Leptos and Axum - Single Page Demo");

    // Create a simple handler that renders the app
    let app = Router::new()
        .route("/", get({
            let css_clone = css.clone();
            move || root_handler(css_clone.clone())
        }))
        .route("/styles.css", get({
            let css_clone = css.clone();
            move || css_handler(css_clone.clone())
        }));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler(css: String) -> Html<String> {
    // For demo purposes, return a simple HTML page with SSR content
    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1"/>
    <meta name="description" content="Tailwind-RS SSR Demo with Trie-Powered CSS Generation"/>
    <title>🚀 Tailwind-RS Trie-Powered SSR Demo</title>
    <style>{css}</style>
</head>
<body class="font-sans min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 overflow-x-hidden">
    <!-- ✨ Animated Background -->
    <div class="fixed inset-0 overflow-hidden pointer-events-none">
        <!-- Animated Orbs -->
        <div class="absolute top-1/4 left-1/4 w-64 h-64 bg-purple-500/20 rounded-full blur-3xl animate-pulse" style="animation-delay: 0s;"></div>
        <div class="absolute top-3/4 right-1/4 w-96 h-96 bg-blue-500/20 rounded-full blur-3xl animate-pulse" style="animation-delay: 2s;"></div>
        <div class="absolute bottom-1/4 left-1/3 w-80 h-80 bg-pink-500/20 rounded-full blur-3xl animate-pulse" style="animation-delay: 4s;"></div>
    </div>

    <div class="relative z-10 min-h-screen">
        <!-- 🎭 Advanced Effects Section -->
        <section class="min-h-screen flex items-center justify-center px-4 py-16">
            <div class="max-w-7xl mx-auto text-center">

                <!-- Status Banner -->
                <div class="inline-flex items-center gap-4 mb-12 px-8 py-4 bg-white/10 backdrop-blur-2xl rounded-full border border-white/20 shadow-2xl">
                    <div class="w-3 h-3 bg-green-400 rounded-full animate-pulse"></div>
                    <span class="text-white/90 font-medium">🚀 Tailwind-RS Advanced Effects • Trie Engine Online</span>
                </div>

                <!-- Interactive Cards -->
                <div class="relative mb-16">
                    <div class="flex justify-center items-center gap-8 flex-wrap">

                        <!-- Card 1 - Performance -->
                        <div class="group relative hover:scale-110 transition-all duration-500 cursor-pointer">
                            <div class="absolute inset-0 bg-gradient-to-r from-purple-600 to-pink-600 rounded-3xl blur-2xl opacity-75 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative bg-black/60 backdrop-blur-2xl border border-white/30 rounded-3xl p-8 shadow-2xl">
                                <div class="w-20 h-20 bg-gradient-to-r from-purple-500 to-pink-500 rounded-2xl flex items-center justify-center mb-6 mx-auto shadow-2xl animate-spin" style="animation-duration: 8s;">
                                    <span class="text-3xl">⚡</span>
                                </div>
                                <h3 class="text-2xl font-bold bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent mb-4">Lightning Fast</h3>
                                <p class="text-white/80 leading-relaxed">
                                    O(1) trie lookups • Instant CSS generation • Zero runtime overhead
                                </p>
                            </div>
                        </div>

                        <!-- Card 2 - Backdrop Effects -->
                        <div class="group relative hover:scale-110 transition-all duration-500 cursor-pointer">
                            <div class="absolute inset-0 bg-gradient-to-r from-blue-600 to-cyan-600 rounded-3xl blur-2xl opacity-75 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative bg-black/60 backdrop-blur-2xl border border-white/30 rounded-3xl p-8 shadow-2xl">
                                <div class="w-20 h-20 bg-gradient-to-r from-blue-500 to-cyan-500 rounded-2xl flex items-center justify-center mb-6 mx-auto shadow-2xl group-hover:backdrop-blur-3xl transition-all duration-300">
                                    <span class="text-3xl">🌊</span>
                                </div>
                                <h3 class="text-2xl font-bold bg-gradient-to-r from-blue-400 to-cyan-400 bg-clip-text text-transparent mb-4">Backdrop Effects</h3>
                                <p class="text-white/80 leading-relaxed">
                                    Backdrop blur • Opacity effects • Glass morphism • Layer blending
                                </p>
                            </div>
                        </div>

                        <!-- Card 3 - Filter Effects -->
                        <div class="group relative hover:scale-110 transition-all duration-500 cursor-pointer">
                            <div class="absolute inset-0 bg-gradient-to-r from-green-600 to-emerald-600 rounded-3xl blur-2xl opacity-75 group-hover:opacity-100 transition-opacity duration-300"></div>
                            <div class="relative bg-black/60 backdrop-blur-2xl border border-white/30 rounded-3xl p-8 shadow-2xl">
                                <div class="w-20 h-20 bg-gradient-to-r from-green-500 to-emerald-500 rounded-2xl flex items-center justify-center mb-6 mx-auto shadow-2xl group-hover:brightness-125 group-hover:saturate-150 transition-all duration-300">
                                    <span class="text-3xl">🎨</span>
                                </div>
                                <h3 class="text-2xl font-bold bg-gradient-to-r from-green-400 to-emerald-400 bg-clip-text text-transparent mb-4">Filter Effects</h3>
                                <p class="text-white/80 leading-relaxed">
                                    Brightness • Saturation • Contrast • Advanced visual filters
                                </p>
                            </div>
                        </div>

                    </div>
                </div>

                <!-- 🌟 Hero Title with Advanced Effects -->
                <div class="mb-16">
                    <h1 class="text-8xl md:text-9xl font-black mb-8">
                        <span class="bg-gradient-to-r from-yellow-400 via-pink-500 to-purple-600 bg-clip-text text-transparent animate-rainbow drop-shadow-2xl">
                            🚀 TAILWIND-RS
                        </span>
                    </h1>

                    <div class="text-5xl md:text-6xl font-bold mb-8">
                        <span class="bg-gradient-to-r from-cyan-400 via-blue-500 to-purple-600 bg-clip-text text-transparent">
                            Advanced Effects
                        </span>
                    </div>

                    <p class="text-xl md:text-2xl text-white/90 max-w-4xl mx-auto mb-12 leading-relaxed drop-shadow-lg">
                        Experience advanced CSS generation with <strong class="text-cyan-400">trie-powered lookups</strong>,
                        <strong class="text-pink-400">backdrop filters</strong>, <strong class="text-purple-400">mix-blend modes</strong>,
                        and <strong class="text-green-400">advanced visual effects</strong>.
                    </p>
                </div>

            </div>
        </section>

        <!-- 🎨 Mix-Blend-Mode Gallery -->
        <section class="py-24 bg-gradient-to-r from-transparent via-black/20 to-transparent backdrop-blur-sm">
            <div class="max-w-7xl mx-auto px-4">
                <h2 class="text-5xl font-bold text-center mb-16 bg-gradient-to-r from-pink-400 to-purple-400 bg-clip-text text-transparent">
                    Mix Blend Gallery
                </h2>

                <div class="grid md:grid-cols-4 gap-8">
                    <div class="group relative overflow-hidden rounded-2xl transform hover:scale-105 transition-all duration-500">
                        <div class="absolute inset-0 bg-gradient-to-br from-red-500 to-yellow-500 mix-blend-multiply"></div>
                        <div class="absolute inset-0 bg-gradient-to-tl from-blue-500 to-green-500 mix-blend-screen"></div>
                        <div class="relative bg-black/60 backdrop-blur-xl p-8 text-center">
                            <h3 class="text-2xl font-bold text-white mb-4">Multiply + Screen</h3>
                            <p class="text-white/80">Advanced blend modes create stunning visual effects</p>
                        </div>
                    </div>

                    <div class="group relative overflow-hidden rounded-2xl transform hover:scale-105 transition-all duration-500">
                        <div class="absolute inset-0 bg-gradient-radial from-purple-600 to-transparent mix-blend-difference"></div>
                        <div class="absolute inset-0 bg-gradient-conic from-cyan-400 to-pink-400 mix-blend-overlay"></div>
                        <div class="relative bg-black/60 backdrop-blur-xl p-8 text-center">
                            <h3 class="text-2xl font-bold text-white mb-4">Difference + Overlay</h3>
                            <p class="text-white/80">Complex gradients with advanced blending</p>
                        </div>
                    </div>

                    <div class="group relative overflow-hidden rounded-2xl transform hover:scale-105 transition-all duration-500">
                        <div class="absolute inset-0 bg-gradient-to-r from-orange-500 via-red-500 to-pink-500 mix-blend-color-dodge"></div>
                        <div class="absolute inset-0 bg-gradient-to-l from-blue-500 via-purple-500 to-indigo-500 mix-blend-color-burn"></div>
                        <div class="relative bg-black/60 backdrop-blur-xl p-8 text-center">
                            <h3 class="text-2xl font-bold text-white mb-4">Color Dodge + Burn</h3>
                            <p class="text-white/80">Photoshop-style color manipulation</p>
                        </div>
                    </div>

                    <div class="group relative overflow-hidden rounded-2xl transform hover:scale-105 transition-all duration-500">
                        <div class="absolute inset-0 bg-gradient-conic from-green-400 to-blue-400 mix-blend-hard-light"></div>
                        <div class="absolute inset-0 bg-gradient-radial from-yellow-400 to-red-400 mix-blend-soft-light"></div>
                        <div class="relative bg-black/60 backdrop-blur-xl p-8 text-center">
                            <h3 class="text-2xl font-bold text-white mb-4">Hard + Soft Light</h3>
                            <p class="text-white/80">Professional lighting effects</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <!-- ✨ Interactive Effects Showcase -->
        <section class="py-24">
            <div class="max-w-7xl mx-auto px-4">
                <h2 class="text-5xl font-bold text-center mb-16 bg-gradient-to-r from-cyan-400 to-blue-400 bg-clip-text text-transparent">
                    Interactive Effects
                </h2>

                <div class="grid md:grid-cols-3 gap-12">

                    <!-- Hover Transform Card -->
                    <div class="group relative bg-black/40 backdrop-blur-2xl border border-white/20 rounded-3xl p-8 transform hover:rotate-6 hover:scale-110 hover:skew-x-3 transition-all duration-500 cursor-pointer">
                        <div class="absolute inset-0 bg-gradient-to-r from-purple-600/20 to-pink-600/20 rounded-3xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                        <div class="relative text-center">
                            <div class="w-16 h-16 bg-gradient-conic from-purple-500 to-pink-500 rounded-full flex items-center justify-center mb-6 mx-auto shadow-2xl group-hover:shadow-purple-500/50 transition-all duration-300">
                                <span class="text-2xl">🎯</span>
                            </div>
                            <h3 class="text-2xl font-bold text-white mb-4">3D Hover Effects</h3>
                            <p class="text-white/80">Rotate • Scale • Skew • Transform on hover</p>
                        </div>
                    </div>

                    <!-- Backdrop Filter Card -->
                    <div class="group relative bg-black/40 backdrop-blur-2xl border border-white/20 rounded-3xl p-8 transform hover:backdrop-blur-3xl hover:backdrop-brightness-125 transition-all duration-500 cursor-pointer">
                        <div class="absolute inset-0 bg-gradient-radial from-blue-600/20 to-cyan-600/20 rounded-3xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                        <div class="relative text-center">
                            <div class="w-16 h-16 bg-gradient-to-r from-blue-500 to-cyan-500 rounded-full flex items-center justify-center mb-6 mx-auto shadow-2xl group-hover:shadow-blue-500/50 transition-all duration-300">
                                <span class="text-2xl">🌊</span>
                            </div>
                            <h3 class="text-2xl font-bold text-white mb-4">Backdrop Filters</h3>
                            <p class="text-white/80">Blur • Brightness • Contrast • Saturation effects</p>
                        </div>
                    </div>

                    <!-- Filter Effects Card -->
                    <div class="group relative bg-black/40 backdrop-blur-2xl border border-white/20 rounded-3xl p-8 transform hover:brightness-125 hover:saturate-150 transition-all duration-500 cursor-pointer">
                        <div class="absolute inset-0 bg-gradient-radial from-green-600/20 to-emerald-600/20 rounded-3xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                        <div class="relative text-center">
                            <div class="w-16 h-16 bg-gradient-conic from-green-500 to-emerald-500 rounded-full flex items-center justify-center mb-6 mx-auto shadow-2xl group-hover:shadow-green-500/50 transition-all duration-300">
                                <span class="text-2xl">🎨</span>
                            </div>
                            <h3 class="text-2xl font-bold text-white mb-4">Filter Effects</h3>
                            <p class="text-white/80">Brightness • Saturation • Hue • Advanced filters</p>
                        </div>
                    </div>

                </div>
            </div>
        </section>

        <!-- 🚀 Final Status Section -->
        <section class="py-16 bg-gradient-to-t from-black/60 to-transparent backdrop-blur-sm">
            <div class="max-w-4xl mx-auto px-4 text-center">

                <!-- Status Cards -->
                <div class="grid md:grid-cols-3 gap-6 mb-12">
                    <div class="bg-black/60 backdrop-blur-xl border border-white/20 rounded-2xl p-6 hover:scale-105 transition-all duration-300 cursor-pointer">
                        <div class="text-3xl mb-2">⚡</div>
                        <div class="text-sm text-green-400 font-mono">Trie Engine</div>
                        <div class="text-xs text-white/60">O(1) Performance</div>
                    </div>

                    <div class="bg-black/60 backdrop-blur-xl border border-white/20 rounded-2xl p-6 hover:scale-105 transition-all duration-300 cursor-pointer">
                        <div class="text-3xl mb-2">🎨</div>
                        <div class="text-sm text-purple-400 font-mono">SSR Rendering</div>
                        <div class="text-xs text-white/60">Server Generated</div>
                    </div>

                    <div class="bg-black/60 backdrop-blur-xl border border-white/20 rounded-2xl p-6 hover:scale-105 transition-all duration-300 cursor-pointer">
                        <div class="text-3xl mb-2">✨</div>
                        <div class="text-sm text-pink-400 font-mono">Advanced Effects</div>
                        <div class="text-xs text-white/60">Backdrop & Filters</div>
                    </div>
                </div>

                <!-- Final Message -->
                <div class="bg-gradient-to-r from-gray-900/90 to-gray-800/90 backdrop-blur-2xl rounded-3xl border border-white/20 p-8 shadow-2xl">
                    <h3 class="text-3xl font-bold text-white mb-4 bg-gradient-to-r from-yellow-400 to-pink-400 bg-clip-text text-transparent">
                        🚀 Tailwind-RS: Advanced CSS Generation
                    </h3>
                    <p class="text-white/80 text-lg leading-relaxed mb-6">
                        Experience advanced CSS generation with trie-powered lookups, server-side rendering,
                        backdrop filters, mix-blend modes, and sophisticated visual effects.
                    </p>
                    <div class="flex justify-center gap-4">
                        <div class="px-4 py-2 bg-green-500/20 border border-green-400/50 rounded-full">
                            <span class="text-green-400 font-mono text-sm">✅ Trie Engine Active</span>
                        </div>
                        <div class="px-4 py-2 bg-blue-500/20 border border-blue-400/50 rounded-full">
                            <span class="text-blue-400 font-mono text-sm">🎨 SSR Rendering</span>
                        </div>
                        <div class="px-4 py-2 bg-purple-500/20 border border-purple-400/50 rounded-full">
                            <span class="text-purple-400 font-mono text-sm">✨ Advanced Effects</span>
                        </div>
                    </div>
                </div>

            </div>
        </section>

    </div>
</body>
</html>"#,
);
    Html(html)
}

async fn css_handler(css: String) -> Response<Body> {
    Response::builder()
        .header("content-type", "text/css")
        .body(Body::from(css))
        .unwrap()
}

fn generate_app_css() -> String {
    let mut generator = CssGenerator::new();

    // Comprehensive Tailwind class set - using the trie for fast lookups!
    let classes = vec![
        // Base styles
        "font-sans",

        // Layout and containers
        "min-h-screen", "container", "mx-auto", "px-4", "py-8", "py-12", "py-16", "py-24",
        "max-w-7xl", "max-w-2xl", "max-w-md", "flex", "justify-center", "justify-between",
        "items-center", "flex-col", "space-y-4", "space-y-6", "space-y-8", "space-y-12", "space-y-16",
        "space-x-4", "space-x-8", "grid", "grid-cols-1", "md:grid-cols-2", "md:grid-cols-3", "lg:grid-cols-3",
        "gap-8", "gap-12",

        // Colors and backgrounds
        "bg-white", "bg-gray-50", "bg-gray-100", "bg-gray-500", "bg-gray-600", "bg-gray-800", "bg-gray-900",
        "bg-blue-50", "bg-blue-100", "bg-blue-500", "bg-blue-600", "bg-blue-700",
        "bg-purple-50", "bg-purple-100", "bg-purple-500", "bg-purple-600", "bg-purple-700",
        "bg-green-50", "bg-green-100", "bg-green-400", "bg-green-500", "bg-green-600", "bg-green-700",
        "bg-indigo-100", "bg-teal-50", "bg-teal-500",
        "text-white", "text-gray-600", "text-gray-700", "text-gray-800", "text-gray-900",
        "text-blue-600", "text-blue-700", "text-blue-800", "text-blue-900",
        "text-purple-600", "text-purple-700", "text-purple-800", "text-purple-900",
        "text-green-600", "text-green-700", "text-green-800",
        "text-indigo-600", "text-teal-600",

        // Borders and shadows
        "border", "border-gray-100", "border-gray-200", "border-gray-300", "border-gray-400",
        "border-transparent", "rounded", "rounded-lg", "rounded-xl", "rounded-full",
        "shadow-lg", "shadow-xl",

        // Typography
        "text-6xl", "text-5xl", "text-4xl", "text-3xl", "text-2xl", "text-xl", "text-lg", "text-base", "text-sm",
        "font-black", "font-bold", "font-semibold", "font-medium", "text-center", "text-left",
        "bg-clip-text", "text-transparent", "bg-gradient-to-r",

        // Gradient backgrounds and directions
        "bg-gradient-to-br", "bg-gradient-to-r",
        "from-blue-50", "from-blue-600", "from-purple-600", "from-purple-700",
        "from-red-500", "from-yellow-400", "from-blue-500",
        "to-indigo-100", "to-indigo-600", "to-purple-600", "to-blue-600", "to-blue-500",
        "via-purple-600", "via-yellow-400",

        // Positioning and sizing
        "relative", "absolute", "w-8", "w-12", "w-16", "w-32", "h-8", "h-12", "h-16", "h-32",
        "w-full", "h-full",

        // Hover and transitions
        "hover:bg-blue-600", "hover:bg-blue-700", "hover:bg-purple-700", "hover:bg-gray-600",
        "hover:text-white", "hover:border-gray-400", "hover:scale-105", "hover:translate-x-4", "hover:translate-y-2",
        "hover:scale-x-110", "hover:scale-y-95",
        "transition-all", "transition-colors", "transition-opacity", "transition-transform", "duration-200", "duration-300", "duration-500", "transform",
        "cursor-pointer", "disabled:opacity-50", "disabled:cursor-not-allowed",
        "opacity-0", "opacity-25", "opacity-50", "opacity-75", "opacity-100",

        // Group hover variants
        "group-hover:opacity-100", "group-hover:backdrop-blur-3xl", "group-hover:brightness-125", "group-hover:saturate-150",
        "group-hover:shadow-purple-500/50", "group-hover:shadow-blue-500/50", "group-hover:shadow-green-500/50",

        // Responsive utilities
        "sm:px-6", "sm:flex", "sm:justify-center", "sm:ml-3", "sm:mt-0", "sm:text-lg", "sm:text-5xl",
        "md:flex", "md:grid-cols-3", "md:py-4", "md:text-lg", "md:px-10", "md:mt-8", "md:max-w-3xl",
        "lg:px-8", "lg:grid-cols-3",

        // Focus states
        "focus:ring-2", "focus:ring-blue-500", "focus:border-transparent",

        // Transform utilities (new parsers)
        "translate-x-4", "translate-y-2", "scale-x-110", "scale-y-95",

        // Overflow and scrolling
        "overflow-y-auto", "max-h-48",

        // Misc
        "mb-4", "mb-6", "mb-8", "mb-12", "mb-16", "mt-16", "pt-8", "pb-8",
        "block", "inline-block", "hidden", "flex-shrink-0", "mx-auto",

        // 🎨 Advanced Backdrop Effects
        "backdrop-blur-none", "backdrop-blur-sm", "backdrop-blur", "backdrop-blur-md", "backdrop-blur-lg", "backdrop-blur-xl", "backdrop-blur-2xl", "backdrop-blur-3xl",
        "backdrop-brightness-0", "backdrop-brightness-50", "backdrop-brightness-75", "backdrop-brightness-90", "backdrop-brightness-95", "backdrop-brightness-100", "backdrop-brightness-105", "backdrop-brightness-110", "backdrop-brightness-125", "backdrop-brightness-150", "backdrop-brightness-200",
        "backdrop-contrast-0", "backdrop-contrast-50", "backdrop-contrast-75", "backdrop-contrast-100", "backdrop-contrast-125", "backdrop-contrast-150", "backdrop-contrast-200",
        "backdrop-grayscale", "backdrop-grayscale-0",
        "backdrop-hue-rotate-0", "backdrop-hue-rotate-15", "backdrop-hue-rotate-30", "backdrop-hue-rotate-60", "backdrop-hue-rotate-90", "backdrop-hue-rotate-180",
        "backdrop-invert", "backdrop-invert-0",
        "backdrop-opacity-0", "backdrop-opacity-5", "backdrop-opacity-10", "backdrop-opacity-20", "backdrop-opacity-25", "backdrop-opacity-30", "backdrop-opacity-40", "backdrop-opacity-50", "backdrop-opacity-60", "backdrop-opacity-70", "backdrop-opacity-75", "backdrop-opacity-80", "backdrop-opacity-90", "backdrop-opacity-95", "backdrop-opacity-100",
        "backdrop-saturate-0", "backdrop-saturate-50", "backdrop-saturate-100", "backdrop-saturate-150", "backdrop-saturate-200",
        "backdrop-sepia", "backdrop-sepia-0",

        // 🎭 Mix Blend Modes & Effects
        "mix-blend-normal", "mix-blend-multiply", "mix-blend-screen", "mix-blend-overlay", "mix-blend-darken", "mix-blend-lighten", "mix-blend-color-dodge", "mix-blend-color-burn", "mix-blend-hard-light", "mix-blend-soft-light", "mix-blend-difference", "mix-blend-exclusion", "mix-blend-hue", "mix-blend-saturation", "mix-blend-color", "mix-blend-luminosity",
        "bg-blend-normal", "bg-blend-multiply", "bg-blend-screen", "bg-blend-overlay", "bg-blend-darken", "bg-blend-lighten", "bg-blend-color-dodge", "bg-blend-color-burn", "bg-blend-hard-light", "bg-blend-soft-light", "bg-blend-difference", "bg-blend-exclusion", "bg-blend-hue", "bg-blend-saturation", "bg-blend-color", "bg-blend-luminosity",

        // ✨ Advanced 3D Transforms & Perspective
        "perspective-none", "perspective-1000", "perspective-2000", "perspective-3000", "perspective-4000", "perspective-5000", "perspective-6000", "perspective-7000", "perspective-8000", "perspective-9000", "perspective-10000",
        "transform-style-flat", "transform-style-3d", "transform-style-preserve-3d",
        "backface-hidden", "backface-visible",
        "rotate-0", "rotate-1", "rotate-2", "rotate-3", "rotate-6", "rotate-12", "rotate-45", "rotate-90", "rotate-180",
        "rotate-x-0", "rotate-x-1", "rotate-x-2", "rotate-x-3", "rotate-x-6", "rotate-x-12", "rotate-x-45", "rotate-x-90", "rotate-x-180",
        "rotate-y-0", "rotate-y-1", "rotate-y-2", "rotate-y-3", "rotate-y-6", "rotate-y-12", "rotate-y-45", "rotate-y-90", "rotate-y-180",
        "rotate-z-0", "rotate-z-1", "rotate-z-2", "rotate-z-3", "rotate-z-6", "rotate-z-12", "rotate-z-45", "rotate-z-90", "rotate-z-180",

        // 🎬 Advanced Animations & Keyframes
        "animate-none", "animate-spin", "animate-ping", "animate-pulse", "animate-bounce", "animate-flash", "animate-shake", "animate-wiggle",
        "animate-fade-in", "animate-fade-out", "animate-slide-in-left", "animate-slide-in-right", "animate-slide-in-up", "animate-slide-in-down",
        "animate-bounce-in", "animate-zoom-in", "animate-rotate-in", "animate-flip-in",

        // 🌈 Complex Gradients (Conic, Radial, Mesh)
        "bg-gradient-conic", "bg-gradient-radial", "bg-gradient-ellipse",
        "from-0%", "from-10%", "from-20%", "from-30%", "from-40%", "from-50%", "from-60%", "from-70%", "from-80%", "from-90%", "from-100%",
        "via-0%", "via-10%", "via-20%", "via-30%", "via-40%", "via-50%", "via-60%", "via-70%", "via-80%", "via-90%", "via-100%",
        "to-0%", "to-10%", "to-20%", "to-30%", "to-40%", "to-50%", "to-60%", "to-70%", "to-80%", "to-90%", "to-100%",

        // 🎨 Advanced Filters & Effects
        "blur-none", "blur-sm", "blur", "blur-md", "blur-lg", "blur-xl", "blur-2xl", "blur-3xl",
        "brightness-0", "brightness-50", "brightness-75", "brightness-90", "brightness-95", "brightness-100", "brightness-105", "brightness-110", "brightness-125", "brightness-150", "brightness-200",
        "contrast-0", "contrast-50", "contrast-75", "contrast-100", "contrast-125", "contrast-150", "contrast-200",
        "drop-shadow-none", "drop-shadow-sm", "drop-shadow", "drop-shadow-md", "drop-shadow-lg", "drop-shadow-xl", "drop-shadow-2xl",
        "grayscale", "grayscale-0",
        "hue-rotate-0", "hue-rotate-15", "hue-rotate-30", "hue-rotate-60", "hue-rotate-90", "hue-rotate-180",
        "invert", "invert-0",
        "saturate-0", "saturate-50", "saturate-100", "saturate-150", "saturate-200",
        "sepia", "sepia-0",

        // 🏔️ Advanced Shadows & Lighting
        "shadow-inner", "shadow-none", "shadow-sm", "shadow", "shadow-md", "shadow-lg", "shadow-xl", "shadow-2xl", "shadow-inner",
        "shadow-cyan-500/50", "shadow-pink-500/50", "shadow-purple-500/50", "shadow-blue-500/50", "shadow-green-500/50", "shadow-red-500/50",
        "shadow-cyan-500/25", "shadow-pink-500/25", "shadow-purple-500/25", "shadow-blue-500/25", "shadow-green-500/25", "shadow-red-500/25",
        "shadow-cyan-500/75", "shadow-pink-500/75", "shadow-purple-500/75", "shadow-blue-500/75", "shadow-green-500/75", "shadow-red-500/75",

        // 🎭 Clip Path & Mask Effects
        "clip-path-none", "clip-path-margin", "clip-path-border", "clip-path-padding", "clip-path-content",
        "clip-path-circle", "clip-path-ellipse", "clip-path-polygon", "clip-path-inset",
        "mask-none", "mask-image", "mask-repeat", "mask-position", "mask-size",

        // ✨ Particle Effects (CSS-based)
        "animate-float", "animate-drift", "animate-glow", "animate-rainbow", "animate-shimmer", "animate-twinkle",

        // 🎪 Advanced Typography
        "text-shadow", "text-shadow-sm", "text-shadow-md", "text-shadow-lg", "text-shadow-xl", "text-shadow-2xl",
        "text-stroke", "text-stroke-sm", "text-stroke-md", "text-stroke-lg",
        "font-variation-settings", "font-feature-settings", "font-variant-numeric", "font-variant-ligatures",

        // 🎨 Interactive States
        "hover:rotate-3", "hover:rotate-6", "hover:rotate-12", "hover:scale-110", "hover:scale-125", "hover:scale-150",
        "hover:skew-x-3", "hover:skew-x-6", "hover:skew-x-12", "hover:skew-y-3", "hover:skew-y-6", "hover:skew-y-12",
        "hover:translate-x-2", "hover:translate-x-4", "hover:translate-x-8", "hover:translate-y-2", "hover:translate-y-4", "hover:translate-y-8",
        "hover:brightness-110", "hover:brightness-125", "hover:brightness-150", "hover:saturate-150",
        "hover:backdrop-blur-lg", "hover:backdrop-blur-xl", "hover:backdrop-blur-2xl",

        // 🌟 Special Effects
        "will-change-auto", "will-change-scroll", "will-change-contents", "will-change-transform", "will-change-opacity",
        "isolation-isolate", "isolation-auto",
        "contain-none", "contain-layout", "contain-style", "contain-paint", "contain-size", "contain-content", "contain-strict",

        // 🎯 Advanced Positioning
        "sticky", "fixed", "absolute", "relative",
        "z-0", "z-10", "z-20", "z-30", "z-40", "z-50", "z-auto",

        // 🌊 Scroll Effects
        "scroll-smooth", "scroll-auto", "overscroll-auto", "overscroll-contain", "overscroll-none",
        "scrollbar-hide", "scrollbar-default", "scrollbar-thin",

        // 🎨 Color Manipulation (advanced)
        "hue-rotate-0", "hue-rotate-15", "hue-rotate-30", "hue-rotate-45", "hue-rotate-60", "hue-rotate-90", "hue-rotate-180",
        "saturate-0", "saturate-50", "saturate-100", "saturate-150", "saturate-200",

        // ✨ Advanced Borders
        "border-double", "border-dashed", "border-dotted", "border-hidden", "border-none",
        "divide-solid", "divide-dashed", "divide-dotted", "divide-double", "divide-none",
        "divide-cyan-200", "divide-pink-200", "divide-purple-200", "divide-blue-200", "divide-green-200",
    ];

    let mut success_count = 0;
    let mut fail_count = 0;
    let mut success_count = 0;
    let mut fail_count = 0;
    for class in &classes {
        match generator.add_class(class) {
            Ok(_) => success_count += 1,
            Err(e) => {
                fail_count += 1;
                eprintln!("Failed to add class {}: {}", class, e);
            }
        }
    }

    println!("CSS Generation Summary: {} successes, {} failures out of {} total classes", success_count, fail_count, classes.len());

    generator.generate_css()
}
