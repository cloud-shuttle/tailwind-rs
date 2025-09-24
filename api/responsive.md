# Responsive Design API Reference

This document provides comprehensive API reference for responsive design functionality in `tailwind-rs`, following our Test-Driven Development approach and comprehensive testing strategy.

## 🎯 Core Responsive Types

### Responsive
The primary type for responsive class generation.

```rust
pub struct Responsive {
    pub sm: Option<String>,
    pub md: Option<String>,
    pub lg: Option<String>,
    pub xl: Option<String>,
    pub xl2: Option<String>,
}
```

#### Methods

##### `new() -> Self`
Creates a new responsive configuration.

```rust
let responsive = Responsive::new();
```

**Example:**
```rust
use tailwind_rs::*;

let responsive = Responsive::new();
assert!(responsive.sm.is_none());
assert!(responsive.md.is_none());
```

##### `sm(self, classes: &str) -> Self`
Sets classes for small screens (640px+).

```rust
let responsive = Responsive::new().sm("text-base");
```

**Example:**
```rust
use tailwind_rs::*;

let responsive = Responsive::new().sm("text-base");
assert_eq!(responsive.sm, Some("text-base".to_string()));
```

##### `md(self, classes: &str) -> Self`
Sets classes for medium screens (768px+).

```rust
let responsive = Responsive::new().md("text-lg");
```

**Example:**
```rust
use tailwind_rs::*;

let responsive = Responsive::new().md("text-lg");
assert_eq!(responsive.md, Some("text-lg".to_string()));
```

##### `lg(self, classes: &str) -> Self`
Sets classes for large screens (1024px+).

```rust
let responsive = Responsive::new().lg("text-xl");
```

**Example:**
```rust
use tailwind_rs::*;

let responsive = Responsive::new().lg("text-xl");
assert_eq!(responsive.lg, Some("text-xl".to_string()));
```

##### `xl(self, classes: &str) -> Self`
Sets classes for extra large screens (1280px+).

```rust
let responsive = Responsive::new().xl("text-2xl");
```

**Example:**
```rust
use tailwind_rs::*;

let responsive = Responsive::new().xl("text-2xl");
assert_eq!(responsive.xl, Some("text-2xl".to_string()));
```

##### `xl2(self, classes: &str) -> Self`
Sets classes for extra extra large screens (1536px+).

```rust
let responsive = Responsive::new().xl2("text-3xl");
```

**Example:**
```rust
use tailwind_rs::*;

let responsive = Responsive::new().xl2("text-3xl");
assert_eq!(responsive.xl2, Some("text-3xl".to_string()));
```

##### `build(self) -> String`
Builds the final responsive class string.

```rust
let responsive = Responsive::new()
    .sm("text-base")
    .md("text-lg")
    .lg("text-xl");
let classes = responsive.build();
```

**Example:**
```rust
use tailwind_rs::*;

let responsive = Responsive::new()
    .sm("text-base")
    .md("text-lg")
    .lg("text-xl");
let classes = responsive.build();

assert!(classes.contains("sm:text-base"));
assert!(classes.contains("md:text-lg"));
assert!(classes.contains("lg:text-xl"));
```

### Breakpoint
Represents responsive breakpoints.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Breakpoint {
    Sm,   // 640px
    Md,   // 768px
    Lg,   // 1024px
    Xl,   // 1280px
    Xl2,  // 1536px
}
```

#### Methods

##### `prefix(&self) -> &'static str`
Returns the CSS prefix for the breakpoint.

```rust
assert_eq!(Breakpoint::Sm.prefix(), "sm");
assert_eq!(Breakpoint::Md.prefix(), "md");
assert_eq!(Breakpoint::Lg.prefix(), "lg");
```

**Example:**
```rust
use tailwind_rs::*;

assert_eq!(Breakpoint::Sm.prefix(), "sm");
assert_eq!(Breakpoint::Md.prefix(), "md");
assert_eq!(Breakpoint::Lg.prefix(), "lg");
assert_eq!(Breakpoint::Xl.prefix(), "xl");
assert_eq!(Breakpoint::Xl2.prefix(), "2xl");
```

##### `min_width(&self) -> u32`
Returns the minimum width in pixels.

