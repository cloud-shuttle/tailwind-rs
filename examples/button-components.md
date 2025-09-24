# Button Component Examples

This guide demonstrates various button components using `tailwind-rs`, showcasing different styles, states, and patterns.

## Basic Button Variants

### Primary Button
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn PrimaryButton(children: Children) -> impl IntoView {
    let button_classes = classes!(
        "bg-blue-500",
        "hover:bg-blue-700",
        "text-white",
        "font-semibold",
        "py-2",
        "px-4",
        "rounded",
        "transition-colors",
        "duration-200",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-500",
        "focus:ring-offset-2"
    );
    
    view! {
        <button class=button_classes>
            {children()}
        </button>
    }
}
```

### Secondary Button
```rust
#[component]
pub fn SecondaryButton(children: Children) -> impl IntoView {
    let button_classes = classes!(
        "bg-gray-500",
        "hover:bg-gray-700",
        "text-white",
        "font-semibold",
        "py-2",
        "px-4",
        "rounded",
        "transition-colors",
        "duration-200",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-gray-500",
        "focus:ring-offset-2"
    );
    
    view! {
        <button class=button_classes>
            {children()}
        </button>
    }
}
```

### Outline Button
```rust
#[component]
pub fn OutlineButton(children: Children) -> impl IntoView {
    let button_classes = classes!(
        "bg-transparent",
        "hover:bg-blue-500",
        "text-blue-500",
        "hover:text-white",
        "font-semibold",
        "py-2",
        "px-4",
        "border",
        "border-blue-500",
        "rounded",
        "transition-colors",
        "duration-200",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-500",
        "focus:ring-offset-2"
    );
    
    view! {
        <button class=button_classes>
            {children()}
        </button>
    }
}
```

## Button Sizes

### Small Button
```rust
#[component]
pub fn SmallButton(children: Children) -> impl IntoView {
    let button_classes = classes!(
        "bg-blue-500",
        "hover:bg-blue-700",
        "text-white",
        "font-semibold",
        "py-1",
        "px-2",
        "text-sm",
        "rounded",
        "transition-colors",
        "duration-200"
    );
    
    view! {
        <button class=button_classes>
            {children()}
        </button>
    }
}
```

### Large Button
```rust
#[component]
pub fn LargeButton(children: Children) -> impl IntoView {
    let button_classes = classes!(
        "bg-blue-500",
        "hover:bg-blue-700",
        "text-white",
        "font-semibold",
        "py-3",
        "px-6",
        "text-lg",
        "rounded",
        "transition-colors",
        "duration-200"
    );
    
    view! {
        <button class=button_classes>
            {children()}
        </button>
    }
}
```

## Button States

### Disabled Button
```rust
#[component]
pub fn DisabledButton(children: Children) -> impl IntoView {
    let button_classes = classes!(
        "bg-gray-400",
        "text-gray-700",
        "font-semibold",
        "py-2",
        "px-4",
        "rounded",
        "cursor-not-allowed",
        "opacity-60"
    );
    
    view! {
        <button class=button_classes disabled=true>
            {children()}
        </button>
    }
}
```

### Loading Button
```rust
#[component]
pub fn LoadingButton(children: Children, loading: bool) -> impl IntoView {
    let base_classes = classes!(
        "font-semibold",
        "py-2",
        "px-4",
        "rounded",
        "transition-colors",
        "duration-200",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-offset-2"
    );
    
    let state_classes = if loading {
        classes!(
            "bg-gray-400",
            "text-gray-700",
            "cursor-not-allowed",
            "opacity-60"
        )
    } else {
        classes!(
            "bg-blue-500",
            "hover:bg-blue-700",
            "text-white",
            "focus:ring-blue-500"
        )
    };
    
    view! {
        <button 
            class=classes!(base_classes, state_classes)
            disabled=loading
        >
            {if loading {
                view! {
                    <span class="flex items-center">
                        <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        "Loading..."
                    </span>
                }
            } else {
                children()
            }}
        </button>
    }
}
```

## Advanced Button Components

### Icon Button
```rust
#[component]
pub fn IconButton(icon: String, children: Children) -> impl IntoView {
    let button_classes = classes!(
        "bg-blue-500",
        "hover:bg-blue-700",
        "text-white",
        "font-semibold",
        "py-2",
        "px-4",
        "rounded",
        "transition-colors",
        "duration-200",
        "flex",
        "items-center",
        "space-x-2"
    );
    
    view! {
        <button class=button_classes>
            <span class="text-lg">{icon}</span>
            <span>{children()}</span>
        </button>
    }
}
```

### Button Group
```rust
#[component]
pub fn ButtonGroup(children: Children) -> impl IntoView {
    let group_classes = classes!(
        "inline-flex",
        "rounded-md",
        "shadow-sm"
    );
    
    view! {
        <div class=group_classes role="group">
            {children()}
        </div>
    }
}

