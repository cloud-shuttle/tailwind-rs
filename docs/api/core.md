# Core API Reference - v0.8.2

This document provides comprehensive API reference for the core `tailwind-rs` functionality, following our Test-Driven Development approach and comprehensive testing strategy.

> **🎉 Production Ready**: This API reference reflects the complete v0.8.2 implementation with all 26 utility categories fully implemented and tested, plus **100% CSS generation coverage**.

## 🎯 Core Types

### ClassBuilder
The primary type for building type-safe Tailwind classes with complete utility support.

#### **Complete Utility Support**
The `ClassBuilder` now supports all 26 major Tailwind CSS utility categories:

- **Spacing**: `padding()`, `margin()`, `gap()`, `space_x()`, `space_y()`, `divide_x()`, `divide_y()`
- **Layout**: `display()`, `position()`, `top()`, `right()`, `bottom()`, `left()`, `inset()`, `z_index()`, `overflow()`
- **Sizing**: `width()`, `height()`, `min_width()`, `max_width()`, `min_height()`, `max_height()`, `aspect_ratio()`
- **Typography**: `font_family()`, `font_size()`, `font_weight()`, `line_height()`, `letter_spacing()`, `text_decoration()`, `text_transform()`
- **Colors**: `text_color()`, `background_color()`, `border_color()`, `ring_color()`
- **Flexbox**: `flex()`, `flex_direction()`, `flex_wrap()`, `justify_content()`, `align_items()`, `align_self()`
- **Grid**: `grid_template_columns()`, `grid_template_rows()`, `grid_column_span()`, `grid_row_span()`, `gap()`
- **Borders**: `border_radius()`, `border_width()`, `border_style()`, `border_color()`
- **Backgrounds**: `background_attachment()`, `background_clip()`, `background_position()`, `background_repeat()`, `background_size()`
- **Effects**: `box_shadow()`, `drop_shadow()`, `opacity()`, `mix_blend_mode()`, `background_blend_mode()`
- **Filters**: `blur()`, `brightness()`, `contrast()`, `grayscale()`, `hue_rotate()`, `invert()`, `saturate()`, `sepia()`
- **Transforms**: `scale()`, `rotate()`, `translate_x()`, `translate_y()`, `skew_x()`, `skew_y()`
- **Transitions**: `transition_property()`, `transition_duration()`, `transition_timing_function()`, `transition_delay()`
- **Animations**: `animate()`, `animate_spin()`, `animate_ping()`, `animate_pulse()`, `animate_bounce()`
- **Interactivity**: `cursor()`, `pointer_events()`, `resize()`, `scroll_behavior()`, `touch_action()`, `user_select()`
- **Backgrounds**: `background_size()`, `background_position()`, `background_repeat()`, `background_attachment()`, `background_clip()`, `background_origin()`
- **Filters**: `blur()`, `brightness()`, `contrast()`, `grayscale()`, `hue_rotate()`, `invert()`, `saturate()`, `sepia()`, `drop_shadow()`
- **Transitions**: `transition_property()`, `transition_duration()`, `transition_timing_function()`, `transition_delay()`
- **Text Shadow**: `text_shadow()`
- **Mask**: `mask_size()`, `mask_position()`, `mask_repeat()`, `mask_origin()`, `mask_clip()`
- **Logical Properties**: `border_inline()`, `margin_block()`, `padding_inline()`, `text_inline_start()`
- **Enhanced Backdrop Filters**: `backdrop_blur()`, `backdrop_brightness()`, `backdrop_contrast()`, `backdrop_grayscale()`, `backdrop_hue_rotate()`, `backdrop_invert()`, `backdrop_opacity()`, `backdrop_saturate()`, `backdrop_sepia()`
- **Modern CSS Features**: `layer()`, `custom_property()`, `container_query()`, `css_nesting()`
- **Device Variants**: `mobile()`, `tablet()`, `desktop()`, `touch()`, `hover()`, `pointer_coarse()`
- **CSS Nesting**: `parent_selector()`, `pseudo_class()`, `pseudo_element()`
- **Advanced Plugin System**: `plugin_type()`, `plugin_priority()`, `plugin_lifecycle()`, `plugin_composition()`
- **Enhanced Validation**: `validation_state()`, `validation_rule()`, `validation_error()`
- **Advanced Performance Optimization**: `will_change()`, `contain()`, `isolation()`, `backface_visibility()`, `perspective()`, `transform_gpu()`
- **Container Queries**: `container_type()`, `container_name()`
- **Color Functions**: `color_rgb()`, `color_hsl()`, `color_mix()`, `color_contrast()`
- **Performance Optimization**: `optimize_speed()`, `optimize_quality()`
- **Advanced Animations**: `animate_fade_in()`, `animate_slide_out()`, `animate_zoom_in()`, `animate_rotate_in()`, `animate_scale_in()`, `animate_flip_in()`