```rust
assert_eq!(Breakpoint::Sm.min_width(), 640);
assert_eq!(Breakpoint::Md.min_width(), 768);
assert_eq!(Breakpoint::Lg.min_width(), 1024);
```

**Example:**
```rust
use tailwind_rs::*;

assert_eq!(Breakpoint::Sm.min_width(), 640);
assert_eq!(Breakpoint::Md.min_width(), 768);
assert_eq!(Breakpoint::Lg.min_width(), 1024);
assert_eq!(Breakpoint::Xl.min_width(), 1280);
assert_eq!(Breakpoint::Xl2.min_width(), 1536);
```

##### `max_width(&self) -> Option<u32>`
Returns the maximum width in pixels (if applicable).

```rust
assert_eq!(Breakpoint::Sm.max_width(), Some(767));
assert_eq!(Breakpoint::Md.max_width(), Some(1023));
assert_eq!(Breakpoint::Lg.max_width(), Some(1279));
```

**Example:**
```rust
use tailwind_rs::*;

assert_eq!(Breakpoint::Sm.max_width(), Some(767));
assert_eq!(Breakpoint::Md.max_width(), Some(1023));
assert_eq!(Breakpoint::Lg.max_width(), Some(1279));
assert_eq!(Breakpoint::Xl.max_width(), Some(1535));
assert_eq!(Breakpoint::Xl2.max_width(), None);
```

##### `media_query(&self) -> String`
Returns the CSS media query for the breakpoint.

```rust
assert_eq!(Breakpoint::Sm.media_query(), "@media (min-width: 640px)");
assert_eq!(Breakpoint::Md.media_query(), "@media (min-width: 768px)");
```

**Example:**
```rust
use tailwind_rs::*;

assert_eq!(Breakpoint::Sm.media_query(), "@media (min-width: 640px)");
assert_eq!(Breakpoint::Md.media_query(), "@media (min-width: 768px)");
assert_eq!(Breakpoint::Lg.media_query(), "@media (min-width: 1024px)");
assert_eq!(Breakpoint::Xl.media_query(), "@media (min-width: 1280px)");
assert_eq!(Breakpoint::Xl2.media_query(), "@media (min-width: 1536px)");
```

## 🎨 Responsive Utilities

### ResponsiveBuilder
Builder pattern for responsive class generation.

```rust
pub struct ResponsiveBuilder {
    base_classes: Vec<String>,
    breakpoint_classes: HashMap<Breakpoint, Vec<String>>,
}
```

#### Methods

##### `new() -> Self`
Creates a new responsive builder.

```rust
let builder = ResponsiveBuilder::new();
```

**Example:**
```rust
use tailwind_rs::*;

let builder = ResponsiveBuilder::new();
assert!(builder.base_classes.is_empty());
assert!(builder.breakpoint_classes.is_empty());
```

##### `base(self, classes: &str) -> Self`
Adds base classes.

```rust
let builder = ResponsiveBuilder::new().base("text-sm");
```

**Example:**
```rust
use tailwind_rs::*;

let builder = ResponsiveBuilder::new().base("text-sm");
assert!(builder.base_classes.contains(&"text-sm".to_string()));
```

##### `breakpoint(self, breakpoint: Breakpoint, classes: &str) -> Self`
Adds classes for a specific breakpoint.

```rust
let builder = ResponsiveBuilder::new()
    .base("text-sm")
    .breakpoint(Breakpoint::Sm, "text-base")
    .breakpoint(Breakpoint::Md, "text-lg");
```

**Example:**
```rust
use tailwind_rs::*;

let builder = ResponsiveBuilder::new()
    .base("text-sm")
    .breakpoint(Breakpoint::Sm, "text-base")
    .breakpoint(Breakpoint::Md, "text-lg");

let classes = builder.build();
assert!(classes.contains("text-sm"));
assert!(classes.contains("sm:text-base"));
assert!(classes.contains("md:text-lg"));
```

##### `build(self) -> String`
Builds the final responsive class string.

```rust
let classes = ResponsiveBuilder::new()
    .base("text-sm")
    .breakpoint(Breakpoint::Sm, "text-base")
    .breakpoint(Breakpoint::Md, "text-lg")
    .build();
```

