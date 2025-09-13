#!/bin/bash

# Build working WASM Leptos demo using published tailwind-rs-wasm crate
set -e

echo "🔧 Building working WASM Leptos demo..."

# Clean previous builds
rm -rf pkg-working/
rm -rf target/wasm32-unknown-unknown/

# Copy working Cargo.toml
cp Cargo-working.toml Cargo.toml

# Build with wasm-pack
wasm-pack build \
    --target web \
    --out-dir pkg-working \
    --release \
    --no-typescript \
    --no-pack

# Restore original Cargo.toml
git checkout Cargo.toml

echo "✅ Working WASM build completed successfully!"
echo "📦 Output files are in the pkg-working/ directory"
echo "🎯 This demo uses the published tailwind-rs-wasm crate to avoid WASM_BIGINT issues"
echo "🚀 The solution is to use published crates instead of direct WASM compilation"
