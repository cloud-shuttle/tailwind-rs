# Migrating from Styled Components

This guide helps you migrate from styled component libraries to `tailwind-rs` for better performance and type safety.

## Current Setup Analysis

### Common Styled Components Patterns
```rust
// Styled components with CSS-in-Rust
use styled::*;

#[styled]
fn StyledButton() -> StyledComponent {
    styled! {
        button {
            background-color: #3b82f6;
            color: white;
            padding: 0.5rem 1rem;
            border-radius: 0.25rem;
            border: none;
            cursor: pointer;
            font-weight: 600;
            transition: all 0.2s ease-in-out;
            
            &:hover {
                background-color: #2563eb;
                transform: translateY(-1px);
            }
            
            &:active {
                transform: translateY(0);
            }
            
            &:disabled {
                background-color: #9ca3af;
                cursor: not-allowed;
                opacity: 0.6;
            }
        }
    }
}

// Themed styled components
#[styled]
fn ThemedCard(theme: Theme) -> StyledComponent {
    styled! {
        div {
            background-color: ${theme.colors.background};
            color: ${theme.colors.text};
            padding: ${theme.spacing.md};
            border-radius: ${theme.borderRadius.md};
            box-shadow: ${theme.shadows.md};
            
            &:hover {
                box-shadow: ${theme.shadows.lg};
            }
        }
    }
}
```

## Key Benefits of Migration

| Feature | Styled Components | tailwind-rs |
|---------|------------------|-------------|
| **Performance** | ❌ Runtime CSS generation | ✅ Build-time optimization |
| **Bundle Size** | ❌ Large runtime + CSS | ✅ Minimal runtime |
| **Type Safety** | ❌ Runtime theme errors | ✅ Compile-time validation |
| **Tree Shaking** | ❌ Limited | ✅ Full tree-shaking |
| **SSR Support** | ❌ Complex hydration | ✅ Native SSR support |
| **Developer Experience** | ❌ Complex setup | ✅ Simple configuration |

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
# Map your existing theme tokens
[theme.colors]
primary = "#3b82f6"
primary-hover = "#2563eb"
secondary = "#64748b"
background = "#ffffff"
text = "#1f2937"
disabled = "#9ca3af"

[theme.spacing]
xs = "0.25rem"
sm = "0.5rem"
md = "1rem"
lg = "1.5rem"
xl = "2rem"

[theme.borderRadius]
sm = "0.125rem"
md = "0.375rem"
lg = "0.5rem"

[theme.boxShadow]
sm = "0 1px 2px 0 rgba(0, 0, 0, 0.05)"
md = "0 4px 6px -1px rgba(0, 0, 0, 0.1)"
lg = "0 10px 15px -3px rgba(0, 0, 0, 0.1)"
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
            transition: all 0.2s ease-in-out;
            
            &:hover {
                background-color: #2563eb;
                transform: translateY(-1px);
            }
            
            &:active {
                transform: translateY(0);
            }
            
            &:disabled {
                background-color: #9ca3af;
                cursor: not-allowed;
                opacity: 0.6;
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
        "font-semibold",
        "transition-all",
        "duration-200",
        "ease-in-out"
    );
    
    let hover_classes = classes!(
        "hover:bg-blue-600",
        "hover:-translate-y-0.5"
    );
    
    let active_classes = classes!("active:translate-y-0");
    
    let disabled_classes = if disabled {
        classes!(
            "bg-gray-400",
            "cursor-not-allowed",
            "opacity-60"
        )
    } else {
        classes!()
    };
    
    view! {
        <button 
            class=classes!(base_classes, hover_classes, active_classes, disabled_classes)
            disabled=disabled
        >
            "Click me"
        </button>
    }
}
```

### 5. Migrate Themed Components

**Before (Themed Styled Components)**
```rust
use styled::*;

#[derive(Clone)]
struct Theme {
    colors: Colors,
    spacing: Spacing,
    borderRadius: BorderRadius,
    shadows: Shadows,
}

#[styled]
fn ThemedCard(theme: Theme) -> StyledComponent {
    styled! {
        div {
            background-color: ${theme.colors.background};
            color: ${theme.colors.text};
            padding: ${theme.spacing.md};
            border-radius: ${theme.borderRadius.md};
            box-shadow: ${theme.shadows.md};
            
            &:hover {
                box-shadow: ${theme.shadows.lg};
            }
        }
    }
}
```

**After (tailwind-rs with Theme System)**
```rust
use leptos::*;
use tailwind_rs::{classes, theme};

