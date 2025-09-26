//! Spacing System Example
//!
//! This example demonstrates the complete spacing system implementation
//! including padding, margin, and gap utilities.

use tailwind_rs_core::*;

fn main() {
    println!("🎨 Tailwind CSS v4.1 Spacing System Example\n");

    // Basic padding examples
    println!("📏 Padding Examples:");
    let padding_classes = ClassBuilder::new()
        .padding(SpacingValue::Integer(4)) // p-4
        .padding_x(SpacingValue::Integer(6)) // px-6
        .padding_y(SpacingValue::Integer(2)) // py-2
        .build();
    println!("  Basic padding: {}", padding_classes.to_css_classes());

    // Fractional spacing
    let fractional_classes = ClassBuilder::new()
        .padding(SpacingValue::Fractional(0.5)) // p-0.5
        .padding_x(SpacingValue::Fractional(1.5)) // px-1.5
        .padding_y(SpacingValue::Fractional(2.5)) // py-2.5
        .build();
    println!(
        "  Fractional padding: {}",
        fractional_classes.to_css_classes()
    );

    // Special values
    let special_classes = ClassBuilder::new()
        .padding(SpacingValue::Auto) // p-auto
        .padding_x(SpacingValue::Full) // px-full
        .padding_y(SpacingValue::Screen) // py-screen
        .build();
    println!("  Special padding: {}", special_classes.to_css_classes());

    println!();

    // Basic margin examples
    println!("📐 Margin Examples:");
    let margin_classes = ClassBuilder::new()
        .margin(SpacingValue::Integer(8)) // m-8
        .margin_x(SpacingValue::Integer(4)) // mx-4
        .margin_y(SpacingValue::Integer(2)) // my-2
        .build();
    println!("  Basic margin: {}", margin_classes.to_css_classes());

    // Negative margins
    let negative_classes = ClassBuilder::new()
        .margin_negative(SpacingValue::Integer(4)) // -m-4
        .margin_x_negative(SpacingValue::Integer(2)) // -mx-2
        .margin_y_negative(SpacingValue::Integer(1)) // -my-1
        .build();
    println!("  Negative margin: {}", negative_classes.to_css_classes());

    println!();

    // Gap examples
    println!("🔲 Gap Examples:");
    let gap_classes = ClassBuilder::new()
        .gap_x(SpacingValue::Integer(4)) // gap-x-4
        .gap_x(SpacingValue::Integer(6)) // gap-x-6
        .gap_y(SpacingValue::Integer(2)) // gap-y-2
        .build();
    println!("  Basic gap: {}", gap_classes.to_css_classes());

    println!();

    // Complex combination
    println!("🎯 Complex Combination:");
    let complex_classes = ClassBuilder::new()
        .padding(SpacingValue::Integer(4)) // p-4
        .margin(SpacingValue::Integer(2)) // m-2
        .gap_x(SpacingValue::Integer(3)) // gap-x-3
        .padding_x(SpacingValue::Integer(6)) // px-6
        .margin_y(SpacingValue::Integer(1)) // my-1
        .gap_x(SpacingValue::Integer(2)) // gap-x-2
        .build();
    println!("  Complex: {}", complex_classes.to_css_classes());

    println!();

    // CSS Value demonstration
    println!("🎨 CSS Values:");
    println!(
        "  p-4 CSS value: {}",
        SpacingValue::Integer(4).to_css_value()
    );
    println!(
        "  px-0.5 CSS value: {}",
        SpacingValue::Fractional(0.5).to_css_value()
    );
    println!("  m-auto CSS value: {}", SpacingValue::Auto.to_css_value());
    println!(
        "  gap-full CSS value: {}",
        SpacingValue::Full.to_css_value()
    );

    println!();

    // All spacing values
    println!("📋 All Available Spacing Values:");
    for value in SpacingValue::all_values() {
        println!("  {} -> {}", value.to_class_name(), value.to_css_value());
    }

    println!("\n✅ Spacing system implementation complete!");
    println!(
        "   - {} spacing values supported",
        SpacingValue::all_values().len()
    );
    println!("   - Padding, margin, and gap utilities");
    println!("   - Fractional and integer values");
    println!("   - Special values (auto, full, screen, etc.)");
    println!("   - Type-safe and compile-time validated");
}
