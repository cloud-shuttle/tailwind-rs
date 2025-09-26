use tailwind_rs_core::*;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

/// Generate CSS for the SSR demo
fn generate_demo_css() -> String {
    let mut generator = CssGenerator::new();
    
    // Add all classes used in the demo
    let classes = vec![
        // Layout
        "min-h-screen", "bg-gray-100", "dark:bg-gray-900",
        "container", "mx-auto", "px-4", "py-8",
        "max-w-4xl", "mx-auto",
        
        // Typography
        "text-4xl", "font-bold", "text-center", "mb-8",
        "text-gray-800", "dark:text-white",
        "text-lg", "text-gray-600", "dark:text-gray-300",
        "text-2xl", "font-semibold", "mb-4",
        
        // Cards
        "bg-white", "dark:bg-gray-800", "rounded-lg", "shadow-md", "p-6",
        "border", "border-gray-200", "dark:border-gray-700",
        
        // Buttons
        "px-4", "py-2", "bg-blue-500", "text-white", "rounded",
        "hover:bg-blue-600", "transition-colors",
        "bg-green-500", "hover:bg-green-600",
        "bg-red-500", "hover:bg-red-600",
        "bg-gray-500", "hover:bg-gray-600",
        
        // Forms
        "w-full", "px-3", "py-2", "border", "border-gray-300",
        "dark:border-gray-600", "rounded-md", "focus:outline-none",
        "focus:ring-2", "focus:ring-blue-500", "dark:bg-gray-700",
        "dark:text-white",
        
        // Grid
        "grid", "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3", "gap-4",
        "space-y-4", "space-x-4",
        
        // Status indicators
        "bg-green-100", "dark:bg-green-900", "border-green-400",
        "dark:border-green-600", "text-green-800", "dark:text-green-200",
        "bg-blue-100", "dark:bg-blue-900", "border-blue-400",
        "dark:border-blue-600", "text-blue-800", "dark:text-blue-200",
        
        // Responsive
        "sm:px-4", "md:px-6", "lg:px-8",
        "sm:text-5xl", "md:text-6xl",
        
        // Interactive
        "hover:scale-105", "transition-transform",
        "shadow-lg", "hover:shadow-xl",
        
        // Dark mode
        "dark:bg-gray-800", "dark:text-white", "dark:border-gray-700",
    ];
    
    for class in classes {
        generator.add_class(class);
    }
    
    generator.generate_css()
}

