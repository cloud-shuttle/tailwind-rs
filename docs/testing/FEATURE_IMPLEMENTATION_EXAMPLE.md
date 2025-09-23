# 🎯 Feature Implementation Example: Text Shadow Utilities

## 📋 Overview

This document demonstrates how to implement a new Tailwind v4.1 feature (Text Shadow) with comprehensive testing to prevent regressions.

## 🚀 Implementation Steps

### **Step 1: Create the Text Shadow Module**

#### **`crates/tailwind-rs-core/src/text_shadow.rs`**
```rust
use serde::{Deserialize, Serialize};
use std::fmt;

/// Text shadow utilities for Tailwind-RS
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextShadow {
    /// No text shadow
    None,
    /// Small text shadow
    Sm,
    /// Default text shadow
    Default,
    /// Large text shadow
    Lg,
    /// Extra large text shadow
    Xl,
    /// Custom text shadow value
    Custom(String),
    /// Arbitrary text shadow value
    Arbitrary(String),
}

impl TextShadow {
    /// Convert to CSS string
    pub fn to_css(&self) -> String {
        match self {
            TextShadow::None => "text-shadow: none;".to_string(),
            TextShadow::Sm => "text-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);".to_string(),
            TextShadow::Default => "text-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);".to_string(),
            TextShadow::Lg => "text-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);".to_string(),
            TextShadow::Xl => "text-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);".to_string(),
            TextShadow::Custom(value) => format!("text-shadow: {};", value),
            TextShadow::Arbitrary(value) => format!("text-shadow: {};", value),
        }
    }

    /// Get the CSS class name
    pub fn class_name(&self) -> String {
        match self {
            TextShadow::None => "text-shadow-none".to_string(),
            TextShadow::Sm => "text-shadow-sm".to_string(),
            TextShadow::Default => "text-shadow".to_string(),
            TextShadow::Lg => "text-shadow-lg".to_string(),
            TextShadow::Xl => "text-shadow-xl".to_string(),
            TextShadow::Custom(_) => "text-shadow-custom".to_string(),
            TextShadow::Arbitrary(_) => "text-shadow-arbitrary".to_string(),
        }
    }

    /// Get the variant name for testing
    pub fn variant(&self) -> &'static str {
        match self {
            TextShadow::None => "none",
            TextShadow::Sm => "sm",
            TextShadow::Default => "default",
            TextShadow::Lg => "lg",
            TextShadow::Xl => "xl",
            TextShadow::Custom(_) => "custom",
            TextShadow::Arbitrary(_) => "arbitrary",
        }
    }
}

impl fmt::Display for TextShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_shadow_none() {
        let shadow = TextShadow::None;
        assert_eq!(shadow.to_css(), "text-shadow: none;");
        assert_eq!(shadow.class_name(), "text-shadow-none");
        assert_eq!(shadow.variant(), "none");
    }

    #[test]
    fn test_text_shadow_sm() {
        let shadow = TextShadow::Sm;
        let css = shadow.to_css();
        assert!(css.contains("text-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05)"));
        assert_eq!(shadow.class_name(), "text-shadow-sm");
        assert_eq!(shadow.variant(), "sm");
    }

    #[test]
    fn test_text_shadow_lg() {
        let shadow = TextShadow::Lg;
        let css = shadow.to_css();
        assert!(css.contains("text-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1)"));
        assert_eq!(shadow.class_name(), "text-shadow-lg");
        assert_eq!(shadow.variant(), "lg");
    }

    #[test]
    fn test_text_shadow_custom() {
        let shadow = TextShadow::Custom("0 2px 4px rgba(0,0,0,0.1)".to_string());
        assert_eq!(shadow.to_css(), "text-shadow: 0 2px 4px rgba(0,0,0,0.1);");
        assert_eq!(shadow.class_name(), "text-shadow-custom");
        assert_eq!(shadow.variant(), "custom");
    }

    #[test]
    fn test_text_shadow_arbitrary() {
        let shadow = TextShadow::Arbitrary("0 0 10px #ff0000".to_string());
        assert_eq!(shadow.to_css(), "text-shadow: 0 0 10px #ff0000;");
        assert_eq!(shadow.class_name(), "text-shadow-arbitrary");
        assert_eq!(shadow.variant(), "arbitrary");
    }
}
```

### **Step 2: Integrate with ClassBuilder**

#### **`crates/tailwind-rs-core/src/lib.rs`**
```rust
pub mod text_shadow;

// ... existing code ...

use text_shadow::TextShadow;

impl ClassBuilder {
    /// Add text shadow utility
    pub fn text_shadow(mut self, shadow: TextShadow) -> Self {
        self.classes.push(shadow.class_name());
        self.css_rules.push(shadow.to_css());
        self
    }
}
```

### **Step 3: Create Comprehensive Tests**

