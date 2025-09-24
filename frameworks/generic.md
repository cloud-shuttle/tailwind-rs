# Generic Web Framework Integration Guide

This guide covers the integration of `tailwind-rs` with generic web frameworks and custom implementations, following our Test-Driven Development approach and comprehensive testing strategy.

## 🎯 Overview

`tailwind-rs-generic` provides flexible integration for any web framework or custom implementation, offering:

- **Type-safe class generation** with compile-time validation
- **Framework-agnostic styling** with consistent design systems
- **Performance optimization** with intelligent caching
- **Comprehensive testing** with unit, integration, and E2E tests

## 🚀 Quick Start

### Installation
```toml
# Cargo.toml
[dependencies]
tailwind-rs = "0.1.0"
tailwind-rs-generic = "0.1.0"
wasm-bindgen = "0.2.101"
web-sys = "0.3.77"
```

### Basic Usage
```rust
use tailwind_rs::*;
use tailwind_rs_generic::*;

// Generic class generation
let classes = classes! {
    base: "min-h-screen bg-gray-100",
    typography: "font-sans",
};

// Apply to any HTML element
let html = format!(r#"
    <div class="{}">
        <header class="bg-white shadow-sm border-b px-6 py-4">
            <h1 class="text-2xl font-bold text-gray-900">
                My Generic App
            </h1>
        </header>
        <main class="container mx-auto px-6 py-8">
            <div class="bg-white rounded-lg shadow-md p-6 border border-gray-200">
                <h2 class="text-xl font-semibold text-gray-800 mb-4">
                    Welcome to tailwind-rs!
                </h2>
                <p class="text-gray-600 mb-6">
                    This demonstrates generic web framework integration.
                </p>
                <button class="{}">
                    Get Started
                </button>
            </div>
        </main>
    </div>
"#, classes, button_classes);
```

## 🎨 Component System

### Generic Component Trait
```rust
use tailwind_rs::*;
use tailwind_rs_generic::*;

pub trait GenericComponent {
    fn generate_classes(&self) -> String;
    fn render(&self) -> String;
    fn validate(&self) -> Result<(), ValidationError>;
}

pub struct GenericButton {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub children: String,
}

impl GenericButton {
    pub fn new() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            disabled: false,
            children: String::new(),
        }
    }
    
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }
}

impl GenericComponent for GenericButton {
    fn generate_classes(&self) -> String {
        classes! {
            base: "px-4 py-2 rounded-md font-medium transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500",
            variant: match self.variant {
                ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
                ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
                ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
            },
            size: match self.size {
                ButtonSize::Small => "px-2 py-1 text-sm",
                ButtonSize::Medium => "px-4 py-2 text-base",
                ButtonSize::Large => "px-6 py-3 text-lg",
            },
            state: if self.disabled {
                "opacity-50 cursor-not-allowed"
            } else {
                "cursor-pointer"
            },
        }.build()
    }
    
    fn render(&self) -> String {
        let classes = self.generate_classes();
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        
        format!(
            r#"<button class="{}"{}>{}</button>"#,
            classes, disabled_attr, self.children
        )
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate button configuration
        if self.children.is_empty() {
            return Err(ValidationError::EmptyContent);
        }
        
        Ok(())
    }
}
```

### Generic Input Component
```rust
use tailwind_rs::*;
use tailwind_rs_generic::*;

pub struct GenericInput {
    pub variant: InputVariant,
    pub placeholder: String,
    pub value: String,
    pub disabled: bool,
    pub required: bool,
}

impl GenericInput {
    pub fn new() -> Self {
        Self {
            variant: InputVariant::Default,
            placeholder: String::new(),
            value: String::new(),
            disabled: false,
            required: false,
        }
    }
    
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }
    
    pub fn value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

impl GenericComponent for GenericInput {
    fn generate_classes(&self) -> String {
        classes! {
            base: "block w-full rounded-md border shadow-sm focus:outline-none focus:ring-1",
            variant: match self.variant {
                InputVariant::Default => "border-gray-300 focus:border-blue-500 focus:ring-blue-500",
                InputVariant::Error => "border-red-500 focus:border-red-500 focus:ring-red-500",
                InputVariant::Success => "border-green-500 focus:border-green-500 focus:ring-green-500",
            },
            state: if self.disabled {
                "bg-gray-50 text-gray-500 cursor-not-allowed"
            } else {
                "bg-white text-gray-900"
            },
        }.build()
    }
    
    fn render(&self) -> String {
        let classes = self.generate_classes();
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        let required_attr = if self.required { " required" } else { "" };
        
        format!(
            r#"<input class="{}" placeholder="{}" value="{}"{}{} />"#,
            classes, self.placeholder, self.value, disabled_attr, required_attr
        )
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate input configuration
        if self.required && self.value.is_empty() {
            return Err(ValidationError::RequiredField);
        }
        
        Ok(())
    }
}
```

