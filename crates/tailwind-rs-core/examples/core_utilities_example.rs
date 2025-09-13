//! Core Utilities Example
//!
//! This example demonstrates all the core utility systems we've implemented:
//! - Spacing (padding, margin, gap)
//! - Sizing (width, height, min/max)
//! - Typography (font, text, line-height)
//! - Colors (text, background, border, ring)

use tailwind_rs_core::utilities::spacing::{SpacingValue, PaddingUtilities, MarginUtilities, GapUtilities};
use tailwind_rs_core::utilities::sizing::{SizingValue, Fraction, GridFraction, WidthUtilities, HeightUtilities};
use tailwind_rs_core::utilities::typography::{
    FontFamily, FontSize, FontWeight, TextAlign, LineHeight, LetterSpacing,
    FontFamilyUtilities, FontSizeUtilities, FontWeightUtilities, TextAlignUtilities,
    LineHeightUtilities, LetterSpacingUtilities,
};
use tailwind_rs_core::utilities::colors::{
    Color, ColorPalette, ColorShade, TextColorUtilities, BackgroundColorUtilities,
    BorderColorUtilities, RingColorUtilities,
};
use tailwind_rs_core::*;

fn main() {
    println!("🎨 Tailwind CSS v4.1 Core Utilities Example\n");

    // Comprehensive example combining all core utilities
    println!("🎯 Comprehensive Core Utilities Example:");
    let comprehensive_classes = ClassBuilder::new()
        // Spacing utilities
        .padding(SpacingValue::Integer(4))                    // p-4
        .padding_x(SpacingValue::Integer(6))                  // px-6
        .padding_y(SpacingValue::Integer(2))                  // py-2
        .margin(SpacingValue::Integer(2))                     // m-2
        .margin_x(SpacingValue::Auto)                         // mx-auto
        .gap(SpacingValue::Integer(3))                        // gap-3
        
        // Sizing utilities
        .width(SizingValue::Full)                             // w-full
        .height(SizingValue::Screen)                          // h-screen
        .min_width(SizingValue::Integer(32))                  // min-w-32
        .max_width(SizingValue::Fraction(Fraction::TwoThirds)) // max-w-2/3
        .min_height(SizingValue::Integer(64))                 // min-h-64
        .max_height(SizingValue::Integer(96))                 // max-h-96
        
        // Typography utilities
        .font_family(FontFamily::Sans)                        // font-sans
        .font_size(FontSize::Lg)                              // text-lg
        .font_weight(FontWeight::Bold)                        // font-bold
        .text_align(TextAlign::Center)                        // text-center
        .line_height(LineHeight::Relaxed)                     // leading-relaxed
        .letter_spacing(LetterSpacing::Wide)                  // tracking-wide
        
        // Color utilities
        .text_white()                                                          // text-white
        .background_color(Color::new(ColorPalette::Blue, ColorShade::Shade500)) // bg-blue-500
        .border_color(Color::new(ColorPalette::Blue, ColorShade::Shade600))     // border-blue-600
        .ring_color(Color::new(ColorPalette::Blue, ColorShade::Shade400))       // ring-blue-400
        
        .build();
    
    println!("  Comprehensive: {}", comprehensive_classes.to_css_classes());

    println!();

    // Spacing system demonstration
    println!("📏 Spacing System:");
    let spacing_classes = ClassBuilder::new()
        .padding(SpacingValue::Integer(4))                    // p-4
        .padding_x(SpacingValue::Fractional(1.5))             // px-1.5
        .padding_y(SpacingValue::Fractional(2.5))             // py-2.5
        .margin(SpacingValue::Integer(8))                     // m-8
        .margin_x(SpacingValue::Integer(4))                   // mx-4
        .margin_y(SpacingValue::Integer(2))                   // my-2
        .gap(SpacingValue::Integer(6))                        // gap-6
        .gap_x(SpacingValue::Integer(4))                      // gap-x-4
        .gap_y(SpacingValue::Integer(2))                      // gap-y-2
        .build();
    println!("  Spacing: {}", spacing_classes.to_css_classes());

    // Sizing system demonstration
    println!("📐 Sizing System:");
    let sizing_classes = ClassBuilder::new()
        .width(SizingValue::Fraction(Fraction::Half))         // w-1/2
        .height(SizingValue::Fraction(Fraction::Third))       // h-1/3
        .min_width(SizingValue::GridFraction(GridFraction::SixTwelfths)) // min-w-6/12
        .max_width(SizingValue::GridFraction(GridFraction::NineTwelfths)) // max-w-9/12
        .min_height(SizingValue::Integer(32))                 // min-h-32
        .max_height(SizingValue::Integer(64))                 // max-h-64
        .build();
    println!("  Sizing: {}", sizing_classes.to_css_classes());

    // Typography system demonstration
    println!("📝 Typography System:");
    let typography_classes = ClassBuilder::new()
        .font_family(FontFamily::Serif)                       // font-serif
        .font_size(FontSize::Xl2)                             // text-2xl
        .font_weight(FontWeight::SemiBold)                    // font-semibold
        .text_align(TextAlign::Justify)                       // text-justify
        .line_height(LineHeight::Loose)                       // leading-loose
        .letter_spacing(LetterSpacing::Wider)                 // tracking-wider
        .build();
    println!("  Typography: {}", typography_classes.to_css_classes());

    // Color system demonstration
    println!("🎨 Color System:");
    let color_classes = ClassBuilder::new()
        .text_color(Color::new(ColorPalette::Gray, ColorShade::Shade900))       // text-gray-900
        .background_color(Color::new(ColorPalette::Green, ColorShade::Shade100)) // bg-green-100
        .border_color(Color::new(ColorPalette::Green, ColorShade::Shade300))    // border-green-300
        .ring_color(Color::new(ColorPalette::Green, ColorShade::Shade500))      // ring-green-500
        .build();
    println!("  Colors: {}", color_classes.to_css_classes());

    println!();

    // CSS Values demonstration
    println!("🎨 CSS Values:");
    println!("  p-4 CSS value: {}", SpacingValue::Integer(4).to_css_value());
    println!("  w-1/2 CSS value: {}", SizingValue::Fraction(Fraction::Half).to_css_value());
    println!("  text-lg CSS value: {}", FontSize::Lg.to_css_value());
    println!("  bg-blue-500 CSS value: {}", Color::new(ColorPalette::Blue, ColorShade::Shade500).to_css_value());

    println!();

    // System statistics
    println!("📊 System Statistics:");
    println!("  Spacing values: {}", SpacingValue::all_values().len());
    println!("  Sizing values: {}", SizingValue::all_values().len());
    println!("  Color palettes: {}", ColorPalette::all_palettes().len());
    println!("  Color shades: {}", ColorShade::all_shades().len());
    println!("  Total colors: {}", Color::all_colors().len());

    println!();

    // Utility categories
    println!("🔧 Utility Categories:");
    println!("  ✅ Spacing: Padding, Margin, Gap");
    println!("  ✅ Sizing: Width, Height, Min/Max");
    println!("  ✅ Typography: Font, Text, Line-height");
    println!("  ✅ Colors: Text, Background, Border, Ring");
    println!("  ⏳ Layout: Display, Position, Overflow (Next)");
    println!("  ⏳ Flexbox: Flex, Justify, Align (Phase 2)");
    println!("  ⏳ Grid: Grid, Columns, Rows (Phase 2)");
    println!("  ⏳ Backgrounds: Background, Gradient (Phase 2)");
    println!("  ⏳ Borders: Border, Outline (Phase 2)");
    println!("  ⏳ Effects: Shadow, Opacity (Phase 2)");

    println!("\n✅ Core utilities implementation complete!");
    println!("   - 4 major utility systems implemented");
    println!("   - Type-safe and compile-time validated");
    println!("   - Comprehensive coverage of Tailwind CSS v4.1 core features");
    println!("   - Ready for Phase 2: Advanced Utilities");
}
