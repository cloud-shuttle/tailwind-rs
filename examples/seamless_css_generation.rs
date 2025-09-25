//! Seamless CSS Generation Example
//! 
//! This example demonstrates the new seamless CSS generation functionality
//! that integrates ClassBuilder with CSS file generation, as requested in the GitHub issue.

use tailwind_rs_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Seamless CSS Generation Demo");
    println!("==============================");
    
    // Example 1: Generate CSS with specific classes from ClassBuilder
    println!("\n1. Generating CSS with specific classes...");
    demo_specific_classes()?;
    
    // Example 2: Generate comprehensive CSS with all utilities
    println!("\n2. Generating comprehensive CSS...");
    demo_comprehensive_css()?;
    
    // Example 3: Generate CSS with custom configuration
    println!("\n3. Generating CSS with custom configuration...");
    demo_custom_configuration()?;
    
    // Example 4: Demonstrate the seamless workflow from the GitHub issue
    println!("\n4. Demonstrating seamless workflow...");
    demo_seamless_workflow()?;
    
    println!("\n✅ All examples completed successfully!");
    
    Ok(())
}

fn demo_specific_classes() -> Result<(), Box<dyn std::error::Error>> {
    // Create classes using ClassBuilder (as shown in the GitHub issue)
    let classes = ClassBuilder::new()
        .padding(SpacingValue::Integer(4))
        .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
        .text_color(Color::new(ColorPalette::White, ColorShade::Shade100))
        .build();
    
    // Generate CSS file with these specific classes
    generate_css_file("target/site/pkg/styles.css", Some(&classes))?;
    
    println!("  ✅ Generated CSS file with specific classes");
    println!("  📁 Output: target/site/pkg/styles.css");
    
    Ok(())
}

fn demo_comprehensive_css() -> Result<(), Box<dyn std::error::Error>> {
    // Generate comprehensive CSS with all utilities
    generate_css_file("target/site/pkg/comprehensive.css", None)?;
    
    println!("  ✅ Generated comprehensive CSS file");
    println!("  📁 Output: target/site/pkg/comprehensive.css");
    
    Ok(())
}

fn demo_custom_configuration() -> Result<(), Box<dyn std::error::Error>> {
    // Create custom configuration
    let mut config = CssGenerationConfig::default();
    config.include_colors = true;
    config.include_spacing = true;
    config.include_typography = true;
    config.include_layout = true;
    config.include_flexbox = true;
    config.include_grid = true;
    config.include_borders = true;
    config.include_effects = true;
    config.include_transforms = true;
    config.include_animations = true;
    config.include_interactivity = true;
    config.color_palettes = vec![
        "blue".to_string(),
        "gray".to_string(),
        "purple".to_string(),
    ];
    config.include_responsive = true;
    config.include_dark_mode = true;
    
    // Generate CSS with custom configuration
    generate_comprehensive_css("target/site/pkg/custom.css", &config)?;
    
    println!("  ✅ Generated CSS file with custom configuration");
    println!("  📁 Output: target/site/pkg/custom.css");
    
    Ok(())
}

fn demo_seamless_workflow() -> Result<(), Box<dyn std::error::Error>> {
    println!("  🔄 Demonstrating the seamless workflow from the GitHub issue...");
    
    // This is exactly what was requested in the GitHub issue:
    // "Generate CSS file with all necessary classes"
    
    // Step 1: Create classes using ClassBuilder
    let classes = ClassBuilder::new()
        .padding(SpacingValue::Integer(4))
        .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
        .text_color(Color::new(ColorPalette::White, ColorShade::Shade100))
        .build();
    
    println!("    📝 Created classes with ClassBuilder");
    
    // Step 2: Generate CSS file with all necessary classes
    generate_css_file("target/site/pkg/seamless.css", Some(&classes))?;
    
    println!("    🎨 Generated CSS file with all necessary classes");
    println!("    📁 Output: target/site/pkg/seamless.css");
    
    // Step 3: Show the generated CSS classes
    let css_classes = classes.to_css_classes();
    println!("    📋 Generated CSS classes: {}", css_classes);
    
    // Step 4: Demonstrate that all classes are now available in the CSS file
    println!("    ✅ All classes are now available in the CSS file!");
    println!("    🚀 No external tools or CDN dependencies needed!");
    
    Ok(())
}

// Additional helper function to demonstrate advanced usage
fn demo_advanced_usage() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n5. Advanced usage example...");
    
    // Create a more complex set of classes
    let classes = ClassBuilder::new()
        .padding(SpacingValue::Integer(4))
        .margin(SpacingValue::Integer(2))
        .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500))
        .text_color(Color::new(ColorPalette::White, ColorShade::Shade100))
        .border_radius(BorderRadius::Medium)
        .box_shadow(BoxShadow::Large)
        .display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .build();
    
    // Generate CSS with these classes
    generate_css_file("target/site/pkg/advanced.css", Some(&classes))?;
    
    println!("  ✅ Generated advanced CSS file");
    println!("  📁 Output: target/site/pkg/advanced.css");
    
    Ok(())
}