### Generic Card Component
```rust
use tailwind_rs::*;
use tailwind_rs_generic::*;

pub struct GenericCard {
    pub variant: CardVariant,
    pub children: String,
}

impl GenericCard {
    pub fn new() -> Self {
        Self {
            variant: CardVariant::Default,
            children: String::new(),
        }
    }
    
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }
}

impl GenericComponent for GenericCard {
    fn generate_classes(&self) -> String {
        classes! {
            base: "rounded-lg p-6",
            variant: match self.variant {
                CardVariant::Default => "bg-white border border-gray-200",
                CardVariant::Elevated => "bg-white shadow-lg",
                CardVariant::Outlined => "bg-white border-2 border-gray-300",
            },
        }.build()
    }
    
    fn render(&self) -> String {
        let classes = self.generate_classes();
        
        format!(
            r#"<div class="{}">{}</div>"#,
            classes, self.children
        )
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate card configuration
        if self.children.is_empty() {
            return Err(ValidationError::EmptyContent);
        }
        
        Ok(())
    }
}
```

## 🎭 State Management

### Generic State Manager
```rust
use tailwind_rs::*;
use tailwind_rs_generic::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct GenericStateManager {
    state: Arc<Mutex<HashMap<String, String>>>,
    listeners: Arc<Mutex<Vec<Box<dyn Fn(&str, &str) + Send + Sync>>>>,
}

impl GenericStateManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn set(&self, key: &str, value: &str) -> Result<(), StateError> {
        let mut state = self.state.lock().map_err(|_| StateError::LockError)?;
        state.insert(key.to_string(), value.to_string());
        
        // Notify listeners
        let listeners = self.listeners.lock().map_err(|_| StateError::LockError)?;
        for listener in listeners.iter() {
            listener(key, value);
        }
        
        Ok(())
    }
    
    pub fn get(&self, key: &str) -> Result<Option<String>, StateError> {
        let state = self.state.lock().map_err(|_| StateError::LockError)?;
        Ok(state.get(key).cloned())
    }
    
    pub fn subscribe<F>(&self, listener: F) -> Result<(), StateError>
    where
        F: Fn(&str, &str) + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.lock().map_err(|_| StateError::LockError)?;
        listeners.push(Box::new(listener));
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("Lock error")]
    LockError,
    #[error("Invalid state key: {0}")]
    InvalidKey(String),
}
```

### Reactive Component
```rust
use tailwind_rs::*;
use tailwind_rs_generic::*;

pub struct ReactiveButton {
    pub variant: ButtonVariant,
    pub state_manager: Arc<GenericStateManager>,
    pub state_key: String,
}

impl ReactiveButton {
    pub fn new(state_manager: Arc<GenericStateManager>, state_key: &str) -> Self {
        Self {
            variant: ButtonVariant::Primary,
            state_manager,
            state_key: state_key.to_string(),
        }
    }
    
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn get_current_state(&self) -> Result<Option<String>, StateError> {
        self.state_manager.get(&self.state_key)
    }
    
    pub fn update_state(&self, value: &str) -> Result<(), StateError> {
        self.state_manager.set(&self.state_key, value)
    }
}

impl GenericComponent for ReactiveButton {
    fn generate_classes(&self) -> String {
        let current_state = self.get_current_state().unwrap_or_default();
        
        classes! {
            base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
            variant: match self.variant {
                ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
                ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
                ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
            },
            state: if current_state == Some("loading".to_string()) {
                "opacity-50 cursor-not-allowed"
            } else {
                "cursor-pointer"
            },
        }.build()
    }
    
    fn render(&self) -> String {
        let classes = self.generate_classes();
        let current_state = self.get_current_state().unwrap_or_default();
        let button_text = match current_state {
            Some(state) if state == "loading" => "Loading...",
            Some(state) if state == "success" => "Success!",
            Some(state) if state == "error" => "Error!",
            _ => "Click me",
        };
        
        format!(
            r#"<button class="{}" data-state="{}">{}</button>"#,
            classes, 
            current_state.unwrap_or_default().unwrap_or_default(),
            button_text
        )
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate reactive button configuration
        if self.state_key.is_empty() {
            return Err(ValidationError::InvalidConfiguration);
        }
        
        Ok(())
    }
}
```

## 🎨 Advanced Theming

