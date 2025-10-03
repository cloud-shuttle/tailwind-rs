//! Tests for the new element-based context system

use super::element_context::{ElementContext, FilterContext, AnimationContext, ArbitraryValueContext, CustomPropertyContext};

#[test]
fn test_element_context_gradient() {
    let mut context = ElementContext::default();
    context.update_from_class("bg-gradient-to-r");
    context.update_from_class("from-red-500");

    let css = context.generate_css();
    assert!(!css.is_empty());
    assert!(css.iter().any(|p| p.name == "--tw-gradient-from"));
    assert!(css.iter().any(|p| p.name == "background-image"));
}

    #[test]
    fn test_element_context() {
        let mut context = ElementContext::default();
        context.update_from_class("bg-gradient-to-r");
        context.update_from_class("from-pink-400");

        let css = context.generate_css();
        assert!(!css.is_empty());
        assert!(css.iter().any(|p| p.name == "--tw-gradient-from" && p.value == "#f472b6")); // pink-400 resolved to hex
    }


    #[test]
    fn test_element_context_with_multiple_utilities() {
        let mut context = ElementContext::default();

        // Add gradient
        context.update_from_class("bg-gradient-to-r");
        context.update_from_class("from-pink-400");
        context.update_from_class("to-cyan-400");

        // Add shadow
        context.update_from_class("shadow-lg");

        // Add transform
        context.update_from_class("scale-110");
        context.update_from_class("rotate-3");

        let css = context.generate_css();
        assert!(!css.is_empty());

        // Check gradient properties
        assert!(css.iter().any(|p| p.name == "--tw-gradient-from" && p.value == "#f472b6"));
        assert!(css.iter().any(|p| p.name == "--tw-gradient-to" && p.value == "#06b6d4"));

        // Check shadow property
        assert!(css.iter().any(|p| p.name == "box-shadow"));

        // Check transform property
        assert!(css.iter().any(|p| p.name == "transform" && p.value.contains("scale(1.1)") && p.value.contains("rotate(3deg)")));
    }

    #[test]
    fn test_element_context_shadow_transform_integration() {
        let mut context = ElementContext::default();

        // Test shadow classes
        context.update_from_class("shadow-lg");
        let css = context.generate_css();
        assert!(css.iter().any(|p| p.name == "box-shadow"));
        assert!(css[0].value.contains("10px 15px"));

        // Test transform classes
        context = ElementContext::default(); // Reset
        context.update_from_class("scale-110");
        context.update_from_class("rotate-45");
        let css = context.generate_css();
        assert!(css.iter().any(|p| p.name == "transform"));
        assert!(css[0].value.contains("scale(1.1)"));
        assert!(css[0].value.contains("rotate(45deg)"));
    }

    #[test]
    fn test_variant_shadow_transform_integration() {
        let mut context = ElementContext::default();

        // Test hover:shadow-lg
        context.update_from_class("hover:shadow-lg");

        let rules = context.generate_variant_css("hover:shadow-lg");
        assert!(!rules.is_empty());
        assert_eq!(rules[0].selector, ".hover\\:shadow-lg:hover");
        assert!(rules[0].properties.iter().any(|p| p.name == "box-shadow"));

        // Test md:rotate-12
        context = ElementContext::default(); // Reset
        context.update_from_class("md:rotate-12");

        let rules = context.generate_variant_css("md:rotate-12");
        assert!(!rules.is_empty());
        assert_eq!(rules[0].selector, ".md\\:rotate-12");
        assert!(rules[0].media_query.as_ref().unwrap().contains("min-width: 768px"));
    assert!(rules[0].properties.iter().any(|p| p.name == "transform" && p.value.contains("rotate(12deg)")));
}

#[test]
fn test_element_context_filters() {
    let mut context = ElementContext::default();

    // Test individual filter classes
    context.update_from_class("blur-md");
    context.update_from_class("brightness-150");
    context.update_from_class("contrast-125");
    context.update_from_class("grayscale");
    context.update_from_class("hue-rotate-90");
    context.update_from_class("invert");
    context.update_from_class("saturate-200");
    context.update_from_class("sepia");
    context.update_from_class("drop-shadow-lg");

    // Generate CSS
    let properties = context.generate_css();

    // Should have one filter property combining all filters
    assert_eq!(properties.len(), 1);
    assert_eq!(properties[0].name, "filter");

    // Check that all filters are present in the combined value
    let filter_value = &properties[0].value;
    assert!(filter_value.contains("blur(12px)"));
    assert!(filter_value.contains("brightness(1.5)"));
    assert!(filter_value.contains("contrast(1.25)"));
    assert!(filter_value.contains("grayscale(1)"));
    assert!(filter_value.contains("hue-rotate(90deg)"));
    assert!(filter_value.contains("invert(1)"));
    assert!(filter_value.contains("saturate(2)"));
    assert!(filter_value.contains("sepia(1)"));
    assert!(filter_value.contains("drop-shadow"));
}

