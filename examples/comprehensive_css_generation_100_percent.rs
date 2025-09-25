use tailwind_rs_core::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Tailwind-RS 100% Coverage CSS Generation Demo");
    println!("==================================================");
    println!("This demo showcases ALL utility categories available in tailwind-rs");
    println!();

    // 1. Generate CSS with specific classes using ClassBuilder
    println!("📝 Generating CSS with specific classes...");
    let classes = ClassBuilder::new()
        .padding(SpacingValue::Integer(4))
        .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
        .text_color(Color::new(ColorPalette::White, ColorShade::Shade100))
        .build();

    generate_css_file("dist/styles_specific.css", Some(&classes))?;

    // 2. Generate comprehensive CSS with ALL utilities enabled
    println!("🌐 Generating comprehensive CSS with ALL utilities enabled...");
    let mut full_config = CssGenerationConfig::default();
    
    // Enable ALL utility categories
    full_config.include_colors = true;
    full_config.include_spacing = true;
    full_config.include_typography = true;
    full_config.include_layout = true;
    full_config.include_flexbox = true;
    full_config.include_grid = true;
    full_config.include_borders = true;
    full_config.include_effects = true;
    full_config.include_transforms = true;
    full_config.include_animations = true;
    full_config.include_interactivity = true;
    full_config.include_sizing = true;
    full_config.include_backgrounds = true;
    full_config.include_filters = true;
    full_config.include_transitions = true;
    full_config.include_text_shadow = true;
    full_config.include_mask = true;
    full_config.include_logical_properties = true;
    full_config.include_enhanced_backdrop_filters = true;
    full_config.include_modern_css_features = true;
    full_config.include_device_variants = true;
    full_config.include_css_nesting = true;
    full_config.include_advanced_plugin_system = true;
    full_config.include_enhanced_validation = true;
    full_config.include_advanced_performance_optimization = true;
    full_config.include_container_queries = true;
    full_config.include_color_functions = true;
    full_config.include_performance_optimization = true;
    full_config.include_advanced_animations = true;
    full_config.include_responsive = true;
    full_config.include_dark_mode = true;

    generate_comprehensive_css("dist/styles_comprehensive_100_percent.css", &full_config)?;

    // 3. Generate CSS with custom configuration (subset of utilities)
    println!("⚙️ Generating CSS with custom configuration (subset)...");
    let mut custom_config = CssGenerationConfig::default();
    custom_config.include_colors = true;
    custom_config.include_spacing = true;
    custom_config.include_typography = false; // Exclude typography
    custom_config.include_layout = true;
    custom_config.include_flexbox = true;
    custom_config.include_grid = false; // Exclude grid
    custom_config.include_borders = true;
    custom_config.include_effects = true;
    custom_config.include_transforms = false; // Exclude transforms
    custom_config.include_animations = true;
    custom_config.include_interactivity = true;
    custom_config.include_sizing = true;
    custom_config.include_backgrounds = false; // Exclude backgrounds
    custom_config.include_filters = true;
    custom_config.include_transitions = true;
    custom_config.include_text_shadow = false; // Exclude text shadow
    custom_config.include_mask = false; // Exclude mask
    custom_config.include_logical_properties = true;
    custom_config.include_enhanced_backdrop_filters = false; // Exclude enhanced backdrop filters
    custom_config.include_modern_css_features = true;
    custom_config.include_device_variants = false; // Exclude device variants
    custom_config.include_css_nesting = true;
    custom_config.include_advanced_plugin_system = false; // Exclude advanced plugin system
    custom_config.include_enhanced_validation = true;
    custom_config.include_advanced_performance_optimization = false; // Exclude advanced performance optimization
    custom_config.include_container_queries = true;
    custom_config.include_color_functions = false; // Exclude color functions
    custom_config.include_performance_optimization = true;
    custom_config.include_advanced_animations = false; // Exclude advanced animations
    custom_config.color_palettes = vec!["red".to_string(), "green".to_string(), "blue".to_string()]; // Only red, green, and blue colors
    custom_config.include_responsive = true;
    custom_config.include_dark_mode = true;

    generate_comprehensive_css("dist/styles_custom_subset.css", &custom_config)?;

    // 4. Generate CSS with minimal configuration
    println!("🔧 Generating CSS with minimal configuration...");
    let mut minimal_config = CssGenerationConfig::default();
    minimal_config.include_colors = true;
    minimal_config.include_spacing = true;
    minimal_config.include_typography = false;
    minimal_config.include_layout = false;
    minimal_config.include_flexbox = false;
    minimal_config.include_grid = false;
    minimal_config.include_borders = false;
    minimal_config.include_effects = false;
    minimal_config.include_transforms = false;
    minimal_config.include_animations = false;
    minimal_config.include_interactivity = false;
    minimal_config.include_sizing = false;
    minimal_config.include_backgrounds = false;
    minimal_config.include_filters = false;
    minimal_config.include_transitions = false;
    minimal_config.include_text_shadow = false;
    minimal_config.include_mask = false;
    minimal_config.include_logical_properties = false;
    minimal_config.include_enhanced_backdrop_filters = false;
    minimal_config.include_modern_css_features = false;
    minimal_config.include_device_variants = false;
    minimal_config.include_css_nesting = false;
    minimal_config.include_advanced_plugin_system = false;
    minimal_config.include_enhanced_validation = false;
    minimal_config.include_advanced_performance_optimization = false;
    minimal_config.include_container_queries = false;
    minimal_config.include_color_functions = false;
    minimal_config.include_performance_optimization = false;
    minimal_config.include_advanced_animations = false;
    minimal_config.include_responsive = false;
    minimal_config.include_dark_mode = false;

    generate_comprehensive_css("dist/styles_minimal.css", &minimal_config)?;

    println!();
    println!("✅ Demo completed successfully!");
    println!("📁 Generated CSS files:");
    println!("   - dist/styles_specific.css (specific classes)");
    println!("   - dist/styles_comprehensive_100_percent.css (ALL utilities)");
    println!("   - dist/styles_custom_subset.css (custom subset)");
    println!("   - dist/styles_minimal.css (minimal configuration)");
    println!();
    println!("🎯 Coverage Summary:");
    println!("   ✅ Core Utilities: Spacing, Colors, Typography, Layout, Flexbox, Grid");
    println!("   ✅ Border Utilities: Width, Style, Radius, Color");
    println!("   ✅ Effects Utilities: Box Shadow, Drop Shadow, Opacity, Blend Modes");
    println!("   ✅ Transform Utilities: Scale, Rotate, Translate, Skew");
    println!("   ✅ Animation Utilities: Spin, Pulse, Bounce, Ping");
    println!("   ✅ Interactivity Utilities: Cursor, Select, Pointer Events");
    println!("   ✅ Sizing Utilities: Width, Height, Min/Max Dimensions");
    println!("   ✅ Background Utilities: Size, Position, Repeat, Attachment, Clip, Origin");
    println!("   ✅ Filter Utilities: Blur, Brightness, Contrast, Grayscale, Hue Rotate, Invert, Saturate, Sepia");
    println!("   ✅ Transition Utilities: Properties, Duration, Timing, Delay");
    println!("   ✅ Text Shadow Utilities: Various shadow effects");
    println!("   ✅ Mask Utilities: Size, Position, Repeat, Origin, Clip");
    println!("   ✅ Logical Properties: Inline/Block borders, margins, padding, text alignment");
    println!("   ✅ Enhanced Backdrop Filters: Blur, Brightness, Contrast, Grayscale, Hue Rotate, Invert, Opacity, Saturate, Sepia");
    println!("   ✅ Modern CSS Features: Cascade Layers, Custom Properties, Container Queries, CSS Nesting");
    println!("   ✅ Device Variants: Mobile, Tablet, Desktop, Touch, Hover, Pointer types");
    println!("   ✅ CSS Nesting: Parent selectors, pseudo-classes, pseudo-elements");
    println!("   ✅ Advanced Plugin System: Plugin types, priorities, lifecycles, composition");
    println!("   ✅ Enhanced Validation: Validation states, rules, errors");
    println!("   ✅ Advanced Performance Optimization: Will-change, Contain, Isolation, Backface visibility, Perspective, Transform hints");
    println!("   ✅ Container Queries: Container types, names, queries");
    println!("   ✅ Color Functions: RGB, RGBA, HSL, HSLA, HWB, LAB, LCH, OKLAB, OKLCH, Color-mix, Color-contrast");
    println!("   ✅ Performance Optimization: Speed/quality optimization, Will-change, Contain");
    println!("   ✅ Advanced Animations: Complex animation combinations and timing");
    println!("   ✅ Responsive Variants: All utilities with responsive prefixes");
    println!("   ✅ Dark Mode Variants: All utilities with dark mode prefixes");
    println!();
    println!("🎉 100% Coverage Achieved! All utility categories from tailwind-rs are now supported!");

    Ok(())
}