### Generic Theme System
```rust
use tailwind_rs::*;
use tailwind_rs_generic::*;

pub struct GenericTheme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
    pub background_color: Color,
    pub text_color: Color,
    pub spacing: SpacingScale,
    pub typography: TypographyScale,
}

impl Default for GenericTheme {
    fn default() -> Self {
        Self {
            primary_color: Color::Blue,
            secondary_color: Color::Gray,
            accent_color: Color::Green,
            background_color: Color::Gray,
            text_color: Color::Gray,
            spacing: SpacingScale::new(),
            typography: TypographyScale::new(),
        }
    }
}

impl GenericTheme {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn primary_color(mut self, color: Color) -> Self {
        self.primary_color = color;
        self
    }
    
    pub fn secondary_color(mut self, color: Color) -> Self {
        self.secondary_color = color;
        self
    }
    
    pub fn accent_color(mut self, color: Color) -> Self {
        self.accent_color = color;
        self
    }
    
    pub fn apply_to_component(&self, component: &dyn ThemedComponent) -> String {
        component.apply_theme(self)
    }
}

pub trait ThemedComponent {
    fn apply_theme(&self, theme: &GenericTheme) -> String;
}

impl ThemedComponent for GenericButton {
    fn apply_theme(&self, theme: &GenericTheme) -> String {
        classes! {
            base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
            background: theme.primary_color.background(600),
            text: theme.primary_color.text(600),
            hover: theme.primary_color.hover(700),
            focus: "focus:outline-none focus:ring-2",
            ring: theme.primary_color.ring(500),
        }.build()
    }
}
```

### Theme Presets
```rust
use tailwind_rs::*;
use tailwind_rs_generic::*;

pub enum ThemePreset {
    Default,
    Dark,
    Light,
    HighContrast,
    Minimal,
    Colorful,
    Professional,
    Playful,
}

impl ThemePreset {
    pub fn create(&self) -> GenericTheme {
        match self {
            ThemePreset::Default => GenericTheme::new(),
            ThemePreset::Dark => GenericTheme::new()
                .primary_color(Color::Blue)
                .secondary_color(Color::Gray)
                .accent_color(Color::Green)
                .background_color(Color::Gray)
                .text_color(Color::Gray),
            ThemePreset::Light => GenericTheme::new()
                .primary_color(Color::Blue)
                .secondary_color(Color::Gray)
                .accent_color(Color::Green)
                .background_color(Color::Gray)
                .text_color(Color::Gray),
            ThemePreset::HighContrast => GenericTheme::new()
                .primary_color(Color::Blue)
                .secondary_color(Color::Gray)
                .accent_color(Color::Green)
                .background_color(Color::Gray)
                .text_color(Color::Gray),
            ThemePreset::Minimal => GenericTheme::new()
                .primary_color(Color::Gray)
                .secondary_color(Color::Gray)
                .accent_color(Color::Gray)
                .background_color(Color::Gray)
                .text_color(Color::Gray),
            ThemePreset::Colorful => GenericTheme::new()
                .primary_color(Color::Purple)
                .secondary_color(Color::Pink)
                .accent_color(Color::Orange)
                .background_color(Color::Gray)
                .text_color(Color::Gray),
            ThemePreset::Professional => GenericTheme::new()
                .primary_color(Color::Blue)
                .secondary_color(Color::Gray)
                .accent_color(Color::Green)
                .background_color(Color::Gray)
                .text_color(Color::Gray),
            ThemePreset::Playful => GenericTheme::new()
                .primary_color(Color::Pink)
                .secondary_color(Color::Yellow)
                .accent_color(Color::Green)
                .background_color(Color::Gray)
                .text_color(Color::Gray),
        }
    }
}
```

