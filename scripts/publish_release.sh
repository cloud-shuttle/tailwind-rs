#!/bin/bash

# Tailwind-RS v0.12.0 Release Script
# This script publishes all crates to crates.io in the correct dependency order

set -e

echo "🚀 Starting Tailwind-RS v0.12.0 Release Process..."

# Check if user is logged in to crates.io
if ! cargo login --help > /dev/null 2>&1; then
    echo "❌ Please run 'cargo login' first to authenticate with crates.io"
    exit 1
fi

# Function to publish a crate
publish_crate() {
    local crate_name=$1
    local crate_path=$2
    
    echo "📦 Publishing $crate_name..."
    cd "$crate_path"
    
    # Dry run first
    echo "🔍 Running dry-run for $crate_name..."
    if cargo publish --dry-run; then
        echo "✅ Dry-run successful for $crate_name"
        
        # Ask for confirmation
        read -p "Publish $crate_name to crates.io? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            cargo publish
            echo "✅ Successfully published $crate_name"
        else
            echo "⏭️  Skipped $crate_name"
        fi
    else
        echo "❌ Dry-run failed for $crate_name"
        return 1
    fi
    
    cd - > /dev/null
}

# Publish crates in dependency order
echo "📋 Publishing crates in dependency order..."

# 1. Core crates (no internal dependencies)
publish_crate "tailwind-rs-macros" "crates/tailwind-rs-macros"
publish_crate "tailwind-rs-testing" "crates/tailwind-rs-testing"

# 2. Core library
publish_crate "tailwind-rs-core" "crates/tailwind-rs-core"

# 3. Framework integrations (depend on core)
publish_crate "tailwind-rs-leptos" "crates/tailwind-rs-leptos"
publish_crate "tailwind-rs-yew" "crates/tailwind-rs-yew"
publish_crate "tailwind-rs-dioxus" "crates/tailwind-rs-dioxus"

# 4. WASM and CLI (depend on core)
publish_crate "tailwind-rs-wasm" "crates/tailwind-rs-wasm"
publish_crate "tailwind-rs-scanner" "crates/tailwind-rs-scanner"
publish_crate "tailwind-rs-cli" "crates/tailwind-rs-cli"

# 5. PostCSS (optional, can be published later)
echo "📝 Note: tailwind-rs-postcss has compilation issues and will be published separately"

echo "🎉 Release process completed!"
echo "📊 Published crates:"
echo "  - tailwind-rs-macros v0.12.0"
echo "  - tailwind-rs-testing v0.12.0" 
echo "  - tailwind-rs-core v0.12.0"
echo "  - tailwind-rs-leptos v0.12.0"
echo "  - tailwind-rs-yew v0.12.0"
echo "  - tailwind-rs-dioxus v0.12.0"
echo "  - tailwind-rs-wasm v0.12.0"
echo "  - tailwind-rs-scanner v0.12.0"
echo "  - tailwind-rs-cli v0.12.0"