**Example:**
```rust
use tailwind_rs::*;

let classes = ResponsiveBuilder::new()
    .base("text-sm")
    .breakpoint(Breakpoint::Sm, "text-base")
    .breakpoint(Breakpoint::Md, "text-lg")
    .build();

assert_eq!(classes, "text-sm sm:text-base md:text-lg");
```

### ResponsiveGrid
Specialized responsive grid system.

```rust
pub struct ResponsiveGrid {
    cols: HashMap<Breakpoint, u32>,
    gap: HashMap<Breakpoint, String>,
    auto_fit: bool,
}
```

#### Methods

##### `new() -> Self`
Creates a new responsive grid.

```rust
let grid = ResponsiveGrid::new();
```

**Example:**
```rust
use tailwind_rs::*;

let grid = ResponsiveGrid::new();
assert!(grid.cols.is_empty());
assert!(grid.gap.is_empty());
```

##### `cols(self, breakpoint: Breakpoint, count: u32) -> Self`
Sets the number of columns for a breakpoint.

```rust
let grid = ResponsiveGrid::new()
    .cols(Breakpoint::Sm, 2)
    .cols(Breakpoint::Md, 3)
    .cols(Breakpoint::Lg, 4);
```

**Example:**
```rust
use tailwind_rs::*;

let grid = ResponsiveGrid::new()
    .cols(Breakpoint::Sm, 2)
    .cols(Breakpoint::Md, 3)
    .cols(Breakpoint::Lg, 4);

let classes = grid.build();
assert!(classes.contains("grid-cols-2"));
assert!(classes.contains("sm:grid-cols-2"));
assert!(classes.contains("md:grid-cols-3"));
assert!(classes.contains("lg:grid-cols-4"));
```

##### `gap(self, breakpoint: Breakpoint, gap: &str) -> Self`
Sets the gap for a breakpoint.

```rust
let grid = ResponsiveGrid::new()
    .gap(Breakpoint::Sm, "gap-4")
    .gap(Breakpoint::Md, "gap-6")
    .gap(Breakpoint::Lg, "gap-8");
```

**Example:**
```rust
use tailwind_rs::*;

let grid = ResponsiveGrid::new()
    .gap(Breakpoint::Sm, "gap-4")
    .gap(Breakpoint::Md, "gap-6")
    .gap(Breakpoint::Lg, "gap-8");

let classes = grid.build();
assert!(classes.contains("gap-4"));
assert!(classes.contains("sm:gap-4"));
assert!(classes.contains("md:gap-6"));
assert!(classes.contains("lg:gap-8"));
```

##### `auto_fit(self, enabled: bool) -> Self`
Enables or disables auto-fit columns.

```rust
let grid = ResponsiveGrid::new().auto_fit(true);
```

**Example:**
```rust
use tailwind_rs::*;

let grid = ResponsiveGrid::new().auto_fit(true);
let classes = grid.build();
assert!(classes.contains("grid-cols-auto-fit"));
```

##### `build(self) -> String`
Builds the final grid class string.

```rust
let classes = ResponsiveGrid::new()
    .cols(Breakpoint::Sm, 2)
    .cols(Breakpoint::Md, 3)
    .gap(Breakpoint::Sm, "gap-4")
    .gap(Breakpoint::Md, "gap-6")
    .build();
```

**Example:**
```rust
use tailwind_rs::*;

let classes = ResponsiveGrid::new()
    .cols(Breakpoint::Sm, 2)
    .cols(Breakpoint::Md, 3)
    .gap(Breakpoint::Sm, "gap-4")
    .gap(Breakpoint::Md, "gap-6")
    .build();

assert!(classes.contains("grid"));
assert!(classes.contains("grid-cols-2"));
assert!(classes.contains("sm:grid-cols-2"));
assert!(classes.contains("md:grid-cols-3"));
assert!(classes.contains("gap-4"));
assert!(classes.contains("sm:gap-4"));
assert!(classes.contains("md:gap-6"));
```

### ResponsiveFlex
Specialized responsive flexbox system.

