# Unit Testing Examples

This guide demonstrates comprehensive unit testing patterns for `tailwind-rs` components, following our TDD-first approach and testing pyramid strategy.

## Testing Setup

### Dependencies

**Cargo.toml**
```toml
[dev-dependencies]
tailwind-rs-testing = "0.1.0"
leptos-testing = "0.6"
wasm-bindgen-test = "0.3"
```

### Test Configuration

**tests/test_config.rs**
```rust
use tailwind_rs::testing::*;

pub fn setup_test_environment() {
    // Initialize test environment
    init_test_env();
}

pub fn create_test_app<F>(component: F) -> TestApp
where
    F: Fn() -> impl IntoView + 'static,
{
    create_test_app_with_config(component, TestConfig::default())
}
```

## Basic Component Testing

### Button Component Tests

**src/components/button.rs**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    let button_classes = classes!(
        "bg-blue-500",
        "hover:bg-blue-700",
        "text-white",
        "font-semibold",
        "py-2",
        "px-4",
        "rounded"
    );
    
    view! {
        <button class=button_classes>
            {children()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_button_renders_with_correct_classes() {
        let button = Button(|| view! { "Test Button" });
        let classes = extract_classes(button);
        
        // Test base classes
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("font-semibold"));
        assert!(classes.contains("py-2"));
        assert!(classes.contains("px-4"));
        assert!(classes.contains("rounded"));
        
        // Test hover classes
        assert!(classes.contains("hover:bg-blue-700"));
    }
    
    #[test]
    fn test_button_renders_children() {
        let button = Button(|| view! { "Click me!" });
        let html = render_to_string(button);
        
        assert!(html.contains("Click me!"));
        assert!(html.contains("<button"));
    }
    
    #[test]
    fn test_button_has_correct_html_structure() {
        let button = Button(|| view! { "Test" });
        let html = render_to_string(button);
        
        // Check HTML structure
        assert!(html.starts_with("<button"));
        assert!(html.ends_with("</button>"));
        assert!(html.contains("class="));
    }
}
```

### Card Component Tests

**src/components/card.rs**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    let card_classes = classes!(
        "bg-white",
        "shadow-lg",
        "rounded-lg",
        "p-6"
    );
    
    view! {
        <div class=card_classes>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(children: Children) -> impl IntoView {
    let title_classes = classes!(
        "text-xl",
        "font-bold",
        "text-gray-800",
        "mb-4"
    );
    
    view! {
        <h2 class=title_classes>
            {children()}
        </h2>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_card_renders_with_correct_classes() {
        let card = Card(|| view! { "Card content" });
        let classes = extract_classes(card);
        
        assert!(classes.contains("bg-white"));
        assert!(classes.contains("shadow-lg"));
        assert!(classes.contains("rounded-lg"));
        assert!(classes.contains("p-6"));
    }
    
    #[test]
    fn test_card_title_renders_with_correct_classes() {
        let title = CardTitle(|| view! { "Card Title" });
        let classes = extract_classes(title);
        
        assert!(classes.contains("text-xl"));
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("text-gray-800"));
        assert!(classes.contains("mb-4"));
    }
    
    #[test]
    fn test_card_composition() {
        let card = Card(|| {
            view! {
                <CardTitle>"Test Title"</CardTitle>
                <p>"Test content"</p>
            }
        });
        
        let html = render_to_string(card);
        assert!(html.contains("Test Title"));
        assert!(html.contains("Test content"));
        assert!(html.contains("<h2"));
        assert!(html.contains("<p"));
    }
}
```

## Dynamic Component Testing

### Conditional Styling Tests