#[component]
pub fn ThemedCard() -> impl IntoView {
    let card_classes = classes!(
        "bg-white",
        "text-gray-800",
        "p-6",
        "rounded-lg",
        "shadow-md",
        "hover:shadow-lg",
        "transition-shadow",
        "duration-200"
    );
    
    view! {
        <div class=card_classes>
            "Themed Card Content"
        </div>
    }
}

// For dynamic theming
#[component]
pub fn DynamicThemedCard(theme_name: String) -> impl IntoView {
    let base_classes = classes!(
        "p-6",
        "rounded-lg",
        "shadow-md",
        "hover:shadow-lg",
        "transition-shadow",
        "duration-200"
    );
    
    let theme_classes = match theme_name.as_str() {
        "light" => classes!("bg-white", "text-gray-800"),
        "dark" => classes!("bg-gray-800", "text-white"),
        "primary" => classes!("bg-blue-500", "text-white"),
        _ => classes!("bg-gray-100", "text-gray-900"),
    };
    
    view! {
        <div class=classes!(base_classes, theme_classes)>
            "Dynamic Themed Card"
        </div>
    }
}
```

## Advanced Migration Features

### Complex Styled Components
```rust
// Before: Complex styled component with multiple states
#[styled]
fn ComplexButton(variant: String, size: String, disabled: bool) -> StyledComponent {
    styled! {
        button {
            // Base styles
            border: none;
            cursor: pointer;
            font-weight: 600;
            transition: all 0.2s ease-in-out;
            
            // Variant styles
            ${match variant.as_str() {
                "primary" => css! {
                    background-color: #3b82f6;
                    color: white;
                    
                    &:hover {
                        background-color: #2563eb;
                    }
                },
                "secondary" => css! {
                    background-color: #6b7280;
                    color: white;
                    
                    &:hover {
                        background-color: #4b5563;
                    }
                },
                "outline" => css! {
                    background-color: transparent;
                    color: #3b82f6;
                    border: 2px solid #3b82f6;
                    
                    &:hover {
                        background-color: #3b82f6;
                        color: white;
                    }
                },
                _ => css! {
                    background-color: #e5e7eb;
                    color: #374151;
                }
            }}
            
            // Size styles
            ${match size.as_str() {
                "sm" => css! { padding: 0.25rem 0.5rem; font-size: 0.875rem; },
                "md" => css! { padding: 0.5rem 1rem; font-size: 1rem; },
                "lg" => css! { padding: 0.75rem 1.5rem; font-size: 1.125rem; },
                _ => css! { padding: 0.5rem 1rem; font-size: 1rem; }
            }}
            
            // Disabled styles
            ${if disabled {
                css! {
                    background-color: #9ca3af !important;
                    color: #6b7280 !important;
                    cursor: not-allowed !important;
                    opacity: 0.6;
                }
            } else {
                css! {}
            }}
        }
    }
}
```

**After (tailwind-rs)**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn ComplexButton(
    variant: String,
    size: String, 
    disabled: bool
) -> impl IntoView {
    let base_classes = classes!(
        "border-none",
        "cursor-pointer",
        "font-semibold",
        "transition-all",
        "duration-200",
        "ease-in-out"
    );
    
    let variant_classes = match variant.as_str() {
        "primary" => classes!(
            "bg-blue-500",
            "text-white",
            "hover:bg-blue-600"
        ),
        "secondary" => classes!(
            "bg-gray-500",
            "text-white",
            "hover:bg-gray-600"
        ),
        "outline" => classes!(
            "bg-transparent",
            "text-blue-500",
            "border-2",
            "border-blue-500",
            "hover:bg-blue-500",
            "hover:text-white"
        ),
        _ => classes!(
            "bg-gray-200",
            "text-gray-700"
        ),
    };
    
    let size_classes = match size.as_str() {
        "sm" => classes!("px-2", "py-1", "text-sm"),
        "md" => classes!("px-4", "py-2", "text-base"),
        "lg" => classes!("px-6", "py-3", "text-lg"),
        _ => classes!("px-4", "py-2", "text-base"),
    };
    
    let disabled_classes = if disabled {
        classes!(
            "bg-gray-400",
            "text-gray-500",
            "cursor-not-allowed",
            "opacity-60"
        )
    } else {
        classes!()
    };
    
    view! {
        <button 
            class=classes!(base_classes, variant_classes, size_classes, disabled_classes)
            disabled=disabled
        >
            "Complex Button"
        </button>
    }
}
```