```rust
pub struct ResponsiveFlex {
    direction: HashMap<Breakpoint, FlexDirection>,
    wrap: HashMap<Breakpoint, FlexWrap>,
    justify: HashMap<Breakpoint, JustifyContent>,
    align: HashMap<Breakpoint, AlignItems>,
    gap: HashMap<Breakpoint, String>,
}
```

#### Methods

##### `new() -> Self`
Creates a new responsive flex container.

```rust
let flex = ResponsiveFlex::new();
```

**Example:**
```rust
use tailwind_rs::*;

let flex = ResponsiveFlex::new();
assert!(flex.direction.is_empty());
assert!(flex.wrap.is_empty());
```

##### `direction(self, breakpoint: Breakpoint, direction: FlexDirection) -> Self`
Sets the flex direction for a breakpoint.

```rust
let flex = ResponsiveFlex::new()
    .direction(Breakpoint::Sm, FlexDirection::Column)
    .direction(Breakpoint::Md, FlexDirection::Row);
```

**Example:**
```rust
use tailwind_rs::*;

let flex = ResponsiveFlex::new()
    .direction(Breakpoint::Sm, FlexDirection::Column)
    .direction(Breakpoint::Md, FlexDirection::Row);

let classes = flex.build();
assert!(classes.contains("flex-col"));
assert!(classes.contains("sm:flex-col"));
assert!(classes.contains("md:flex-row"));
```

##### `wrap(self, breakpoint: Breakpoint, wrap: FlexWrap) -> Self`
Sets the flex wrap for a breakpoint.

```rust
let flex = ResponsiveFlex::new()
    .wrap(Breakpoint::Sm, FlexWrap::Wrap)
    .wrap(Breakpoint::Md, FlexWrap::NoWrap);
```

**Example:**
```rust
use tailwind_rs::*;

let flex = ResponsiveFlex::new()
    .wrap(Breakpoint::Sm, FlexWrap::Wrap)
    .wrap(Breakpoint::Md, FlexWrap::NoWrap);

let classes = flex.build();
assert!(classes.contains("flex-wrap"));
assert!(classes.contains("sm:flex-wrap"));
assert!(classes.contains("md:flex-nowrap"));
```

##### `justify(self, breakpoint: Breakpoint, justify: JustifyContent) -> Self`
Sets the justify content for a breakpoint.

```rust
let flex = ResponsiveFlex::new()
    .justify(Breakpoint::Sm, JustifyContent::Center)
    .justify(Breakpoint::Md, JustifyContent::SpaceBetween);
```

**Example:**
```rust
use tailwind_rs::*;

let flex = ResponsiveFlex::new()
    .justify(Breakpoint::Sm, JustifyContent::Center)
    .justify(Breakpoint::Md, JustifyContent::SpaceBetween);

let classes = flex.build();
assert!(classes.contains("justify-center"));
assert!(classes.contains("sm:justify-center"));
assert!(classes.contains("md:justify-between"));
```

##### `align(self, breakpoint: Breakpoint, align: AlignItems) -> Self`
Sets the align items for a breakpoint.

```rust
let flex = ResponsiveFlex::new()
    .align(Breakpoint::Sm, AlignItems::Center)
    .align(Breakpoint::Md, AlignItems::Stretch);
```

**Example:**
```rust
use tailwind_rs::*;

let flex = ResponsiveFlex::new()
    .align(Breakpoint::Sm, AlignItems::Center)
    .align(Breakpoint::Md, AlignItems::Stretch);

let classes = flex.build();
assert!(classes.contains("items-center"));
assert!(classes.contains("sm:items-center"));
assert!(classes.contains("md:items-stretch"));
```

##### `gap(self, breakpoint: Breakpoint, gap: &str) -> Self`
Sets the gap for a breakpoint.

```rust
let flex = ResponsiveFlex::new()
    .gap(Breakpoint::Sm, "gap-4")
    .gap(Breakpoint::Md, "gap-6");
```

**Example:**
```rust
use tailwind_rs::*;

let flex = ResponsiveFlex::new()
    .gap(Breakpoint::Sm, "gap-4")
    .gap(Breakpoint::Md, "gap-6");

let classes = flex.build();
assert!(classes.contains("gap-4"));
assert!(classes.contains("sm:gap-4"));
assert!(classes.contains("md:gap-6"));
```