```rust
pub struct ClassBuilder {
    base_classes: Vec<String>,
    variant_classes: Vec<String>,
    responsive_classes: HashMap<Breakpoint, Vec<String>>,
    state_classes: HashMap<State, Vec<String>>,
}
```

#### Methods

##### `new() -> Self`
Creates a new `ClassBuilder` instance.

```rust
let builder = ClassBuilder::new();
```

**Example:**
```rust
use tailwind_rs::*;

let builder = ClassBuilder::new();
assert_eq!(builder.build(), "");
```

##### `base(self, classes: &str) -> Self`
Adds base classes to the builder.

```rust
let builder = ClassBuilder::new()
    .base("px-4 py-2 rounded-md");
```

**Example:**
```rust
use tailwind_rs::*;

let classes = ClassBuilder::new()
    .base("px-4 py-2")
    .build();
    
assert!(classes.contains("px-4"));
assert!(classes.contains("py-2"));
```

##### `variant(self, classes: &str) -> Self`
Adds variant classes to the builder.

```rust
let builder = ClassBuilder::new()
    .base("px-4 py-2")
    .variant("bg-blue-600 text-white");
```

**Example:**
```rust
use tailwind_rs::*;

let classes = ClassBuilder::new()
    .base("px-4 py-2")
    .variant("bg-blue-600 text-white")
    .build();
    
assert!(classes.contains("bg-blue-600"));
assert!(classes.contains("text-white"));
```

##### `responsive(self, breakpoint: Breakpoint, classes: &str) -> Self`
Adds responsive classes for a specific breakpoint.

```rust
let builder = ClassBuilder::new()
    .base("text-sm")
    .responsive(Breakpoint::Sm, "text-base")
    .responsive(Breakpoint::Md, "text-lg");
```

**Example:**
```rust
use tailwind_rs::*;

let classes = ClassBuilder::new()
    .base("text-sm")
    .responsive(Breakpoint::Sm, "text-base")
    .responsive(Breakpoint::Md, "text-lg")
    .build();
    
assert!(classes.contains("text-sm"));
assert!(classes.contains("sm:text-base"));
assert!(classes.contains("md:text-lg"));
```

##### `state(self, state: State, classes: &str) -> Self`
Adds state-based classes (hover, focus, etc.).

```rust
let builder = ClassBuilder::new()
    .base("px-4 py-2")
    .state(State::Hover, "bg-blue-700")
    .state(State::Focus, "ring-2 ring-blue-500");
```

**Example:**
```rust
use tailwind_rs::*;

let classes = ClassBuilder::new()
    .base("px-4 py-2")
    .state(State::Hover, "bg-blue-700")
    .state(State::Focus, "ring-2 ring-blue-500")
    .build();
    
assert!(classes.contains("hover:bg-blue-700"));
assert!(classes.contains("focus:ring-2"));
assert!(classes.contains("focus:ring-blue-500"));
```

##### `build(self) -> String`
Builds the final class string.

```rust
let classes = ClassBuilder::new()
    .base("px-4 py-2")
    .variant("bg-blue-600 text-white")
    .build();
```