### Responsive Styled Components
```rust
// Before: Responsive styled components
#[styled]
fn ResponsiveGrid() -> StyledComponent {
    styled! {
        div {
            display: grid;
            gap: 1rem;
            
            @media (min-width: 640px) {
                grid-template-columns: repeat(1, minmax(0, 1fr));
            }
            
            @media (min-width: 768px) {
                grid-template-columns: repeat(2, minmax(0, 1fr));
            }
            
            @media (min-width: 1024px) {
                grid-template-columns: repeat(3, minmax(0, 1fr));
            }
            
            @media (min-width: 1280px) {
                grid-template-columns: repeat(4, minmax(0, 1fr));
            }
        }
    }
}
```

**After (tailwind-rs)**
```rust
use leptos::*;
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
        assert!(classes.contains("transition-all"));
    }
    
    #[test]
    fn test_complex_button_variants() {
        let primary_button = ComplexButton("primary".to_string(), "md".to_string(), false);
        let classes = extract_classes(primary_button);
        
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("hover:bg-blue-600"));
    }
    
    #[test]
    fn test_complex_button_sizes() {
        let small_button = ComplexButton("primary".to_string(), "sm".to_string(), false);
        let classes = extract_classes(small_button);
        
        assert!(classes.contains("px-2"));
        assert!(classes.contains("py-1"));
        assert!(classes.contains("text-sm"));
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
    fn test_button_rendering() {
        let app = create_test_app(|| view! { <PrimaryButton disabled=false /> });
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

test('primary button renders with correct styles', async ({ page }) => {
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
      borderRadius: computed.borderRadius,
      transition: computed.transition
    };
  });
  
  expect(styles.backgroundColor).toBe('rgb(59, 130, 246)'); // blue-500
  expect(styles.color).toBe('rgb(255, 255, 255)'); // white
  expect(styles.padding).toBe('8px 16px'); // px-4 py-2
  expect(styles.borderRadius).toBe('4px'); // rounded
  expect(styles.transition).toContain('all');
});

test('button hover state works', async ({ page }) => {
  await page.goto('/');
  
  const button = page.locator('button');
  await button.hover();
  
  const styles = await button.evaluate((el) => {
    const computed = getComputedStyle(el);
    return {
      backgroundColor: computed.backgroundColor,
      transform: computed.transform
    };
  });
  
  expect(styles.backgroundColor).toBe('rgb(37, 99, 235)'); // blue-600
  expect(styles.transform).toContain('translateY'); // hover:-translate-y-0.5
});

test('disabled button state works', async ({ page }) => {
  await page.goto('/');
  
  const disabledButton = page.locator('button[disabled]');
  await expect(disabledButton).toBeVisible();
  
  const styles = await disabledButton.evaluate((el) => {
    const computed = getComputedStyle(el);
    return {
      backgroundColor: computed.backgroundColor,
      cursor: computed.cursor,
      opacity: computed.opacity
    };
  });
  
  expect(styles.backgroundColor).toBe('rgb(156, 163, 175)'); // gray-400
  expect(styles.cursor).toBe('not-allowed');
  expect(styles.opacity).toBe('0.6');
});
```

## Performance Comparison

### Bundle Size
```rust
// Before: Styled Components
// Runtime CSS generation: ~100KB
// Style computation overhead: High
// Theme system overhead: High

// After: tailwind-rs
// Build-time CSS generation: ~15KB
// Runtime optimization: Minimal
// Theme system: Build-time only
```

### Runtime Performance
```rust
// Before: Styled Components
// Style computation: O(n) per render
// Theme resolution: O(n) per render
// Memory usage: High (style objects + theme)

// After: tailwind-rs
// Class concatenation: O(1) per render
// Theme resolution: Build-time only
// Memory usage: Low (string concatenation)
```

## Migration Checklist

- [ ] Remove styled component dependencies
- [ ] Add tailwind-rs dependencies
- [ ] Create build configuration
- [ ] Map theme tokens to Tailwind configuration
- [ ] Migrate styled components to classes! macro
- [ ] Convert complex styled components
- [ ] Add responsive design utilities
- [ ] Implement dynamic styling features
- [ ] Add comprehensive test suite
- [ ] Update CI/CD pipeline
- [ ] Deploy and monitor performance

## Troubleshooting

### Common Issues

**Issue**: Complex styled components not migrating properly
```rust
// Solution: Break down complex components into smaller parts
let base_classes = classes!("base-styles");
let variant_classes = match variant { /* ... */ };
let size_classes = match size { /* ... */ };
let state_classes = if disabled { /* ... */ } else { /* ... */ };

view! {
    <button class=classes!(base_classes, variant_classes, size_classes, state_classes)>
        "Button"
    </button>
}
```

**Issue**: Theme system not working
```rust
// Solution: Use Tailwind's built-in theme system
// Map your theme tokens to tailwind.toml
[theme.colors]
primary = "#3b82f6"
primary-hover = "#2563eb"
```

**Issue**: Responsive styles not working
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

