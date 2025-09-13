#!/bin/bash

# Build minimal WASM demo to avoid WASM_BIGINT issues
set -e

echo "🔧 Building minimal Leptos demo for WASM..."

# Clean previous builds
rm -rf pkg-minimal/
rm -rf target/wasm32-unknown-unknown/

# Copy minimal Cargo.toml
cp Cargo-minimal.toml Cargo.toml

# Build with wasm-pack
wasm-pack build \
    --target web \
    --out-dir pkg-minimal \
    --release \
    --no-typescript \
    --no-pack

# Restore original Cargo.toml
git checkout Cargo.toml

echo "✅ Minimal WASM build completed successfully!"
echo "📦 Output files are in the pkg-minimal/ directory"