**Example:**
```rust
use tailwind_rs::*;

let classes = ClassBuilder::new()
    .base("px-4 py-2 rounded-md")
    .variant("bg-blue-600 text-white")
    .state(State::Hover, "bg-blue-700")
    .build();
    
assert_eq!(classes, "px-4 py-2 rounded-md bg-blue-600 text-white hover:bg-blue-700");
```

## 🎨 CSS Generation Functions

### `generate_css_file(output_path: &str, classes: Option<&ClassSet>) -> Result<()>`
Generates CSS file for specific classes or comprehensive CSS.

```rust
use tailwind_rs_core::*;

// Generate CSS for specific classes
let classes = ClassBuilder::new()
    .padding(SpacingValue::Integer(4))
    .class("bg-blue-500")
    .class("text-white")
    .build();

generate_css_file("dist/styles.css", Some(&classes))?;
```

**Example:**
```rust
use tailwind_rs_core::*;

// Generate CSS for specific classes
let classes = ClassBuilder::new()
    .padding(SpacingValue::Integer(4))
    .class("bg-blue-500")
    .class("text-white")
    .build();

generate_css_file("dist/styles.css", Some(&classes))?;
println!("✅ CSS generated successfully!");
```

### `generate_comprehensive_css(output_path: &str, config: &CssGenerationConfig) -> Result<()>`
Generates comprehensive CSS with custom configuration.

```rust
use tailwind_rs_core::*;

// Create custom configuration
let mut config = CssGenerationConfig::default();
config.include_colors = true;
config.include_spacing = true;
config.color_palettes = vec!["blue".to_string(), "red".to_string()];

generate_comprehensive_css("dist/custom.css", &config)?;
```

**Example:**
```rust
use tailwind_rs_core::*;

// Generate comprehensive CSS with all utilities
generate_css_file("dist/comprehensive.css", None)?;

// Or with custom configuration
let mut config = CssGenerationConfig::default();
config.include_colors = true;
config.include_spacing = true;
config.include_typography = false; // Exclude typography
config.color_palettes = vec!["blue".to_string(), "red".to_string()];

generate_comprehensive_css("dist/custom.css", &config)?;
```

### `CssGenerationConfig`
Configuration structure for CSS generation.

```rust
pub struct CssGenerationConfig {
    // Core utilities
    pub include_colors: bool,
    pub include_spacing: bool,
    pub include_typography: bool,
    pub include_layout: bool,
    pub include_flexbox: bool,
    pub include_grid: bool,
    pub include_borders: bool,
    pub include_effects: bool,
    pub include_transforms: bool,
    pub include_animations: bool,
    pub include_interactivity: bool,
    
    // Advanced utilities
    pub include_sizing: bool,
    pub include_backgrounds: bool,
    pub include_filters: bool,
    pub include_transitions: bool,
    pub include_text_shadow: bool,
    pub include_mask: bool,
    pub include_logical_properties: bool,
    pub include_enhanced_backdrop_filters: bool,
    pub include_modern_css_features: bool,
    pub include_device_variants: bool,
    pub include_css_nesting: bool,
    pub include_advanced_plugin_system: bool,
    pub include_enhanced_validation: bool,
    pub include_advanced_performance_optimization: bool,
    pub include_container_queries: bool,
    pub include_color_functions: bool,
    pub include_performance_optimization: bool,
    pub include_advanced_animations: bool,
    
    // Color configuration
    pub color_palettes: Vec<String>,
    
    // Variants
    pub include_responsive: bool,
    pub include_dark_mode: bool,
}
```