**src/components/conditional_button.rs**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn ConditionalButton(disabled: bool, loading: bool) -> impl IntoView {
    let base_classes = classes!(
        "font-semibold",
        "py-2",
        "px-4",
        "rounded",
        "transition-colors",
        "duration-200"
    );
    
    let state_classes = if disabled {
        classes!(
            "bg-gray-400",
            "text-gray-700",
            "cursor-not-allowed",
            "opacity-60"
        )
    } else if loading {
        classes!(
            "bg-blue-400",
            "text-white",
            "cursor-wait"
        )
    } else {
        classes!(
            "bg-blue-500",
            "hover:bg-blue-700",
            "text-white"
        )
    };
    
    view! {
        <button 
            class=classes!(base_classes, state_classes)
            disabled=disabled || loading
        >
            {if loading {
                "Loading..."
            } else {
                "Click me"
            }}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_button_normal_state() {
        let button = ConditionalButton(false, false);
        let classes = extract_classes(button);
        
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("text-white"));
        assert!(!classes.contains("bg-gray-400"));
        assert!(!classes.contains("cursor-not-allowed"));
    }
    
    #[test]
    fn test_button_disabled_state() {
        let button = ConditionalButton(true, false);
        let classes = extract_classes(button);
        
        assert!(classes.contains("bg-gray-400"));
        assert!(classes.contains("text-gray-700"));
        assert!(classes.contains("cursor-not-allowed"));
        assert!(classes.contains("opacity-60"));
        assert!(!classes.contains("bg-blue-500"));
    }
    
    #[test]
    fn test_button_loading_state() {
        let button = ConditionalButton(false, true);
        let classes = extract_classes(button);
        
        assert!(classes.contains("bg-blue-400"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("cursor-wait"));
        assert!(!classes.contains("bg-gray-400"));
    }
    
    #[test]
    fn test_button_text_changes_with_state() {
        let normal_button = ConditionalButton(false, false);
        let normal_html = render_to_string(normal_button);
        assert!(normal_html.contains("Click me"));
        
        let loading_button = ConditionalButton(false, true);
        let loading_html = render_to_string(loading_button);
        assert!(loading_html.contains("Loading..."));
    }
}
```

### Variant Component Tests

**src/components/variant_button.rs**
```rust
use leptos::*;
use tailwind_rs::classes;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
}

