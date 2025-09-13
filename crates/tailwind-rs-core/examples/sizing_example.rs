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
        .width(SizingValue::Full)                 // w-full
        .min_width(SizingValue::Integer(4))       // min-w-4
        .max_width(SizingValue::Integer(8))       // max-w-8
        .build();
    println!("  Basic width: {}", width_classes.to_css_classes());

    // Basic height examples
    println!("📏 Height Examples:");
    let height_classes = ClassBuilder::new()
        .height(SizingValue::Screen)              // h-screen
        .min_height(SizingValue::Integer(4))      // min-h-4
        .max_height(SizingValue::Integer(8))      // max-h-8
        .build();
    println!("  Basic height: {}", height_classes.to_css_classes());

    // Fractional sizing
    println!("🔢 Fractional Sizing:");
    let fractional_classes = ClassBuilder::new()
        .width(SizingValue::Fraction(Fraction::Half))      // w-1/2
        .height(SizingValue::Fraction(Fraction::Third))    // h-1/3
        .min_width(SizingValue::Fraction(Fraction::Quarter)) // min-w-1/4
        .max_width(SizingValue::Fraction(Fraction::ThreeQuarters)) // max-w-3/4
        .build();
    println!("  Fractional: {}", fractional_classes.to_css_classes());

    // Grid fractions
    println!("🎯 Grid Fractions:");
    let grid_classes = ClassBuilder::new()
        .width(SizingValue::GridFraction(GridFraction::SixTwelfths))   // w-6/12
        .height(SizingValue::GridFraction(GridFraction::FourTwelfths)) // h-4/12
        .min_width(SizingValue::GridFraction(GridFraction::ThreeTwelfths)) // min-w-3/12
        .max_width(SizingValue::GridFraction(GridFraction::NineTwelfths)) // max-w-9/12
        .build();
    println!("  Grid fractions: {}", grid_classes.to_css_classes());

    // Special values
    println!("⭐ Special Values:");
    let special_classes = ClassBuilder::new()
        .width(SizingValue::Auto)                 // w-auto
        .height(SizingValue::Fit)                 // h-fit
        .min_width(SizingValue::Min)              // min-w-min
        .max_width(SizingValue::Max)              // max-w-max
        .min_height(SizingValue::Screen)          // min-h-screen
        .max_height(SizingValue::Full)            // max-h-full
        .build();
    println!("  Special: {}", special_classes.to_css_classes());

    // Complex combination
    println!("🎨 Complex Combination:");
    let complex_classes = ClassBuilder::new()
        .width(SizingValue::Fraction(Fraction::TwoThirds))     // w-2/3
        .height(SizingValue::Integer(64))                      // h-64
        .min_width(SizingValue::Integer(32))                   // min-w-32
        .max_width(SizingValue::Full)                          // max-w-full
        .min_height(SizingValue::Screen)                       // min-h-screen
        .max_height(SizingValue::Integer(96))                  // max-h-96
        .build();
    println!("  Complex: {}", complex_classes.to_css_classes());

    println!();

    // CSS Value demonstration
    println!("🎨 CSS Values:");
    println!("  w-full CSS value: {}", SizingValue::Full.to_css_value_width());
    println!("  h-screen CSS value: {}", SizingValue::Screen.to_css_value_height());
    println!("  w-1/2 CSS value: {}", SizingValue::Fraction(Fraction::Half).to_css_value());
    println!("  w-6/12 CSS value: {}", SizingValue::GridFraction(GridFraction::SixTwelfths).to_css_value());
    println!("  w-auto CSS value: {}", SizingValue::Auto.to_css_value());

    println!();

    // All sizing values
    println!("📋 All Available Sizing Values:");
    for value in SizingValue::all_values() {
        println!("  {} -> {}", value.to_class_name(), value.to_css_value());
    }

    println!();

    // Fraction examples
    println!("🔢 Fraction Examples:");
    for fraction in [
        Fraction::Half, Fraction::Third, Fraction::TwoThirds,
        Fraction::Quarter, Fraction::ThreeQuarters,
        Fraction::Fifth, Fraction::TwoFifths, Fraction::ThreeFifths, Fraction::FourFifths,
        Fraction::Sixth, Fraction::TwoSixths, Fraction::ThreeSixths, Fraction::FourSixths, Fraction::FiveSixths,
    ] {
        println!("  {} -> {}", fraction.to_class_name(), fraction.to_css_value());
    }

    println!();

    // Grid fraction examples
    println!("🎯 Grid Fraction Examples:");
    for grid_fraction in [
        GridFraction::OneTwelfth, GridFraction::TwoTwelfths, GridFraction::ThreeTwelfths,
        GridFraction::FourTwelfths, GridFraction::FiveTwelfths, GridFraction::SixTwelfths,
        GridFraction::SevenTwelfths, GridFraction::EightTwelfths, GridFraction::NineTwelfths,
        GridFraction::TenTwelfths, GridFraction::ElevenTwelfths,
    ] {
        println!("  {} -> {}", grid_fraction.to_class_name(), grid_fraction.to_css_value());
    }

    println!("\n✅ Sizing system implementation complete!");
    println!("   - {} sizing values supported", SizingValue::all_values().len());
    println!("   - Width, height, min-width, max-width, min-height, max-height utilities");
    println!("   - Fractional and integer values");
    println!("   - Grid fractions (1/12 to 11/12)");
    println!("   - Special values (auto, full, screen, min, max, fit)");
    println!("   - Type-safe and compile-time validated");
}
