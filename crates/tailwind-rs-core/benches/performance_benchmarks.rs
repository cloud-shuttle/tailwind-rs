//! Performance benchmarks for tailwind-rs-core
//!
//! This module contains benchmarks to measure the performance of
//! various components and ensure they meet performance requirements.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tailwind_rs_core::css_generator::CssGenerator;
use tailwind_rs_core::classes::ClassBuilder;
use tailwind_rs_core::theme::ThemeConfig;

/// Benchmark CSS generation performance
fn benchmark_css_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("css_generation");
    
    // Test with different numbers of classes
    for class_count in [10, 100, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::new("generate_css", class_count),
            class_count,
            |b, &class_count| {
                let mut generator = CssGenerator::new();
                
                // Pre-populate with classes
                for i in 0..class_count {
                    let class = format!("test-class-{}", i);
                    generator.add_class(&class).unwrap();
                }
                
                b.iter(|| {
                    black_box(generator.generate_css())
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark class parsing performance
fn benchmark_class_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("class_parsing");
    
    let test_classes = vec![
        "bg-blue-500",
        "text-white",
        "p-4",
        "rounded-lg",
        "hover:bg-blue-600",
        "focus:ring-2",
        "focus:ring-blue-300",
        "translate-x-1",
        "translate-y-2",
        "scale-x-50",
        "scale-y-75",
        "border-2",
        "border-gray-300",
        "shadow-lg",
        "transition-all",
        "duration-300",
        "ease-in-out",
        "transform",
        "rotate-45",
        "skew-x-12",
    ];
    
    group.bench_function("parse_single_class", |b| {
        let mut generator = CssGenerator::new();
        b.iter(|| {
            for class in &test_classes {
                black_box(generator.add_class(class).unwrap());
            }
        });
    });
    
    group.bench_function("parse_batch_classes", |b| {
        b.iter(|| {
            let mut generator = CssGenerator::new();
            for class in &test_classes {
                black_box(generator.add_class(class).unwrap());
            }
            black_box(generator.generate_css())
        });
    });
    
    group.finish();
}

/// Benchmark new parser performance
fn benchmark_new_parsers(c: &mut Criterion) {
    let mut group = c.benchmark_group("new_parsers");
    
    // Test BasicTransformsParser performance
    group.bench_function("basic_transforms_parser", |b| {
        let mut generator = CssGenerator::new();
        let transform_classes = vec![
            "translate-x-1", "translate-x-2", "translate-x-4", "translate-x-8",
            "translate-y-1", "translate-y-2", "translate-y-4", "translate-y-8",
            "translate-x-px", "translate-y-px",
        ];
        
        b.iter(|| {
            for class in &transform_classes {
                black_box(generator.add_class(class).unwrap());
            }
        });
    });
    
    // Test ScaleParser performance
    group.bench_function("scale_parser", |b| {
        let mut generator = CssGenerator::new();
        let scale_classes = vec![
            "scale-x-50", "scale-x-75", "scale-x-90", "scale-x-100",
            "scale-y-50", "scale-y-75", "scale-y-90", "scale-y-100",
            "scale-x-105", "scale-x-110", "scale-x-125", "scale-x-150",
        ];
        
        b.iter(|| {
            for class in &scale_classes {
                black_box(generator.add_class(class).unwrap());
            }
        });
    });
    
    group.finish();
}

/// Benchmark theme system performance
fn benchmark_theme_system(c: &mut Criterion) {
    let mut group = c.benchmark_group("theme_system");
    
    group.bench_function("create_default_theme", |b| {
        b.iter(|| {
            black_box(ThemeConfig::default())
        });
    });
    
    group.bench_function("create_custom_theme", |b| {
        b.iter(|| {
            black_box(ThemeConfig::new("custom"))
        });
    });
    
    group.finish();
}

/// Benchmark class builder performance
fn benchmark_class_builder(c: &mut Criterion) {
    let mut group = c.benchmark_group("class_builder");
    
    group.bench_function("build_single_class", |b| {
        b.iter(|| {
            let builder = ClassBuilder::new();
            let class_set = builder.class("test-class").build();
            black_box(class_set.to_css_classes())
        });
    });
    
    group.bench_function("build_multiple_classes", |b| {
        b.iter(|| {
            let builder = ClassBuilder::new();
            let class_set = builder
                .class("bg-blue-500")
                .class("text-white")
                .class("p-4")
                .class("rounded-lg")
                .build();
            black_box(class_set.to_css_classes())
        });
    });
    
    group.finish();
}

/// Benchmark memory usage
fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    group.bench_function("large_css_generation", |b| {
        b.iter(|| {
            let mut generator = CssGenerator::new();
            
            // Generate a large number of classes
            for i in 0..10000 {
                let class = format!("test-class-{}", i);
                generator.add_class(&class).unwrap();
            }
            
            black_box(generator.generate_css())
        });
    });
    
    group.finish();
}

/// Benchmark concurrent access
fn benchmark_concurrent_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_access");
    
    group.bench_function("concurrent_css_generation", |b| {
        b.iter(|| {
            let mut generator = CssGenerator::new();
            
            // Simulate concurrent access patterns
            for i in 0..1000 {
                let class = format!("concurrent-class-{}", i);
                generator.add_class(&class).unwrap();
            }
            
            black_box(generator.generate_css())
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_css_generation,
    benchmark_class_parsing,
    benchmark_new_parsers,
    benchmark_theme_system,
    benchmark_class_builder,
    benchmark_memory_usage,
    benchmark_concurrent_access
);

criterion_main!(benches);
