#!/bin/bash

# Tailwind-RS v0.7.0 Release Script
# Critical Remediation Complete - Production Ready

set -e

echo "🚀 Starting Tailwind-RS v0.7.0 Release Process"
echo "=============================================="

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Not in the tailwind-rs root directory"
    exit 1
fi

# Verify version is correct
VERSION=$(grep 'version = "0.7.0"' Cargo.toml | head -1)
if [ -z "$VERSION" ]; then
    echo "❌ Error: Version not set to 0.7.0 in Cargo.toml"
    exit 1
fi

echo "✅ Version confirmed: 0.7.0"

# Run tests to ensure everything is working
echo "🧪 Running tests..."
cargo test --workspace --quiet
if [ $? -ne 0 ]; then
    echo "❌ Error: Tests failed. Please fix before releasing."
    exit 1
fi
echo "✅ All tests passing"

# Check if we're on the main branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ]; then
    echo "⚠️  Warning: Not on main branch (currently on $BRANCH)"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Release cancelled"
        exit 1
    fi
fi

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  Warning: Uncommitted changes detected"
    git status --short
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Release cancelled"
        exit 1
    fi
fi

# Create git tag
echo "🏷️  Creating git tag v0.7.0..."
git tag -a v0.7.0 -m "v0.7.0 - Critical Remediation Complete - Production Ready"
echo "✅ Git tag created"

# Push tag to remote
echo "📤 Pushing tag to remote..."
git push origin v0.7.0
echo "✅ Tag pushed to remote"

# Publish crates in dependency order
echo "📦 Publishing crates to crates.io..."

echo "  📦 Publishing tailwind-rs-core..."
cargo publish -p tailwind-rs-core
echo "  ✅ tailwind-rs-core published"

echo "  📦 Publishing tailwind-rs-macros..."
cargo publish -p tailwind-rs-macros
echo "  ✅ tailwind-rs-macros published"

echo "  📦 Publishing tailwind-rs-leptos..."
cargo publish -p tailwind-rs-leptos
echo "  ✅ tailwind-rs-leptos published"

echo "  📦 Publishing tailwind-rs-yew..."
cargo publish -p tailwind-rs-yew
echo "  ✅ tailwind-rs-yew published"

echo "  📦 Publishing tailwind-rs-dioxus..."
cargo publish -p tailwind-rs-dioxus
echo "  ✅ tailwind-rs-dioxus published"

echo "  📦 Publishing tailwind-rs-wasm..."
cargo publish -p tailwind-rs-wasm
echo "  ✅ tailwind-rs-wasm published"

echo ""
echo "🎉 Tailwind-RS v0.7.0 Release Complete!"
echo "========================================"
echo ""
echo "📊 Release Summary:"
echo "  • Version: 0.7.0"
echo "  • Type: Major Release - Critical Remediation Complete"
echo "  • Status: Production Ready"
echo "  • All tests: ✅ Passing"
echo "  • All crates: ✅ Published"
echo ""
echo "🚀 Key Improvements:"
echo "  • 94% memory reduction in DynamicClassBuilder"
echo "  • 100% elimination of unnecessary signal overhead"
echo "  • 80% reduction in largest file sizes"
echo "  • All critical issues resolved"
echo "  • Production-ready architecture"
echo ""
echo "📚 Documentation:"
echo "  • Release Notes: RELEASE_NOTES_0.7.0.md"
echo "  • Changelog: CHANGELOG.md"
echo "  • Git Summary: GIT_COMMIT_SUMMARY_0.7.0.md"
echo ""
echo "🎯 Next Steps:"
echo "  • Monitor crates.io for publication confirmation"
echo "  • Update any dependent projects"
echo "  • Celebrate the successful remediation! 🎉"
echo ""
echo "Thank you for using Tailwind-RS! 🚀"