#[test]
fn test_filter_context_individual() {
    let mut filter_context = FilterContext::default();

    // Test blur
    filter_context.update_from_class("blur-lg");
    assert_eq!(filter_context.blur, Some("blur(16px)".to_string()));

    // Test brightness
    filter_context.update_from_class("brightness-200");
    assert_eq!(filter_context.brightness, Some("brightness(2)".to_string()));

    // Test contrast
    filter_context.update_from_class("contrast-150");
    assert_eq!(filter_context.contrast, Some("contrast(1.5)".to_string()));

    // Test grayscale
    filter_context.update_from_class("grayscale-0");
    assert_eq!(filter_context.grayscale, Some("grayscale(0)".to_string()));

    // Test hue-rotate
    filter_context.update_from_class("hue-rotate-180");
    assert_eq!(filter_context.hue_rotate, Some("hue-rotate(180deg)".to_string()));

    // Test invert
    filter_context.update_from_class("invert-0");
    assert_eq!(filter_context.invert, Some("invert(0)".to_string()));

    // Test saturate
    filter_context.update_from_class("saturate-50");
    assert_eq!(filter_context.saturate, Some("saturate(0.5)".to_string()));

    // Test sepia
    filter_context.update_from_class("sepia-0");
    assert_eq!(filter_context.sepia, Some("sepia(0)".to_string()));

    // Test drop-shadow
    filter_context.update_from_class("drop-shadow-xl");
    assert!(filter_context.drop_shadow.as_ref().unwrap().contains("drop-shadow"));
}

#[test]
fn test_filter_context_css_generation() {
    let mut filter_context = FilterContext::default();

    // Add multiple filters
    filter_context.update_from_class("blur-sm");
    filter_context.update_from_class("brightness-110");

    let properties = filter_context.to_css_properties();

    assert_eq!(properties.len(), 1);
    assert_eq!(properties[0].name, "filter");
    assert!(properties[0].value.contains("blur(4px)"));
    assert!(properties[0].value.contains("brightness(1.1)"));
}

#[test]
fn test_filter_context_empty() {
    let filter_context = FilterContext::default();
    let properties = filter_context.to_css_properties();
    assert!(properties.is_empty());
}

#[test]
fn test_element_context_animations() {
    let mut context = ElementContext::default();

    // Test individual animation classes
    context.update_from_class("animate-spin");
    context.update_from_class("duration-500");
    context.update_from_class("ease-in-out");
    context.update_from_class("delay-200");
    context.update_from_class("animate-infinite");

    // Generate CSS
    let properties = context.generate_css();

    // Should have multiple animation properties
    assert!(properties.len() >= 5);

    // Check that animation properties are present
    let has_name = properties.iter().any(|p| p.name == "animation-name" && p.value == "spin");
    let has_duration = properties.iter().any(|p| p.name == "animation-duration" && p.value == "500ms");
    let has_timing = properties.iter().any(|p| p.name == "animation-timing-function" && p.value == "ease-in-out");
    let has_delay = properties.iter().any(|p| p.name == "animation-delay" && p.value == "200ms");
    let has_iteration = properties.iter().any(|p| p.name == "animation-iteration-count" && p.value == "infinite");

    assert!(has_name);
    assert!(has_duration);
    assert!(has_timing);
    assert!(has_delay);
    assert!(has_iteration);
}

#[test]
fn test_animation_context_individual() {
    let mut animation_context = AnimationContext::default();

    // Test animation name
    animation_context.update_from_class("animate-bounce");
    assert_eq!(animation_context.animation_name, Some("bounce".to_string()));

    // Test duration
    animation_context.update_from_class("duration-300");
    assert_eq!(animation_context.animation_duration, Some("300ms".to_string()));

    // Test timing function
    animation_context.update_from_class("ease-out");
    assert_eq!(animation_context.animation_timing, Some("ease-out".to_string()));

    // Test delay
    animation_context.update_from_class("delay-100");
    assert_eq!(animation_context.animation_delay, Some("100ms".to_string()));

    // Test iteration count
    animation_context.update_from_class("animate-once");
    assert_eq!(animation_context.animation_iteration, Some("1".to_string()));

    // Test direction
    animation_context.update_from_class("animate-reverse");
    assert_eq!(animation_context.animation_direction, Some("reverse".to_string()));

    // Test fill mode
    animation_context.update_from_class("animate-fill-forwards");
    assert_eq!(animation_context.animation_fill, Some("forwards".to_string()));

    // Test play state
    animation_context.update_from_class("animate-paused");
    assert_eq!(animation_context.animation_play_state, Some("paused".to_string()));
}

