//! Tests for seamless CSS generation functionality
//! 
//! These tests verify that the new CSS generation functionality works correctly
//! and provides the seamless integration requested in the GitHub issue.

use tailwind_rs_core::*;
use std::fs;
use std::path::Path;

#[test]
fn test_generate_css_file_with_specific_classes() {
    // Create classes using ClassBuilder
    let classes = ClassBuilder::new()
        .class("p-4")
        .class("bg-blue-500")
        .class("text-white")
        .build();
    
    // Generate CSS file
    let output_path = "test_output_specific.css";
    generate_css_file(output_path, Some(&classes)).unwrap();
    
    // Verify file was created
    assert!(Path::new(output_path).exists());
    
    // Read and verify content
    let css_content = fs::read_to_string(output_path).unwrap();
    assert!(css_content.contains(".p-4"));
    assert!(css_content.contains("padding: 1rem"));
    assert!(css_content.contains(".bg-blue-500"));
    assert!(css_content.contains("background-color: #3b82f6"));
    assert!(css_content.contains(".text-white"));
    assert!(css_content.contains("color: #ffffff"));
    
    // Clean up
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_generate_css_file_comprehensive() {
    // Generate comprehensive CSS
    let output_path = "test_output_comprehensive.css";
    generate_css_file(output_path, None).unwrap();
    
    // Verify file was created
    assert!(Path::new(output_path).exists());
    
    // Read and verify content
    let css_content = fs::read_to_string(output_path).unwrap();
    
    // Should contain various utility classes
    assert!(css_content.contains(".p-4"));
    assert!(css_content.contains(".bg-blue-500"));
    assert!(css_content.contains(".text-white"));
    assert!(css_content.contains(".flex"));
    assert!(css_content.contains(".grid"));
    assert!(css_content.contains(".rounded-md"));
    assert!(css_content.contains(".shadow-lg"));
    
    // Clean up
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_generate_comprehensive_css_with_config() {
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
    ];
    config.include_responsive = true;
    config.include_dark_mode = true;
    
    // Generate CSS with custom configuration
    let output_path = "test_output_custom.css";
    generate_comprehensive_css(output_path, &config).unwrap();
    
    // Verify file was created
    assert!(Path::new(output_path).exists());
    
    // Read and verify content
    let css_content = fs::read_to_string(output_path).unwrap();
    
    // Should contain configured utilities
    assert!(css_content.contains(".p-4"));
    assert!(css_content.contains(".bg-blue-500"));
    assert!(css_content.contains(".bg-gray-500"));
    assert!(css_content.contains(".text-blue-500"));
    assert!(css_content.contains(".text-gray-500"));
    assert!(css_content.contains(".flex"));
    assert!(css_content.contains(".grid"));
    assert!(css_content.contains(".rounded-md"));
    assert!(css_content.contains(".shadow-lg"));
    
    // Clean up
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_css_generator_comprehensive_generation() {
    let mut generator = CssGenerator::new();
    let config = CssGenerationConfig::default();
    
    // Generate comprehensive CSS
    let css = generator.generate_comprehensive_css(&config).unwrap();
    
    // Verify CSS contains various utilities
    assert!(css.contains(".p-4"));
    assert!(css.contains(".bg-blue-500"));
    assert!(css.contains(".text-white"));
    assert!(css.contains(".flex"));
    assert!(css.contains(".grid"));
    assert!(css.contains(".rounded-md"));
    assert!(css.contains(".shadow-lg"));
    
    // Verify generator has rules
    assert!(generator.rule_count() > 0);
}

#[test]
fn test_css_generator_spacing_utilities() {
    let mut generator = CssGenerator::new();
    
    // Generate spacing utilities
    generator.generate_spacing_utilities().unwrap();
    
    // Verify spacing classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific spacing classes
    generator.add_class("p-4").unwrap();
    generator.add_class("m-2").unwrap();
    generator.add_class("pt-1").unwrap();
    generator.add_class("mb-8").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".p-4"));
    assert!(css.contains(".m-2"));
    assert!(css.contains(".pt-1"));
    assert!(css.contains(".mb-8"));
}

#[test]
fn test_css_generator_color_utilities() {
    let mut generator = CssGenerator::new();
    
    // Generate color utilities
    let palettes = vec!["blue".to_string(), "gray".to_string()];
    generator.generate_color_utilities(&palettes).unwrap();
    
    // Verify color classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific color classes
    generator.add_class("bg-blue-500").unwrap();
    generator.add_class("text-gray-700").unwrap();
    generator.add_class("border-blue-300").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".bg-blue-500"));
    assert!(css.contains(".text-gray-700"));
    assert!(css.contains(".border-blue-300"));
}

#[test]
fn test_css_generator_typography_utilities() {
    let mut generator = CssGenerator::new();
    
    // Generate typography utilities
    generator.generate_typography_utilities().unwrap();
    
    // Verify typography classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific typography classes
    generator.add_class("text-lg").unwrap();
    generator.add_class("font-bold").unwrap();
    generator.add_class("text-center").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".text-lg"));
    assert!(css.contains(".font-bold"));
    assert!(css.contains(".text-center"));
}

#[test]
fn test_css_generator_layout_utilities() {
    let mut generator = CssGenerator::new();
    
    // Generate layout utilities
    generator.generate_layout_utilities().unwrap();
    
    // Verify layout classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific layout classes
    generator.add_class("flex").unwrap();
    generator.add_class("grid").unwrap();
    generator.add_class("hidden").unwrap();
    generator.add_class("relative").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".flex"));
    assert!(css.contains(".grid"));
    assert!(css.contains(".hidden"));
    assert!(css.contains(".relative"));
}