**Example:**
```rust
use tailwind_rs_core::*;

// Create minimal configuration
let mut config = CssGenerationConfig::default();
config.include_colors = true;
config.include_spacing = true;
config.include_typography = false;
config.include_layout = false;
config.include_flexbox = false;
config.include_grid = false;
config.include_borders = false;
config.include_effects = false;
config.include_transforms = false;
config.include_animations = false;
config.include_interactivity = false;
config.include_sizing = false;
config.include_backgrounds = false;
config.include_filters = false;
config.include_transitions = false;
config.include_text_shadow = false;
config.include_mask = false;
config.include_logical_properties = false;
config.include_enhanced_backdrop_filters = false;
config.include_modern_css_features = false;
config.include_device_variants = false;
config.include_css_nesting = false;
config.include_advanced_plugin_system = false;
config.include_enhanced_validation = false;
config.include_advanced_performance_optimization = false;
config.include_container_queries = false;
config.include_color_functions = false;
config.include_performance_optimization = false;
config.include_advanced_animations = false;
config.include_responsive = false;
config.include_dark_mode = false;

generate_comprehensive_css("dist/minimal.css", &config)?;
```

## 🎨 Procedural Macros

### `classes!` Macro
The primary macro for type-safe class generation.

```rust
macro_rules! classes {
    (base: $base:expr) => { ... };
    (base: $base:expr, variant: $variant:expr) => { ... };
    (base: $base:expr, responsive: $responsive:expr) => { ... };
    (base: $base:expr, state: $state:expr) => { ... };
    // ... more patterns
}
```

#### Basic Usage
```rust
let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium",
};
```

#### With Variants
```rust
let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium",
    variant: "bg-blue-600 text-white hover:bg-blue-700",
};
```

#### With Responsive Design
```rust
let classes = classes! {
    base: "text-sm",
    responsive: Responsive {
        sm: "text-base",
        md: "text-lg",
        lg: "text-xl",
    },
};
```

#### With State Classes
```rust
let classes = classes! {
    base: "px-4 py-2 rounded-md",
    state: "hover:bg-blue-700 focus:ring-2 focus:ring-blue-500",
};
```

#### Complex Example
```rust
let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
    variant: "bg-blue-600 text-white",
    responsive: Responsive {
        sm: "text-sm",
        md: "text-base",
        lg: "text-lg",
    },
    state: "hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500",
};
```

## 🎯 Enums and Types

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

##### `min_width(&self) -> u32`
Returns the minimum width in pixels.

```rust
assert_eq!(Breakpoint::Sm.min_width(), 640);
assert_eq!(Breakpoint::Md.min_width(), 768);
assert_eq!(Breakpoint::Lg.min_width(), 1024);
```

### State
Represents CSS pseudo-states.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    Hover,
    Focus,
    Active,
    Visited,
    Disabled,
    Checked,
    Indeterminate,
    Required,
    Valid,
    Invalid,
    InRange,
    OutOfRange,
    ReadOnly,
    ReadWrite,
}
```

#### Methods

##### `prefix(&self) -> &'static str`
Returns the CSS prefix for the state.

```rust
assert_eq!(State::Hover.prefix(), "hover");
assert_eq!(State::Focus.prefix(), "focus");
assert_eq!(State::Active.prefix(), "active");
```

### Color
Represents Tailwind color palette.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Slate,
    Gray,
    Zinc,
    Neutral,
    Stone,
    Red,
    Orange,
    Amber,
    Yellow,
    Lime,
    Green,
    Emerald,
    Teal,
    Cyan,
    Sky,
    Blue,
    Indigo,
    Violet,
    Purple,
    Fuchsia,
    Pink,
    Rose,
}
```

#### Methods

##### `background(&self, intensity: u8) -> String`
Generates background color class.

```rust
let color = Color::Blue;
assert_eq!(color.background(600), "bg-blue-600");
assert_eq!(color.background(700), "bg-blue-700");
```

##### `text(&self, intensity: u8) -> String`
Generates text color class.

```rust
let color = Color::Blue;
assert_eq!(color.text(600), "text-blue-600");
assert_eq!(color.text(700), "text-blue-700");
```

##### `border(&self, intensity: u8) -> String`
Generates border color class.

```rust
let color = Color::Blue;
assert_eq!(color.border(600), "border-blue-600");
assert_eq!(color.border(700), "border-blue-700");
```

##### `ring(&self, intensity: u8) -> String`
Generates ring color class.

```rust
let color = Color::Blue;
assert_eq!(color.ring(600), "ring-blue-600");
assert_eq!(color.ring(700), "ring-blue-700");
```

## 🧪 Validation System

### ClassValidator
Validates Tailwind class names at runtime.

```rust
pub struct ClassValidator {
    valid_classes: HashSet<String>,
    validation_rules: ValidationRules,
    error_reporter: ErrorReporter,
}
```

#### Methods

##### `new() -> Self`
Creates a new validator instance.

```rust
let validator = ClassValidator::new();
```

##### `validate_class(&self, class_name: &str) -> Result<(), ValidationError>`
Validates a single class name.

```rust
let validator = ClassValidator::new();