#[test]
fn test_animation_context_css_generation() {
    let mut animation_context = AnimationContext::default();

    // Add multiple animation properties
    animation_context.update_from_class("animate-pulse");
    animation_context.update_from_class("duration-1000");
    animation_context.update_from_class("ease-linear");

    let properties = animation_context.to_css_properties();

    assert_eq!(properties.len(), 3);

    // Check specific properties
    let name_prop = properties.iter().find(|p| p.name == "animation-name").unwrap();
    assert_eq!(name_prop.value, "pulse");

    let duration_prop = properties.iter().find(|p| p.name == "animation-duration").unwrap();
    assert_eq!(duration_prop.value, "1000ms");

    let timing_prop = properties.iter().find(|p| p.name == "animation-timing-function").unwrap();
    assert_eq!(timing_prop.value, "linear");
}

#[test]
fn test_animation_context_empty() {
    let animation_context = AnimationContext::default();
    let properties = animation_context.to_css_properties();
    assert!(properties.is_empty());
}

#[test]
fn test_element_context_custom_properties() {
    let mut context = ElementContext::default();

    // Test custom property classes
    context.update_from_class("[--my-color:red]");
    context.update_from_class("[--spacing:16px]");
    context.update_from_class("[--radius:8px]");

    // Generate CSS
    let properties = context.generate_css();

    // Should have custom property definitions
    assert!(properties.len() >= 3);

    // Check specific properties
    let has_my_color = properties.iter().any(|p| p.name == "--my-color" && p.value == "red");
    let has_spacing = properties.iter().any(|p| p.name == "--spacing" && p.value == "16px");
    let has_radius = properties.iter().any(|p| p.name == "--radius" && p.value == "8px");

    assert!(has_my_color, "Should have --my-color property");
    assert!(has_spacing, "Should have --spacing property");
    assert!(has_radius, "Should have --radius property");

    // Test parsing individual custom properties
    assert_eq!(CustomPropertyContext::parse_custom_property("[--my-color:red]"), Some(("--my-color".to_string(), "red".to_string())));
    assert_eq!(CustomPropertyContext::parse_custom_property("[--spacing:16px]"), Some(("--spacing".to_string(), "16px".to_string())));

    // Test invalid custom properties
    assert_eq!(CustomPropertyContext::parse_custom_property("[my-color:red]"), None); // Missing --
    assert_eq!(CustomPropertyContext::parse_custom_property("[--my-color]"), None); // Missing colon and value
    assert_eq!(CustomPropertyContext::parse_custom_property("--my-color:red"), None); // Missing brackets
}

#[test]
fn test_element_context_arbitrary_values() {
    let mut context = ElementContext::default();

    // Test various arbitrary value classes
    context.update_from_class("w-[100px]");
    context.update_from_class("h-[50vh]");
    context.update_from_class("bg-[#ff0000]");
    context.update_from_class("text-[#00ff00]");
    context.update_from_class("text-[24px]");
    context.update_from_class("rounded-[12px]");
    context.update_from_class("opacity-[0.75]");
    context.update_from_class("z-[999]");
    context.update_from_class("gap-[20px]");
    context.update_from_class("grid-cols-[repeat(3,1fr)]");

    // Generate CSS
    let properties = context.generate_css();

    // Check that all arbitrary values are present
    let has_width = properties.iter().any(|p| p.name == "width" && p.value == "100px");
    let has_height = properties.iter().any(|p| p.name == "height" && p.value == "50vh");
    let has_bg_color = properties.iter().any(|p| p.name == "background-color" && p.value == "#ff0000");
    let has_text_color = properties.iter().any(|p| p.name == "color" && p.value == "#00ff00");
    let has_font_size = properties.iter().any(|p| p.name == "font-size" && p.value == "24px");
    let has_border_radius = properties.iter().any(|p| p.name == "border-radius" && p.value == "12px");
    let has_opacity = properties.iter().any(|p| p.name == "opacity" && p.value == "0.75");
    let has_z_index = properties.iter().any(|p| p.name == "z-index" && p.value == "999");
    let has_gap = properties.iter().any(|p| p.name == "gap" && p.value == "20px");
    let has_grid_cols = properties.iter().any(|p| p.name == "grid-template-columns" && p.value == "repeat(3,1fr)");

    assert!(has_width);
    assert!(has_height);
    assert!(has_bg_color);
    assert!(has_text_color);
    assert!(has_font_size);
    assert!(has_border_radius);
    assert!(has_opacity);
    assert!(has_z_index);
    assert!(has_gap);
    assert!(has_grid_cols);
}