/// Generate the HTML page
fn generate_html() -> String {
    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Tailwind-RS SSR Demo</title>
    <meta name="description" content="Server-Side Rendering demo for Tailwind-RS">
    
    <!-- Tailwind CSS from CDN -->
    <script src="https://cdn.tailwindcss.com"></script>
    
    <!-- Custom styles -->
    <link rel="stylesheet" href="/styles.css">
    
    <style>
        /* Custom animations */
        .loading {{
            display: inline-block;
            width: 20px;
            height: 20px;
            border: 3px solid #f3f3f3;
            border-top: 3px solid #3498db;
            border-radius: 50%;
            animation: spin 1s linear infinite;
        }}
        
        @keyframes spin {{
            0% {{ transform: rotate(0deg); }}
            100% {{ transform: rotate(360deg); }}
        }}
    </style>
</head>
<body>
    <div class="min-h-screen bg-gray-100 dark:bg-gray-900">
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-4xl font-bold text-center mb-8 text-gray-800 dark:text-white">
                Tailwind-RS Server-Side Rendering Demo
            </h1>
            
            <div class="max-w-4xl mx-auto space-y-6">
                <!-- SSR Status -->
                <div class="bg-green-100 dark:bg-green-900 border border-green-400 dark:border-green-600 rounded-lg p-6">
                    <h2 class="text-2xl font-semibold text-green-800 dark:text-green-200 mb-4">
                        ✅ Server-Side Rendering Active!
                    </h2>
                    <p class="text-green-700 dark:text-green-300 mb-4">
                        This page is rendered on the server with Tailwind-RS classes.
                    </p>
                    <div class="bg-green-200 dark:bg-green-800 rounded p-3">
                        <p class="text-sm text-green-800 dark:text-green-200">
                            Server: localhost | SSR: Active | Framework: Rust + Tailwind-RS
                        </p>
                    </div>
                </div>

                <!-- Interactive Counter -->
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold mb-4 text-gray-800 dark:text-white">
                        Interactive Counter
                    </h2>
                    <p class="text-lg mb-4 text-gray-600 dark:text-gray-300">
                        Count: <span id="count">0</span>
                    </p>
                    <div class="space-x-4">
                        <button
                            class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                            onclick="increment()"
                        >
                            Increment
                        </button>
                        <button
                            class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition-colors"
                            onclick="decrement()"
                        >
                            Decrement
                        </button>
                        <button
                            class="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600 transition-colors"
                            onclick="reset()"
                        >
                            Reset
                        </button>
                    </div>
                </div>

                <!-- Name Input Demo -->
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold mb-4 text-gray-800 dark:text-white">
                        Name Input Demo
                    </h2>
                    <p class="text-lg mb-4 text-gray-600 dark:text-gray-300">
                        Hello, <span id="name-display">Tailwind-RS SSR</span>!
                    </p>
                    <input
                        type="text"
                        id="name-input"
                        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
                        placeholder="Enter your name"
                        oninput="updateName(this.value)"
                    />
                </div>

                <!-- Tailwind Classes Demo -->
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold mb-4 text-gray-800 dark:text-white">
                        Tailwind Classes Demo
                    </h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                        <div class="p-4 bg-gradient-to-r from-purple-400 to-pink-400 rounded-lg text-white text-center">
                            Gradient Card
                        </div>
                        <div class="p-4 bg-blue-500 rounded-lg text-white text-center transform hover:scale-105 transition-transform">
                            Hover Effect
                        </div>
                        <div class="p-4 bg-green-500 rounded-lg text-white text-center shadow-lg">
                            Shadow Card
                        </div>
                    </div>
                </div>

                <!-- SSR Features -->
                <div class="bg-blue-100 dark:bg-blue-900 border border-blue-400 dark:border-blue-600 rounded-lg p-6">
                    <h3 class="text-lg font-semibold text-blue-800 dark:text-blue-200 mb-2">
                        🚀 SSR Features
                    </h3>
                    <ul class="text-blue-700 dark:text-blue-300 space-y-1">
                        <li>• Server-side rendering with Rust</li>
                        <li>• Tailwind-RS class generation</li>
                        <li>• Client-side interactivity</li>
                        <li>• SEO-friendly HTML output</li>
                        <li>• Fast initial page load</li>
                    </ul>
                </div>

                <!-- Code Example -->
                <div class="bg-gray-100 dark:bg-gray-800 rounded-lg p-6">
                    <h3 class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4">
                        💻 Rust Code Example
                    </h3>
                    <pre class="text-sm text-gray-700 dark:text-gray-300 overflow-x-auto"><code>use tailwind_rs_core::*;

fn generate_demo_css() -> String {{
    let mut generator = CssGenerator::new();
    
    // Add classes used in components
    let classes = vec![
        "min-h-screen", "bg-gray-100", "dark:bg-gray-900",
        "container", "mx-auto", "px-4", "py-8",
        "text-4xl", "font-bold", "text-center", "mb-8",
        "bg-white", "dark:bg-gray-800", "rounded-lg", "shadow-md",
        "px-4", "py-2", "bg-blue-500", "text-white", "rounded",
        "hover:bg-blue-600", "transition-colors",
    ];
    
    for class in classes {{
        generator.add_class(class);
    }}
    
    generator.generate_css()
}}

fn main() {{
    let css = generate_demo_css();
    std::fs::write("styles.css", css).unwrap();
    println!("✅ Generated CSS for SSR demo");
}}</code></pre>
                </div>

                <div class="text-center text-gray-600 dark:text-gray-400">
                    <p>Built with Rust and Tailwind-RS</p>
                </div>
            </div>
        </div>
    </div>

    <script>
        // Simple JavaScript for interactivity
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
        
        function updateName(value) {{
            document.getElementById('name-display').textContent = value || 'Tailwind-RS SSR';
        }}
        
        // Dark mode toggle
        function toggleDarkMode() {{
            document.documentElement.classList.toggle('dark');
        }}
        
        // Add dark mode toggle button
        document.addEventListener('DOMContentLoaded', function() {{
            const header = document.querySelector('h1');
            const toggleButton = document.createElement('button');
            toggleButton.textContent = '🌙';
            toggleButton.className = 'absolute top-4 right-4 px-3 py-2 bg-gray-200 dark:bg-gray-700 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors';
            toggleButton.onclick = toggleDarkMode;
            header.parentElement.style.position = 'relative';
            header.parentElement.appendChild(toggleButton);
        }});
    </script>
</body>
</html>
"#)
}

/// Handle HTTP requests
fn handle_request(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    
    let request = String::from_utf8_lossy(&buffer[..]);
    let response = if request.starts_with("GET /styles.css") {
        let css = generate_demo_css();
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
    println!("🚀 Starting Tailwind-RS SSR Demo server...");
    println!("📱 Open http://localhost:8080 in your browser");
    
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    
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