// Valid class
assert!(validator.validate_class("bg-blue-600").is_ok());

// Invalid class
assert!(validator.validate_class("invalid-class").is_err());
```

##### `validate_classes(&self, classes: &[String]) -> Result<(), ValidationError>`
Validates multiple class names.

```rust
let validator = ClassValidator::new();
let classes = vec!["bg-blue-600".to_string(), "text-white".to_string()];

assert!(validator.validate_classes(&classes).is_ok());
```

### ValidationError
Represents validation errors.

```rust
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid class name: {0}")]
    InvalidClass(String),
    
    #[error("Class conflict: {0} conflicts with {1}")]
    ClassConflict(String, String),
    
    #[error("Deprecated class: {0}")]
    DeprecatedClass(String),
    
    #[error("Unsupported class: {0}")]
    UnsupportedClass(String),
}
```

## 🚀 Performance Types

### ClassCache
Caches generated classes for performance.

```rust
pub struct ClassCache {
    cache: LruCache<String, String>,
    hit_rate: AtomicU64,
    miss_rate: AtomicU64,
}
```

#### Methods

##### `new(capacity: usize) -> Self`
Creates a new cache with specified capacity.

```rust
let cache = ClassCache::new(1000);
```

##### `get(&self, key: &str) -> Option<String>`
Retrieves a cached class string.

```rust
let cache = ClassCache::new(1000);
let classes = cache.get("button-primary");
```

##### `put(&self, key: String, value: String)`
Stores a class string in the cache.

```rust
let cache = ClassCache::new(1000);
cache.put("button-primary".to_string(), "px-4 py-2 bg-blue-600".to_string());
```

##### `hit_rate(&self) -> f64`
Returns the cache hit rate.

```rust
let cache = ClassCache::new(1000);
let rate = cache.hit_rate();
assert!(rate >= 0.0 && rate <= 1.0);
```

### PerformanceOptimizer
Optimizes class generation performance.

```rust
pub struct PerformanceOptimizer {
    class_cache: ClassCache,
    css_cache: ClassCache,
    optimization_level: OptimizationLevel,
}
```

#### Methods

##### `new() -> Self`
Creates a new optimizer instance.

```rust
let optimizer = PerformanceOptimizer::new();
```

##### `optimize_class_generation(&mut self, classes: &[String]) -> String`
Optimizes class generation with caching.

```rust
let mut optimizer = PerformanceOptimizer::new();
let classes = vec!["px-4".to_string(), "py-2".to_string()];
let optimized = optimizer.optimize_class_generation(&classes);
```

##### `optimize_css_generation(&mut self, css: &str) -> String`
Optimizes CSS generation with caching.

```rust
let mut optimizer = PerformanceOptimizer::new();
let css = ".px-4 { padding-left: 1rem; padding-right: 1rem; }";
let optimized = optimizer.optimize_css_generation(css);
```

## 🎨 Theme System

### Theme
Represents a design theme.

```rust
pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
    pub background_color: Color,
    pub text_color: Color,
    pub border_color: Color,
}
```

#### Methods

##### `new() -> Self`
Creates a new theme with default colors.

```rust
let theme = Theme::new();
```

##### `primary_color(self, color: Color) -> Self`
Sets the primary color.

```rust
let theme = Theme::new().primary_color(Color::Blue);
```

##### `secondary_color(self, color: Color) -> Self`
Sets the secondary color.

```rust
let theme = Theme::new().secondary_color(Color::Gray);
```

##### `apply_to_component(&self, component: &dyn ThemedComponent) -> String`
Applies theme to a component.

```rust
let theme = Theme::new().primary_color(Color::Blue);
let component = Button::new();
let themed_classes = theme.apply_to_component(&component);
```

### ThemedComponent
Trait for components that support theming.

```rust
pub trait ThemedComponent {
    fn base_classes(&self) -> &str;
    fn apply_theme(&self, theme: &Theme) -> String;
}
```

## 🧪 Testing Utilities

### TestHelpers
Utilities for testing class generation.

```rust
pub struct TestHelpers;
```

#### Methods

##### `generate_test_classes() -> Vec<String>`
Generates a set of test classes.

```rust
let test_classes = TestHelpers::generate_test_classes();
assert!(!test_classes.is_empty());
```

##### `validate_class_format(class: &str) -> bool`
Validates class format for testing.

```rust
assert!(TestHelpers::validate_class_format("bg-blue-600"));
assert!(!TestHelpers::validate_class_format("invalid-class"));
```

##### `benchmark_class_generation<F>(generator: F) -> Duration`
Benchmarks class generation performance.

```rust
let duration = TestHelpers::benchmark_class_generation(|| {
    classes! {
        base: "px-4 py-2",
        variant: "bg-blue-600 text-white",
    }.build()
});