##### `build(self) -> String`
Builds the final flex class string.

```rust
let classes = ResponsiveFlex::new()
    .direction(Breakpoint::Sm, FlexDirection::Column)
    .direction(Breakpoint::Md, FlexDirection::Row)
    .justify(Breakpoint::Sm, JustifyContent::Center)
    .justify(Breakpoint::Md, JustifyContent::SpaceBetween)
    .build();
```

**Example:**
```rust
use tailwind_rs::*;

let classes = ResponsiveFlex::new()
    .direction(Breakpoint::Sm, FlexDirection::Column)
    .direction(Breakpoint::Md, FlexDirection::Row)
    .justify(Breakpoint::Sm, JustifyContent::Center)
    .justify(Breakpoint::Md, JustifyContent::SpaceBetween)
    .build();

assert!(classes.contains("flex"));
assert!(classes.contains("flex-col"));
assert!(classes.contains("sm:flex-col"));
assert!(classes.contains("md:flex-row"));
assert!(classes.contains("justify-center"));
assert!(classes.contains("sm:justify-center"));
assert!(classes.contains("md:justify-between"));
```

## 🎯 Flexbox Types

### FlexDirection
Represents flex direction values.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}
```

#### Methods

##### `class(&self) -> &'static str`
Returns the CSS class for the flex direction.

```rust
assert_eq!(FlexDirection::Row.class(), "flex-row");
assert_eq!(FlexDirection::Column.class(), "flex-col");
```

**Example:**
```rust
use tailwind_rs::*;

assert_eq!(FlexDirection::Row.class(), "flex-row");
assert_eq!(FlexDirection::Column.class(), "flex-col");
assert_eq!(FlexDirection::RowReverse.class(), "flex-row-reverse");
assert_eq!(FlexDirection::ColumnReverse.class(), "flex-col-reverse");
```

### FlexWrap
Represents flex wrap values.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}
```

#### Methods

##### `class(&self) -> &'static str`
Returns the CSS class for the flex wrap.

```rust
assert_eq!(FlexWrap::NoWrap.class(), "flex-nowrap");
assert_eq!(FlexWrap::Wrap.class(), "flex-wrap");
```

**Example:**
```rust
use tailwind_rs::*;

assert_eq!(FlexWrap::NoWrap.class(), "flex-nowrap");
assert_eq!(FlexWrap::Wrap.class(), "flex-wrap");
assert_eq!(FlexWrap::WrapReverse.class(), "flex-wrap-reverse");
```

### JustifyContent
Represents justify content values.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}
```

#### Methods

##### `class(&self) -> &'static str`
Returns the CSS class for justify content.

```rust
assert_eq!(JustifyContent::Start.class(), "justify-start");
assert_eq!(JustifyContent::Center.class(), "justify-center");
```

**Example:**
```rust
use tailwind_rs::*;

assert_eq!(JustifyContent::Start.class(), "justify-start");
assert_eq!(JustifyContent::End.class(), "justify-end");
assert_eq!(JustifyContent::Center.class(), "justify-center");
assert_eq!(JustifyContent::SpaceBetween.class(), "justify-between");
assert_eq!(JustifyContent::SpaceAround.class(), "justify-around");
assert_eq!(JustifyContent::SpaceEvenly.class(), "justify-evenly");
```

### AlignItems
Represents align items values.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}
```

#### Methods

##### `class(&self) -> &'static str`
Returns the CSS class for align items.

```rust
assert_eq!(AlignItems::Start.class(), "items-start");
assert_eq!(AlignItems::Center.class(), "items-center");
```

**Example:**
```rust
use tailwind_rs::*;

assert_eq!(AlignItems::Start.class(), "items-start");
assert_eq!(AlignItems::End.class(), "items-end");
assert_eq!(AlignItems::Center.class(), "items-center");
assert_eq!(AlignItems::Baseline.class(), "items-baseline");
assert_eq!(AlignItems::Stretch.class(), "items-stretch");
```

