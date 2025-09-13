# Installation Guide

This guide will help you install and set up `tailwind-rs` in your Rust web project, following our established standards and best practices.

## 📋 Prerequisites

### System Requirements
- **Rust**: 1.89.0+ (latest stable recommended)
- **Cargo**: Latest version
- **Node.js**: 18+ (for Playwright testing)
- **PNPM**: 8+ (for package management, following ADR-005)

### Development Tools
- **VS Code** with rust-analyzer (recommended)
- **Git** for version control
- **Playwright** for end-to-end testing

## 🚀 Quick Installation

### 1. Create New Project
```bash
# Create a new Rust project
cargo new my-tailwind-app
cd my-tailwind-app

# Initialize as a workspace if needed
cargo init --lib
```

### 2. Add Dependencies
```toml
# Cargo.toml
[package]
name = "my-tailwind-app"
version = "0.1.0"
edition = "2021"
rust-version = "1.89.0"

[dependencies]
# Core tailwind-rs
tailwind-rs = "0.1.0"

# Framework-specific integration (choose one)
tailwind-rs-leptos = "0.1.0"  # For Leptos
# tailwind-rs-yew = "0.1.0"   # For Yew
# tailwind-rs-dioxus = "0.1.0" # For Dioxus

# Framework dependencies
leptos = { version = "0.8.8", features = ["csr"] }
leptos_router = "0.8.8"

# Supporting crates
wasm-bindgen = "0.2.101"
web-sys = "0.3.77"
console_error_panic_hook = "0.1.7"
console_log = "1.0.0"
log = "0.4.20"
serde = { version = "1.0", features = ["derive"] }

[build-dependencies]
tailwind-rs-build = "0.1.0"

[dev-dependencies]
# Testing dependencies (following ADR-002)
criterion = { version = "0.5", features = ["html_reports"] }
proptest = "1.4"
mockall = "0.12"
wasm-bindgen-test = "0.3"
```

### 3. Create Build Script
```rust
// build.rs
use tailwind_rs_build::TailwindBuilder;

fn main() {
    TailwindBuilder::new()
        .scan_source("src/")
        .generate_css("dist/styles.css")
        .optimize()
        .enable_tree_shaking()
        .enable_minification()
        .build()
        .expect("Failed to build Tailwind CSS");
}
```

### 4. Setup Testing (Following ADR-002)
```bash
# Install Node.js dependencies for Playwright testing
pnpm init
pnpm add -D @playwright/test playwright
pnpm add -D concurrently sass

# Install Playwright browsers
pnpm run playwright:install
```

### 5. Configure Package.json
```json
{
  "name": "my-tailwind-app",
  "version": "1.0.0",
  "packageManager": "pnpm@8.15.0",
  "engines": {
    "node": ">=18.0.0",
    "pnpm": ">=8.0.0"
  },
  "scripts": {
    "install": "pnpm install --frozen-lockfile",
    "build": "pnpm run build:css && pnpm run build:wasm",
    "build:css": "sass src/style/main.scss src/style/main.css",
    "build:wasm": "wasm-pack build --target web --out-dir pkg",
    "dev": "concurrently \"pnpm run watch:css\" \"pnpm run serve\"",
    "watch:css": "sass --watch src/style/main.scss:src/style/main.css",
    "serve": "python3 -m http.server 8080",
    "test": "pnpm run test:unit && pnpm run test:integration && pnpm run test:e2e",
    "test:unit": "cargo test",
    "test:integration": "cargo test --test integration",
    "test:e2e": "playwright test",
    "test:e2e:ui": "playwright test --ui",
    "test:e2e:headed": "playwright test --headed",
    "test:all": "pnpm run test && pnpm run test:e2e",
    "test:watch": "playwright test --watch",
    "playwright:install": "playwright install",
    "playwright:install-deps": "playwright install-deps"
  },
  "devDependencies": {
    "sass": "^1.69.5",
    "concurrently": "^8.2.2",
    "@playwright/test": "^1.40.0",
    "playwright": "^1.40.0"
  }
}
```

