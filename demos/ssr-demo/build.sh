#!/bin/bash

echo "🚀 Building Tailwind-RS SSR Demo..."

# Create necessary directories
mkdir -p static
mkdir -p target/site

# Generate CSS
echo "📦 Generating CSS..."
cargo run --bin css-generator

# Build the application
echo "🔨 Building application..."
cargo build --release

echo "✅ SSR Demo build complete!"
echo "📱 Run with: cargo run --release"
echo "🌐 Open http://localhost:3000 in your browser"