#[component]
pub fn ButtonGroupItem(children: Children, #[prop(optional)] first: bool, #[prop(optional)] last: bool) -> impl IntoView {
    let base_classes = classes!(
        "px-4",
        "py-2",
        "text-sm",
        "font-medium",
        "text-gray-900",
        "bg-white",
        "border",
        "border-gray-200",
        "hover:bg-gray-50",
        "focus:z-10",
        "focus:ring-2",
        "focus:ring-blue-500",
        "focus:border-blue-500"
    );
    
    let position_classes = if first {
        classes!("rounded-l-md")
    } else if last {
        classes!("rounded-r-md")
    } else {
        classes!("border-t", "border-b")
    };
    
    view! {
        <button class=classes!(base_classes, position_classes)>
            {children()}
        </button>
    }
}
```

### Floating Action Button
```rust
#[component]
pub fn FloatingActionButton(icon: String) -> impl IntoView {
    let button_classes = classes!(
        "fixed",
        "bottom-6",
        "right-6",
        "bg-blue-500",
        "hover:bg-blue-700",
        "text-white",
        "w-14",
        "h-14",
        "rounded-full",
        "shadow-lg",
        "hover:shadow-xl",
        "transition-all",
        "duration-200",
        "flex",
        "items-center",
        "justify-center",
        "text-2xl",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-500",
        "focus:ring-offset-2"
    );
    
    view! {
        <button class=button_classes>
            {icon}
        </button>
    }
}
```

## Dynamic Button Component

### Configurable Button
```rust
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn ConfigurableButton(
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    loading: bool,
    children: Children
) -> impl IntoView {
    let base_classes = classes!(
        "font-semibold",
        "rounded",
        "transition-colors",
        "duration-200",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-offset-2"
    );
    
    let variant_classes = match variant {
        ButtonVariant::Primary => classes!(
            "bg-blue-500",
            "hover:bg-blue-700",
            "text-white",
            "focus:ring-blue-500"
        ),
        ButtonVariant::Secondary => classes!(
            "bg-gray-500",
            "hover:bg-gray-700",
            "text-white",
            "focus:ring-gray-500"
        ),
        ButtonVariant::Outline => classes!(
            "bg-transparent",
            "hover:bg-blue-500",
            "text-blue-500",
            "hover:text-white",
            "border",
            "border-blue-500",
            "focus:ring-blue-500"
        ),
        ButtonVariant::Ghost => classes!(
            "bg-transparent",
            "hover:bg-blue-50",
            "text-blue-500",
            "focus:ring-blue-500"
        ),
    };
    
    let size_classes = match size {
        ButtonSize::Small => classes!("py-1", "px-2", "text-sm"),
        ButtonSize::Medium => classes!("py-2", "px-4", "text-base"),
        ButtonSize::Large => classes!("py-3", "px-6", "text-lg"),
    };
    
    let state_classes = if disabled || loading {
        classes!(
            "bg-gray-400",
            "text-gray-700",
            "cursor-not-allowed",
            "opacity-60"
        )
    } else {
        classes!()
    };
    
    view! {
        <button 
            class=classes!(base_classes, variant_classes, size_classes, state_classes)
            disabled=disabled || loading
        >
            {if loading {
                view! {
                    <span class="flex items-center">
                        <svg class="animate-spin -ml-1 mr-3 h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        "Loading..."
                    </span>
                }
            } else {
                children()
            }}
        </button>
    }
}
```

## Testing Examples

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_primary_button_classes() {
        let button = PrimaryButton(|| view! { "Test" });
        let classes = extract_classes(button);
        
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("font-semibold"));
    }
    
    #[test]
    fn test_button_sizes() {
        let small_button = SmallButton(|| view! { "Small" });
        let small_classes = extract_classes(small_button);
        assert!(small_classes.contains("py-1"));
        assert!(small_classes.contains("px-2"));
        assert!(small_classes.contains("text-sm"));
        
        let large_button = LargeButton(|| view! { "Large" });
        let large_classes = extract_classes(large_button);
        assert!(large_classes.contains("py-3"));
        assert!(large_classes.contains("px-6"));
        assert!(large_classes.contains("text-lg"));
    }
    
    #[test]
    fn test_loading_button_state() {
        let loading_button = LoadingButton(|| view! { "Test" }, true);
        let classes = extract_classes(loading_button);
        
        assert!(classes.contains("bg-gray-400"));
        assert!(classes.contains("cursor-not-allowed"));
        assert!(classes.contains("opacity-60"));
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
    fn test_button_group_rendering() {
        let button_group = ButtonGroup(|| {
            view! {
                <ButtonGroupItem first=true>"First"</ButtonGroupItem>
                <ButtonGroupItem>"Middle"</ButtonGroupItem>
                <ButtonGroupItem last=true>"Last"</ButtonGroupItem>
            }
        });
        
        let html = render_to_string(button_group);
        assert!(html.contains("inline-flex"));
        assert!(html.contains("rounded-md"));
        assert!(html.contains("shadow-sm"));
    }
}
```

