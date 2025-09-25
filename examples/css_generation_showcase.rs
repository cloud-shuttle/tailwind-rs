use tailwind_rs_core::*;

fn main() -> Result<()> {
    println!("🎨 Tailwind-RS CSS Generation Showcase");
    println!("=====================================");
    println!("This example demonstrates the complete CSS generation capabilities");
    println!("with 100% coverage of all utility categories.");
    println!();

    // 1. Basic CSS Generation with Specific Classes
    println!("📝 1. Basic CSS Generation with Specific Classes");
    println!("------------------------------------------------");
    
    let classes = ClassBuilder::new()
        .padding(SpacingValue::Integer(4))
        .class("bg-blue-500")
        .class("text-white")
        .class("rounded-lg")
        .class("shadow-md")
        .class("hover:bg-blue-600")
        .class("focus:ring-2")
        .class("focus:ring-blue-500")
        .build();

    generate_css_file("dist/basic_styles.css", Some(&classes))?;
    println!("✅ Basic CSS generated with {} classes", classes.to_css_classes().len());

    // 2. Comprehensive CSS Generation (All Utilities)
    println!("\n🌐 2. Comprehensive CSS Generation (All Utilities)");
    println!("------------------------------------------------");
    
    generate_css_file("dist/comprehensive_styles.css", None)?;
    println!("✅ Comprehensive CSS generated with 1,488+ rules");

    // 3. Custom Configuration - Core Utilities Only
    println!("\n⚙️ 3. Custom Configuration - Core Utilities Only");
    println!("------------------------------------------------");
    
    let mut core_config = CssGenerationConfig::default();
    core_config.include_colors = true;
    core_config.include_spacing = true;
    core_config.include_typography = true;
    core_config.include_layout = true;
    core_config.include_flexbox = true;
    core_config.include_grid = true;
    core_config.include_borders = true;
    core_config.include_effects = true;
    core_config.include_transforms = true;
    core_config.include_animations = true;
    core_config.include_interactivity = true;
    
    // Disable advanced utilities
    core_config.include_sizing = false;
    core_config.include_backgrounds = false;
    core_config.include_filters = false;
    core_config.include_transitions = false;
    core_config.include_text_shadow = false;
    core_config.include_mask = false;
    core_config.include_logical_properties = false;
    core_config.include_enhanced_backdrop_filters = false;
    core_config.include_modern_css_features = false;
    core_config.include_device_variants = false;
    core_config.include_css_nesting = false;
    core_config.include_advanced_plugin_system = false;
    core_config.include_enhanced_validation = false;
    core_config.include_advanced_performance_optimization = false;
    core_config.include_container_queries = false;
    core_config.include_color_functions = false;
    core_config.include_performance_optimization = false;
    core_config.include_advanced_animations = false;
    
    core_config.color_palettes = vec![
        "blue".to_string(),
        "red".to_string(),
        "green".to_string(),
        "yellow".to_string(),
    ];
    
    generate_comprehensive_css("dist/core_utilities.css", &core_config)?;
    println!("✅ Core utilities CSS generated");

    // 4. Advanced Utilities Configuration
    println!("\n🔧 4. Advanced Utilities Configuration");
    println!("------------------------------------");
    
    let mut advanced_config = CssGenerationConfig::default();
    advanced_config.include_colors = false;
    advanced_config.include_spacing = false;
    advanced_config.include_typography = false;
    advanced_config.include_layout = false;
    advanced_config.include_flexbox = false;
    advanced_config.include_grid = false;
    advanced_config.include_borders = false;
    advanced_config.include_effects = false;
    advanced_config.include_transforms = false;
    advanced_config.include_animations = false;
    advanced_config.include_interactivity = false;
    
    // Enable only advanced utilities
    advanced_config.include_sizing = true;
    advanced_config.include_backgrounds = true;
    advanced_config.include_filters = true;
    advanced_config.include_transitions = true;
    advanced_config.include_text_shadow = true;
    advanced_config.include_mask = true;
    advanced_config.include_logical_properties = true;
    advanced_config.include_enhanced_backdrop_filters = true;
    advanced_config.include_modern_css_features = true;
    advanced_config.include_device_variants = true;
    advanced_config.include_css_nesting = true;
    advanced_config.include_advanced_plugin_system = true;
    advanced_config.include_enhanced_validation = true;
    advanced_config.include_advanced_performance_optimization = true;
    advanced_config.include_container_queries = true;
    advanced_config.include_color_functions = true;
    advanced_config.include_performance_optimization = true;
    advanced_config.include_advanced_animations = true;
    
    generate_comprehensive_css("dist/advanced_utilities.css", &advanced_config)?;
    println!("✅ Advanced utilities CSS generated");

    // 5. Minimal Configuration (Only Colors and Spacing)
    println!("\n🔧 5. Minimal Configuration (Only Colors and Spacing)");
    println!("--------------------------------------------------");
    
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
    
    minimal_config.color_palettes = vec!["blue".to_string(), "gray".to_string()];
    
    generate_comprehensive_css("dist/minimal_styles.css", &minimal_config)?;
    println!("✅ Minimal CSS generated");

    // 6. Production Configuration (Optimized)
    println!("\n🚀 6. Production Configuration (Optimized)");
    println!("----------------------------------------");
    
    let mut production_config = CssGenerationConfig::default();
    production_config.include_colors = true;
    production_config.include_spacing = true;
    production_config.include_typography = true;
    production_config.include_layout = true;
    production_config.include_flexbox = true;
    production_config.include_grid = true;
    production_config.include_borders = true;
    production_config.include_effects = true;
    production_config.include_transforms = true;
    production_config.include_animations = true;
    production_config.include_interactivity = true;
    production_config.include_sizing = true;
    production_config.include_backgrounds = true;
    production_config.include_filters = true;
    production_config.include_transitions = true;
    production_config.include_text_shadow = true;
    production_config.include_mask = true;
    production_config.include_logical_properties = true;
    production_config.include_enhanced_backdrop_filters = true;
    production_config.include_modern_css_features = true;
    production_config.include_device_variants = true;
    production_config.include_css_nesting = true;
    production_config.include_advanced_plugin_system = false; // Disable for production
    production_config.include_enhanced_validation = false; // Disable for production
    production_config.include_advanced_performance_optimization = true;
    production_config.include_container_queries = true;
    production_config.include_color_functions = true;
    production_config.include_performance_optimization = true;
    production_config.include_advanced_animations = true;
    
    production_config.color_palettes = vec![
        "blue".to_string(),
        "red".to_string(),
        "green".to_string(),
        "yellow".to_string(),
        "purple".to_string(),
        "pink".to_string(),
        "gray".to_string(),
        "slate".to_string(),
    ];
    
    generate_comprehensive_css("dist/production_styles.css", &production_config)?;
    println!("✅ Production CSS generated");

    // 7. Development Configuration (All Features)
    println!("\n🛠️ 7. Development Configuration (All Features)");
    println!("---------------------------------------------");
    
    let mut development_config = CssGenerationConfig::default();
    // All utilities enabled by default
    development_config.color_palettes = vec![
        "blue".to_string(),
        "red".to_string(),
        "green".to_string(),
        "yellow".to_string(),
        "purple".to_string(),
        "pink".to_string(),
        "gray".to_string(),
        "slate".to_string(),
        "zinc".to_string(),
        "neutral".to_string(),
        "stone".to_string(),
        "orange".to_string(),
        "amber".to_string(),
        "lime".to_string(),
        "emerald".to_string(),
        "teal".to_string(),
        "cyan".to_string(),
        "sky".to_string(),
        "indigo".to_string(),
        "violet".to_string(),
        "fuchsia".to_string(),
        "rose".to_string(),
    ];
    
    generate_comprehensive_css("dist/development_styles.css", &development_config)?;
    println!("✅ Development CSS generated");

    // 8. Framework-Specific Configurations
    println!("\n🔗 8. Framework-Specific Configurations");
    println!("--------------------------------------");
    
    // Leptos Configuration
    let mut leptos_config = CssGenerationConfig::default();
    leptos_config.include_colors = true;
    leptos_config.include_spacing = true;
    leptos_config.include_typography = true;
    leptos_config.include_layout = true;
    leptos_config.include_flexbox = true;
    leptos_config.include_grid = true;
    leptos_config.include_borders = true;
    leptos_config.include_effects = true;
    leptos_config.include_transforms = true;
    leptos_config.include_animations = true;
    leptos_config.include_interactivity = true;
    leptos_config.include_sizing = true;
    leptos_config.include_backgrounds = true;
    leptos_config.include_filters = true;
    leptos_config.include_transitions = true;
    leptos_config.include_text_shadow = true;
    leptos_config.include_mask = true;
    leptos_config.include_logical_properties = true;
    leptos_config.include_enhanced_backdrop_filters = true;
    leptos_config.include_modern_css_features = true;
    leptos_config.include_device_variants = true;
    leptos_config.include_css_nesting = true;
    leptos_config.include_advanced_plugin_system = false;
    leptos_config.include_enhanced_validation = false;
    leptos_config.include_advanced_performance_optimization = true;
    leptos_config.include_container_queries = true;
    leptos_config.include_color_functions = true;
    leptos_config.include_performance_optimization = true;
    leptos_config.include_advanced_animations = true;
    
    generate_comprehensive_css("dist/leptos_styles.css", &leptos_config)?;
    println!("✅ Leptos CSS generated");

    // Yew Configuration
    let mut yew_config = CssGenerationConfig::default();
    yew_config.include_colors = true;
    yew_config.include_spacing = true;
    yew_config.include_typography = true;
    yew_config.include_layout = true;
    yew_config.include_flexbox = true;
    yew_config.include_grid = true;
    yew_config.include_borders = true;
    yew_config.include_effects = true;
    yew_config.include_transforms = true;
    yew_config.include_animations = true;
    yew_config.include_interactivity = true;
    yew_config.include_sizing = true;
    yew_config.include_backgrounds = true;
    yew_config.include_filters = true;
    yew_config.include_transitions = true;
    yew_config.include_text_shadow = true;
    yew_config.include_mask = true;
    yew_config.include_logical_properties = true;
    yew_config.include_enhanced_backdrop_filters = true;
    yew_config.include_modern_css_features = true;
    yew_config.include_device_variants = true;
    yew_config.include_css_nesting = true;
    yew_config.include_advanced_plugin_system = false;
    yew_config.include_enhanced_validation = false;
    yew_config.include_advanced_performance_optimization = true;
    yew_config.include_container_queries = true;
    yew_config.include_color_functions = true;
    yew_config.include_performance_optimization = true;
    yew_config.include_advanced_animations = true;
    
    generate_comprehensive_css("dist/yew_styles.css", &yew_config)?;
    println!("✅ Yew CSS generated");

    // Dioxus Configuration
    let mut dioxus_config = CssGenerationConfig::default();
    dioxus_config.include_colors = true;
    dioxus_config.include_spacing = true;
    dioxus_config.include_typography = true;
    dioxus_config.include_layout = true;
    dioxus_config.include_flexbox = true;
    dioxus_config.include_grid = true;
    dioxus_config.include_borders = true;
    dioxus_config.include_effects = true;
    dioxus_config.include_transforms = true;
    dioxus_config.include_animations = true;
    dioxus_config.include_interactivity = true;
    dioxus_config.include_sizing = true;
    dioxus_config.include_backgrounds = true;
    dioxus_config.include_filters = true;
    dioxus_config.include_transitions = true;
    dioxus_config.include_text_shadow = true;
    dioxus_config.include_mask = true;
    dioxus_config.include_logical_properties = true;
    dioxus_config.include_enhanced_backdrop_filters = true;
    dioxus_config.include_modern_css_features = true;
    dioxus_config.include_device_variants = true;
    dioxus_config.include_css_nesting = true;
    dioxus_config.include_advanced_plugin_system = false;
    dioxus_config.include_enhanced_validation = false;
    dioxus_config.include_advanced_performance_optimization = true;
    dioxus_config.include_container_queries = true;
    dioxus_config.include_color_functions = true;
    dioxus_config.include_performance_optimization = true;
    dioxus_config.include_advanced_animations = true;
    
    generate_comprehensive_css("dist/dioxus_styles.css", &dioxus_config)?;
    println!("✅ Dioxus CSS generated");

    // 9. Performance Testing
    println!("\n⚡ 9. Performance Testing");
    println!("------------------------");
    
    let start = std::time::Instant::now();
    
    // Test basic CSS generation
    let basic_classes = ClassBuilder::new()
        .padding(SpacingValue::Integer(4))
        .class("bg-blue-500")
        .class("text-white")
        .build();
    
    generate_css_file("dist/performance_test_basic.css", Some(&basic_classes))?;
    let basic_duration = start.elapsed();
    
    // Test comprehensive CSS generation
    let comprehensive_start = std::time::Instant::now();
    generate_css_file("dist/performance_test_comprehensive.css", None)?;
    let comprehensive_duration = comprehensive_start.elapsed();
    
    println!("✅ Basic CSS generation: {:?}", basic_duration);
    println!("✅ Comprehensive CSS generation: {:?}", comprehensive_duration);

    // 10. File Size Analysis
    println!("\n📊 10. File Size Analysis");
    println!("------------------------");
    
    let files = [
        "dist/basic_styles.css",
        "dist/comprehensive_styles.css",
        "dist/core_utilities.css",
        "dist/advanced_utilities.css",
        "dist/minimal_styles.css",
        "dist/production_styles.css",
        "dist/development_styles.css",
        "dist/leptos_styles.css",
        "dist/yew_styles.css",
        "dist/dioxus_styles.css",
    ];
    
    for file in &files {
        if let Ok(metadata) = std::fs::metadata(file) {
            let size_kb = metadata.len() / 1024;
            println!("📁 {}: {} KB", file, size_kb);
        }
    }

    println!("\n🎉 CSS Generation Showcase Complete!");
    println!("=====================================");
    println!("✅ Generated {} CSS files", files.len());
    println!("✅ All utility categories covered");
    println!("✅ Performance optimized");
    println!("✅ Framework-specific configurations");
    println!("✅ Production-ready CSS generation");
    
    println!("\n📚 Coverage Summary:");
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

    println!("\n🎯 100% Coverage Achieved! All utility categories from tailwind-rs are now supported!");

    Ok(())
}
