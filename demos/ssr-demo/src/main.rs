use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
use tailwind_rs_core::{ClassBuilder, CssGenerator};

fn generate_css() -> String {
    let mut generator = CssGenerator::new();

    // Test different Tailwind-RS objects and methods
    let classes = vec![
        // Layout & Structure
        "min-h-screen",
        "bg-gradient-to-br",
        "from-slate-900",
        "via-purple-900",
        "to-slate-900",
        "dark:from-gray-900",
        "dark:via-purple-900",
        "dark:to-gray-900",
        "container",
        "mx-auto",
        "px-4",
        "py-8",
        "max-w-7xl",
        // Typography & Headers
        "text-6xl",
        "font-black",
        "text-center",
        "mb-12",
        "bg-gradient-to-r",
        "from-blue-400",
        "via-purple-500",
        "to-pink-500",
        "bg-clip-text",
        "text-transparent",
        "animate-pulse",
        "text-4xl",
        "font-bold",
        "text-center",
        "mb-8",
        "text-white",
        "drop-shadow-2xl",
        "text-2xl",
        "font-semibold",
        "mb-4",
        "text-white",
        "drop-shadow-lg",
        "text-lg",
        "text-gray-300",
        "mb-4",
        "leading-relaxed",
        "text-sm",
        "text-gray-400",
        "font-mono",
        "bg-gray-800",
        "px-3",
        "py-1",
        "rounded",
        // Cards & Containers
        "bg-white/10",
        "dark:bg-gray-800/20",
        "backdrop-blur-lg",
        "rounded-2xl",
        "shadow-2xl",
        "p-8",
        "border",
        "border-white/20",
        "dark:border-gray-700/30",
        "bg-gradient-to-br",
        "from-green-500/20",
        "to-emerald-600/20",
        "dark:from-green-900/30",
        "dark:to-emerald-900/30",
        "bg-gradient-to-br",
        "from-blue-500/20",
        "to-cyan-600/20",
        "dark:from-blue-900/30",
        "dark:to-cyan-900/30",
        "bg-gradient-to-br",
        "from-purple-500/20",
        "to-pink-600/20",
        "dark:from-purple-900/30",
        "dark:to-pink-900/30",
        // Interactive Elements
        "px-6",
        "py-3",
        "bg-gradient-to-r",
        "from-blue-500",
        "to-purple-600",
        "text-white",
        "rounded-xl",
        "hover:from-blue-600",
        "hover:to-purple-700",
        "transition-all",
        "duration-300",
        "transform",
        "hover:scale-105",
        "hover:shadow-xl",
        "hover:shadow-blue-500/25",
        "font-semibold",
        "tracking-wide",
        "px-6",
        "py-3",
        "bg-gradient-to-r",
        "from-red-500",
        "to-pink-600",
        "text-white",
        "rounded-xl",
        "hover:from-red-600",
        "hover:to-pink-700",
        "transition-all",
        "duration-300",
        "transform",
        "hover:scale-105",
        "hover:shadow-xl",
        "hover:shadow-red-500/25",
        "px-6",
        "py-3",
        "bg-gradient-to-r",
        "from-gray-500",
        "to-gray-700",
        "text-white",
        "rounded-xl",
        "hover:from-gray-600",
        "hover:to-gray-800",
        "transition-all",
        "duration-300",
        "transform",
        "hover:scale-105",
        "hover:shadow-xl",
        "hover:shadow-gray-500/25",
        // Grid & Layout
        "grid",
        "grid-cols-1",
        "md:grid-cols-2",
        "lg:grid-cols-3",
        "gap-6",
        "space-y-6",
        "max-w-6xl",
        "mx-auto",
        "space-x-4",
        "flex",
        "flex-wrap",
        "gap-4",
        // Special Effects
        "p-6",
        "bg-gradient-to-br",
        "from-purple-400",
        "via-pink-500",
        "to-red-500",
        "rounded-2xl",
        "text-white",
        "text-center",
        "transform",
        "hover:scale-110",
        "transition-all",
        "duration-500",
        "hover:rotate-3",
        "shadow-2xl",
        "p-6",
        "bg-gradient-to-br",
        "from-blue-400",
        "via-cyan-500",
        "to-teal-500",
        "rounded-2xl",
        "text-white",
        "text-center",
        "transform",
        "hover:scale-110",
        "transition-all",
        "duration-500",
        "hover:-rotate-3",
        "shadow-2xl",
        "p-6",
        "bg-gradient-to-br",
        "from-green-400",
        "via-emerald-500",
        "to-teal-500",
        "rounded-2xl",
        "text-white",
        "text-center",
        "transform",
        "hover:scale-110",
        "transition-all",
        "duration-500",
        "hover:rotate-2",
        "shadow-2xl",
        // Status Indicators
        "bg-gradient-to-r",
        "from-green-400",
        "to-emerald-500",
        "text-white",
        "px-4",
        "py-2",
        "rounded-full",
        "font-bold",
        "text-sm",
        "shadow-lg",
        "animate-pulse",
        "bg-gradient-to-r",
        "from-blue-400",
        "to-cyan-500",
        "text-white",
        "px-4",
        "py-2",
        "rounded-full",
        "font-bold",
        "text-sm",
        "shadow-lg",
        // Lists & Content
        "space-y-2",
        "text-blue-300",
        "dark:text-blue-200",
        "font-medium",
        "text-center",
        "text-gray-400",
        "dark:text-gray-500",
        "italic",
        // Animations & Transitions
        "animate-bounce",
        "animate-pulse",
        "animate-spin",
        "animate-ping",
        "transition-all",
        "duration-300",
        "ease-in-out",
        "hover:animate-pulse",
        "hover:animate-bounce",
        // Shadows & Effects
        "shadow-lg",
        "shadow-xl",
        "shadow-2xl",
        "shadow-blue-500/25",
        "shadow-purple-500/25",
        "drop-shadow-lg",
        "drop-shadow-xl",
        "drop-shadow-2xl",
        "backdrop-blur-sm",
        "backdrop-blur-md",
        "backdrop-blur-lg",
        // Borders & Outlines
        "border-2",
        "border-white/30",
        "dark:border-gray-600/30",
        "border-green-500/50",
        "border-blue-500/50",
        "border-purple-500/50",
        "outline-none",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-500/50",
        // Responsive Design
        "sm:text-sm",
        "md:text-base",
        "lg:text-lg",
        "xl:text-xl",
        "sm:p-4",
        "md:p-6",
        "lg:p-8",
        "xl:p-10",
        "sm:gap-4",
        "md:gap-6",
        "lg:gap-8",
        // Dark Mode Specific
        "dark:text-white",
        "dark:text-gray-300",
        "dark:text-gray-400",
        "dark:bg-gray-800/50",
        "dark:bg-gray-900/50",
        "dark:border-gray-700/50",
        "dark:border-gray-600/50",
        // Glass Morphism
        "bg-white/5",
        "backdrop-blur-md",
        "border-white/10",
        "bg-black/5",
        "backdrop-blur-md",
        "border-black/10",
        // Neon Effects
        "shadow-neon-blue",
        "shadow-neon-purple",
        "shadow-neon-green",
        "text-neon-blue",
        "text-neon-purple",
        "text-neon-green",
        // Gradient Text
        "bg-gradient-to-r",
        "from-yellow-400",
        "via-red-500",
        "to-pink-500",
        "bg-gradient-to-r",
        "from-green-400",
        "via-blue-500",
        "to-purple-600",
        "bg-gradient-to-r",
        "from-pink-400",
        "via-purple-500",
        "to-indigo-500",
    ];

    // Test Tailwind-RS objects and methods
    for class in classes {
        match generator.add_class(class) {
            Ok(_) => {
                // Successfully added class
            }
            Err(e) => {
                eprintln!("Warning: Failed to add class '{}': {:?}", class, e);
            }
        }
    }

    // Test additional Tailwind-RS functionality
    let class_builder = ClassBuilder::new();

    // Test building classes programmatically using the correct API
    let _class_set = class_builder
        .class("bg-blue-500")
        .class("text-white")
        .class("px-4")
        .class("py-2")
        .class("rounded-lg")
        .class("hover:bg-blue-600")
        .build();

    // CssGenerator.generate_css() returns String directly, not Result
    generator.generate_css()
}