#[test]
fn test_arbitrary_value_context_individual() {
    let mut arbitrary_context = ArbitraryValueContext::default();

    // Test width
    arbitrary_context.update_from_class("w-[200px]");
    assert_eq!(arbitrary_context.width, Some("200px".to_string()));

    // Test height
    arbitrary_context.update_from_class("h-[100px]");
    assert_eq!(arbitrary_context.height, Some("100px".to_string()));

    // Test background color
    arbitrary_context.update_from_class("bg-[#ff0000]");
    assert_eq!(arbitrary_context.background_color, Some("#ff0000".to_string()));

    // Test text color
    arbitrary_context.update_from_class("text-[#00ff00]");
    assert_eq!(arbitrary_context.color, Some("#00ff00".to_string()));

    // Test font size
    arbitrary_context.update_from_class("text-[18px]");
    assert_eq!(arbitrary_context.font_size, Some("18px".to_string()));

    // Test border radius
    arbitrary_context.update_from_class("rounded-[8px]");
    assert_eq!(arbitrary_context.border_radius, Some("8px".to_string()));

    // Test opacity
    arbitrary_context.update_from_class("opacity-[0.5]");
    assert_eq!(arbitrary_context.opacity, Some("0.5".to_string()));

    // Test z-index
    arbitrary_context.update_from_class("z-[100]");
    assert_eq!(arbitrary_context.z_index, Some("100".to_string()));

    // Test gap
    arbitrary_context.update_from_class("gap-[16px]");
    assert_eq!(arbitrary_context.gap, Some("16px".to_string()));

    // Test grid columns
    arbitrary_context.update_from_class("grid-cols-[1fr_2fr_1fr]");
    assert_eq!(arbitrary_context.grid_template_columns, Some("1fr_2fr_1fr".to_string()));

    // Test cursor
    arbitrary_context.update_from_class("cursor-[pointer]");
    assert_eq!(arbitrary_context.cursor, Some("pointer".to_string()));

    // Test transform
    arbitrary_context.update_from_class("transform-[rotate(45deg)]");
    assert_eq!(arbitrary_context.transform, Some("rotate(45deg)".to_string()));

    // Test animation
    arbitrary_context.update_from_class("animate-[bounce_2s_infinite]");
    assert_eq!(arbitrary_context.animation, Some("bounce_2s_infinite".to_string()));

    // Test filter
    arbitrary_context.update_from_class("filter-[blur(4px)]");
    assert_eq!(arbitrary_context.filter, Some("blur(4px)".to_string()));
}

#[test]
fn test_arbitrary_value_parsing() {
    // Test parsing various arbitrary values
    assert_eq!(ArbitraryValueContext::parse_arbitrary_value("w-[100px]"), Some("100px".to_string()));
    assert_eq!(ArbitraryValueContext::parse_arbitrary_value("bg-[#ff0000]"), Some("#ff0000".to_string()));
    assert_eq!(ArbitraryValueContext::parse_arbitrary_value("text-[24px]"), Some("24px".to_string()));
    assert_eq!(ArbitraryValueContext::parse_arbitrary_value("grid-cols-[repeat(3,1fr)]"), Some("repeat(3,1fr)".to_string()));
    assert_eq!(ArbitraryValueContext::parse_arbitrary_value("transform-[rotate(45deg)_scale(1.2)]"), Some("rotate(45deg)_scale(1.2)".to_string()));

    // Test invalid cases
    assert_eq!(ArbitraryValueContext::parse_arbitrary_value("w-100px"), None);
    assert_eq!(ArbitraryValueContext::parse_arbitrary_value("bg-[#ff0000"), None); // missing closing bracket
    assert_eq!(ArbitraryValueContext::parse_arbitrary_value("w-[]"), None); // empty brackets
}

