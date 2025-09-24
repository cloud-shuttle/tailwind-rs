# Migrating from Manual Tailwind Setup

This guide helps you migrate from a manually configured Tailwind CSS setup to `tailwind-rs`.

## Current Setup Analysis

### Typical Manual Setup
```bash
# Current project structure
my-app/
├── src/
├── styles/
│   ├── input.css
│   └── output.css
├── tailwind.config.js
├── package.json
└── postcss.config.js
```

### Current Configuration Files

**tailwind.config.js**
```javascript
module.exports = {
  content: [
    "./src/**/*.{html,js,ts,jsx,tsx}",
    "./index.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

**postcss.config.js**
```javascript
module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

## Migration Steps

### 1. Add tailwind-rs Dependencies

**Cargo.toml**
```toml
[dependencies]
tailwind-rs = "0.1.0"
tailwind-rs-leptos = "0.1.0"  # For Leptos projects
# or
tailwind-rs-yew = "0.1.0"     # For Yew projects
# or
tailwind-rs-dioxus = "0.1.0"  # For Dioxus projects

[build-dependencies]
tailwind-rs-cli = "0.1.0"
```

### 2. Create Build Script

**build.rs**
```rust
fn main() {
    tailwind_rs_cli::build()
        .input("src/**/*.rs")
        .output("dist/styles.css")
        .watch(true)
        .run()
        .expect("Failed to build Tailwind CSS");
}
```

### 3. Update Project Structure

```bash
# New project structure
my-app/
├── src/
├── build.rs
├── Cargo.toml
└── tailwind.toml  # New configuration file
```

### 4. Create Tailwind Configuration

**tailwind.toml**
```toml
[build]
input = ["src/**/*.rs"]
output = "dist/styles.css"
watch = true

[theme]
# Your existing theme configuration
[theme.colors]
primary = "#3b82f6"
secondary = "#64748b"

[theme.spacing]
sm = "0.5rem"
md = "1rem"
lg = "1.5rem"
```

### 5. Update Rust Code

**Before (Manual CSS classes)**
```rust
use leptos::*;

#[component]
pub fn Button() -> impl IntoView {
    view! {
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
            "Click me"
        </button>
    }
}
```

**After (Type-safe classes)**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn Button() -> impl IntoView {
    let button_classes = classes!(
        "bg-blue-500",
        "hover:bg-blue-700", 
        "text-white",
        "font-bold",
        "py-2",
        "px-4",
        "rounded"
    );
    
    view! {
        <button class=button_classes>
            "Click me"
        </button>
    }
}
```

### 6. Remove Old Files

```bash
# Remove old configuration files
rm tailwind.config.js
rm postcss.config.js
rm package.json  # If only used for Tailwind
rm -rf node_modules
```

## Advanced Migration Features

### Dynamic Styling
```rust
use tailwind_rs::{classes, theme};

#[component]
pub fn DynamicButton(variant: String) -> impl IntoView {
    let base_classes = classes!("font-bold", "py-2", "px-4", "rounded");
    let variant_classes = match variant.as_str() {
        "primary" => classes!("bg-blue-500", "hover:bg-blue-700", "text-white"),
        "secondary" => classes!("bg-gray-500", "hover:bg-gray-700", "text-white"),
        _ => classes!("bg-gray-300", "text-gray-700"),
    };
    
    view! {
        <button class=classes!(base_classes, variant_classes)>
            "Dynamic Button"
        </button>
    }
}
```

### Theme Integration
```rust
use tailwind_rs::theme;

#[component]
pub fn ThemedComponent() -> impl IntoView {
    let primary_color = theme::color("primary");
    let spacing = theme::spacing("md");
    
    view! {
        <div style=format!("color: {}; padding: {}", primary_color, spacing)>
            "Themed content"
        </div>
    }
}
```

### Responsive Design
```rust
use tailwind_rs::{classes, responsive};

#[component]
pub fn ResponsiveGrid() -> impl IntoView {
    let grid_classes = classes!(
        "grid",
        responsive::sm("grid-cols-1"),
        responsive::md("grid-cols-2"), 
        responsive::lg("grid-cols-3"),
        responsive::xl("grid-cols-4")
    );
    
    view! {
        <div class=grid_classes>
            // Grid items
        </div>
    }
}
```

## Testing Migration

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_button_classes() {
        let button = Button();
        let classes = extract_classes(button);
        
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("text-white"));
    }
}
```

### Integration Tests
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_component_rendering() {
        let app = create_test_app(|| view! { <Button /> });
        let html = render_to_string(app);
        
        assert!(html.contains("bg-blue-500"));
        assert!(html.contains("Click me"));
    }
}
```

### Playwright E2E Tests
```typescript
// tests/button.spec.ts
import { test, expect } from '@playwright/test';

test('button renders with correct styles', async ({ page }) => {
  await page.goto('/');
  
  const button = page.locator('button');
  await expect(button).toBeVisible();
  
  // Check computed styles
  const styles = await button.evaluate((el) => {
    const computed = getComputedStyle(el);
    return {
      backgroundColor: computed.backgroundColor,
      color: computed.color,
      padding: computed.padding
    };
  });
  
  expect(styles.backgroundColor).toBe('rgb(59, 130, 246)'); // blue-500
  expect(styles.color).toBe('rgb(255, 255, 255)'); // white
});
```

## Performance Comparison

### Before Migration
- **Bundle size**: ~200KB (full Tailwind CSS)
- **Build time**: 3-5 seconds
- **Runtime**: No optimization

### After Migration
- **Bundle size**: ~15KB (tree-shaken)
- **Build time**: 1-2 seconds
- **Runtime**: Optimized class generation

## Troubleshooting

### Common Issues

**Issue**: Classes not being detected
```toml
# Solution: Update tailwind.toml
[build]
input = ["src/**/*.rs", "examples/**/*.rs"]  # Add all source paths
```

**Issue**: Build errors
```rust
// Solution: Check build.rs
fn main() {
    tailwind_rs_cli::build()
        .input("src/**/*.rs")
        .output("dist/styles.css")
        .run()  // Remove .watch(true) for production builds
        .expect("Failed to build Tailwind CSS");
}
```

**Issue**: Runtime class generation not working
```rust
// Solution: Ensure proper imports
use tailwind_rs::{classes, theme, responsive};
```

## Next Steps

1. ✅ Complete basic migration
2. 🧪 Run test suite to validate functionality
3. 🚀 Deploy and monitor performance
4. 📈 Leverage advanced features (theming, responsive design)
5. 🔄 Iterate and optimize

## Additional Resources

- [Getting Started Guide](../getting-started.md)
- [API Reference](../api/README.md)
- [Testing Guidelines](../testing.md)
- [Architecture Documentation](../architecture.md)

