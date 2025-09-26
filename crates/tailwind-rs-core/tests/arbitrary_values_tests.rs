//! TDD Tests for Arbitrary Values System
//!
//! This module tests the arbitrary values functionality that allows users
//! to specify custom values for Tailwind CSS properties using square brackets.

use tailwind_rs_core::*;

#[test]
fn test_arbitrary_width_values() {
    // Test arbitrary width values
    let builder = ClassBuilder::new()
        .class("w-[100px]")
        .class("w-[50%]")
        .class("w-[calc(100%-2rem)]")
        .class("w-[theme(spacing.32)]");

    let result = builder.build();
    assert!(result.has_class("w-[100px]"));
    assert!(result.has_class("w-[50%]"));
    assert!(result.has_class("w-[calc(100%-2rem)]"));
    assert!(result.has_class("w-[theme(spacing.32)]"));
}

#[test]
fn test_arbitrary_color_values() {
    // Test arbitrary color values
    let builder = ClassBuilder::new()
        .class("bg-[#1da1f2]")
        .class("text-[rgb(255,0,0)]")
        .class("border-[hsl(120,100%,50%)]")
        .class("bg-[var(--my-color)]");

    let result = builder.build();
    assert!(result.has_class("bg-[#1da1f2]"));
    assert!(result.has_class("text-[rgb(255,0,0)]"));
    assert!(result.has_class("border-[hsl(120,100%,50%)]"));
    assert!(result.has_class("bg-[var(--my-color)]"));
}

#[test]
fn test_arbitrary_spacing_values() {
    // Test arbitrary spacing values
    let builder = ClassBuilder::new()
        .class("p-[13px]")
        .class("m-[0.5rem]")
        .class("gap-[2.5]")
        .class("space-x-[1.5rem]");

    let result = builder.build();
    assert!(result.has_class("p-[13px]"));
    assert!(result.has_class("m-[0.5rem]"));
    assert!(result.has_class("gap-[2.5]"));
    assert!(result.has_class("space-x-[1.5rem]"));
}

#[test]
fn test_arbitrary_font_size_values() {
    // Test arbitrary font size values
    let builder = ClassBuilder::new()
        .class("text-[13px]")
        .class("text-[0.875rem]")
        .class("text-[1.2em]")
        .class("text-[clamp(1rem,2.5vw,2rem)]");

    let result = builder.build();
    assert!(result.has_class("text-[13px]"));
    assert!(result.has_class("text-[0.875rem]"));
    assert!(result.has_class("text-[1.2em]"));
    assert!(result.has_class("text-[clamp(1rem,2.5vw,2rem)]"));
}

#[test]
fn test_arbitrary_shadow_values() {
    // Test arbitrary shadow values
    let builder = ClassBuilder::new()
        .class("shadow-[0_4px_6px_-1px_rgba(0,0,0,0.1)]")
        .class("shadow-[inset_0_2px_4px_0_rgba(0,0,0,0.06)]")
        .class("shadow-[0_0_0_1px_rgba(59,130,246,0.5)]");

    let result = builder.build();
    assert!(result.has_class("shadow-[0_4px_6px_-1px_rgba(0,0,0,0.1)]"));
    assert!(result.has_class("shadow-[inset_0_2px_4px_0_rgba(0,0,0,0.06)]"));
    assert!(result.has_class("shadow-[0_0_0_1px_rgba(59,130,246,0.5)]"));
}

#[test]
fn test_arbitrary_transform_values() {
    // Test arbitrary transform values
    let builder = ClassBuilder::new()
        .class("rotate-[17deg]")
        .class("scale-[1.7]")
        .class("translate-x-[117px]")
        .class("skew-x-[12deg]");

    let result = builder.build();
    assert!(result.has_class("rotate-[17deg]"));
    assert!(result.has_class("scale-[1.7]"));
    assert!(result.has_class("translate-x-[117px]"));
    assert!(result.has_class("skew-x-[12deg]"));
}

#[test]
fn test_arbitrary_animation_values() {
    // Test arbitrary animation values
    let builder = ClassBuilder::new()
        .class("animate-[spin_1s_linear_infinite]")
        .class("animate-[bounce_1s_infinite]")
        .class("animate-[ping_1s_cubic-bezier(0,0,0.2,1)_infinite]");

    let result = builder.build();
    assert!(result.has_class("animate-[spin_1s_linear_infinite]"));
    assert!(result.has_class("animate-[bounce_1s_infinite]"));
    assert!(result.has_class("animate-[ping_1s_cubic-bezier(0,0,0.2,1)_infinite]"));
}

#[test]
fn test_arbitrary_values_validation() {
    // Test that arbitrary values are properly validated
    let builder = ClassBuilder::new();

    // Valid arbitrary values should be accepted
    let valid_values = vec![
        "w-[100px]",
        "bg-[#ff0000]",
        "text-[1.5rem]",
        "p-[calc(100%-2rem)]",
        "shadow-[0_4px_6px_rgba(0,0,0,0.1)]",
    ];

    for value in valid_values {
        let result = builder.clone().class(value).build();
        assert!(
            result.has_class(value),
            "Failed to include valid arbitrary value: {}",
            value
        );
    }
}

#[test]
fn test_arbitrary_values_with_breakpoints() {
    // Test arbitrary values with responsive breakpoints
    let builder = ClassBuilder::new()
        .class("w-[100px]")
        .class("md:w-[200px]")
        .class("lg:w-[300px]")
        .class("xl:w-[400px]");

    let result = builder.build();
    assert!(result.has_class("w-[100px]"));
    assert!(result.has_class("md:w-[200px]"));
    assert!(result.has_class("lg:w-[300px]"));
    assert!(result.has_class("xl:w-[400px]"));
}

#[test]
fn test_arbitrary_values_with_pseudo_classes() {
    // Test arbitrary values with pseudo-classes
    let builder = ClassBuilder::new()
        .class("w-[100px]")
        .class("hover:w-[200px]")
        .class("focus:w-[300px]")
        .class("active:w-[400px]");

    let result = builder.build();
    assert!(result.has_class("w-[100px]"));
    assert!(result.has_class("hover:w-[200px]"));
    assert!(result.has_class("focus:w-[300px]"));
    assert!(result.has_class("active:w-[400px]"));
}

#[test]
fn test_arbitrary_values_performance() {
    // Test performance with many arbitrary values
    let mut builder = ClassBuilder::new();

    // Add 100 arbitrary values
    for i in 0..100 {
        builder = builder.class(&format!("w-[{}px]", i));
    }

    let start = std::time::Instant::now();
    let result = builder.build();
    let duration = start.elapsed();

    // Should complete in reasonable time (< 10ms)
    assert!(
        duration.as_millis() < 10,
        "Arbitrary values processing took too long: {:?}",
        duration
    );

    // Should contain all values
    for i in 0..100 {
        assert!(result.has_class(&format!("w-[{}px]", i)));
    }
}