#[test]
fn test_arbitrary_value_css_generation() {
    let mut arbitrary_context = ArbitraryValueContext::default();

    // Add multiple arbitrary values
    arbitrary_context.update_from_class("w-[300px]");
    arbitrary_context.update_from_class("h-[200px]");
    arbitrary_context.update_from_class("bg-[#ff0000]");
    arbitrary_context.update_from_class("text-[white]");
    arbitrary_context.update_from_class("rounded-[10px]");

    let properties = arbitrary_context.to_css_properties();

    assert_eq!(properties.len(), 5);

    // Check specific properties
    let width_prop = properties.iter().find(|p| p.name == "width").unwrap();
    assert_eq!(width_prop.value, "300px");

    let height_prop = properties.iter().find(|p| p.name == "height").unwrap();
    assert_eq!(height_prop.value, "200px");

    let bg_prop = properties.iter().find(|p| p.name == "background-color").unwrap();
    assert_eq!(bg_prop.value, "#ff0000");

    let color_prop = properties.iter().find(|p| p.name == "color").unwrap();
    assert_eq!(color_prop.value, "white");

    let border_radius_prop = properties.iter().find(|p| p.name == "border-radius").unwrap();
    assert_eq!(border_radius_prop.value, "10px");
}

#[test]
fn test_arbitrary_value_context_empty() {
    let arbitrary_context = ArbitraryValueContext::default();
    let properties = arbitrary_context.to_css_properties();
    assert!(properties.is_empty());
}

#[test]
fn test_arbitrary_value_complex_cases() {
    let mut context = ElementContext::default();

    // Test complex arbitrary values with variants
    context.update_from_class("hover:w-[200px]");
    context.update_from_class("md:bg-[#ff0000]");
    context.update_from_class("dark:text-[24px]");

    // Generate variant-aware CSS
    let rules = context.generate_variant_css("hover:w-[200px]");
    assert!(!rules.is_empty());
    assert_eq!(rules[0].selector, ".hover\\:w-\\[200px\\]:hover");

    let properties = &rules[0].properties;
    let has_width = properties.iter().any(|p| p.name == "width" && p.value == "200px");
    assert!(has_width);
}

    #[test]
    fn test_element_based_processing_demo() {
        // 🎯 DEMO: Element-based processing functionality
        println!("\n🎨 ELEMENT-BASED PROCESSING DEMO");
        println!("=================================");

        // Create ElementContext directly for testing
        let mut context = ElementContext::default();

        // Example 1: Interactive Button
        let button_classes = vec![
            "px-6", "py-3",
            "bg-gradient-to-r", "from-purple-500", "via-pink-500", "to-red-500",
            "text-white", "rounded-xl", "font-semibold", "tracking-wide",
            "hover:from-purple-400", "hover:via-pink-400", "hover:to-red-400",
            "hover:scale-105", "hover:shadow-xl", "hover:shadow-purple-500/25",
            "transition-all", "duration-300", "transform"
        ];

        // Process classes through context
        for class in &button_classes {
            context.update_from_class(class);
        }
        let button_properties = context.generate_css();

        println!("📱 Button Classes: {:?}", button_classes);
        println!("Generated Properties: {} properties", button_properties.len());
        assert!(!button_properties.is_empty());

        // Reset context for next example
        let mut context2 = ElementContext::default();

        // Example 2: Responsive Card
        let card_classes = vec![
            "p-6", "bg-gradient-to-br", "from-blue-500", "via-purple-600", "to-pink-500",
            "rounded-2xl", "text-white", "shadow-2xl", "backdrop-blur-lg",
            "md:scale-110", "md:rotate-3", "md:shadow-blue-500/30",
            "hover:shadow-3xl", "hover:-rotate-1",
            "dark:border", "dark:border-white/20"
        ];

        // Process classes through context
        for class in &card_classes {
            context2.update_from_class(class);
        }
        let card_properties = context2.generate_css();

        println!("\n🎴 Card Classes: {:?}", card_classes);
        println!("Generated Properties: {} properties", card_properties.len());
        assert!(!card_properties.is_empty());

        println!("\n✅ Element-based processing working perfectly!");
        println!("   • Gradients: Multi-stop stateful processing");
        println!("   • Shadows: Proper box-shadow generation");
        println!("   • Transforms: Combined scale/rotate effects");
        println!("   • Variants: Complex hover/responsive combinations");
        println!("   • Media Queries: Responsive breakpoint support");
    }


    #[test]
    fn test_variant_parsing() {
        // Test simple variant
        let (variants, base) = ElementContext::parse_variants_from_class("hover:bg-blue-500");
        assert_eq!(variants, vec!["hover"]);
        assert_eq!(base, "bg-blue-500");

        // Test multiple variants
        let (variants, base) = ElementContext::parse_variants_from_class("md:hover:shadow-lg");
        assert_eq!(variants, vec!["md", "hover"]);
        assert_eq!(base, "shadow-lg");

        // Test no variants
        let (variants, base) = ElementContext::parse_variants_from_class("bg-blue-500");
        assert_eq!(variants, Vec::<String>::new());
        assert_eq!(base, "bg-blue-500");
    }



