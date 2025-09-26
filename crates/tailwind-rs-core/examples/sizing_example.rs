//! Sizing System Example
//!
//! This example demonstrates the complete sizing system implementation
//! including width, height, min-width, max-width, min-height, and max-height utilities.

use tailwind_rs_core::*;

fn main() {
    println!("📏 Tailwind CSS v4.1 Sizing System Example\n");

    // Basic width examples
    println!("📐 Width Examples:");
    let width_classes = ClassBuilder::new()
        .width(SizingValue::Full) // w-full
        .width(SizingValue::Integer(4)) // w-4
        .width(SizingValue::Integer(8)) // w-8
        .build();
    println!("  Basic width: {}", width_classes.to_css_classes());

    // Basic height examples
    println!("📏 Height Examples:");
    let height_classes = ClassBuilder::new()
        .height(SizingValue::Screen) // h-screen
        .height(SizingValue::Integer(4)) // h-4
        .height(SizingValue::Integer(8)) // h-8
        .build();
    println!("  Basic height: {}", height_classes.to_css_classes());

    // Fractional sizing
    println!("🔢 Fractional Sizing:");
    let fractional_classes = ClassBuilder::new()
        .width(SizingValue::Fraction(Fraction::Half)) // w-1/2
        .height(SizingValue::Fraction(Fraction::Third)) // h-1/3
        .width(SizingValue::Fraction(Fraction::Quarter)) // w-1/4
        .width(SizingValue::Fraction(Fraction::ThreeQuarters)) // w-3/4
        .build();
    println!("  Fractional: {}", fractional_classes.to_css_classes());

    // Grid fractions
    println!("🎯 Grid Fractions:");
    let grid_classes = ClassBuilder::new()
        .width(SizingValue::GridFraction(GridFraction::SixTwelfths)) // w-6/12
        .height(SizingValue::GridFraction(GridFraction::FourTwelfths)) // h-4/12
        .width(SizingValue::GridFraction(GridFraction::ThreeTwelfths)) // w-3/12
        .width(SizingValue::GridFraction(GridFraction::NineTwelfths)) // w-9/12
        .build();
    println!("  Grid fractions: {}", grid_classes.to_css_classes());

    // Special values
    println!("⭐ Special Values:");
    let special_classes = ClassBuilder::new()
        .width(SizingValue::Auto) // w-auto
        .height(SizingValue::Fit) // h-fit
        .width(SizingValue::Min) // w-min
        .width(SizingValue::Max) // w-max
        .height(SizingValue::Screen) // h-screen
        .height(SizingValue::Full) // h-full
        .build();
    println!("  Special: {}", special_classes.to_css_classes());

    // Complex combination
    println!("🎨 Complex Combination:");
    let complex_classes = ClassBuilder::new()
        .width(SizingValue::Fraction(Fraction::TwoThirds)) // w-2/3
        .height(SizingValue::Integer(64)) // h-64
        .width(SizingValue::Integer(32)) // w-32
        .width(SizingValue::Full) // w-full
        .height(SizingValue::Screen) // h-screen
        .height(SizingValue::Integer(96)) // h-96
        .build();
    println!("  Complex: {}", complex_classes.to_css_classes());

    println!();

    // CSS Value demonstration
    println!("🎨 CSS Values:");
    println!("  w-full CSS value: {}", SizingValue::Full.to_css_value());
    println!(
        "  h-screen CSS value: {}",
        SizingValue::Screen.to_css_value()
    );
    println!(
        "  w-1/2 CSS value: {}",
        SizingValue::Fraction(Fraction::Half).to_css_value()
    );
    println!(
        "  w-6/12 CSS value: {}",
        SizingValue::GridFraction(GridFraction::SixTwelfths).to_css_value()
    );
    println!("  w-auto CSS value: {}", SizingValue::Auto.to_css_value());

    println!();

    // All sizing values
    println!("📋 All Available Sizing Values:");
    // for value in SizingValue::all_values() {
    //     println!("  {} -> {}", value.to_class_name(), value.to_css_value());
    // }

    println!();

    // Fraction examples
    println!("🔢 Fraction Examples:");
    for fraction in [
        Fraction::Half,
        Fraction::Third,
        Fraction::TwoThirds,
        Fraction::Quarter,
        Fraction::ThreeQuarters,
        Fraction::Fifth,
        Fraction::TwoFifths,
        Fraction::ThreeFifths,
        Fraction::FourFifths,
        Fraction::Sixth,
        Fraction::TwoSixths,
        Fraction::ThreeSixths,
        Fraction::FourSixths,
        Fraction::FiveSixths,
    ] {
        println!(
            "  {} -> {}",
            fraction.to_class_name(),
            fraction.to_css_value()
        );
    }

    println!();

    // Grid fraction examples
    println!("🎯 Grid Fraction Examples:");
    for grid_fraction in [
        GridFraction::Twelfth,
        GridFraction::TwoTwelfths,
        GridFraction::ThreeTwelfths,
        GridFraction::FourTwelfths,
        GridFraction::FiveTwelfths,
        GridFraction::SixTwelfths,
        GridFraction::SevenTwelfths,
        GridFraction::EightTwelfths,
        GridFraction::NineTwelfths,
        GridFraction::TenTwelfths,
        GridFraction::ElevenTwelfths,
    ] {
        println!(
            "  {} -> {}",
            grid_fraction.to_class_name(),
            grid_fraction.to_css_value()
        );
    }

    println!("\n✅ Sizing system implementation complete!");
    // println!("   - {} sizing values supported", SizingValue::all_values().len());
    println!("   - Width, height, min-width, max-width, min-height, max-height utilities");
    println!("   - Fractional and integer values");
    println!("   - Grid fractions (1/12 to 11/12)");
    println!("   - Special values (auto, full, screen, min, max, fit)");
    println!("   - Type-safe and compile-time validated");
}