## 🧪 Testing Generic Components

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_button() {
        let button = GenericButton::new()
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Medium)
            .disabled(false)
            .children("Click me");
        
        let classes = button.generate_classes();
        
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
    }

    #[test]
    fn test_generic_input() {
        let input = GenericInput::new()
            .variant(InputVariant::Default)
            .placeholder("Enter text")
            .value("test value")
            .disabled(false)
            .required(true);
        
        let classes = input.generate_classes();
        
        assert!(classes.contains("border-gray-300"));
        assert!(classes.contains("focus:border-blue-500"));
        assert!(classes.contains("bg-white"));
    }

    #[test]
    fn test_generic_card() {
        let card = GenericCard::new()
            .variant(CardVariant::Elevated)
            .children("<h2>Test Card</h2>");
        
        let classes = card.generate_classes();
        
        assert!(classes.contains("bg-white"));
        assert!(classes.contains("shadow-lg"));
        assert!(classes.contains("rounded-lg"));
        assert!(classes.contains("p-6"));
    }

    #[test]
    fn test_state_manager() {
        let state_manager = GenericStateManager::new();
        
        // Test setting and getting state
        state_manager.set("test_key", "test_value").unwrap();
        let value = state_manager.get("test_key").unwrap();
        
        assert_eq!(value, Some("test_value".to_string()));
    }

    #[test]
    fn test_reactive_button() {
        let state_manager = Arc::new(GenericStateManager::new());
        let button = ReactiveButton::new(state_manager.clone(), "button_state")
            .variant(ButtonVariant::Primary);
        
        // Test initial state
        let classes = button.generate_classes();
        assert!(classes.contains("bg-blue-600"));
        
        // Test state change
        state_manager.set("button_state", "loading").unwrap();
        let classes = button.generate_classes();
        assert!(classes.contains("opacity-50"));
    }

    #[test]
    fn test_theme_system() {
        let theme = GenericTheme::new()
            .primary_color(Color::Green)
            .secondary_color(Color::Purple);
        
        let button = GenericButton::new().variant(ButtonVariant::Primary);
        let themed_classes = theme.apply_to_component(&button);
        
        assert!(themed_classes.contains("bg-green-600"));
        assert!(themed_classes.contains("text-white"));
    }

    #[test]
    fn test_theme_presets() {
        let dark_theme = ThemePreset::Dark.create();
        let light_theme = ThemePreset::Light.create();
        
        assert_ne!(dark_theme.primary_color, light_theme.primary_color);
        
        let colorful_theme = ThemePreset::Colorful.create();
        assert_eq!(colorful_theme.primary_color, Color::Purple);
        assert_eq!(colorful_theme.secondary_color, Color::Pink);
    }
}
```

### Integration Testing
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_component_integration() {
        let state_manager = Arc::new(GenericStateManager::new());
        
        let button = ReactiveButton::new(state_manager.clone(), "button_state")
            .variant(ButtonVariant::Primary);
        
        let input = GenericInput::new()
            .variant(InputVariant::Default)
            .placeholder("Enter text");
        
        let card = GenericCard::new()
            .variant(CardVariant::Default)
            .children("Test content");
        
        // Test component rendering
        let button_html = button.render();
        let input_html = input.render();
        let card_html = card.render();
        
        assert!(button_html.contains("<button"));
        assert!(input_html.contains("<input"));
        assert!(card_html.contains("<div"));
        
        // Test state integration
        state_manager.set("button_state", "loading").unwrap();
        let updated_button_html = button.render();
        assert!(updated_button_html.contains("Loading..."));
    }

    #[test]
    fn test_theme_integration() {
        let theme = GenericTheme::new()
            .primary_color(Color::Green)
            .secondary_color(Color::Purple);
        
        let button = GenericButton::new().variant(ButtonVariant::Primary);
        let input = GenericInput::new().variant(InputVariant::Default);
        let card = GenericCard::new().variant(CardVariant::Default);
        
        let button_classes = theme.apply_to_component(&button);
        let input_classes = input.generate_classes();
        let card_classes = card.generate_classes();
        
        assert!(button_classes.contains("bg-green-600"));
        assert!(!input_classes.is_empty());
        assert!(!card_classes.is_empty());
    }
}
```

### End-to-End Testing with Playwright
```typescript
import { test, expect } from '@playwright/test';

test.describe('Generic Framework Integration', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/demo/generic');
    await page.waitForLoadState('networkidle');
  });

  test('should render generic components correctly', async ({ page }) => {
    // Test header
    await expect(page.locator('header')).toBeVisible();
    await expect(page.locator('h1')).toContainText('My Generic App');
    
    // Test main content
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('[data-testid="card"]')).toBeVisible();
    
    // Test button
    await expect(page.locator('[data-testid="button"]')).toBeVisible();
    await expect(page.locator('[data-testid="button"]'))
      .toHaveClass(/bg-blue-600/);
  });

  test('should handle state management', async ({ page }) => {
    // Test reactive button
    await expect(page.locator('[data-testid="reactive-button"]'))
      .toContainText('Click me');
    
    // Click button
    await page.click('[data-testid="reactive-button"]');
    
    // Wait for loading state
    await expect(page.locator('[data-testid="reactive-button"]'))
      .toContainText('Loading...');
    
    // Wait for completion
    await expect(page.locator('[data-testid="reactive-button"]'))
      .toContainText('Success!', { timeout: 2000 });
  });

  test('should handle theme switching', async ({ page }) => {
    // Test light theme
    await expect(page.locator('body')).toHaveClass(/bg-white/);
    
    // Switch to dark theme
    await page.click('[data-testid="theme-toggle"]');
    
    // Test dark theme
    await expect(page.locator('body')).toHaveClass(/bg-gray-900/);
    
    // Switch back to light theme
    await page.click('[data-testid="theme-toggle"]');
    
    // Test light theme again
    await expect(page.locator('body')).toHaveClass(/bg-white/);
  });

  test('should be responsive', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('[data-testid="responsive-card"]'))
      .toHaveClass(/p-4/);
    
    // Test tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.locator('[data-testid="responsive-card"]'))
      .toHaveClass(/md:p-6/);
    
    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator('[data-testid="responsive-card"]'))
      .toHaveClass(/lg:p-8/);
  });
});
```