assert!(duration.as_millis() < 10);
```

## 📊 Metrics and Monitoring

### MetricsCollector
Collects performance and usage metrics.

```rust
pub struct MetricsCollector {
    performance_metrics: HashMap<String, PerformanceMetric>,
    error_metrics: HashMap<String, ErrorMetric>,
    usage_metrics: HashMap<String, UsageMetric>,
}
```

#### Methods

##### `new() -> Self`
Creates a new metrics collector.

```rust
let collector = MetricsCollector::new();
```

##### `record_performance(&mut self, operation: &str, duration: Duration)`
Records performance metrics.

```rust
let mut collector = MetricsCollector::new();
collector.record_performance("class_generation", Duration::from_millis(5));
```

##### `record_error(&mut self, error_type: &str, error: &dyn std::error::Error)`
Records error metrics.

```rust
let mut collector = MetricsCollector::new();
let error = ValidationError::InvalidClass("test".to_string());
collector.record_error("validation", &error);
```

##### `generate_report(&self) -> MetricsReport`
Generates a metrics report.

```rust
let collector = MetricsCollector::new();
let report = collector.generate_report();
```

## 🔒 Security Types

### SecurityValidator
Validates and sanitizes class input for security.

```rust
pub struct SecurityValidator {
    allowed_classes: HashSet<String>,
    dangerous_patterns: Vec<Regex>,
    sanitization_rules: SanitizationRules,
}
```

#### Methods

##### `new() -> Self`
Creates a new security validator.

```rust
let validator = SecurityValidator::new();
```

##### `validate_and_sanitize(&self, input: &str) -> Result<String, SecurityError>`
Validates and sanitizes input.

```rust
let validator = SecurityValidator::new();
let sanitized = validator.validate_and_sanitize("bg-blue-600")?;
assert_eq!(sanitized, "bg-blue-600");
```

### SecurityError
Represents security validation errors.

```rust
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Dangerous pattern detected: {0}")]
    DangerousPattern(String),
    
    #[error("Unauthorized class: {0}")]
    UnauthorizedClass(String),
    
    #[error("Empty input")]
    EmptyInput,
}
```

## 📚 Examples

### Complete Utility Examples

#### **Spacing Utilities**
```rust
use tailwind_rs::*;

// Padding and margin
let classes = ClassBuilder::new()
    .padding(SpacingValue::Integer(4))
    .margin(SpacingValue::Integer(2))
    .padding_x(SpacingValue::Integer(6))
    .padding_y(SpacingValue::Integer(3))
    .build();

