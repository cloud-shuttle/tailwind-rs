use axum::Router;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tailwind_rs_core::css_generator::CssGenerator;
use tailwind_rs_leptos_ssr_demo::app::*;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Generate CSS for the application
    let css = generate_app_css();

    // Save CSS to file
    std::fs::create_dir_all("target/site").unwrap_or_default();
    std::fs::write("target/site/styles.css", css).expect("Failed to write CSS file");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would be passed in from the leptos tool if it were being used
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    println!("🚀 Starting Tailwind-RS SSR Demo server...");
    println!("📱 Open http://{} in your browser", addr);
    println!("🔒 API Contracts: http://{}/contracts", addr);
    println!("🎨 Transform Parsers: http://{}/transforms", addr);
    println!("⚡ Server-Side Rendering with Leptos and Axum");

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(leptos_axum::file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn generate_app_css() -> String {
    let mut generator = CssGenerator::new();

    // Core layout and structure
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
        "text-5xl", "text-4xl", "text-3xl", "text-2xl", "text-xl", "text-lg", "text-base", "text-sm",
        "font-black", "font-bold", "font-semibold", "font-medium", "text-center", "text-left",
        "bg-clip-text", "bg-gradient-to-r",

        // Positioning and sizing
        "relative", "absolute", "w-8", "w-12", "w-16", "w-32", "h-8", "h-12", "h-16", "h-32",
        "w-full", "h-full",

        // Hover and transitions
        "hover:bg-blue-600", "hover:bg-blue-700", "hover:bg-purple-700", "hover:bg-gray-600",
        "hover:text-white", "hover:border-gray-400", "hover:scale-105", "hover:translate-x-4", "hover:translate-y-2",
        "hover:scale-x-110", "hover:scale-y-95",
        "transition-all", "transition-colors", "duration-200", "duration-300", "transform",
        "cursor-pointer", "disabled:opacity-50", "disabled:cursor-not-allowed",

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
    ];

    for class in classes {
        let _ = generator.add_class(class);
    }

    generator.generate_css()
}
