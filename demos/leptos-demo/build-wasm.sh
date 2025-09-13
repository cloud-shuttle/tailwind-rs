#!/bin/bash

# Custom WASM build script to avoid WASM_BIGINT issues
set -e

echo "🔧 Building Leptos demo for WASM..."

# Clean previous builds
rm -rf pkg/
rm -rf target/wasm32-unknown-unknown/

# Set environment variables to avoid WASM_BIGINT
export RUSTFLAGS="-C link-arg=--no-entry -C link-arg=--export-dynamic -C link-arg=--allow-undefined"

# Build with wasm-pack using specific flags
wasm-pack build \
    --target web \
    --out-dir pkg \
    --release \
    --no-typescript \
    --no-pack \
    -- --no-default-features

echo "✅ WASM build completed successfully!"
echo "📦 Output files are in the pkg/ directory"