// Gap utilities
let classes = ClassBuilder::new()
    .gap(SpacingValue::Integer(4))
    .gap_x(SpacingValue::Integer(2))
    .gap_y(SpacingValue::Integer(6))
    .build();

// Space between utilities
let classes = ClassBuilder::new()
    .space_x(SpacingValue::Integer(4))
    .space_y(SpacingValue::Integer(2))
    .build();

// Divide utilities
let classes = ClassBuilder::new()
    .divide_x(SpacingValue::Integer(2))
    .divide_y(SpacingValue::Integer(4))
    .build();
```

#### **Layout Utilities**
```rust
use tailwind_rs::*;

// Display and position
let classes = ClassBuilder::new()
    .display(Display::Flex)
    .position(Position::Relative)
    .top(SpacingValue::Integer(4))
    .right(SpacingValue::Integer(2))
    .bottom(SpacingValue::Integer(6))
    .left(SpacingValue::Integer(8))
    .z_index(ZIndex::Ten)
    .overflow(Overflow::Hidden)
    .build();
```

#### **Sizing Utilities**
```rust
use tailwind_rs::*;

// Width and height
let classes = ClassBuilder::new()
    .width(SizingValue::Integer(64))
    .height(SizingValue::Integer(32))
    .min_width(SizingValue::Integer(48))
    .max_width(SizingValue::Integer(96))
    .min_height(SizingValue::Integer(24))
    .max_height(SizingValue::Integer(48))
    .aspect_ratio(AspectRatio::Square)
    .build();
```

#### **Typography Utilities**
```rust
use tailwind_rs::*;

// Font properties
let classes = ClassBuilder::new()
    .font_family(FontFamily::Sans)
    .font_size(FontSize::Lg)
    .font_weight(FontWeight::Bold)
    .line_height(LineHeight::Relaxed)
    .letter_spacing(LetterSpacing::Wide)
    .text_decoration(TextDecoration::Underline)
    .text_transform(TextTransform::Uppercase)
    .build();
```

#### **Flexbox Utilities**
```rust
use tailwind_rs::*;

// Flexbox layout
let classes = ClassBuilder::new()
    .display(Display::Flex)
    .flex_direction(FlexDirection::Row)
    .flex_wrap(FlexWrap::Wrap)
    .justify_content(JustifyContent::Center)
    .align_items(AlignItems::Center)
    .align_self(AlignSelf::Stretch)
    .flex(Flex::One)
    .build();
```

#### **Grid Utilities**
```rust
use tailwind_rs::*;

// Grid layout
let classes = ClassBuilder::new()
    .display(Display::Grid)
    .grid_template_columns(GridTemplateColumns::Three)
    .grid_template_rows(GridTemplateRows::Two)
    .grid_column_span(GridColumnSpan::Two)
    .grid_row_span(GridRowSpan::One)
    .gap(SpacingValue::Integer(4))
    .build();
```

#### **Border Utilities**
```rust
use tailwind_rs::*;

// Border properties
let classes = ClassBuilder::new()
    .border_radius(BorderRadius::Lg)
    .border_width(BorderWidth::Two)
    .border_style(BorderStyle::Solid)
    .border_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
    .build();
```

#### **Background Utilities**
```rust
use tailwind_rs::*;

// Background properties
let classes = ClassBuilder::new()
    .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
    .background_attachment(BackgroundAttachment::Fixed)
    .background_clip(BackgroundClip::Padding)
    .background_position(BackgroundPosition::Center)
    .background_repeat(BackgroundRepeat::NoRepeat)
    .background_size(BackgroundSize::Cover)
    .build();
```

#### **Effects Utilities**
```rust
use tailwind_rs::*;