#[component]
pub fn VariantButton(variant: ButtonVariant, children: Children) -> impl IntoView {
    let base_classes = classes!(
        "font-semibold",
        "py-2",
        "px-4",
        "rounded",
        "transition-colors",
        "duration-200"
    );
    
    let variant_classes = match variant {
        ButtonVariant::Primary => classes!(
            "bg-blue-500",
            "hover:bg-blue-700",
            "text-white"
        ),
        ButtonVariant::Secondary => classes!(
            "bg-gray-500",
            "hover:bg-gray-700",
            "text-white"
        ),
        ButtonVariant::Outline => classes!(
            "bg-transparent",
            "hover:bg-blue-500",
            "text-blue-500",
            "hover:text-white",
            "border",
            "border-blue-500"
        ),
    };
    
    view! {
        <button class=classes!(base_classes, variant_classes)>
            {children()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_primary_variant() {
        let button = VariantButton(ButtonVariant::Primary, || view! { "Primary" });
        let classes = extract_classes(button);
        
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("text-white"));
        assert!(!classes.contains("bg-transparent"));
        assert!(!classes.contains("border"));
    }
    
    #[test]
    fn test_secondary_variant() {
        let button = VariantButton(ButtonVariant::Secondary, || view! { "Secondary" });
        let classes = extract_classes(button);
        
        assert!(classes.contains("bg-gray-500"));
        assert!(classes.contains("hover:bg-gray-700"));
        assert!(classes.contains("text-white"));
        assert!(!classes.contains("bg-blue-500"));
    }
    
    #[test]
    fn test_outline_variant() {
        let button = VariantButton(ButtonVariant::Outline, || view! { "Outline" });
        let classes = extract_classes(button);
        
        assert!(classes.contains("bg-transparent"));
        assert!(classes.contains("hover:bg-blue-500"));
        assert!(classes.contains("text-blue-500"));
        assert!(classes.contains("hover:text-white"));
        assert!(classes.contains("border"));
        assert!(classes.contains("border-blue-500"));
    }
}
```

## Form Component Testing

### Input Component Tests

**src/components/input.rs**
```rust
use leptos::*;
use tailwind_rs::classes;

#[component]
pub fn Input(
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] input_type: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
) -> impl IntoView {
    let base_classes = classes!(
        "w-full",
        "px-3",
        "py-2",
        "border",
        "border-gray-300",
        "rounded-md",
        "focus:outline-none",
        "focus:ring-2",
        "focus:ring-blue-500",
        "focus:border-transparent"
    );
    
    let state_classes = if disabled {
        classes!(
            "bg-gray-100",
            "cursor-not-allowed",
            "opacity-60"
        )
    } else {
        classes!()
    };
    
    view! {
        <input
            class=classes!(base_classes, state_classes)
            type=input_type.unwrap_or_else(|| "text".to_string())
            placeholder=placeholder.unwrap_or_else(|| "".to_string())
            required=required
            disabled=disabled
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_input_default_classes() {
        let input = Input(None, None, false, false);
        let classes = extract_classes(input);
        
        assert!(classes.contains("w-full"));
        assert!(classes.contains("px-3"));
        assert!(classes.contains("py-2"));
        assert!(classes.contains("border"));
        assert!(classes.contains("border-gray-300"));
        assert!(classes.contains("rounded-md"));
        assert!(classes.contains("focus:outline-none"));
        assert!(classes.contains("focus:ring-2"));
        assert!(classes.contains("focus:ring-blue-500"));
    }
    
    #[test]
    fn test_input_disabled_state() {
        let input = Input(None, None, false, true);
        let classes = extract_classes(input);
        
        assert!(classes.contains("bg-gray-100"));
        assert!(classes.contains("cursor-not-allowed"));
        assert!(classes.contains("opacity-60"));
    }
    
    #[test]
    fn test_input_attributes() {
        let input = Input(
            Some("Enter text".to_string()),
            Some("email".to_string()),
            true,
            false
        );
        
        let html = render_to_string(input);
        assert!(html.contains("placeholder=\"Enter text\""));
        assert!(html.contains("type=\"email\""));
        assert!(html.contains("required"));
    }
}
```

## Responsive Component Testing

### Responsive Grid Tests

**src/components/responsive_grid.rs**
```rust
use leptos::*;
use tailwind_rs::{classes, responsive};

#[component]
pub fn ResponsiveGrid(children: Children) -> impl IntoView {
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
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_responsive_grid_classes() {
        let grid = ResponsiveGrid(|| view! { "Grid content" });
        let classes = extract_classes(grid);
        
        assert!(classes.contains("grid"));
        assert!(classes.contains("gap-4"));
        assert!(classes.contains("sm:grid-cols-1"));
        assert!(classes.contains("md:grid-cols-2"));
        assert!(classes.contains("lg:grid-cols-3"));
        assert!(classes.contains("xl:grid-cols-4"));
    }
    
    #[test]
    fn test_responsive_grid_renders_children() {
        let grid = ResponsiveGrid(|| {
            view! {
                <div>"Item 1"</div>
                <div>"Item 2"</div>
                <div>"Item 3"</div>
            }
        });
        
        let html = render_to_string(grid);
        assert!(html.contains("Item 1"));
        assert!(html.contains("Item 2"));
        assert!(html.contains("Item 3"));
    }
}
```

## Theme System Testing

### Themed Component Tests

**src/components/themed_component.rs**
```rust
use leptos::*;
use tailwind_rs::{classes, theme};

#[component]
pub fn ThemedComponent() -> impl IntoView {
    let component_classes = classes!(
        "p-4",
        "rounded-lg",
        "border",
        "transition-colors",
        "duration-200"
    );
    
    // Use theme system for dynamic colors
    let primary_color = theme::color("primary");
    let spacing = theme::spacing("md");
    
    view! {
        <div 
            class=component_classes
            style=format!("border-color: {}; margin: {}", primary_color, spacing)
        >
            "Themed Component"
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailwind_rs::testing::*;

    #[test]
    fn test_themed_component_classes() {
        let component = ThemedComponent();
        let classes = extract_classes(component);
        
        assert!(classes.contains("p-4"));
        assert!(classes.contains("rounded-lg"));
        assert!(classes.contains("border"));
        assert!(classes.contains("transition-colors"));
        assert!(classes.contains("duration-200"));
    }
    
    #[test]
    fn test_themed_component_renders() {
        let component = ThemedComponent();
        let html = render_to_string(component);
        
        assert!(html.contains("Themed Component"));
        assert!(html.contains("style="));
    }
}
```

## Test Utilities

### Custom Test Helpers

**tests/test_helpers.rs**
```rust
use tailwind_rs::testing::*;

pub fn assert_classes_contain(classes: &str, expected_classes: &[&str]) {
    for expected_class in expected_classes {
        assert!(
            classes.contains(expected_class),
            "Expected class '{}' not found in classes: {}",
            expected_class,
            classes
        );
    }
}

pub fn assert_classes_not_contain(classes: &str, unexpected_classes: &[&str]) {
    for unexpected_class in unexpected_classes {
        assert!(
            !classes.contains(unexpected_class),
            "Unexpected class '{}' found in classes: {}",
            unexpected_class,
            classes
        );
    }
}

pub fn assert_html_contains(html: &str, expected_content: &[&str]) {
    for expected in expected_content {
        assert!(
            html.contains(expected),
            "Expected content '{}' not found in HTML: {}",
            expected,
            html
        );
    }
}

pub fn create_test_component_with_props<F, P>(
    component_fn: F,
    props: P
) -> impl IntoView
where
    F: Fn(P) -> impl IntoView,
    P: Clone,
{
    component_fn(props)
}
```

### Test Configuration

**tests/test_config.rs**
```rust
use tailwind_rs::testing::*;

pub struct TestConfig {
    pub theme: Option<String>,
    pub breakpoint: Option<String>,
    pub dark_mode: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            theme: None,
            breakpoint: None,
            dark_mode: false,
        }
    }
}