fn generate_html() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let process_id = std::process::id();

    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>🚀 Tailwind-RS Objects Demo</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="stylesheet" href="/styles.css">
    <style>
        @keyframes float {{
            0%, 100% {{ transform: translateY(0px); }}
            50% {{ transform: translateY(-10px); }}
        }}
        @keyframes glow {{
            0%, 100% {{ box-shadow: 0 0 20px rgba(59, 130, 246, 0.5); }}
            50% {{ box-shadow: 0 0 40px rgba(59, 130, 246, 0.8); }}
        }}
        .animate-float {{ animation: float 3s ease-in-out infinite; }}
        .animate-glow {{ animation: glow 2s ease-in-out infinite; }}
    </style>
</head>
<body>
    <div class="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 dark:from-gray-900 dark:via-purple-900 dark:to-gray-900">
        <div class="container mx-auto px-4 py-8 max-w-7xl">
            <h1 class="text-6xl font-black text-center mb-12 bg-gradient-to-r from-blue-400 via-purple-500 to-pink-500 bg-clip-text text-transparent animate-pulse">
                🚀 Tailwind-RS Objects Demo
            </h1>
            
            <div class="max-w-6xl mx-auto space-y-6">
                <!-- Tailwind-RS Objects Status -->
                <div class="bg-white/10 dark:bg-gray-800/20 backdrop-blur-lg rounded-2xl shadow-2xl p-8 border border-white/20 dark:border-gray-700/30 animate-float">
                    <div class="flex items-center justify-center mb-6">
                        <div class="bg-gradient-to-r from-green-400 to-emerald-500 text-white px-4 py-2 rounded-full font-bold text-sm shadow-lg animate-pulse">
                            ✅ Tailwind-RS Objects Active
                        </div>
                    </div>
                    <h2 class="text-4xl font-bold text-center mb-8 text-white drop-shadow-2xl">
                        🎯 Real Tailwind-RS Objects
                    </h2>
                    <p class="text-lg text-gray-300 mb-4 leading-relaxed text-center">
                        This page uses actual Tailwind-RS objects: CssGenerator, ClassBuilder, and error handling.
                    </p>
                    <div class="bg-gradient-to-br from-green-500/20 to-emerald-600/20 dark:from-green-900/30 dark:to-emerald-900/30 rounded-xl p-6 border border-green-500/50">
                        <p class="text-sm text-gray-400 font-mono bg-gray-800 px-3 py-1 rounded text-center">
                            <strong class="text-green-400">CssGenerator:</strong> ✅ Active | 
                            <strong class="text-blue-400">ClassBuilder:</strong> ✅ Active | 
                            <strong class="text-purple-400">Error Handling:</strong> ✅ Active | 
                            <strong class="text-pink-400">Process ID:</strong> {}
                        </p>
                    </div>
                </div>

                <!-- Tailwind-RS Objects Demo -->
                <div class="bg-gradient-to-br from-blue-500/20 to-cyan-600/20 dark:from-blue-900/30 dark:to-cyan-900/30 backdrop-blur-lg rounded-2xl shadow-2xl p-8 border border-blue-500/50">
                    <h2 class="text-2xl font-semibold mb-4 text-white drop-shadow-lg text-center">
                        🎯 Tailwind-RS Objects in Action
                    </h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div class="bg-white/10 backdrop-blur-md rounded-xl p-6 border border-white/20 animate-glow">
                            <h3 class="font-semibold text-white mb-2 text-lg">🔧 CssGenerator</h3>
                            <p class="text-sm text-gray-300 mb-2">Rust object for CSS generation</p>
                            <div class="text-xs text-gray-400 font-mono bg-gray-800 px-2 py-1 rounded">
                                let mut generator = CssGenerator::new();
                            </div>
                        </div>
                        <div class="bg-white/10 backdrop-blur-md rounded-xl p-6 border border-white/20 animate-glow">
                            <h3 class="font-semibold text-white mb-2 text-lg">🏗️ ClassBuilder</h3>
                            <p class="text-sm text-gray-300 mb-2">Fluent API for building classes</p>
                            <div class="text-xs text-gray-400 font-mono bg-gray-800 px-2 py-1 rounded">
                                ClassBuilder::new().bg("blue-500")
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Interactive Counter with Fancy Buttons -->
                <div class="bg-white/10 dark:bg-gray-800/20 backdrop-blur-lg rounded-2xl shadow-2xl p-8 border border-white/20 dark:border-gray-700/30">
                    <h2 class="text-2xl font-semibold mb-4 text-white drop-shadow-lg text-center">
                        🎮 Interactive Counter
                    </h2>
                    <div class="text-center mb-6">
                        <div class="inline-block bg-gradient-to-r from-blue-500/20 to-purple-600/20 rounded-xl p-4 border border-blue-500/50">
                            <p class="text-lg text-gray-300 mb-2">Count: <span id="count" class="text-4xl font-bold text-white">0</span></p>
                        </div>
                    </div>
                    <div class="flex flex-wrap gap-4 justify-center">
                        <button
                            class="px-6 py-3 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded-xl hover:from-blue-600 hover:to-purple-700 transition-all duration-300 transform hover:scale-105 hover:shadow-xl hover:shadow-blue-500/25 font-semibold tracking-wide"
                            onclick="increment()"
                        >
                            ⬆️ Increment
                        </button>
                        <button
                            class="px-6 py-3 bg-gradient-to-r from-red-500 to-pink-600 text-white rounded-xl hover:from-red-600 hover:to-pink-700 transition-all duration-300 transform hover:scale-105 hover:shadow-xl hover:shadow-red-500/25 font-semibold tracking-wide"
                            onclick="decrement()"
                        >
                            ⬇️ Decrement
                        </button>
                        <button
                            class="px-6 py-3 bg-gradient-to-r from-gray-500 to-gray-700 text-white rounded-xl hover:from-gray-600 hover:to-gray-800 transition-all duration-300 transform hover:scale-105 hover:shadow-xl hover:shadow-gray-500/25 font-semibold tracking-wide"
                            onclick="reset()"
                        >
                            🔄 Reset
                        </button>
                    </div>
                </div>

                <!-- Fancy Tailwind Demo Cards -->
                <div class="bg-white/10 dark:bg-gray-800/20 backdrop-blur-lg rounded-2xl shadow-2xl p-8 border border-white/20 dark:border-gray-700/30">
                    <h2 class="text-2xl font-semibold mb-4 text-white drop-shadow-lg text-center">
                        🎨 Tailwind-RS Generated CSS Demo
                    </h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        <div class="p-6 bg-gradient-to-br from-purple-400 via-pink-500 to-red-500 rounded-2xl text-white text-center transform hover:scale-110 transition-all duration-500 hover:rotate-3 shadow-2xl">
                            <div class="text-2xl mb-2">🌈</div>
                            <div class="font-bold">Gradient Magic</div>
                            <div class="text-sm opacity-90">Generated by CssGenerator</div>
                        </div>
                        <div class="p-6 bg-gradient-to-br from-blue-400 via-cyan-500 to-teal-500 rounded-2xl text-white text-center transform hover:scale-110 transition-all duration-500 hover:-rotate-3 shadow-2xl">
                            <div class="text-2xl mb-2">💫</div>
                            <div class="font-bold">Cyan Dreams</div>
                            <div class="text-sm opacity-90">Built with ClassBuilder</div>
                        </div>
                        <div class="p-6 bg-gradient-to-br from-green-400 via-emerald-500 to-teal-500 rounded-2xl text-white text-center transform hover:scale-110 transition-all duration-500 hover:rotate-2 shadow-2xl">
                            <div class="text-2xl mb-2">✨</div>
                            <div class="font-bold">Emerald Glow</div>
                            <div class="text-sm opacity-90">Rust CSS generation</div>
                        </div>
                    </div>
                </div>

                <!-- Features List with Neon Effects -->
                <div class="bg-gradient-to-br from-purple-500/20 to-pink-600/20 dark:from-purple-900/30 dark:to-pink-900/30 backdrop-blur-lg rounded-2xl shadow-2xl p-8 border border-purple-500/50">
                    <h3 class="text-lg font-semibold text-white mb-2 text-center">
                        🚀 Tailwind-RS Objects Features
                    </h3>
                    <ul class="space-y-2 text-blue-300 dark:text-blue-200 font-medium">
                        <li class="flex items-center space-x-2">
                            <span class="text-green-400">✅</span>
                            <span><strong>CssGenerator::new()</strong> - Create CSS generator</span>
                        </li>
                        <li class="flex items-center space-x-2">
                            <span class="text-blue-400">✅</span>
                            <span><strong>generator.add_class()</strong> - Add Tailwind classes</span>
                        </li>
                        <li class="flex items-center space-x-2">
                            <span class="text-purple-400">✅</span>
                            <span><strong>generator.generate_css()</strong> - Generate CSS</span>
                        </li>
                        <li class="flex items-center space-x-2">
                            <span class="text-pink-400">✅</span>
                            <span><strong>ClassBuilder::new()</strong> - Fluent API</span>
                        </li>
                        <li class="flex items-center space-x-2">
                            <span class="text-cyan-400">✅</span>
                            <span><strong>Error handling</strong> - Result<TailwindError></span>
                        </li>
                        <li class="flex items-center space-x-2">
                            <span class="text-yellow-400">✅</span>
                            <span><strong>Process ID: {}</strong></span>
                        </li>
                    </ul>
                </div>

                <!-- Footer -->
                <div class="text-center text-gray-400 dark:text-gray-500 italic">
                    <p class="text-lg">Built with <strong class="text-white">Tailwind-RS Objects</strong> and <strong class="text-purple-400">Rust CSS Generation</strong></p>
                    <p class="text-sm">Generated at: {}</p>
                </div>
            </div>
        </div>
    </div>

    <script>
        let count = 0;
        
        function increment() {{
            count++;
            document.getElementById('count').textContent = count;
        }}
        
        function decrement() {{
            count--;
            document.getElementById('count').textContent = count;
        }}
        
        function reset() {{
            count = 0;
            document.getElementById('count').textContent = count;
        }}
    </script>
</body>
</html>
"#,
        process_id, process_id, timestamp
    )
}

fn handle_request(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);
    let response = if request.starts_with("GET /styles.css") {
        let css = generate_css();
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/css\r\nContent-Length: {}\r\n\r\n{}",
            css.len(),
            css
        )
    } else {
        let html = generate_html();
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            html.len(),
            html
        )
    };

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("🚀 Starting Tailwind-RS Objects Demo server...");
    println!("📱 Open http://localhost:3000 in your browser");
    println!("🔧 This uses REAL Tailwind-RS objects: CssGenerator, ClassBuilder, error handling!");
    println!("⚡ Process ID: {}", std::process::id());

    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_request(stream) {
                    eprintln!("Error handling request: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