#### **`crates/tailwind-rs-core/tests/unit/text_shadow_tests.rs`**
```rust
use tailwind_rs_core::text_shadow::*;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod text_shadow_unit_tests {
    use super::*;

    #[test]
    fn test_text_shadow_none() {
        let shadow = TextShadow::None;
        assert_eq!(shadow.to_css(), "text-shadow: none;");
    }

    #[test]
    fn test_text_shadow_sm() {
        let shadow = TextShadow::Sm;
        let css = shadow.to_css();
        assert!(css.contains("text-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05)"));
    }

    #[test]
    fn test_text_shadow_lg() {
        let shadow = TextShadow::Lg;
        let css = shadow.to_css();
        assert!(css.contains("text-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1)"));
    }

    #[test]
    fn test_text_shadow_custom() {
        let shadow = TextShadow::Custom("0 2px 4px rgba(0,0,0,0.1)".to_string());
        assert_eq!(shadow.to_css(), "text-shadow: 0 2px 4px rgba(0,0,0,0.1);");
    }

    #[test]
    fn test_text_shadow_arbitrary() {
        let shadow = TextShadow::Arbitrary("0 0 10px #ff0000".to_string());
        assert_eq!(shadow.to_css(), "text-shadow: 0 0 10px #ff0000;");
    }
}

#[cfg(test)]
mod text_shadow_integration_tests {
    use super::*;

    #[test]
    fn test_text_shadow_with_other_utilities() {
        let builder = ClassBuilder::new()
            .text_shadow(TextShadow::Sm)
            .color(Color::new(ColorPalette::Blue, ColorShade::Shade500));
        
        let classes = builder.build();
        assert!(classes.contains("text-shadow-sm"));
        assert!(classes.contains("text-blue-500"));
    }

    #[test]
    fn test_text_shadow_responsive() {
        let builder = ClassBuilder::new()
            .text_shadow(TextShadow::Sm)
            .responsive(ResponsiveBreakpoint::Md, |b| b.text_shadow(TextShadow::Lg));
        
        let classes = builder.build();
        assert!(classes.contains("text-shadow-sm"));
        assert!(classes.contains("md:text-shadow-lg"));
    }

    #[test]
    fn test_text_shadow_dark_mode() {
        let builder = ClassBuilder::new()
            .text_shadow(TextShadow::Sm)
            .dark_mode(|b| b.text_shadow(TextShadow::Lg));
        
        let classes = builder.build();
        assert!(classes.contains("text-shadow-sm"));
        assert!(classes.contains("dark:text-shadow-lg"));
    }
}
```

#### **`crates/tailwind-rs-core/tests/property/text_shadow_proptest.rs`**
```rust
use proptest::prelude::*;
use tailwind_rs_core::text_shadow::*;

proptest! {
    #[test]
    fn test_text_shadow_arbitrary_values(value in "[a-zA-Z0-9\\s\\-\\+\\*\\(\\)\\,\\.\\#]+") {
        let shadow = TextShadow::Custom(value.clone());
        let css = shadow.to_css();
        assert!(css.starts_with("text-shadow: "));
        assert!(css.ends_with(";"));
        assert!(css.contains(&value));
    }

    #[test]
    fn test_text_shadow_combinations(
        shadow1 in any::<TextShadow>(),
        shadow2 in any::<TextShadow>()
    ) {
        let builder = ClassBuilder::new()
            .text_shadow(shadow1.clone())
            .text_shadow(shadow2.clone());
        
        let classes = builder.build();
        // Should contain the last shadow applied
        assert!(classes.contains(&format!("text-shadow-{}", shadow2.variant())));
    }
}
```

#### **`crates/tailwind-rs-core/tests/performance/text_shadow_performance_tests.rs`**
```rust
use tailwind_rs_core::text_shadow::*;
use tailwind_rs_core::ClassBuilder;
use std::time::Instant;

#[cfg(test)]
mod text_shadow_performance_tests {
    use super::*;

    #[test]
    fn test_text_shadow_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 text shadow classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .text_shadow(TextShadow::Sm)
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Text shadow generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_text_shadow_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many text shadow builders
        let builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().text_shadow(TextShadow::Sm))
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 100_000, "Memory usage too high: {} bytes", memory_increase);
    }
}
```

#### **`crates/tailwind-rs-core/tests/regression/text_shadow_baseline_tests.rs`**
```rust
use tailwind_rs_core::text_shadow::*;
use tailwind_rs_core::ClassBuilder;

#[cfg(test)]
mod text_shadow_baseline_tests {
    use super::*;

    #[test]
    fn test_text_shadow_css_output_baseline() {
        let builder = ClassBuilder::new()
            .text_shadow(TextShadow::Sm)
            .text_shadow(TextShadow::Lg);
        
        let css = builder.to_css();
        let baseline = load_baseline("text-shadow-sm-lg.css");
        
        assert_eq!(css, baseline, "CSS output differs from baseline");
    }

    #[test]
    fn test_text_shadow_class_generation_baseline() {
        let builder = ClassBuilder::new()
            .text_shadow(TextShadow::Lg)
            .text_shadow(TextShadow::Custom("0 2px 4px rgba(0,0,0,0.1)".to_string()));
        
        let classes = builder.build();
        let baseline = load_baseline("text-shadow-lg-custom.classes");
        
        assert_eq!(classes, baseline, "Class generation differs from baseline");
    }
}
```

