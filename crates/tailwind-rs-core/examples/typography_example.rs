//! Typography System Example
//!
//! This example demonstrates the complete typography system implementation
//! including font families, font sizes, font weights, text alignment, line height,
//! letter spacing, text decoration, text transform, text overflow, white space, and word break utilities.

use tailwind_rs_core::utilities::typography::{
    FontFamily, FontSize, FontWeight, TextAlign, LineHeight, LetterSpacing,
    TextDecoration, TextTransform, TextOverflow, WhiteSpace, WordBreak,
    FontFamilyUtilities, FontSizeUtilities, FontWeightUtilities, TextAlignUtilities,
    LineHeightUtilities, LetterSpacingUtilities, TextDecorationUtilities,
    TextTransformUtilities, TextOverflowUtilities, WhiteSpaceUtilities, WordBreakUtilities,
};
use tailwind_rs_core::*;

fn main() {
    println!("📝 Tailwind CSS v4.1 Typography System Example\n");

    // Font family examples
    println!("🔤 Font Family Examples:");
    let font_family_classes = ClassBuilder::new()
        .font_family(FontFamily::Sans)            // font-sans
        .font_family(FontFamily::Serif)           // font-serif
        .font_family(FontFamily::Mono)            // font-mono
        .build();
    println!("  Font families: {}", font_family_classes.to_css_classes());

    // Font size examples
    println!("📏 Font Size Examples:");
    let font_size_classes = ClassBuilder::new()
        .font_size(FontSize::Xs)                  // text-xs
        .font_size(FontSize::Sm)                  // text-sm
        .font_size(FontSize::Base)                // text-base
        .font_size(FontSize::Lg)                  // text-lg
        .font_size(FontSize::Xl)                  // text-xl
        .font_size(FontSize::Xl2)                 // text-2xl
        .font_size(FontSize::Xl3)                 // text-3xl
        .build();
    println!("  Font sizes: {}", font_size_classes.to_css_classes());

    // Font weight examples
    println!("⚖️ Font Weight Examples:");
    let font_weight_classes = ClassBuilder::new()
        .font_weight(FontWeight::Thin)            // font-thin
        .font_weight(FontWeight::Light)           // font-light
        .font_weight(FontWeight::Normal)          // font-normal
        .font_weight(FontWeight::Medium)          // font-medium
        .font_weight(FontWeight::SemiBold)        // font-semibold
        .font_weight(FontWeight::Bold)            // font-bold
        .font_weight(FontWeight::Black)           // font-black
        .build();
    println!("  Font weights: {}", font_weight_classes.to_css_classes());

    // Text alignment examples
    println!("📐 Text Alignment Examples:");
    let text_align_classes = ClassBuilder::new()
        .text_align(TextAlign::Left)              // text-left
        .text_align(TextAlign::Center)            // text-center
        .text_align(TextAlign::Right)             // text-right
        .text_align(TextAlign::Justify)           // text-justify
        .text_align(TextAlign::Start)             // text-start
        .text_align(TextAlign::End)               // text-end
        .build();
    println!("  Text alignment: {}", text_align_classes.to_css_classes());

    // Line height examples
    println!("📏 Line Height Examples:");
    let line_height_classes = ClassBuilder::new()
        .line_height(LineHeight::Tight)           // leading-tight
        .line_height(LineHeight::Normal)          // leading-normal
        .line_height(LineHeight::Relaxed)         // leading-relaxed
        .line_height(LineHeight::Loose)           // leading-loose
        .line_height(LineHeight::Custom(1.75))    // leading-1.75
        .build();
    println!("  Line heights: {}", line_height_classes.to_css_classes());

    // Letter spacing examples
    println!("🔤 Letter Spacing Examples:");
    let letter_spacing_classes = ClassBuilder::new()
        .letter_spacing(LetterSpacing::Tight)     // tracking-tight
        .letter_spacing(LetterSpacing::Normal)    // tracking-normal
        .letter_spacing(LetterSpacing::Wide)      // tracking-wide
        .letter_spacing(LetterSpacing::Wider)     // tracking-wider
        .letter_spacing(LetterSpacing::Custom(0.1)) // tracking-0.1
        .build();
    println!("  Letter spacing: {}", letter_spacing_classes.to_css_classes());

    // Text decoration examples
    println!("🎨 Text Decoration Examples:");
    let text_decoration_classes = ClassBuilder::new()
        .text_decoration(TextDecoration::None)    // no-underline
        .text_decoration(TextDecoration::Underline) // underline
        .text_decoration(TextDecoration::Overline)  // overline
        .text_decoration(TextDecoration::LineThrough) // line-through
        .build();
    println!("  Text decoration: {}", text_decoration_classes.to_css_classes());

    // Text transform examples
    println!("🔄 Text Transform Examples:");
    let text_transform_classes = ClassBuilder::new()
        .text_transform(TextTransform::None)      // normal-case
        .text_transform(TextTransform::Uppercase) // uppercase
        .text_transform(TextTransform::Lowercase) // lowercase
        .text_transform(TextTransform::Capitalize) // capitalize
        .build();
    println!("  Text transform: {}", text_transform_classes.to_css_classes());

    // Text overflow examples
    println!("✂️ Text Overflow Examples:");
    let text_overflow_classes = ClassBuilder::new()
        .text_overflow(TextOverflow::Truncate)    // truncate
        .text_overflow(TextOverflow::Ellipsis)    // text-ellipsis
        .text_overflow(TextOverflow::Clip)        // text-clip
        .build();
    println!("  Text overflow: {}", text_overflow_classes.to_css_classes());

    // White space examples
    println!("⏸️ White Space Examples:");
    let white_space_classes = ClassBuilder::new()
        .white_space(WhiteSpace::Normal)          // whitespace-normal
        .white_space(WhiteSpace::Nowrap)          // whitespace-nowrap
        .white_space(WhiteSpace::Pre)             // whitespace-pre
        .white_space(WhiteSpace::PreLine)         // whitespace-pre-line
        .white_space(WhiteSpace::PreWrap)         // whitespace-pre-wrap
        .build();
    println!("  White space: {}", white_space_classes.to_css_classes());

    // Word break examples
    println!("🔤 Word Break Examples:");
    let word_break_classes = ClassBuilder::new()
        .word_break(WordBreak::Normal)            // break-normal
        .word_break(WordBreak::BreakAll)          // break-all
        .word_break(WordBreak::BreakWords)        // break-words
        .word_break(WordBreak::KeepAll)           // break-keep
        .build();
    println!("  Word break: {}", word_break_classes.to_css_classes());

    // Complex typography combination
    println!("🎨 Complex Typography Combination:");
    let complex_classes = ClassBuilder::new()
        .font_family(FontFamily::Sans)            // font-sans
        .font_size(FontSize::Xl2)                 // text-2xl
        .font_weight(FontWeight::Bold)            // font-bold
        .text_align(TextAlign::Center)            // text-center
        .line_height(LineHeight::Relaxed)         // leading-relaxed
        .letter_spacing(LetterSpacing::Wide)      // tracking-wide
        .text_decoration(TextDecoration::Underline) // underline
        .text_transform(TextTransform::Uppercase) // uppercase
        .build();
    println!("  Complex: {}", complex_classes.to_css_classes());

    println!();

    // CSS Value demonstration
    println!("🎨 CSS Values:");
    println!("  font-sans CSS value: {}", FontFamily::Sans.to_css_value());
    println!("  text-lg CSS value: {}", FontSize::Lg.to_css_value());
    println!("  font-bold CSS value: {}", FontWeight::Bold.to_css_value());
    println!("  text-center CSS value: {}", TextAlign::Center.to_css_value());
    println!("  leading-relaxed CSS value: {}", LineHeight::Relaxed.to_css_value());
    println!("  tracking-wide CSS value: {}", LetterSpacing::Wide.to_css_value());

    println!();

    // All font families
    println!("🔤 All Font Families:");
    for family in [FontFamily::Sans, FontFamily::Serif, FontFamily::Mono] {
        println!("  {} -> {}", family.to_class_name(), family.to_css_value());
    }

    println!();

    // All font sizes
    println!("📏 All Font Sizes:");
    for size in [
        FontSize::Xs, FontSize::Sm, FontSize::Base, FontSize::Lg, FontSize::Xl,
        FontSize::Xl2, FontSize::Xl3, FontSize::Xl4, FontSize::Xl5, FontSize::Xl6,
        FontSize::Xl7, FontSize::Xl8, FontSize::Xl9,
    ] {
        println!("  {} -> {}", size.to_class_name(), size.to_css_value());
    }

    println!();

    // All font weights
    println!("⚖️ All Font Weights:");
    for weight in [
        FontWeight::Thin, FontWeight::ExtraLight, FontWeight::Light, FontWeight::Normal,
        FontWeight::Medium, FontWeight::SemiBold, FontWeight::Bold, FontWeight::ExtraBold,
        FontWeight::Black,
    ] {
        println!("  {} -> {}", weight.to_class_name(), weight.to_css_value());
    }

    println!();

    // All text alignments
    println!("📐 All Text Alignments:");
    for align in [
        TextAlign::Left, TextAlign::Center, TextAlign::Right, TextAlign::Justify,
        TextAlign::Start, TextAlign::End,
    ] {
        println!("  {} -> {}", align.to_class_name(), align.to_css_value());
    }

    println!();

    // All line heights
    println!("📏 All Line Heights:");
    for height in [
        LineHeight::None, LineHeight::Tight, LineHeight::Snug, LineHeight::Normal,
        LineHeight::Relaxed, LineHeight::Loose,
    ] {
        println!("  {} -> {}", height.to_class_name(), height.to_css_value());
    }

    println!();

    // All letter spacings
    println!("🔤 All Letter Spacings:");
    for spacing in [
        LetterSpacing::Tighter, LetterSpacing::Tight, LetterSpacing::Normal,
        LetterSpacing::Wide, LetterSpacing::Wider, LetterSpacing::Widest,
    ] {
        println!("  {} -> {}", spacing.to_class_name(), spacing.to_css_value());
    }

    println!("\n✅ Typography system implementation complete!");
    println!("   - Font families: Sans, Serif, Mono");
    println!("   - Font sizes: xs to 9xl (12 sizes)");
    println!("   - Font weights: Thin to Black (9 weights)");
    println!("   - Text alignment: Left, Center, Right, Justify, Start, End");
    println!("   - Line heights: None, Tight, Snug, Normal, Relaxed, Loose + Custom");
    println!("   - Letter spacing: Tighter to Widest + Custom");
    println!("   - Text decoration: None, Underline, Overline, Line-through");
    println!("   - Text transform: None, Uppercase, Lowercase, Capitalize");
    println!("   - Text overflow: Truncate, Ellipsis, Clip");
    println!("   - White space: Normal, Nowrap, Pre, Pre-line, Pre-wrap, Break-spaces");
    println!("   - Word break: Normal, Break-all, Break-words, Keep-all");
    println!("   - Type-safe and compile-time validated");
}