#[test]
fn test_css_generator_flexbox_utilities() {
    let mut generator = CssGenerator::new();
    
    // Generate flexbox utilities
    generator.generate_flexbox_utilities().unwrap();
    
    // Verify flexbox classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific flexbox classes
    generator.add_class("flex-row").unwrap();
    generator.add_class("justify-center").unwrap();
    generator.add_class("items-center").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".flex-row"));
    assert!(css.contains(".justify-center"));
    assert!(css.contains(".items-center"));
}

#[test]
fn test_css_generator_grid_utilities() {
    let mut generator = CssGenerator::new();
    
    // Generate grid utilities
    generator.generate_grid_utilities().unwrap();
    
    // Verify grid classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific grid classes
    generator.add_class("grid-cols-3").unwrap();
    generator.add_class("grid-rows-2").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".grid-cols-3"));
    assert!(css.contains(".grid-rows-2"));
}

#[test]
fn test_css_generator_border_utilities() {
    let mut generator = CssGenerator::new();
    
    // Generate border utilities
    generator.generate_border_utilities().unwrap();
    
    // Verify border classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific border classes
    generator.add_class("border-2").unwrap();
    generator.add_class("rounded-lg").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".border-2"));
    assert!(css.contains(".rounded-lg"));
}

#[test]
fn test_css_generator_effects_utilities() {
    let mut generator = CssGenerator::new();
    
    // Generate effects utilities
    generator.generate_effects_utilities().unwrap();
    
    // Verify effects classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific effects classes
    generator.add_class("shadow-lg").unwrap();
    generator.add_class("shadow-none").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".shadow-lg"));
    assert!(css.contains(".shadow-none"));
}

#[test]
fn test_css_generator_transform_utilities() {
    let mut generator = CssGenerator::new();
    
    // Generate transform utilities
    generator.generate_transform_utilities().unwrap();
    
    // Verify transform classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific transform classes
    generator.add_class("scale-110").unwrap();
    generator.add_class("scale-50").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".scale-110"));
    assert!(css.contains(".scale-50"));
}

#[test]
fn test_css_generator_animation_utilities() {
    let mut generator = CssGenerator::new();
    
    // Generate animation utilities
    generator.generate_animation_utilities().unwrap();
    
    // Verify animation classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific animation classes
    generator.add_class("animate-spin").unwrap();
    generator.add_class("animate-pulse").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".animate-spin"));
    assert!(css.contains(".animate-pulse"));
}

#[test]
fn test_css_generator_interactivity_utilities() {
    let mut generator = CssGenerator::new();
    
    // Generate interactivity utilities
    generator.generate_interactivity_utilities().unwrap();
    
    // Verify interactivity classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific interactivity classes
    generator.add_class("cursor-pointer").unwrap();
    generator.add_class("select-none").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".cursor-pointer"));
    assert!(css.contains(".select-none"));
}

#[test]
fn test_css_generator_responsive_variants() {
    let mut generator = CssGenerator::new();
    
    // Generate responsive variants
    generator.generate_responsive_variants().unwrap();
    
    // Verify responsive classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific responsive classes
    generator.add_responsive_class(Breakpoint::Md, "p-4").unwrap();
    generator.add_responsive_class(Breakpoint::Lg, "bg-blue-500").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains("@media"));
    assert!(css.contains("md:p-4"));
    assert!(css.contains("lg:bg-blue-500"));
}

#[test]
fn test_css_generator_dark_mode_variants() {
    let mut generator = CssGenerator::new();
    
    // Generate dark mode variants
    generator.generate_dark_mode_variants().unwrap();
    
    // Verify dark mode classes were added
    assert!(generator.rule_count() > 0);
    
    // Test specific dark mode classes
    generator.add_class("bg-gray-800").unwrap();
    generator.add_class("text-white").unwrap();
    
    let css = generator.generate_css();
    assert!(css.contains(".bg-gray-800"));
    assert!(css.contains(".text-white"));
}

#[test]
fn test_css_generation_config_default() {
    let config = CssGenerationConfig::default();
    
    // Verify default configuration
    assert!(config.include_colors);
    assert!(config.include_spacing);
    assert!(config.include_typography);
    assert!(config.include_layout);
    assert!(config.include_flexbox);
    assert!(config.include_grid);
    assert!(config.include_borders);
    assert!(config.include_effects);
    assert!(config.include_transforms);
    assert!(config.include_animations);
    assert!(config.include_interactivity);
    assert!(config.include_responsive);
    assert!(config.include_dark_mode);
    
    // Verify default color palettes
    assert!(config.color_palettes.contains(&"gray".to_string()));
    assert!(config.color_palettes.contains(&"blue".to_string()));
    assert!(config.color_palettes.contains(&"red".to_string()));
    assert!(config.color_palettes.contains(&"green".to_string()));
    assert!(config.color_palettes.contains(&"yellow".to_string()));
    assert!(config.color_palettes.contains(&"purple".to_string()));
}

#[test]
fn test_css_generation_config_custom() {
    let mut config = CssGenerationConfig::default();
    config.include_colors = false;
    config.include_spacing = false;
    config.color_palettes = vec!["blue".to_string()];
    
    // Verify custom configuration
    assert!(!config.include_colors);
    assert!(!config.include_spacing);
    assert_eq!(config.color_palettes.len(), 1);
    assert!(config.color_palettes.contains(&"blue".to_string()));
}