## 🚀 Performance Optimization

### Generic Class Cache
```rust
use tailwind_rs::*;
use tailwind_rs_generic::*;
use std::sync::LazyLock;

static GENERIC_CLASS_CACHE: LazyLock<ClassCache> = LazyLock::new(|| {
    ClassCache::new(1000)
});

pub struct OptimizedGenericButton {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub children: String,
}

impl OptimizedGenericButton {
    pub fn new() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            disabled: false,
            children: String::new(),
        }
    }
    
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    pub fn children(mut self, children: &str) -> Self {
        self.children = children.to_string();
        self
    }
}

impl GenericComponent for OptimizedGenericButton {
    fn generate_classes(&self) -> String {
        let cache_key = format!("button-{:?}-{:?}-{}", self.variant, self.size, self.disabled);
        
        GENERIC_CLASS_CACHE.get_or_generate(&cache_key, || {
            classes! {
                base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
                variant: match self.variant {
                    ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
                    ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
                    ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
                },
                size: match self.size {
                    ButtonSize::Small => "px-2 py-1 text-sm",
                    ButtonSize::Medium => "px-4 py-2 text-base",
                    ButtonSize::Large => "px-6 py-3 text-lg",
                },
                state: if self.disabled {
                    "opacity-50 cursor-not-allowed"
                } else {
                    "cursor-pointer"
                },
            }.build()
        })
    }
    
    fn render(&self) -> String {
        let classes = self.generate_classes();
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        
        format!(
            r#"<button class="{}"{}>{}</button>"#,
            classes, disabled_attr, self.children
        )
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.children.is_empty() {
            return Err(ValidationError::EmptyContent);
        }
        
        Ok(())
    }
}
```

## 📚 Best Practices

### 1. Use Type-Safe Components
```rust
// Good: Type-safe components
pub struct GenericButton {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub children: String,
}

// Avoid: String-based components
pub struct GenericButton {
    pub variant: String,
    pub size: String,
    pub disabled: bool,
    pub children: String,
}
```

### 2. Implement Proper Validation
```rust
// Good: Comprehensive validation
impl GenericComponent for GenericButton {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.children.is_empty() {
            return Err(ValidationError::EmptyContent);
        }
        
        if self.disabled && self.variant == ButtonVariant::Primary {
            return Err(ValidationError::InvalidConfiguration);
        }
        
        Ok(())
    }
}

// Avoid: No validation
impl GenericComponent for GenericButton {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}
```

### 3. Use Efficient Caching
```rust
// Good: Efficient caching
impl GenericComponent for OptimizedGenericButton {
    fn generate_classes(&self) -> String {
        let cache_key = format!("button-{:?}-{:?}-{}", self.variant, self.size, self.disabled);
        
        GENERIC_CLASS_CACHE.get_or_generate(&cache_key, || {
            // Generate classes
        })
    }
}

// Avoid: No caching
impl GenericComponent for GenericButton {
    fn generate_classes(&self) -> String {
        // Generate classes every time
    }
}
```

### 4. Test All Variants
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_button_variants() {
        for variant in [ButtonVariant::Primary, ButtonVariant::Secondary, ButtonVariant::Danger] {
            for size in [ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large] {
                let button = GenericButton::new()
                    .variant(variant)
                    .size(size);
                
                let classes = button.generate_classes();
                
                assert!(!classes.is_empty());
                assert!(classes.contains("rounded-md"));
            }
        }
    }
}
```

## 🎯 Next Steps

Now that you understand generic framework integration:

1. **Explore [Advanced Features](./advanced/dynamic-styling.md)**
2. **Check out [Performance Optimization](./advanced/performance.md)**
3. **See [Example Projects](./examples/README.md)**
4. **Learn about [Testing Strategies](./testing.md)**

---

This generic framework integration guide demonstrates how to build powerful, type-safe, and performant web applications using `tailwind-rs` with any framework or custom implementation. The integration follows our established ADRs and best practices, ensuring reliable and maintainable code.

