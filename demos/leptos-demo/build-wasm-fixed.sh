#!/bin/bash

# Build WASM-fixed Leptos demo using tailwind-rs-wasm crate
set -e

echo "🔧 Building WASM-fixed Leptos demo..."

# Clean previous builds
rm -rf pkg-wasm-fixed/
rm -rf target/wasm32-unknown-unknown/

# Copy WASM-fixed Cargo.toml
cp Cargo-wasm-fixed.toml Cargo.toml

# Build with wasm-pack
wasm-pack build \
    --target web \
    --out-dir pkg-wasm-fixed \
    --release \
    --no-typescript \
    --no-pack

# Restore original Cargo.toml
git checkout Cargo.toml

echo "✅ WASM-fixed build completed successfully!"
echo "📦 Output files are in the pkg-wasm-fixed/ directory"
echo "🌐 This demo avoids the WASM_BIGINT linker error by using tailwind-rs-wasm crate"