## 🧪 Testing Responsive System

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_creation() {
        let responsive = Responsive::new()
            .sm("text-base")
            .md("text-lg")
            .lg("text-xl");
        
        assert_eq!(responsive.sm, Some("text-base".to_string()));
        assert_eq!(responsive.md, Some("text-lg".to_string()));
        assert_eq!(responsive.lg, Some("text-xl".to_string()));
    }

    #[test]
    fn test_responsive_build() {
        let responsive = Responsive::new()
            .sm("text-base")
            .md("text-lg")
            .lg("text-xl");
        
        let classes = responsive.build();
        assert!(classes.contains("sm:text-base"));
        assert!(classes.contains("md:text-lg"));
        assert!(classes.contains("lg:text-xl"));
    }

    #[test]
    fn test_breakpoint_properties() {
        assert_eq!(Breakpoint::Sm.min_width(), 640);
        assert_eq!(Breakpoint::Md.min_width(), 768);
        assert_eq!(Breakpoint::Lg.min_width(), 1024);
        
        assert_eq!(Breakpoint::Sm.prefix(), "sm");
        assert_eq!(Breakpoint::Md.prefix(), "md");
        assert_eq!(Breakpoint::Lg.prefix(), "lg");
    }

    #[test]
    fn test_responsive_builder() {
        let classes = ResponsiveBuilder::new()
            .base("text-sm")
            .breakpoint(Breakpoint::Sm, "text-base")
            .breakpoint(Breakpoint::Md, "text-lg")
            .build();
        
        assert!(classes.contains("text-sm"));
        assert!(classes.contains("sm:text-base"));
        assert!(classes.contains("md:text-lg"));
    }

    #[test]
    fn test_responsive_grid() {
        let classes = ResponsiveGrid::new()
            .cols(Breakpoint::Sm, 2)
            .cols(Breakpoint::Md, 3)
            .gap(Breakpoint::Sm, "gap-4")
            .gap(Breakpoint::Md, "gap-6")
            .build();
        
        assert!(classes.contains("grid"));
        assert!(classes.contains("grid-cols-2"));
        assert!(classes.contains("sm:grid-cols-2"));
        assert!(classes.contains("md:grid-cols-3"));
        assert!(classes.contains("gap-4"));
        assert!(classes.contains("sm:gap-4"));
        assert!(classes.contains("md:gap-6"));
    }

    #[test]
    fn test_responsive_flex() {
        let classes = ResponsiveFlex::new()
            .direction(Breakpoint::Sm, FlexDirection::Column)
            .direction(Breakpoint::Md, FlexDirection::Row)
            .justify(Breakpoint::Sm, JustifyContent::Center)
            .justify(Breakpoint::Md, JustifyContent::SpaceBetween)
            .build();
        
        assert!(classes.contains("flex"));
        assert!(classes.contains("flex-col"));
        assert!(classes.contains("sm:flex-col"));
        assert!(classes.contains("md:flex-row"));
        assert!(classes.contains("justify-center"));
        assert!(classes.contains("sm:justify-center"));
        assert!(classes.contains("md:justify-between"));
    }
}
```

### Integration Testing
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_responsive_system_integration() {
        let responsive = Responsive::new()
            .sm("text-base")
            .md("text-lg")
            .lg("text-xl");
        
        let grid = ResponsiveGrid::new()
            .cols(Breakpoint::Sm, 2)
            .cols(Breakpoint::Md, 3)
            .gap(Breakpoint::Sm, "gap-4");
        
        let flex = ResponsiveFlex::new()
            .direction(Breakpoint::Sm, FlexDirection::Column)
            .direction(Breakpoint::Md, FlexDirection::Row);
        
        let responsive_classes = responsive.build();
        let grid_classes = grid.build();
        let flex_classes = flex.build();
        
        assert!(!responsive_classes.is_empty());
        assert!(!grid_classes.is_empty());
        assert!(!flex_classes.is_empty());
    }
}
```

