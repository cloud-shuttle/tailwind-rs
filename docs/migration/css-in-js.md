# Migrating from CSS-in-JS Solutions

This guide helps you migrate from CSS-in-JS approaches to `tailwind-rs` for better performance and type safety.

## Current Setup Analysis

### Common CSS-in-JS Patterns
```rust
// Styled components approach
use styled::*;

#[styled]
fn Button() -> StyledComponent {
    styled! {
        button {
            background-color: #3b82f6;
            color: white;
            padding: 0.5rem 1rem;
            border-radius: 0.25rem;
            border: none;
            cursor: pointer;
            
            &:hover {
                background-color: #2563eb;
            }
        }
    }
}

// Inline styles approach
use leptos::*;

#[component]
pub fn InlineButton() -> impl IntoView {
    let style = "background-color: #3b82f6; color: white; padding: 0.5rem 1rem; border-radius: 0.25rem;";
    
    view! {
        <button style=style>
            "Click me"
        </button>
    }
}
```

## Key Benefits of Migration

| Feature | CSS-in-JS | tailwind-rs |
|---------|-----------|-------------|
| **Performance** | ❌ Runtime CSS generation | ✅ Build-time optimization |
| **Bundle Size** | ❌ Large runtime | ✅ Minimal runtime |
| **Type Safety** | ❌ Runtime errors | ✅ Compile-time validation |
| **Tree Shaking** | ❌ Limited | ✅ Full tree-shaking |
| **Caching** | ❌ Runtime only | ✅ Build + runtime caching |
| **SSR Support** | ❌ Complex setup | ✅ Native SSR support |

## Migration Steps

### 1. Add tailwind-rs Dependencies

**Cargo.toml**
```toml
[dependencies]
tailwind-rs = "0.1.0"
tailwind-rs-leptos = "0.1.0"  # Choose your framework

[build-dependencies]
tailwind-rs-cli = "0.1.0"
```

### 2. Create Build Configuration

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

### 3. Create Tailwind Configuration

**tailwind.toml**
```toml
[build]
input = ["src/**/*.rs"]
output = "dist/styles.css"
watch = true

[theme]
# Map your existing design tokens
[theme.colors]
primary = "#3b82f6"
primary-hover = "#2563eb"
secondary = "#64748b"
background = "#ffffff"
text = "#1f2937"

[theme.spacing]
xs = "0.25rem"
sm = "0.5rem"
md = "1rem"
lg = "1.5rem"
xl = "2rem"
```

### 4. Migrate Styled Components

**Before (Styled Components)**
```rust
use styled::*;

#[styled]
fn PrimaryButton() -> StyledComponent {
    styled! {
        button {
            background-color: #3b82f6;
            color: white;
            padding: 0.5rem 1rem;
            border-radius: 0.25rem;
            border: none;
            cursor: pointer;
            font-weight: 600;
            
            &:hover {
                background-color: #2563eb;
            }
            
            &:disabled {
                background-color: #9ca3af;
                cursor: not-allowed;
            }
        }
    }
}
```

**After (tailwind-rs)**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn PrimaryButton(disabled: bool) -> impl IntoView {
    let base_classes = classes!(
        "bg-blue-500",
        "text-white", 
        "px-4",
        "py-2",
        "rounded",
        "border-none",
        "cursor-pointer",
        "font-semibold"
    );
    
    let hover_classes = classes!("hover:bg-blue-600");
    let disabled_classes = if disabled {
        classes!("bg-gray-400", "cursor-not-allowed")
    } else {
        classes!()
    };
    
    view! {
        <button 
            class=classes!(base_classes, hover_classes, disabled_classes)
            disabled=disabled
        >
            "Click me"
        </button>
    }
}
```

### 5. Migrate Inline Styles

**Before (Inline Styles)**
```rust
use leptos::*;

#[component]
pub fn Card() -> impl IntoView {
    let card_style = "background: white; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1); border-radius: 0.5rem; padding: 1.5rem;";
    let title_style = "font-size: 1.25rem; font-weight: 700; color: #1f2937; margin-bottom: 1rem;";
    let content_style = "color: #6b7280; line-height: 1.5;";
    
    view! {
        <div style=card_style>
            <h2 style=title_style>
                "Card Title"
            </h2>
            <p style=content_style>
                "Card content goes here"
            </p>
        </div>
    }
}
```

**After (tailwind-rs)**
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
    
    let content_classes = classes!(
        "text-gray-600",
        "leading-relaxed"
    );
    
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

## Advanced Migration Features

### Dynamic Styling with Theme System
```rust
use tailwind_rs::{classes, theme};