### Playwright E2E Tests
```typescript
// tests/button-components.spec.ts
import { test, expect } from '@playwright/test';

test('button variants render correctly', async ({ page }) => {
  await page.goto('/');
  
  // Test primary button
  const primaryButton = page.locator('.bg-blue-500.hover\\:bg-blue-700');
  await expect(primaryButton).toBeVisible();
  
  // Test secondary button
  const secondaryButton = page.locator('.bg-gray-500.hover\\:bg-gray-700');
  await expect(secondaryButton).toBeVisible();
  
  // Test outline button
  const outlineButton = page.locator('.bg-transparent.border.border-blue-500');
  await expect(outlineButton).toBeVisible();
});

test('button hover states work', async ({ page }) => {
  await page.goto('/');
  
  const primaryButton = page.locator('.bg-blue-500.hover\\:bg-blue-700');
  await primaryButton.hover();
  
  const styles = await primaryButton.evaluate((el) => {
    const computed = getComputedStyle(el);
    return {
      backgroundColor: computed.backgroundColor
    };
  });
  
  expect(styles.backgroundColor).toBe('rgb(29, 78, 216)'); // blue-700
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

test('floating action button is positioned correctly', async ({ page }) => {
  await page.goto('/');
  
  const fab = page.locator('.fixed.bottom-6.right-6');
  await expect(fab).toBeVisible();
  
  const styles = await fab.evaluate((el) => {
    const computed = getComputedStyle(el);
    return {
      position: computed.position,
      bottom: computed.bottom,
      right: computed.right,
      width: computed.width,
      height: computed.height,
      borderRadius: computed.borderRadius
    };
  });
  
  expect(styles.position).toBe('fixed');
  expect(styles.bottom).toBe('24px');
  expect(styles.right).toBe('24px');
  expect(styles.width).toBe('56px');
  expect(styles.height).toBe('56px');
  expect(styles.borderRadius).toBe('50%');
});
```

## Usage Examples

### Basic Usage
```rust
view! {
    <div class="space-y-4">
        <PrimaryButton>"Primary Action"</PrimaryButton>
        <SecondaryButton>"Secondary Action"</SecondaryButton>
        <OutlineButton>"Outline Action"</OutlineButton>
    </div>
}
```

### Button Group Usage
```rust
view! {
    <ButtonGroup>
        <ButtonGroupItem first=true>"Left"</ButtonGroupItem>
        <ButtonGroupItem>"Center"</ButtonGroupItem>
        <ButtonGroupItem last=true>"Right"</ButtonGroupItem>
    </ButtonGroup>
}
```

### Configurable Button Usage
```rust
view! {
    <div class="space-y-4">
        <ConfigurableButton 
            variant=ButtonVariant::Primary
            size=ButtonSize::Medium
            disabled=false
            loading=false
        >
            "Primary Medium"
        </ConfigurableButton>
        
        <ConfigurableButton 
            variant=ButtonVariant::Outline
            size=ButtonSize::Large
            disabled=false
            loading=true
        >
            "Outline Large Loading"
        </ConfigurableButton>
    </div>
}
```

## Next Steps

1. 🎨 Explore [Card Components](./card-components.md)
2. 🧪 Learn [Testing Patterns](./unit-testing.md)
3. 🎯 Try [Form Components](./form-components.md)
4. 🏗️ Build [Real-World Applications](./todo-app.md)

## Additional Resources

- [Getting Started Guide](../getting-started.md)
- [API Reference](../api/README.md)
- [Testing Guidelines](../testing.md)
- [Architecture Documentation](../architecture.md)