## 🎯 Framework-Specific Installation

### Leptos Installation (Following ADR-006)
```toml
# Cargo.toml - Leptos configuration
[dependencies]
leptos = { version = "0.8.8", features = ["csr"] }
leptos_router = "0.8.8"
tailwind-rs = "0.1.0"
tailwind-rs-leptos = "0.1.0"

# Additional Leptos ecosystem crates (following ADR-009)
leptos-flow = "0.1.0"
leptos-forms-rs = "0.1.0"
leptos-helios = "0.1.0"
leptos-motion = "0.1.0"
leptos-query-rs = "0.1.0"
leptos-shadcn-ui = "0.1.0"
leptos-state = "0.1.0"
leptos-sync = "0.1.0"
leptos-ws-pro = "0.1.0"
leptos-next-metadata = "0.1.0"
radix-leptos = "0.1.0"
```

### Yew Installation
```toml
# Cargo.toml - Yew configuration
[dependencies]
yew = "0.21"
tailwind-rs = "0.1.0"
tailwind-rs-yew = "0.1.0"
wasm-bindgen = "0.2.101"
web-sys = "0.3.77"
```

### Dioxus Installation
```toml
# Cargo.toml - Dioxus configuration
[dependencies]
dioxus = "0.4"
tailwind-rs = "0.1.0"
tailwind-rs-dioxus = "0.1.0"
```

## 🧪 Testing Setup (Following ADR-002)

### Unit Testing Configuration
```rust
// tests/unit/class_generation.rs
#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::*;
    use proptest::prelude::*;

    #[test]
    fn test_class_generation() {
        // Given
        let classes = classes! {
            base: "px-4 py-2",
            variant: "bg-blue-600 text-white",
        };
        
        // When
        let result = classes.to_string();
        
        // Then
        assert!(result.contains("px-4"));
        assert!(result.contains("bg-blue-600"));
    }

    // Property-based testing
    proptest! {
        #[test]
        fn test_class_validation_properties(class_name in "[a-zA-Z0-9-]+") {
            let result = validate_class(&class_name);
            
            match result {
                Ok(_) | Err(ValidationError::InvalidClass(_)) => {
                    prop_assert!(true);
                }
                _ => prop_assert!(false, "Unexpected error type"),
            }
        }
    }
}
```

### Integration Testing Configuration
```rust
// tests/integration/framework_tests.rs
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_framework_integration() {
        // Test framework-specific integration
        let component = TestComponent::new();
        let classes = component.generate_classes();
        
        assert!(classes.contains("framework-specific"));
    }

    #[tokio::test]
    async fn test_async_integration() {
        // Test async functionality
        let async_generator = AsyncClassGenerator::new();
        let classes = async_generator.generate_classes().await;
        
        assert!(!classes.is_empty());
    }
}
```

### End-to-End Testing Configuration (Following ADR-003)
```typescript
// tests/e2e/demo.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Tailwind-rs Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/demo');
    await page.waitForLoadState('networkidle');
  });

  test('should demonstrate class generation', async ({ page }) => {
    // Test class generation functionality
    await page.click('[data-testid="generate-classes"]');
    
    await expect(page.locator('[data-testid="generated-classes"]'))
      .toBeVisible();
    
    await expect(page.locator('[data-testid="generated-classes"]'))
      .toContainText('bg-blue-600 text-white');
  });

  test('should demonstrate dynamic styling', async ({ page }) => {
    // Test dynamic class generation
    await page.selectOption('[data-testid="color-selector"]', 'green');
    await page.selectOption('[data-testid="intensity-selector"]', '700');
    
    await page.click('[data-testid="apply-dynamic-styles"]');
    
    await expect(page.locator('[data-testid="dynamic-element"]'))
      .toHaveClass(/bg-green-700/);
  });

  test('should meet performance requirements', async ({ page }) => {
    // Test performance
    const startTime = Date.now();
    await page.click('[data-testid="performance-test"]');
    const endTime = Date.now();
    
    expect(endTime - startTime).toBeLessThan(1000);
  });
});
```