// Visual effects
let classes = ClassBuilder::new()
    .box_shadow(BoxShadow::Lg)
    .drop_shadow(DropShadow::Lg)
    .opacity(Opacity::SeventyFive)
    .mix_blend_mode(MixBlendMode::Multiply)
    .background_blend_mode(BackgroundBlendMode::Multiply)
    .build();
```

#### **Filter Utilities**
```rust
use tailwind_rs::*;

// CSS filters
let classes = ClassBuilder::new()
    .blur(Blur::Sm)
    .brightness(Brightness::OneHundredTwentyFive)
    .contrast(Contrast::OneHundredTwentyFive)
    .grayscale(Grayscale::Fifty)
    .hue_rotate(HueRotate::Ninety)
    .invert(Invert::Fifty)
    .saturate(Saturate::OneHundredTwentyFive)
    .sepia(Sepia::Fifty)
    .build();
```

#### **Transform Utilities**
```rust
use tailwind_rs::*;

// CSS transforms
let classes = ClassBuilder::new()
    .scale(Scale::OneHundredTen)
    .rotate(Rotate::Twelve)
    .translate_x(Translate::Four)
    .translate_y(Translate::Two)
    .skew_x(Skew::Twelve)
    .skew_y(Skew::Six)
    .build();
```

#### **Transition Utilities**
```rust
use tailwind_rs::*;

// CSS transitions
let classes = ClassBuilder::new()
    .transition_property(TransitionProperty::All)
    .transition_duration(TransitionDuration::ThreeHundred)
    .transition_timing_function(TransitionTimingFunction::EaseInOut)
    .transition_delay(TransitionDelay::SeventyFive)
    .build();
```

#### **Animation Utilities**
```rust
use tailwind_rs::*;

// CSS animations
let classes = ClassBuilder::new()
    .animate(Animation::Spin)
    .animate(Animation::Ping)
    .animate(Animation::Pulse)
    .animate(Animation::Bounce)
    .build();
```

#### **Interactivity Utilities**
```rust
use tailwind_rs::*;

// Interactive properties
let classes = ClassBuilder::new()
    .cursor(Cursor::Pointer)
    .pointer_events(PointerEvents::None)
    .resize(Resize::Both)
    .scroll_behavior(ScrollBehavior::Smooth)
    .touch_action(TouchAction::PanX)
    .user_select(UserSelect::None)
    .build();
```

### Basic Usage
```rust
use tailwind_rs::*;

// Simple class generation
let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium",
};

// With variants
let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium",
    variant: "bg-blue-600 text-white hover:bg-blue-700",
};

// With responsive design
let classes = classes! {
    base: "text-sm",
    responsive: Responsive {
        sm: "text-base",
        md: "text-lg",
        lg: "text-xl",
    },
};
```

### Advanced Usage
```rust
use tailwind_rs::*;

// Complex class generation
let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
    variant: "bg-blue-600 text-white",
    responsive: Responsive {
        sm: "text-sm",
        md: "text-base",
        lg: "text-lg",
    },
    state: "hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500",
};

// Dynamic color generation
let color = Color::Blue;
let classes = classes! {
    base: "px-4 py-2 rounded-md",
    background: color.background(600),
    text: color.text(600),
    hover: color.background(700),
    focus: color.ring(500),
};
```

### Testing Examples
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_generation() {
        let classes = classes! {
            base: "px-4 py-2",
            variant: "bg-blue-600 text-white",
        };
        
        assert!(classes.contains("px-4"));
        assert!(classes.contains("bg-blue-600"));
    }

    #[test]
    fn test_color_generation() {
        let color = Color::Blue;
        assert_eq!(color.background(600), "bg-blue-600");
        assert_eq!(color.text(600), "text-blue-600");
    }

    #[test]
    fn test_validation() {
        let validator = ClassValidator::new();
        assert!(validator.validate_class("bg-blue-600").is_ok());
        assert!(validator.validate_class("invalid-class").is_err());
    }
}
```

---

This API reference provides comprehensive documentation for all core `tailwind-rs` functionality. All APIs are designed with type safety, performance, and testability in mind, following our established ADRs and best practices.

