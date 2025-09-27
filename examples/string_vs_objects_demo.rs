//! Tailwind-RS: String-Based vs Rust Objects Demo
//!
//! This example demonstrates both approaches to using Tailwind-RS:
//! 1. String-based classes (simple, fast)
//! 2. Rust objects (type-safe, advanced)

use tailwind_rs_core::{ClassBuilder, CssGenerator};

fn main() {
    println!("🎨 Tailwind-RS: String-Based vs Rust Objects Demo\n");

    // ========================================
    // 1. STRING-BASED APPROACH (Simple & Fast)
    // ========================================

    println!("📝 String-Based Classes:");
    println!("========================");

    // Basic string classes
    let button_classes = "px-4 py-2 rounded-md font-medium bg-blue-600 text-white hover:bg-blue-700 transition-colors";
    println!("Basic button: {}", button_classes);

    // Conditional classes
    let is_active = true;
    let conditional_classes = format!(
        "px-4 py-2 rounded-md font-medium {} transition-colors",
        if is_active {
            "bg-blue-600 text-white"
        } else {
            "bg-gray-200 text-gray-700"
        }
    );
    println!("Conditional: {}", conditional_classes);

    // Dynamic classes
    let size = "lg";
    let dynamic_classes = match size {
        "sm" => "px-2 py-1 text-sm bg-blue-600 text-white rounded-md",
        "md" => "px-4 py-2 text-base bg-blue-600 text-white rounded-md",
        "lg" => "px-6 py-3 text-lg bg-blue-600 text-white rounded-md",
        _ => "px-4 py-2 text-base bg-blue-600 text-white rounded-md",
    };
    println!("Dynamic (size={}): {}", size, dynamic_classes);

    println!();

    // ========================================
    // 2. RUST OBJECTS APPROACH (Type-Safe & Advanced)
    // ========================================

    println!("🔧 Rust Objects:");
    println!("================");

    // Using CssGenerator for CSS generation
    let mut generator = CssGenerator::new();
    let _ = generator.add_class("px-4");
    let _ = generator.add_class("py-2");
    let _ = generator.add_class("rounded-md");
    let _ = generator.add_class("font-medium");
    let _ = generator.add_class("bg-blue-600");
    let _ = generator.add_class("text-white");
    let _ = generator.add_class("hover:bg-blue-700");
    let _ = generator.add_class("transition-colors");

    let css = generator.generate_css();
    println!(
        "Generated CSS (first 100 chars): {}...",
        &css[..100.min(css.len())]
    );

    // Using ClassBuilder for programmatic class building
    let class_set = ClassBuilder::new()
        .class("px-4")
        .class("py-2")
        .class("rounded-md")
        .class("font-medium")
        .class("bg-blue-600")
        .class("text-white")
        .class("hover:bg-blue-700")
        .class("focus:ring-2 focus:ring-blue-500")
        .build();

    let classes = class_set.to_css_classes();
    println!("ClassBuilder result: {}", classes);

    println!();

    // ========================================
    // 3. COMPLEX EXAMPLE: Component Variants
    // ========================================

    println!("🎯 Complex Example - Component Variants:");
    println!("=========================================");

    #[derive(Debug, Clone)]
    enum ButtonVariant {
        Primary,
        Secondary,
        Danger,
    }

    #[derive(Debug, Clone)]
    enum ButtonSize {
        Small,
        Medium,
        Large,
    }

    // String-based approach
    fn create_button_string(variant: ButtonVariant, size: ButtonSize) -> String {
        let base = "rounded-md font-medium transition-colors";

        let variant_classes = match variant {
            ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
            ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300",
            ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700",
        };

        let size_classes = match size {
            ButtonSize::Small => "px-2 py-1 text-sm",
            ButtonSize::Medium => "px-4 py-2 text-base",
            ButtonSize::Large => "px-6 py-3 text-lg",
        };

        format!("{} {} {}", base, variant_classes, size_classes)
    }

    // Rust objects approach
    fn create_button_rust(variant: ButtonVariant, size: ButtonSize) -> String {
        let mut builder = ClassBuilder::new().class("rounded-md font-medium transition-colors");

        // Add variant classes
        match variant {
            ButtonVariant::Primary => {
                builder = builder
                    .class("bg-blue-600 text-white")
                    .class("hover:bg-blue-700");
            }
            ButtonVariant::Secondary => {
                builder = builder
                    .class("bg-gray-200 text-gray-900")
                    .class("hover:bg-gray-300");
            }
            ButtonVariant::Danger => {
                builder = builder
                    .class("bg-red-600 text-white")
                    .class("hover:bg-red-700");
            }
        };

        // Add size classes
        match size {
            ButtonSize::Small => builder = builder.class("px-2 py-1 text-sm"),
            ButtonSize::Medium => builder = builder.class("px-4 py-2 text-base"),
            ButtonSize::Large => builder = builder.class("px-6 py-3 text-lg"),
        };

        builder.build().to_css_classes()
    }

    // Test both approaches
    let variants = [
        ButtonVariant::Primary,
        ButtonVariant::Secondary,
        ButtonVariant::Danger,
    ];
    let sizes = [ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    for variant in &variants {
        for size in &sizes {
            let string_result = create_button_string(variant.clone(), size.clone());
            let rust_result = create_button_rust(variant.clone(), size.clone());

            println!("{:?} {:?}:", variant, size);
            println!("  String: {}", string_result);
            println!("  Rust:   {}", rust_result);
            println!("  Match:  {}", string_result == rust_result);
            println!();
        }
    }

    // ========================================
    // 4. PERFORMANCE COMPARISON
    // ========================================

    println!("⚡ Performance Comparison:");
    println!("=========================");

    use std::time::Instant;

    // String-based performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = "px-4 py-2 rounded-md font-medium bg-blue-600 text-white hover:bg-blue-700 transition-colors";
    }
    let string_duration = start.elapsed();

    // Rust objects performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = ClassBuilder::new()
            .class("px-4")
            .class("py-2")
            .class("rounded-md")
            .class("font-medium")
            .class("bg-blue-600")
            .class("text-white")
            .class("hover:bg-blue-700")
            .class("transition-colors")
            .build();
    }
    let rust_duration = start.elapsed();

    println!("String-based (1000 iterations): {:?}", string_duration);
    println!("Rust objects (1000 iterations): {:?}", rust_duration);
    println!(
        "String is {:.1}x faster",
        rust_duration.as_nanos() as f64 / string_duration.as_nanos() as f64
    );

    println!();
    println!("🎉 Demo complete! Both approaches work great - choose based on your needs!");
}