#[component]
pub fn ThemedButton(variant: String) -> impl IntoView {
    let base_classes = classes!(
        "px-4",
        "py-2", 
        "rounded",
        "font-semibold",
        "cursor-pointer"
    );
    
    let variant_classes = match variant.as_str() {
        "primary" => classes!("bg-blue-500", "text-white", "hover:bg-blue-600"),
        "secondary" => classes!("bg-gray-500", "text-white", "hover:bg-gray-600"),
        "outline" => classes!("border-2", "border-blue-500", "text-blue-500", "hover:bg-blue-500", "hover:text-white"),
        _ => classes!("bg-gray-300", "text-gray-700"),
    };
    
    view! {
        <button class=classes!(base_classes, variant_classes)>
            "Themed Button"
        </button>
    }
}
```

### Responsive Design Migration
```rust
use tailwind_rs::{classes, responsive};

#[component]
pub fn ResponsiveGrid() -> impl IntoView {
    let grid_classes = classes!(
        "grid",
        "gap-4",
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

### Conditional Styling
```rust
use tailwind_rs::classes;

#[component]
pub fn ConditionalCard(is_highlighted: bool, size: String) -> impl IntoView {
    let base_classes = classes!(
        "bg-white",
        "rounded-lg",
        "shadow-md"
    );
    
    let highlight_classes = if is_highlighted {
        classes!("ring-2", "ring-blue-500", "shadow-lg")
    } else {
        classes!()
    };
    
    let size_classes = match size.as_str() {
        "sm" => classes!("p-4"),
        "md" => classes!("p-6"),
        "lg" => classes!("p-8"),
        _ => classes!("p-6"),
    };
    
    view! {
        <div class=classes!(base_classes, highlight_classes, size_classes)>
            "Conditional Card"
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
    fn test_primary_button_classes() {
        let button = PrimaryButton(false);
        let classes = extract_classes(button);
        
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("hover:bg-blue-600"));
        assert!(!classes.contains("bg-gray-400"));
    }
    
    #[test]
    fn test_disabled_button_classes() {
        let button = PrimaryButton(true);
        let classes = extract_classes(button);
        
        assert!(classes.contains("bg-gray-400"));
        assert!(classes.contains("cursor-not-allowed"));
        assert!(!classes.contains("bg-blue-500"));
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
        assert!(html.contains("shadow-lg"));
        assert!(html.contains("Card Title"));
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
      padding: computed.padding,
      borderRadius: computed.borderRadius
    };
  });
  
  expect(styles.backgroundColor).toBe('rgb(59, 130, 246)'); // blue-500
  expect(styles.color).toBe('rgb(255, 255, 255)'); // white
  expect(styles.padding).toBe('8px 16px'); // px-4 py-2
  expect(styles.borderRadius).toBe('4px'); // rounded
});

test('button hover state works', async ({ page }) => {
  await page.goto('/');
  
  const button = page.locator('button');
  await button.hover();
  
  const styles = await button.evaluate((el) => {
    const computed = getComputedStyle(el);
    return {
      backgroundColor: computed.backgroundColor
    };
  });
  
  expect(styles.backgroundColor).toBe('rgb(37, 99, 235)'); // blue-600
});
```

## Performance Comparison

### Bundle Size
```rust
// Before: CSS-in-JS
// Runtime CSS generation: ~50KB
// Style computation overhead: High

// After: tailwind-rs
// Build-time CSS generation: ~15KB
// Runtime optimization: Minimal
```

### Runtime Performance
```rust
// Before: CSS-in-JS
// Style computation: O(n) per render
// Memory usage: High (style objects)

// After: tailwind-rs
// Class concatenation: O(1) per render
// Memory usage: Low (string concatenation)
```

## Migration Checklist

- [ ] Remove CSS-in-JS dependencies
- [ ] Add tailwind-rs dependencies
- [ ] Create build configuration
- [ ] Map design tokens to Tailwind theme
- [ ] Migrate styled components to classes! macro
- [ ] Convert inline styles to Tailwind classes
- [ ] Add responsive design utilities
- [ ] Implement dynamic styling features
- [ ] Add comprehensive test suite
- [ ] Update CI/CD pipeline
- [ ] Deploy and monitor performance

## Troubleshooting

### Common Issues

**Issue**: Styles not applying after migration
```rust
// Solution: Ensure CSS is loaded
// Add to your HTML head:
<link rel="stylesheet" href="/dist/styles.css">
```

**Issue**: Dynamic styles not working
```rust
// Solution: Use classes! macro for dynamic styling
let dynamic_classes = classes!(
    "base-class",
    if condition { "conditional-class" } else { "other-class" }
);
```

**Issue**: Responsive classes not working
```rust
// Solution: Use responsive utilities
use tailwind_rs::responsive;

let responsive_classes = classes!(
    "base-class",
    responsive::md("md-class"),
    responsive::lg("lg-class")
);
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