pub fn create_test_app_with_config<F>(
    component: F,
    config: TestConfig
) -> TestApp
where
    F: Fn() -> impl IntoView + 'static,
{
    let mut test_app = create_test_app(component);
    
    if let Some(theme) = config.theme {
        test_app.set_theme(theme);
    }
    
    if let Some(breakpoint) = config.breakpoint {
        test_app.set_breakpoint(breakpoint);
    }
    
    if config.dark_mode {
        test_app.enable_dark_mode();
    }
    
    test_app
}
```

## Running Tests

### Test Commands
```bash
# Run all tests
cargo test

# Run specific test module
cargo test button

# Run tests with output
cargo test -- --nocapture

# Run tests in watch mode
cargo watch -x test

# Run tests with coverage
cargo tarpaulin --out Html
```

### Test Output Example
```bash
running 15 tests
test components::button::tests::test_button_renders_with_correct_classes ... ok
test components::button::tests::test_button_renders_children ... ok
test components::button::tests::test_button_has_correct_html_structure ... ok
test components::card::tests::test_card_renders_with_correct_classes ... ok
test components::card::tests::test_card_title_renders_with_correct_classes ... ok
test components::card::tests::test_card_composition ... ok
test components::conditional_button::tests::test_button_normal_state ... ok
test components::conditional_button::tests::test_button_disabled_state ... ok
test components::conditional_button::tests::test_button_loading_state ... ok
test components::conditional_button::tests::test_button_text_changes_with_state ... ok
test components::variant_button::tests::test_primary_variant ... ok
test components::variant_button::tests::test_secondary_variant ... ok
test components::variant_button::tests::test_outline_variant ... ok
test components::input::tests::test_input_default_classes ... ok
test components::input::tests::test_input_disabled_state ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Best Practices

### 1. Test Structure
- **Arrange**: Set up test data and components
- **Act**: Render components and extract classes
- **Assert**: Verify expected behavior

### 2. Test Naming
- Use descriptive test names
- Include the component and behavior being tested
- Use `test_` prefix for all test functions

### 3. Test Coverage
- Test all component variants
- Test all conditional logic paths
- Test all prop combinations
- Test error states and edge cases

### 4. Test Isolation
- Each test should be independent
- Use fresh component instances for each test
- Avoid shared state between tests

### 5. Test Maintainability
- Use helper functions for common assertions
- Extract test data into constants
- Keep tests simple and focused

## Next Steps

1. 🧪 Explore [Integration Testing](./integration-testing.md)
2. 🎭 Learn [Playwright E2E Testing](./playwright-testing.md)
3. 🎨 Try [Dynamic Styling Examples](./dynamic-styling.md)
4. 🏗️ Build [Real-World Applications](./todo-app.md)

## Additional Resources

- [Testing Guidelines](../testing.md)
- [API Reference](../api/README.md)
- [Architecture Documentation](../architecture.md)
- [Contributing Guidelines](../contributing.md)