### Playwright Configuration
```typescript
// playwright.config.ts
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:8080',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox', use: { ...devices['Desktop Firefox'] } },
    { name: 'webkit', use: { ...devices['Desktop Safari'] } },
    { name: 'mobile', use: { ...devices['iPhone 12'] } },
  ],
  webServer: {
    command: 'python3 -m http.server 8080',
    url: 'http://localhost:8080',
    reuseExistingServer: !process.env.CI,
  },
});
```

## 🔧 Development Configuration

### VS Code Configuration
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll": true
  },
  "files.associations": {
    "*.rs": "rust"
  }
}
```

### Rustfmt Configuration
```toml
# rustfmt.toml
edition = "2021"
max_width = 100
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

### Clippy Configuration
```toml
# .clippy.toml
deny = [
    "clippy::all",
    "clippy::pedantic",
    "clippy::nursery",
    "clippy::cargo",
]
warn = [
    "clippy::missing_docs_in_private_items",
    "clippy::missing_errors_doc",
    "clippy::missing_panics_doc",
]
```

## 🚀 CI/CD Setup (Following ADR-004)

### GitHub Actions Configuration
```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '18'
      
      - name: Setup PNPM
        uses: pnpm/action-setup@v2
        with:
          version: 8
      
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Install dependencies
        run: pnpm install --frozen-lockfile
      
      - name: Install Playwright
        run: pnpm run playwright:install
      
      - name: Run unit tests
        run: cargo test --verbose
      
      - name: Run integration tests
        run: cargo test --test integration
      
      - name: Run E2E tests
        run: pnpm run test:e2e
      
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      
      - name: Run rustfmt
        run: cargo fmt --all -- --check
      
      - name: Generate coverage
        run: cargo tarpaulin --out Html --output-dir coverage
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          file: coverage/tarpaulin-report.html
```

## 🎯 Verification

### Test Installation
```bash
# Run the test suite
cargo test
pnpm run test:e2e

# Check code quality
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check

# Build the project
cargo build --release
```

### Verify Framework Integration
```rust
// src/main.rs - Test basic functionality
use leptos::*;
use tailwind_rs::*;

#[component]
pub fn App() -> impl IntoView {
    let classes = classes! {
        base: "min-h-screen bg-gray-100",
        typography: "font-sans",
    };
    
    view! {
        <div class=classes>
            <h1 class="text-2xl font-bold text-gray-900">
                "Tailwind-rs is working!"
            </h1>
        </div>
    }
}

fn main() {
    mount_to_body(App)
}
```

## 🔧 Troubleshooting

### Common Issues

#### Build Errors
```bash
# Clear cargo cache
cargo clean
rm -rf target/

# Update dependencies
cargo update

# Rebuild
cargo build
```

#### Playwright Issues
```bash
# Reinstall Playwright
pnpm run playwright:install

# Clear Playwright cache
rm -rf node_modules/.cache/playwright
```

#### Rust Version Issues
```bash
# Update Rust
rustup update stable

# Set default toolchain
rustup default stable
```

### Getting Help
- **GitHub Issues**: Report bugs and request features
- **Discord Community**: Real-time support
- **Stack Overflow**: Technical questions with `tailwind-rs` tag
- **Documentation**: Comprehensive guides and examples

## 📚 Next Steps

After successful installation:

1. **Read the [Getting Started Guide](./getting-started.md)**
2. **Explore the [API Reference](./api/core.md)**
3. **Check out [Example Projects](./examples/README.md)**
4. **Join the [Community Discord](https://discord.gg/tailwind-rs)**

---

Congratulations! You've successfully installed `tailwind-rs` and are ready to build amazing Rust web applications with type-safe, performant Tailwind CSS integration.

