# Migrating from tailwindcss-rs

This guide helps you migrate from the existing `tailwindcss-rs` crate to the new `tailwind-rs` library.

## Current Setup Analysis

### Existing tailwindcss-rs Setup
```toml
[dependencies]
tailwindcss-rs = "0.1.0"
```

### Current Usage Pattern
```rust
use tailwindcss_rs::TailwindCSS;

fn main() {
    let css = TailwindCSS::new()
        .content("src/**/*.rs")
        .build()
        .unwrap();
    
    std::fs::write("styles.css", css).unwrap();
}
```

## Key Differences

| Feature | tailwindcss-rs | tailwind-rs |
|---------|----------------|-------------|
| **Type Safety** | ❌ Runtime only | ✅ Compile-time validation |
| **Framework Integration** | ❌ Manual setup | ✅ Native framework support |
| **Dynamic Styling** | ❌ Static only | ✅ Runtime class generation |
| **Performance** | ❌ Basic optimization | ✅ Tree-shaking + caching |
| **Testing** | ❌ Limited support | ✅ Comprehensive test suite |
| **IDE Support** | ❌ Basic | ✅ Full autocomplete |

## Migration Steps

### 1. Update Dependencies

**Before (Cargo.toml)**
```toml
[dependencies]
tailwindcss-rs = "0.1.0"
```

**After (Cargo.toml)**
```toml
[dependencies]
tailwind-rs = "0.1.0"
tailwind-rs-leptos = "0.1.0"  # Choose your framework

[build-dependencies]
tailwind-rs-cli = "0.1.0"
```

### 2. Update Build Configuration

**Before (build.rs)**
```rust
use tailwindcss_rs::TailwindCSS;

fn main() {
    let css = TailwindCSS::new()
        .content("src/**/*.rs")
        .build()
        .unwrap();
    
    std::fs::write("dist/styles.css", css).unwrap();
}
```

**After (build.rs)**
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

### 3. Update Rust Code

**Before (Manual class strings)**
```rust
use leptos::*;

#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div class="bg-white shadow-lg rounded-lg p-6">
            <h2 class="text-xl font-bold text-gray-800 mb-4">
                "Card Title"
            </h2>
            <p class="text-gray-600">
                "Card content goes here"
            </p>
        </div>
    }
}
```

**After (Type-safe classes)**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn Card() -> impl IntoView {
    let card_classes = classes!(
        "bg-white",
        "shadow-lg", 
        "rounded-lg",
        "p-6"
    );
    
    let title_classes = classes!(
        "text-xl",
        "font-bold",
        "text-gray-800",
        "mb-4"
    );
    
    let content_classes = classes!("text-gray-600");
    
    view! {
        <div class=card_classes>
            <h2 class=title_classes>
                "Card Title"
            </h2>
            <p class=content_classes>
                "Card content goes here"
            </p>
        </div>
    }
}
```

### 4. Add Configuration File

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

## Advanced Migration Features

### Dynamic Class Generation
```rust
use tailwind_rs::{classes, theme};

#[component]
pub fn DynamicCard(variant: String, size: String) -> impl IntoView {
    let base_classes = classes!("bg-white", "shadow-lg", "rounded-lg");
    
    let variant_classes = match variant.as_str() {
        "primary" => classes!("border-l-4", "border-blue-500"),
        "success" => classes!("border-l-4", "border-green-500"),
        "warning" => classes!("border-l-4", "border-yellow-500"),
        _ => classes!(),
    };
    
    let size_classes = match size.as_str() {
        "sm" => classes!("p-4"),
        "md" => classes!("p-6"),
        "lg" => classes!("p-8"),
        _ => classes!("p-6"),
    };
    
    view! {
        <div class=classes!(base_classes, variant_classes, size_classes)>
            "Dynamic Card"
        </div>
    }
}
```

### Theme System Integration
```rust
use tailwind_rs::theme;

#[component]
pub fn ThemedComponent() -> impl IntoView {
    let primary_color = theme::color("primary");
    let spacing = theme::spacing("md");
    
    view! {
        <div 
            class="rounded-lg border"
            style=format!("border-color: {}; padding: {}", primary_color, spacing)
        >
            "Themed content"
        </div>
    }
}
```

### Responsive Design
```rust
use tailwind_rs::{classes, responsive};

