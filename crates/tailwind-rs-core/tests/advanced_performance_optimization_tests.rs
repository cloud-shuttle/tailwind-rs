#[cfg(test)]
mod advanced_performance_optimization_tests {
    use std::time::Duration;
    use tailwind_rs_core::utilities::advanced_performance_optimization::*;

    #[test]
    fn test_advanced_css_minifier_creation() {
        let minifier = AdvancedCssMinifier::new();
        assert!(!minifier.strategies.is_empty());
        assert_eq!(minifier.compression_level, 6);
        assert!(minifier.remove_comments);
        assert!(minifier.remove_whitespace);
    }

    #[test]
    fn test_css_minification_whitespace_removal() {
        let minifier = AdvancedCssMinifier::new();
        let css = "
        .test-class {
            color: red;
            background: blue;
        }
        ";

        let minified = minifier.minify(css);
        assert!(!minified.contains("\n"));
        assert!(minified.contains(".test-class"));
    }

    #[test]
    fn test_css_minification_comment_removal() {
        let minifier = AdvancedCssMinifier::new();
        let css = "
        /* This is a comment */
        .test-class {
            color: red;
        }
        ";

        let minified = minifier.minify(css);
        assert!(!minified.contains("/* This is a comment */"));
    }

    #[test]
    fn test_css_minification_color_compression() {
        let minifier = AdvancedCssMinifier::new();
        let css = "
        .test-class {
            color: #ffffff;
            background: #000000;
        }
        "#;

        let minified = minifier.minify(css);
        assert!(minified.contains("#fff"));
        assert!(minified.contains("#000"));
    }

    #[test]
    fn test_critical_css_extractor_creation() {
        let extractor = CriticalCssExtractor::new();
        assert_eq!(extractor.viewport_width, 1920);
        assert_eq!(extractor.viewport_height, 1080);
        assert!(extractor.critical_selectors.is_empty());
    }