### End-to-End Testing with Playwright
```typescript
import { test, expect } from '@playwright/test';

test.describe('Responsive Design', () => {
  test('should adapt to different screen sizes', async ({ page }) => {
    await page.goto('/demo/responsive');
    
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('[data-testid="responsive-element"]'))
      .toHaveClass(/text-sm/);
    
    // Test tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.locator('[data-testid="responsive-element"]'))
      .toHaveClass(/md:text-lg/);
    
    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator('[data-testid="responsive-element"]'))
      .toHaveClass(/lg:text-xl/);
  });

  test('should display grid correctly on different screens', async ({ page }) => {
    await page.goto('/demo/responsive-grid');
    
    // Test mobile grid
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('[data-testid="grid-container"]'))
      .toHaveClass(/grid-cols-1/);
    
    // Test tablet grid
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.locator('[data-testid="grid-container"]'))
      .toHaveClass(/md:grid-cols-2/);
    
    // Test desktop grid
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator('[data-testid="grid-container"]'))
      .toHaveClass(/lg:grid-cols-3/);
  });

  test('should display flex layout correctly on different screens', async ({ page }) => {
    await page.goto('/demo/responsive-flex');
    
    // Test mobile flex
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('[data-testid="flex-container"]'))
      .toHaveClass(/flex-col/);
    
    // Test tablet flex
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.locator('[data-testid="flex-container"]'))
      .toHaveClass(/md:flex-row/);
  });
});
```

## 📚 Examples

### Basic Responsive Usage
```rust
use tailwind_rs::*;

// Simple responsive classes
let responsive = Responsive::new()
    .sm("text-base")
    .md("text-lg")
    .lg("text-xl");

let classes = responsive.build();
// Generates: "sm:text-base md:text-lg lg:text-xl"
```

### Advanced Responsive Usage
```rust
use tailwind_rs::*;

// Complex responsive layout
let responsive = Responsive::new()
    .sm("text-sm p-4")
    .md("text-base p-6")
    .lg("text-lg p-8")
    .xl("text-xl p-10");

let classes = responsive.build();
// Generates: "sm:text-sm sm:p-4 md:text-base md:p-6 lg:text-lg lg:p-8 xl:text-xl xl:p-10"
```

### Responsive Grid Usage
```rust
use tailwind_rs::*;

// Responsive grid layout
let grid = ResponsiveGrid::new()
    .cols(Breakpoint::Sm, 1)
    .cols(Breakpoint::Md, 2)
    .cols(Breakpoint::Lg, 3)
    .cols(Breakpoint::Xl, 4)
    .gap(Breakpoint::Sm, "gap-4")
    .gap(Breakpoint::Md, "gap-6")
    .gap(Breakpoint::Lg, "gap-8");

let classes = grid.build();
// Generates: "grid grid-cols-1 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 sm:gap-4 md:gap-6 lg:gap-8"
```

### Responsive Flex Usage
```rust
use tailwind_rs::*;

// Responsive flex layout
let flex = ResponsiveFlex::new()
    .direction(Breakpoint::Sm, FlexDirection::Column)
    .direction(Breakpoint::Md, FlexDirection::Row)
    .justify(Breakpoint::Sm, JustifyContent::Center)
    .justify(Breakpoint::Md, JustifyContent::SpaceBetween)
    .align(Breakpoint::Sm, AlignItems::Center)
    .align(Breakpoint::Md, AlignItems::Stretch)
    .gap(Breakpoint::Sm, "gap-4")
    .gap(Breakpoint::Md, "gap-6");

let classes = flex.build();
// Generates: "flex flex-col sm:flex-col md:flex-row justify-center sm:justify-center md:justify-between items-center sm:items-center md:items-stretch gap-4 sm:gap-4 md:gap-6"
```

### Responsive Builder Usage
```rust
use tailwind_rs::*;

// Using responsive builder
let classes = ResponsiveBuilder::new()
    .base("text-sm p-4")
    .breakpoint(Breakpoint::Sm, "text-base p-6")
    .breakpoint(Breakpoint::Md, "text-lg p-8")
    .breakpoint(Breakpoint::Lg, "text-xl p-10")
    .build();

// Generates: "text-sm p-4 sm:text-base sm:p-6 md:text-lg md:p-8 lg:text-xl lg:p-10"
```

---

This responsive design API reference provides comprehensive documentation for all responsive functionality in `tailwind-rs`. The responsive system is designed with flexibility, type safety, and performance in mind, following our established ADRs and best practices.

