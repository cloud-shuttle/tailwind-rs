use tailwind_rs_core::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};

/// Generate CSS for the SSR demo
fn generate_demo_css() -> String {
    let mut generator = CssGenerator::new();
    
    // Add all classes used in the demo
    let classes = vec![
        "min-h-screen", "bg-gray-100", "dark:bg-gray-900",
        "container", "mx-auto", "px-4", "py-8",
        "text-4xl", "font-bold", "text-center", "mb-8",
        "text-gray-800", "dark:text-white",
        "bg-white", "dark:bg-gray-800", "rounded-lg", "shadow-md", "p-6",
        "px-4", "py-2", "bg-blue-500", "text-white", "rounded",
        "hover:bg-blue-600", "transition-colors",
        "grid", "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3", "gap-4",
        "bg-green-100", "dark:bg-green-900", "border-green-400",
        "text-green-800", "dark:text-green-200",
        "bg-blue-100", "dark:bg-blue-900", "border-blue-400",
        "text-blue-800", "dark:text-blue-200",
    ];
    
    for class in classes {
        generator.add_class(class);
    }
    
    generator.generate_css()
}

/// Generate dynamic HTML with server-side data
fn generate_html() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Tailwind-RS Real SSR Demo</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="stylesheet" href="/styles.css">
</head>
<body>
    <div class="min-h-screen bg-gray-100 dark:bg-gray-900">
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-4xl font-bold text-center mb-8 text-gray-800 dark:text-white">
                Tailwind-RS Real Server-Side Rendering
            </h1>
            
            <div class="max-w-4xl mx-auto space-y-6">
                <!-- SSR Status with Server Data -->
                <div class="bg-green-100 dark:bg-green-900 border border-green-400 dark:border-green-600 rounded-lg p-6">
                    <h2 class="text-2xl font-semibold text-green-800 dark:text-green-200 mb-4">
                        ✅ Real Server-Side Rendering Active!
                    </h2>
                    <p class="text-green-700 dark:text-green-300 mb-4">
                        This page is rendered on the server with Rust and Tailwind-RS classes.
                    </p>
                    <div class="bg-green-200 dark:bg-green-800 rounded p-3">
                        <p class="text-sm text-green-800 dark:text-green-200">
                            Server: Rust HTTP Server | Timestamp: {} | Framework: Rust + Tailwind-RS
                        </p>
                    </div>
                </div>

                <!-- Server-Generated Data -->
                <div class="bg-blue-100 dark:bg-blue-900 border border-blue-400 dark:border-blue-600 rounded-lg p-6">
                    <h2 class="text-2xl font-semibold text-blue-800 dark:text-blue-200 mb-4">
                        🚀 Server-Generated Data
                    </h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="bg-white dark:bg-gray-800 p-4 rounded-lg">
                            <h3 class="font-semibold text-gray-800 dark:text-white mb-2">Server Info</h3>
                            <p class="text-sm text-gray-600 dark:text-gray-300">Rust HTTP Server with Tailwind-RS</p>
                        </div>
                        <div class="bg-white dark:bg-gray-800 p-4 rounded-lg">
                            <h3 class="font-semibold text-gray-800 dark:text-white mb-2">Request Time</h3>
                            <p class="text-sm text-gray-600 dark:text-gray-300">Unix Timestamp: {}</p>
                        </div>
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
                        🚀 Real SSR Features
                    </h3>
                    <ul class="text-blue-700 dark:text-blue-300 space-y-1">
                        <li>• Server-side rendering with Rust</li>
                        <li>• Tailwind-RS class generation</li>
                        <li>• Dynamic server data injection</li>
                        <li>• SEO-friendly HTML output</li>
                        <li>• Fast initial page load</li>
                        <li>• Real-time server data</li>
                    </ul>
                </div>

                <div class="text-center text-gray-600 dark:text-gray-400">
                    <p>Built with Rust SSR and Tailwind-RS</p>
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
"#, timestamp, timestamp)
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
    println!("🚀 Starting Tailwind-RS Real SSR Demo server...");
    println!("📱 Open http://localhost:3000 in your browser");
    println!("🔧 This is a real Rust HTTP server with SSR!");
    
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
