#[cfg(test)]
mod advanced_performance_optimization_performance_tests {
    use tailwind_rs_core::utilities::advanced_performance_optimization::*;
    use std::time::Instant;

    const ITERATIONS: usize = 1000;
    const LARGE_CSS_SIZE: usize = 10000; // 10KB CSS

    #[test]
    fn test_css_minification_performance() {
        let minifier = AdvancedCssMinifier::new();
        let large_css = generate_large_css(LARGE_CSS_SIZE);
        
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _minified = minifier.minify(&large_css);
        }
        let duration = start.elapsed();
        
        println!("CSS minification performance: {:?} for {} iterations", duration, ITERATIONS);
        assert!(duration.as_millis() < 1000); // Should complete in under 1 second
    }

    #[test]
    fn test_critical_css_extraction_performance() {
        let extractor = CriticalCssExtractor::new();
        let large_css = generate_large_css(LARGE_CSS_SIZE);
        
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _critical = extractor.extract_critical_css(&large_css);
        }
        let duration = start.elapsed();
        
        println!("Critical CSS extraction performance: {:?} for {} iterations", duration, ITERATIONS);
        assert!(duration.as_millis() < 1000); // Should complete in under 1 second
    }

    #[test]
    fn test_lazy_loading_js_generation_performance() {
        let optimizer = LazyLoadingOptimizer::new();
        
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _js = optimizer.generate_lazy_loading_js();
        }
        let duration = start.elapsed();
        
        println!("Lazy loading JS generation performance: {:?} for {} iterations", duration, ITERATIONS);
        assert!(duration.as_millis() < 100); // Should be very fast
    }

    #[test]
    fn test_lazy_loading_css_generation_performance() {
        let optimizer = LazyLoadingOptimizer::new();
        
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _css = optimizer.generate_lazy_loading_css();
        }
        let duration = start.elapsed();
        
        println!("Lazy loading CSS generation performance: {:?} for {} iterations", duration, ITERATIONS);
        assert!(duration.as_millis() < 100); // Should be very fast
    }

    #[test]
    fn test_bundle_splitting_performance() {
        let splitter = BundleSplitter::new();
        let large_css = generate_large_css(LARGE_CSS_SIZE);
        
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _chunks = splitter.split_bundle(&large_css);
        }
        let duration = start.elapsed();
        
        println!("Bundle splitting performance: {:?} for {} iterations", duration, ITERATIONS);
        assert!(duration.as_millis() < 5000); // Should complete in under 5 seconds (CI-friendly threshold)
    }

    #[test]
    fn test_memory_optimization_performance() {
        let optimizer = MemoryOptimizer::new();
        let large_data = generate_large_data(LARGE_CSS_SIZE);
        
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _optimized = optimizer.optimize_memory(&large_data);
        }
        let duration = start.elapsed();
        
        println!("Memory optimization performance: {:?} for {} iterations", duration, ITERATIONS);
        assert!(duration.as_millis() < 1000); // Should complete in under 1 second
    }

    #[test]
    fn test_performance_monitoring_overhead() {
        let monitor = PerformanceMonitor::new();
        
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _metrics = monitor.get_metrics();
            let _alerts = monitor.check_thresholds();
        }
        let duration = start.elapsed();
        
        println!("Performance monitoring overhead: {:?} for {} iterations", duration, ITERATIONS);
        assert!(duration.as_millis() < 100); // Should be very fast
    }

    #[test]
    fn test_advanced_optimization_result_creation_performance() {
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _result = create_sample_optimization_result();
        }
        let duration = start.elapsed();
        
        println!("Advanced optimization result creation performance: {:?} for {} iterations", duration, ITERATIONS);
        assert!(duration.as_millis() < 100); // Should be very fast
    }

    #[test]
    fn test_advanced_optimization_result_display_performance() {
        let result = create_sample_optimization_result();
        
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _display = format!("{}", result);
        }
        let duration = start.elapsed();
        
        println!("Advanced optimization result display performance: {:?} for {} iterations", duration, ITERATIONS);
        assert!(duration.as_millis() < 100); // Should be very fast
    }

    #[test]
    fn test_minification_strategy_performance() {
        let minifier = AdvancedCssMinifier::new();
        let css = generate_large_css(LARGE_CSS_SIZE);
        
        // Test individual strategies
        let strategies = vec![
            MinificationStrategy::WhitespaceRemoval,
            MinificationStrategy::CommentRemoval,
            MinificationStrategy::SelectorOptimization,
            MinificationStrategy::PropertyOptimization,
            MinificationStrategy::ColorCompression,
        ];
        
        for strategy in strategies {
            let mut test_minifier = minifier.clone();
            test_minifier.strategies = vec![strategy.clone()];
            
            let start = Instant::now();
            for _ in 0..100 { // Fewer iterations for individual strategies
                let _minified = test_minifier.minify(&css);
            }
            let duration = start.elapsed();
            
            println!("Strategy {:?} performance: {:?} for 100 iterations", strategy, duration);
            assert!(duration.as_millis() < 500); // Each strategy should be fast
        }
    }

    #[test]
    fn test_bundle_split_strategy_performance() {
        let css = generate_large_css(LARGE_CSS_SIZE);
        
        let strategies = vec![
            SplitStrategy::FeatureBased,
            SplitStrategy::UsageBased,
            SplitStrategy::DependencyBased,
            SplitStrategy::SizeBased,
            SplitStrategy::CriticalPathBased,
        ];
        
        for strategy in strategies {
            let mut splitter = BundleSplitter::new();
            splitter.strategies = vec![strategy.clone()];
            
            let start = Instant::now();
            for _ in 0..100 { // Fewer iterations for individual strategies
                let _chunks = splitter.split_bundle(&css);
            }
            let duration = start.elapsed();
            
            println!("Split strategy {:?} performance: {:?} for 100 iterations", strategy, duration);
            assert!(duration.as_millis() < 500); // Each strategy should be fast
        }
    }

    #[test]
    fn test_memory_optimization_strategy_performance() {
        let data = generate_large_data(LARGE_CSS_SIZE);
        
        let strategies = vec![
            MemoryOptimizationStrategy::ObjectPooling,
            MemoryOptimizationStrategy::StringInterning,
            MemoryOptimizationStrategy::LazyInitialization,
            MemoryOptimizationStrategy::WeakReferences,
            MemoryOptimizationStrategy::MemoryCompression,
            MemoryOptimizationStrategy::GcOptimization,
        ];
        
        for strategy in strategies {
            let mut optimizer = MemoryOptimizer::new();
            optimizer.strategies = vec![strategy.clone()];
            
            let start = Instant::now();
            for _ in 0..100 { // Fewer iterations for individual strategies
                let _optimized = optimizer.optimize_memory(&data);
            }
            let duration = start.elapsed();
            
            println!("Memory strategy {:?} performance: {:?} for 100 iterations", strategy, duration);
            assert!(duration.as_millis() < 500); // Each strategy should be fast
        }
    }

    #[test]
    fn test_lazy_loading_strategy_performance() {
        let strategies = vec![
            LazyLoadingStrategy::ScrollBased,
            LazyLoadingStrategy::IntersectionBased,
            LazyLoadingStrategy::HoverBased,
            LazyLoadingStrategy::FocusBased,
            LazyLoadingStrategy::MediaQueryBased,
        ];
        
        for strategy in strategies {
            let mut optimizer = LazyLoadingOptimizer::new();
            optimizer.strategies = vec![strategy.clone()];
            
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                let _js = optimizer.generate_lazy_loading_js();
                let _css = optimizer.generate_lazy_loading_css();
            }
            let duration = start.elapsed();
            
            println!("Lazy loading strategy {:?} performance: {:?} for {} iterations", strategy, duration, ITERATIONS);
            assert!(duration.as_millis() < 100); // Should be very fast
        }
    }

    #[test]
    fn test_gc_trigger_performance() {
        let triggers = vec![
            GcTrigger::MemoryThreshold(0.8),
            GcTrigger::ObjectCount(5000),
            GcTrigger::TimeInterval(std::time::Duration::from_secs(30)),
            GcTrigger::IdleTime,
        ];
        
        for trigger in triggers {
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                // Simulate trigger checking
                match trigger {
                    GcTrigger::MemoryThreshold(_) => { /* Check memory */ }
                    GcTrigger::ObjectCount(_) => { /* Check object count */ }
                    GcTrigger::TimeInterval(_) => { /* Check time */ }
                    GcTrigger::IdleTime => { /* Check idle time */ }
                }
            }
            let duration = start.elapsed();
            
            println!("GC trigger {:?} performance: {:?} for {} iterations", trigger, duration, ITERATIONS);
            assert!(duration.as_millis() < 10); // Should be extremely fast
        }
    }

    #[test]
    fn test_alert_type_performance() {
        let alert_types = vec![
            AlertType::CpuUsage,
            AlertType::MemoryUsage,
            AlertType::RenderTime,
            AlertType::FrameRate,
            AlertType::JsExecutionTime,
        ];
        
        for alert_type in alert_types {
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                // Simulate alert type checking
                match alert_type {
                    AlertType::CpuUsage => { /* Check CPU */ }
                    AlertType::MemoryUsage => { /* Check memory */ }
                    AlertType::RenderTime => { /* Check render time */ }
                    AlertType::FrameRate => { /* Check frame rate */ }
                    AlertType::JsExecutionTime => { /* Check JS execution time */ }
                }
            }
            let duration = start.elapsed();
            
            println!("Alert type {:?} performance: {:?} for {} iterations", alert_type, duration, ITERATIONS);
            assert!(duration.as_millis() < 10); // Should be extremely fast
        }
    }

    #[test]
    fn test_memory_usage_under_load() {
        let minifier = AdvancedCssMinifier::new();
        let large_css = generate_large_css(LARGE_CSS_SIZE * 10); // 100KB CSS
        
        let start = Instant::now();
        let mut results = Vec::new();
        
        for _ in 0..100 { // Fewer iterations for memory test
            let minified = minifier.minify(&large_css);
            results.push(minified);
        }
        
        let duration = start.elapsed();
        println!("Memory usage under load: {:?} for 100 iterations of 100KB CSS", duration);
        
        // Clean up
        drop(results);
        
        assert!(duration.as_millis() < 5000); // Should complete in under 5 seconds
    }

    #[test]
    fn test_concurrent_optimization_performance() {
        let minifier = AdvancedCssMinifier::new();
        let splitter = BundleSplitter::new();
        let optimizer = MemoryOptimizer::new();
        
        let css = generate_large_css(LARGE_CSS_SIZE);
        let data = generate_large_data(LARGE_CSS_SIZE);
        
        let start = Instant::now();
        for _ in 0..100 { // Fewer iterations for concurrent test
            let _minified = minifier.minify(&css);
            let _chunks = splitter.split_bundle(&css);
            let _optimized = optimizer.optimize_memory(&data);
        }
        let duration = start.elapsed();
        
        println!("Concurrent optimization performance: {:?} for 100 iterations", duration);
        assert!(duration.as_millis() < 2000); // Should complete in under 2 seconds
    }

    // Helper functions
    fn generate_large_css(size: usize) -> String {
        let mut css = String::new();
        let mut current_size = 0;
        
        while current_size < size {
            let class_name = format!(".class-{}", current_size / 10);
            let rule = format!("{} {{ color: red; background: blue; margin: {}px; }}\n", 
                             class_name, current_size % 100);
            css.push_str(&rule);
            current_size += rule.len();
        }
        
        css
    }

    fn generate_large_data(size: usize) -> String {
        "test data ".repeat(size / 10)
    }

    fn create_sample_optimization_result() -> AdvancedOptimizationResult {
        AdvancedOptimizationResult {
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
        }
    }
}
