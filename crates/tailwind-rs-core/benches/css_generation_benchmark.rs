//! CSS Generation Performance Benchmarks
//!
//! This module contains performance benchmarks for CSS generation
//! to ensure the system maintains good performance characteristics.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tailwind_rs_core::css_generator::CssGenerator;
use tailwind_rs_core::classes::ClassBuilder;

fn benchmark_css_generation(c: &mut Criterion) {
    c.bench_function("css_generator_single_class", |b| {
        let mut generator = CssGenerator::new();
        b.iter(|| {
            black_box(generator.add_class("p-4").unwrap());
        });
    });

    c.bench_function("css_generator_multiple_classes", |b| {
        let mut generator = CssGenerator::new();
        let classes = vec![
            "p-4", "m-2", "text-center", "bg-blue-500", "flex", "justify-center",
            "items-center", "w-full", "h-16", "rounded-lg", "shadow-md"
        ];
        b.iter(|| {
            for class in &classes {
                black_box(generator.add_class(class).unwrap());
            }
        });
    });

    c.bench_function("class_builder_simple", |b| {
        b.iter(|| {
            let builder = ClassBuilder::new()
                .class("p-4")
                .class("m-2")
                .class("text-center");
            black_box(builder.build());
        });
    });

    c.bench_function("class_builder_complex", |b| {
        b.iter(|| {
            let builder = ClassBuilder::new()
                .class("p-4")
                .class("m-2")
                .class("text-center")
                .class("bg-blue-500")
                .class("flex")
                .class("justify-center")
                .class("items-center")
                .class("w-full")
                .class("h-16")
                .class("rounded-lg")
                .class("shadow-md");
            black_box(builder.build());
        });
    });

    c.bench_function("css_generator_large_stylesheet", |b| {
        let mut generator = CssGenerator::new();
        b.iter(|| {
            // Add many classes to simulate a large stylesheet
            for i in 1..=50 {
                black_box(generator.add_class(&format!("p-{}", i)).unwrap());
                black_box(generator.add_class(&format!("m-{}", i)).unwrap());
                black_box(generator.add_class(&format!("text-{}", ["xs", "sm", "base", "lg", "xl", "2xl"][i % 6])).unwrap());
            }
            black_box(generator.generate_css());
        });
    });
}

fn benchmark_parser_performance(c: &mut Criterion) {
    use tailwind_rs_core::css_generator::parsers::{basic_transforms::BasicTransformsParser, scale_parser::ScaleParser};

    c.bench_function("basic_transforms_parser", |b| {
        let parser = BasicTransformsParser::new();
        let test_classes = vec![
            "translate-x-1", "translate-x-2", "translate-x-4", "translate-x-8",
            "translate-y-1", "translate-y-2", "translate-y-4", "translate-y-8",
            "-translate-x-1", "-translate-y-2"
        ];

        b.iter(|| {
            for class in &test_classes {
                black_box(parser.parse_class(class));
            }
        });
    });

    c.bench_function("scale_parser", |b| {
        let parser = ScaleParser::new();
        let test_classes = vec![
            "scale-x-50", "scale-x-75", "scale-x-90", "scale-x-95", "scale-x-100",
            "scale-y-50", "scale-y-75", "scale-y-90", "scale-y-95", "scale-y-100"
        ];

        b.iter(|| {
            for class in &test_classes {
                black_box(parser.parse_class(class));
            }
        });
    });
}

criterion_group!(benches, benchmark_css_generation, benchmark_parser_performance);
criterion_main!(benches);