#[component]
pub fn ResponsiveCard() -> impl IntoView {
    let card_classes = classes!(
        "bg-white",
        "shadow-lg",
        "rounded-lg",
        "p-4",
        responsive::sm("p-6"),
        responsive::md("p-8"),
        responsive::lg("p-10")
    );
    
    view! {
        <div class=card_classes>
            "Responsive Card"
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
    fn test_card_classes() {
        let card = Card();
        let classes = extract_classes(card);
        
        assert!(classes.contains("bg-white"));
        assert!(classes.contains("shadow-lg"));
        assert!(classes.contains("rounded-lg"));
        assert!(classes.contains("p-6"));
    }
    
    #[test]
    fn test_dynamic_card_variants() {
        let primary_card = DynamicCard("primary".to_string(), "md".to_string());
        let classes = extract_classes(primary_card);
        
        assert!(classes.contains("border-l-4"));
        assert!(classes.contains("border-blue-500"));
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
    fn test_card_rendering() {
        let app = create_test_app(|| view! { <Card /> });
        let html = render_to_string(app);
        
        assert!(html.contains("bg-white"));
        assert!(html.contains("Card Title"));
    }
}
```

### Playwright E2E Tests
```typescript
// tests/card.spec.ts
import { test, expect } from '@playwright/test';

test('card renders with correct styles', async ({ page }) => {
  await page.goto('/');
  
  const card = page.locator('.bg-white.shadow-lg.rounded-lg');
  await expect(card).toBeVisible();
  
  // Check computed styles
  const styles = await card.evaluate((el) => {
    const computed = getComputedStyle(el);
    return {
      backgroundColor: computed.backgroundColor,
      boxShadow: computed.boxShadow,
      borderRadius: computed.borderRadius,
      padding: computed.padding
    };
  });
  
  expect(styles.backgroundColor).toBe('rgb(255, 255, 255)');
  expect(styles.boxShadow).not.toBe('none');
  expect(styles.borderRadius).not.toBe('0px');
  expect(styles.padding).toBe('24px'); // p-6
});
```

## Performance Improvements

### Bundle Size Comparison
```rust
// Before: tailwindcss-rs
// Full CSS bundle: ~200KB
// No tree-shaking

// After: tailwind-rs  
// Tree-shaken bundle: ~15KB
// Runtime optimization
```

### Build Time Comparison
```bash
# Before: tailwindcss-rs
# Build time: 3-5 seconds
# No incremental builds

# After: tailwind-rs
# Build time: 1-2 seconds  
# Incremental builds with watch mode
```

## Migration Checklist

- [ ] Update `Cargo.toml` dependencies
- [ ] Replace `build.rs` configuration
- [ ] Add `tailwind.toml` configuration
- [ ] Update component code to use `classes!` macro
- [ ] Add type-safe class generation
- [ ] Implement dynamic styling features
- [ ] Add comprehensive test suite
- [ ] Update CI/CD pipeline
- [ ] Deploy and monitor performance

## Troubleshooting

### Common Issues

**Issue**: Build errors after migration
```rust
// Solution: Check build.rs syntax
fn main() {
    tailwind_rs_cli::build()
        .input("src/**/*.rs")
        .output("dist/styles.css")
        .run()  // Remove .watch(true) for CI builds
        .expect("Failed to build Tailwind CSS");
}
```

**Issue**: Classes not being detected
```toml
# Solution: Update tailwind.toml input paths
[build]
input = ["src/**/*.rs", "examples/**/*.rs", "tests/**/*.rs"]
```

**Issue**: Runtime class generation not working
```rust
// Solution: Ensure proper imports
use tailwind_rs::{classes, theme, responsive};
```

## Next Steps

1. ✅ Complete basic migration
2. 🧪 Run comprehensive test suite
3. 🚀 Deploy and monitor performance
4. 📈 Leverage advanced features
5. 🔄 Iterate and optimize

## Additional Resources

- [Getting Started Guide](../getting-started.md)
- [API Reference](../api/README.md)
- [Testing Guidelines](../testing.md)
- [Architecture Documentation](../architecture.md)