    #[test]
    fn test_critical_css_extraction() {
        let extractor = CriticalCssExtractor::new();
        let css = "
        body { margin: 0; padding: 0; }
        .header { background: #fff; }
        .footer { background: #000; }
        .sidebar { width: 200px; }
        "#;

        let critical = extractor.extract_critical_css(css);
        assert!(critical.contains("body"));
        assert!(critical.contains("header"));
    }

    #[test]
    fn test_critical_css_extraction_with_custom_selectors() {
        let mut extractor = CriticalCssExtractor::new();
        extractor
            .critical_selectors
            .insert(".custom-critical".to_string());

        let css = "
        .custom-critical { color: red; }
        .non-critical { color: blue; }
        "#;

        let critical = extractor.extract_critical_css(css);
        assert!(critical.contains("custom-critical"));
    }

    #[test]
    fn test_lazy_loading_optimizer_creation() {
        let optimizer = LazyLoadingOptimizer::new();
        assert!(!optimizer.strategies.is_empty());
        assert_eq!(optimizer.observer_options.threshold, 0.1);
        assert!(optimizer.preload_critical);
    }

    #[test]
    fn test_lazy_loading_js_generation() {
        let optimizer = LazyLoadingOptimizer::new();
        let js = optimizer.generate_lazy_loading_js();

        assert!(js.contains("IntersectionObserver"));
        assert!(js.contains("data-lazy-classes"));
        assert!(js.contains("50px")); // rootMargin
        assert!(js.contains("0.1")); // threshold
    }

    #[test]
    fn test_lazy_loading_css_generation() {
        let optimizer = LazyLoadingOptimizer::new();
        let css = optimizer.generate_lazy_loading_css();

        assert!(css.contains("[data-lazy-classes]"));
        assert!(css.contains("opacity: 0"));
        assert!(css.contains("transition"));
        assert!(css.contains("html, body"));
    }

    #[test]
    fn test_bundle_splitter_creation() {
        let splitter = BundleSplitter::new();
        assert!(!splitter.strategies.is_empty());
        assert!(splitter.chunk_size_limits.is_empty());
        assert!(splitter.dependencies.is_empty());
    }

    #[test]
    fn test_bundle_splitting_by_feature() {
        let splitter = BundleSplitter::new();
        let css = "
        .display-block { display: block; }
        .margin-4 { margin: 1rem; }
        .color-red { color: red; }
        .font-bold { font-weight: bold; }
        "#;

        let chunks = splitter.split_bundle(css);
        assert!(!chunks.is_empty());

        // Should have layout, spacing, colors, and typography chunks
        assert!(
            chunks.contains_key("layout")
                || chunks.contains_key("spacing")
                || chunks.contains_key("colors")
                || chunks.contains_key("typography")
        );
    }

    #[test]
    fn test_bundle_splitting_by_size() {
        let mut splitter = BundleSplitter::new();
        splitter.strategies = vec![SplitStrategy::SizeBased];

        // Create a large CSS string that exceeds the 50KB threshold
        // Use realistic CSS with newlines for better testing
        let css_rule = ".test { color: red; background: blue; padding: 10px; margin: 5px; border: 1px solid black; }\n";
        let large_css = css_rule.repeat(2000); // 2000 rules should exceed 50KB
        let chunks = splitter.split_bundle(&large_css);

        // Debug output
        println!("CSS size: {} bytes", large_css.len());
        println!("Number of chunks: {}", chunks.len());
        for (key, value) in &chunks {
            println!("Chunk {}: {} bytes", key, value.len());
        }

        assert!(!chunks.is_empty());
        assert!(
            chunks.len() > 1,
            "Expected multiple chunks, got {} chunks",
            chunks.len()
        ); // Should be split into multiple chunks
    }

    #[test]
    fn test_memory_optimizer_creation() {
        let optimizer = MemoryOptimizer::new();
        assert_eq!(optimizer.limits.max_heap_size, 50 * 1024 * 1024);
        assert_eq!(optimizer.limits.max_object_count, 10000);
        assert!(!optimizer.strategies.is_empty());
    }

    #[test]
    fn test_memory_optimization() {
        let optimizer = MemoryOptimizer::new();
        let data = "test data for optimization";
        let optimized = optimizer.optimize_memory(data);

        assert_eq!(optimized, data); // Basic implementation returns same data
    }

    #[test]
    fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        let metrics = monitor.get_metrics();

        assert_eq!(metrics.cpu_usage, 0.0);
        assert_eq!(metrics.memory_usage, 0);
        assert_eq!(metrics.render_time, 0.0);
        assert_eq!(metrics.frame_rate, 0.0);
    }

    #[test]
    fn test_performance_threshold_checking() {
        let mut monitor = PerformanceMonitor::new();

        // Set metrics that exceed thresholds
        monitor.metrics.cpu_usage = 90.0; // Above 80% threshold
        monitor.metrics.memory_usage = 150 * 1024 * 1024; // Above 100MB threshold
        monitor.metrics.render_time = 20.0; // Above 16.67ms threshold
        monitor.metrics.frame_rate = 25.0; // Below 30fps threshold
        monitor.metrics.js_execution_time = 6.0; // Above 5s threshold

        let alerts = monitor.check_thresholds();
        assert!(alerts.contains(&AlertType::CpuUsage));
        assert!(alerts.contains(&AlertType::MemoryUsage));
        assert!(alerts.contains(&AlertType::RenderTime));
        assert!(alerts.contains(&AlertType::FrameRate));
        assert!(alerts.contains(&AlertType::JsExecutionTime));
    }

    #[test]
    fn test_advanced_optimization_result_creation() {
        let result = AdvancedOptimizationResult {
            original_metrics: OptimizationMetrics {
                bundle_size: 100000,
                class_count: 1000,
                rule_count: 500,
                parse_time: 10.0,
                render_time: 5.0,
                memory_usage: 50000000,
                cpu_usage: 50.0,
            },
            optimized_metrics: OptimizationMetrics {
                bundle_size: 50000,
                class_count: 500,
                rule_count: 250,
                parse_time: 5.0,
                render_time: 2.5,
                memory_usage: 25000000,
                cpu_usage: 25.0,
            },
            strategies_applied: vec!["minification".to_string(), "tree-shaking".to_string()],
            improvements: PerformanceImprovements {
                size_reduction: 50.0,
                parse_time_improvement: 50.0,
                render_time_improvement: 50.0,
                memory_reduction: 50.0,
                cpu_reduction: 50.0,
            },
            recommendations: vec!["Consider using CSS modules".to_string()],
            warnings: vec!["Some optimizations may affect functionality".to_string()],
        };

        assert_eq!(result.original_metrics.bundle_size, 100000);
        assert_eq!(result.optimized_metrics.bundle_size, 50000);
        assert_eq!(result.improvements.size_reduction, 50.0);
        assert_eq!(result.strategies_applied.len(), 2);
        assert_eq!(result.recommendations.len(), 1);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_advanced_optimization_result_display() {
        let result = AdvancedOptimizationResult {
            original_metrics: OptimizationMetrics {
                bundle_size: 100000,
                class_count: 1000,
                rule_count: 500,
                parse_time: 10.0,
                render_time: 5.0,
                memory_usage: 50000000,
                cpu_usage: 50.0,
            },
            optimized_metrics: OptimizationMetrics {
                bundle_size: 50000,
                class_count: 500,
                rule_count: 250,
                parse_time: 5.0,
                render_time: 2.5,
                memory_usage: 25000000,
                cpu_usage: 25.0,
            },
            strategies_applied: vec!["minification".to_string(), "tree-shaking".to_string()],
            improvements: PerformanceImprovements {
                size_reduction: 50.0,
                parse_time_improvement: 50.0,
                render_time_improvement: 50.0,
                memory_reduction: 50.0,
                cpu_reduction: 50.0,
            },
            recommendations: vec!["Consider using CSS modules".to_string()],
            warnings: vec!["Some optimizations may affect functionality".to_string()],
        };

        let display = format!("{}", result);
        assert!(display.contains("Advanced Optimization Result"));
        assert!(display.contains("100000 bytes"));
        assert!(display.contains("50000 bytes"));
        assert!(display.contains("50.0%"));
        assert!(display.contains("minification"));
        assert!(display.contains("Consider using CSS modules"));
        assert!(display.contains("Some optimizations may affect functionality"));
    }

    #[test]
    fn test_minification_strategies() {
        let strategies = vec![
            MinificationStrategy::WhitespaceRemoval,
            MinificationStrategy::CommentRemoval,
            MinificationStrategy::SelectorOptimization,
            MinificationStrategy::RuleMerging,
            MinificationStrategy::PropertyOptimization,
        ];

        assert_eq!(strategies.len(), 5);
    }

    #[test]
    fn test_lazy_loading_strategies() {
        let strategies = vec![
            LazyLoadingStrategy::ScrollBased,
            LazyLoadingStrategy::IntersectionBased,
            LazyLoadingStrategy::HoverBased,
            LazyLoadingStrategy::FocusBased,
            LazyLoadingStrategy::MediaQueryBased,
        ];

        assert_eq!(strategies.len(), 5);
    }

    #[test]
    fn test_bundle_split_strategies() {
        let strategies = vec![
            SplitStrategy::FeatureBased,
            SplitStrategy::UsageBased,
            SplitStrategy::DependencyBased,
            SplitStrategy::SizeBased,
            SplitStrategy::CriticalPathBased,
        ];

        assert_eq!(strategies.len(), 5);
    }

    #[test]
    fn test_memory_optimization_strategies() {
        let strategies = vec![
            MemoryOptimizationStrategy::ObjectPooling,
            MemoryOptimizationStrategy::StringInterning,
            MemoryOptimizationStrategy::WeakReferences,
            MemoryOptimizationStrategy::LazyInitialization,
            MemoryOptimizationStrategy::MemoryCompression,
            MemoryOptimizationStrategy::GcOptimization,
        ];

        assert_eq!(strategies.len(), 6);
    }

    #[test]
    fn test_gc_triggers() {
        let triggers = vec![
            GcTrigger::MemoryThreshold(0.8),
            GcTrigger::ObjectCount(5000),
            GcTrigger::TimeInterval(Duration::from_secs(30)),
            GcTrigger::IdleTime,
        ];

        assert_eq!(triggers.len(), 4);
    }

    #[test]
    fn test_alert_types() {
        let alert_types = vec![
            AlertType::CpuUsage,
            AlertType::MemoryUsage,
            AlertType::RenderTime,
            AlertType::FrameRate,
            AlertType::JsExecutionTime,
        ];

        assert_eq!(alert_types.len(), 5);
    }

    #[test]
    fn test_observer_options() {
        let options = ObserverOptions {
            root_margin: "50px".to_string(),
            threshold: 0.1,
            observe_once: true,
        };

        assert_eq!(options.root_margin, "50px");
        assert_eq!(options.threshold, 0.1);
        assert!(options.observe_once);
    }

    #[test]
    fn test_memory_limits() {
        let limits = MemoryLimits {
            max_heap_size: 100 * 1024 * 1024,
            max_object_count: 5000,
            max_string_length: 1000,
            max_array_size: 500,
        };

        assert_eq!(limits.max_heap_size, 100 * 1024 * 1024);
        assert_eq!(limits.max_object_count, 5000);
        assert_eq!(limits.max_string_length, 1000);
        assert_eq!(limits.max_array_size, 500);
    }

    #[test]
    fn test_performance_thresholds() {
        let thresholds = PerformanceThresholds {
            max_cpu_usage: 80.0,
            max_memory_usage: 100 * 1024 * 1024,
            max_render_time: 16.67,
            min_frame_rate: 30.0,
            max_js_execution_time: 5.0,
        };

        assert_eq!(thresholds.max_cpu_usage, 80.0);
        assert_eq!(thresholds.max_memory_usage, 100 * 1024 * 1024);
        assert_eq!(thresholds.max_render_time, 16.67);
        assert_eq!(thresholds.min_frame_rate, 30.0);
        assert_eq!(thresholds.max_js_execution_time, 5.0);
    }

    #[test]
    fn test_monitoring_interval() {
        let interval = MonitoringInterval {
            duration: Duration::from_secs(1),
            metrics: vec!["cpu_usage".to_string(), "memory_usage".to_string()],
            callback: "handle_metrics".to_string(),
        };

        assert_eq!(interval.duration, Duration::from_secs(1));
        assert_eq!(interval.metrics.len(), 2);
        assert_eq!(interval.callback, "handle_metrics");
    }

    #[test]
    fn test_alert_handler() {
        let handler = AlertHandler {
            alert_type: AlertType::CpuUsage,
            threshold: 80.0,
            handler: "handle_cpu_alert".to_string(),
            enabled: true,
        };

        assert!(matches!(handler.alert_type, AlertType::CpuUsage));
        assert_eq!(handler.threshold, 80.0);
        assert_eq!(handler.handler, "handle_cpu_alert");
        assert!(handler.enabled);
    }
}