### **Step 4: Create Visual Regression Tests**

#### **`crates/tailwind-rs-wasm/tests/visual/text-shadow-visual.spec.js`**
```javascript
const { test, expect } = require('@playwright/test');

test.describe('Text Shadow Visual Regression Tests', () => {
  test('text shadow utilities render correctly', async ({ page }) => {
    await page.goto('/comprehensive-demo.html');
    
    // Test text shadow utilities
    await page.click('.nav-tab[data-tab="utilities"]');
    await expect(page.locator('.text-shadow-sm')).toBeVisible();
    await expect(page.locator('.text-shadow-lg')).toBeVisible();
    
    // Take screenshot for visual comparison
    await expect(page).toHaveScreenshot('text-shadow-utilities.png');
  });

  test('text shadow with other utilities', async ({ page }) => {
    await page.goto('/comprehensive-demo.html');
    
    // Test text shadow with other utilities
    await page.click('.nav-tab[data-tab="utilities"]');
    await expect(page.locator('.text-shadow-sm.text-blue-500')).toBeVisible();
    await expect(page.locator('.text-shadow-lg.text-red-500')).toBeVisible();
    
    // Take screenshot for visual comparison
    await expect(page).toHaveScreenshot('text-shadow-combined.png');
  });
});
```

### **Step 5: Update Documentation**

#### **`docs/features/text-shadow.md`**
```markdown
# Text Shadow Utilities

Text shadow utilities provide a way to add shadow effects to text elements.

## Basic Usage

```rust
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::text_shadow::TextShadow;

let builder = ClassBuilder::new()
    .text_shadow(TextShadow::Sm);

let classes = builder.build();
// Result: "text-shadow-sm"
```

## Available Variants

- `TextShadow::None` - No text shadow
- `TextShadow::Sm` - Small text shadow
- `TextShadow::Default` - Default text shadow
- `TextShadow::Lg` - Large text shadow
- `TextShadow::Xl` - Extra large text shadow
- `TextShadow::Custom(value)` - Custom text shadow value
- `TextShadow::Arbitrary(value)` - Arbitrary text shadow value

## Examples

### Basic Text Shadow
```rust
let builder = ClassBuilder::new()
    .text_shadow(TextShadow::Sm);
```

### Custom Text Shadow
```rust
let builder = ClassBuilder::new()
    .text_shadow(TextShadow::Custom("0 2px 4px rgba(0,0,0,0.1)".to_string()));
```

### Responsive Text Shadow
```rust
let builder = ClassBuilder::new()
    .text_shadow(TextShadow::Sm)
    .responsive(ResponsiveBreakpoint::Md, |b| b.text_shadow(TextShadow::Lg));
```

### Dark Mode Text Shadow
```rust
let builder = ClassBuilder::new()
    .text_shadow(TextShadow::Sm)
    .dark_mode(|b| b.text_shadow(TextShadow::Lg));
```
```

## 🚀 Implementation Workflow

### **Day 1: Core Implementation**
1. **Create text_shadow.rs module**
2. **Implement TextShadow enum**
3. **Add basic unit tests**
4. **Integrate with ClassBuilder**

### **Day 2: Comprehensive Testing**
1. **Add integration tests**
2. **Add property-based tests**
3. **Add performance tests**
4. **Add baseline tests**

### **Day 3: Visual Testing**
1. **Add Playwright visual tests**
2. **Update demo pages**
3. **Add visual regression tests**
4. **Test cross-browser compatibility**

### **Day 4: Documentation and Examples**
1. **Update documentation**
2. **Create comprehensive examples**
3. **Add migration guides**
4. **Update README**

### **Day 5: Validation and Polish**
1. **Run full test suite**
2. **Validate performance**
3. **Check bundle size**
4. **Final documentation review**

## 📊 Success Metrics

### **Test Coverage**
- **Unit Tests**: 100% coverage for TextShadow enum
- **Integration Tests**: All framework combinations
- **Performance Tests**: Bundle size and runtime performance
- **Visual Tests**: Cross-browser compatibility

### **Quality Gates**
- **All tests pass**: 100% success rate
- **Performance**: No regression in bundle size or runtime
- **Compatibility**: All supported frameworks work
- **Documentation**: All examples compile and run

This example demonstrates how to implement a new Tailwind v4.1 feature with comprehensive testing to prevent regressions while maintaining high quality and performance standards.
