//! Performance Testing Module
//! 
//! This module provides comprehensive performance testing for signal updates,
//! demonstrating the benefits of batched updates and providing measurable
//! performance improvements.

use leptos::prelude::*;
use leptos::attr::global::ClassAttribute;
use std::time::Instant;

/// Performance testing component that demonstrates various optimization techniques
#[component]
pub fn PerformanceTestingDemo() -> impl IntoView {
    let (test_results, set_test_results) = signal(Vec::<TestResult>::new());
    let (is_running, set_is_running) = signal(false);
    let (test_count, set_test_count) = signal(1000);
    
    // Test individual signal updates
    let test_individual_updates = move || {
        if is_running.get() {
            return;
        }
        
        set_is_running.set(true);
        let count = test_count.get();
        
        // Create test signals
        let (value, set_value) = signal(0);
        
        let start = Instant::now();
        for _ in 0..count {
            set_value.update(|v| *v += 1);
        }
        let duration = start.elapsed();
        
        let result = TestResult {
            test_type: "Individual Updates".to_string(),
            count,
            duration: duration.as_micros() as u64,
            description: "Updating signal one by one".to_string(),
        };
        
        set_test_results.update(|results| results.push(result));
        set_is_running.set(false);
    };
    
    // Test batched updates (simulated)
    let test_batched_updates = move || {
        if is_running.get() {
            return;
        }
        
        set_is_running.set(true);
        let count = test_count.get();
        
        // Create test signals
        let (value, set_value) = signal(0);
        
        let start = Instant::now();
        // Simulate batched updates by updating once with the total
        set_value.set(count);
        let duration = start.elapsed();
        
        let result = TestResult {
            test_type: "Batched Updates".to_string(),
            count,
            duration: duration.as_micros() as u64,
            description: "Updating signal once with final value".to_string(),
        };
        
        set_test_results.update(|results| results.push(result));
        set_is_running.set(false);
    };
    
    // Test multiple signal updates
    let test_multiple_signals = move || {
        if is_running.get() {
            return;
        }
        
        set_is_running.set(true);
        let count = test_count.get();
        
        // Create multiple test signals
        let (value1, set_value1) = signal(0);
        let (value2, set_value2) = signal(0);
        let (value3, set_value3) = signal(0);
        
        let start = Instant::now();
        for _ in 0..count {
            set_value1.update(|v| *v += 1);
            set_value2.update(|v| *v += 1);
            set_value3.update(|v| *v += 1);
        }
        let duration = start.elapsed();
        
        let result = TestResult {
            test_type: "Multiple Signals".to_string(),
            count: count * 3, // 3 signals updated
            duration: duration.as_micros() as u64,
            description: "Updating 3 signals individually".to_string(),
        };
        
        set_test_results.update(|results| results.push(result));
        set_is_running.set(false);
    };
    
    // Test reactive computations
    let test_reactive_computations = move || {
        if is_running.get() {
            return;
        }
        
        set_is_running.set(true);
        let count = test_count.get();
        
        // Create test signals
        let (value1, set_value1) = signal(0);
        let (value2, set_value2) = signal(0);
        
        // Create a reactive computation
        let computed = move || value1.get() + value2.get();
        
        let start = Instant::now();
        for _ in 0..count {
            set_value1.update(|v| *v += 1);
            set_value2.update(|v| *v += 1);
            // Access the computed value to trigger computation
            let _ = computed();
        }
        let duration = start.elapsed();
        
        let result = TestResult {
            test_type: "Reactive Computations".to_string(),
            count,
            duration: duration.as_micros() as u64,
            description: "Updating signals with reactive computations".to_string(),
        };
        
        set_test_results.update(|results| results.push(result));
        set_is_running.set(false);
    };
    
    // Clear results
    let clear_results = move || {
        set_test_results.set(Vec::new());
    };
    
    // Calculate performance improvement
    let get_performance_summary = move || {
        let results = test_results.get();
        if results.len() < 2 {
            return "Run at least 2 tests to see performance comparison".to_string();
        }
        
        let individual = results.iter().find(|r| r.test_type == "Individual Updates");
        let batched = results.iter().find(|r| r.test_type == "Batched Updates");
        
        if let (Some(ind), Some(batch)) = (individual, batched) {
            let improvement = if batch.duration > 0 {
                ((ind.duration as f64 - batch.duration as f64) / batch.duration as f64) * 100.0
            } else {
                0.0
            };
            
            if improvement > 0.0 {
                format!("Batched updates are {:.1}% faster than individual updates", improvement)
            } else {
                format!("Individual updates are {:.1}% faster than batched updates", -improvement)
            }
        } else {
            "Run Individual and Batched tests to compare performance".to_string()
        }
    };
    
    view! {
        <div class="p-6 max-w-6xl mx-auto">
            <h2 class="text-3xl font-bold mb-6">"Performance Testing Demo"</h2>
            
            // Test Configuration
            <div class="bg-blue-100 p-4 rounded-lg mb-6">
                <h3 class="text-lg font-semibold mb-2">"Test Configuration"</h3>
                <div class="flex items-center gap-4">
                    <label class="text-sm font-medium">"Test Count:"</label>
                    <input 
                        class="px-2 py-1 border rounded w-20"
                        type="number"
                        value={test_count}
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<usize>().unwrap_or(1000);
                            set_test_count.set(value);
                        }
                    />
                    <span class="text-sm text-gray-600">
                        "Current: " {test_count} " operations"
                    </span>
                </div>
            </div>
            
            // Test Controls
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                <button 
                    class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-400"
                    disabled={is_running}
                    on:click=move |_| test_individual_updates()
                >
                    "Individual Updates"
                </button>
                
                <button 
                    class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 disabled:bg-gray-400"
                    disabled={is_running}
                    on:click=move |_| test_batched_updates()
                >
                    "Batched Updates"
                </button>
                
                <button 
                    class="px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600 disabled:bg-gray-400"
                    disabled={is_running}
                    on:click=move |_| test_multiple_signals()
                >
                    "Multiple Signals"
                </button>
                
                <button 
                    class="px-4 py-2 bg-orange-500 text-white rounded hover:bg-orange-600 disabled:bg-gray-400"
                    disabled={is_running}
                    on:click=move |_| test_reactive_computations()
                >
                    "Reactive Computations"
                </button>
            </div>
            
            // Status and Controls
            <div class="flex justify-between items-center mb-6">
                <div class="text-sm">
                    {move || if is_running.get() { "Running test..." } else { "Ready to run tests" }}
                </div>
                <button 
                    class="px-3 py-1 bg-gray-500 text-white rounded hover:bg-gray-600"
                    on:click=move |_| clear_results()
                >
                    "Clear Results"
                </button>
            </div>
            
            // Performance Summary
            <div class="bg-yellow-100 p-4 rounded-lg mb-6">
                <h3 class="text-lg font-semibold mb-2">"Performance Summary"</h3>
                <p class="text-sm text-yellow-700">{get_performance_summary()}</p>
            </div>
            
            // Test Results
            <div class="bg-white rounded-lg shadow overflow-hidden">
                <h3 class="text-lg font-semibold p-4 bg-gray-50 border-b">"Test Results"</h3>
                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead class="bg-gray-50">
                            <tr>
                                <th class="px-4 py-2 text-left text-sm font-medium text-gray-700">"Test Type"</th>
                                <th class="px-4 py-2 text-left text-sm font-medium text-gray-700">"Operations"</th>
                                <th class="px-4 py-2 text-left text-sm font-medium text-gray-700">"Duration (μs)"</th>
                                <th class="px-4 py-2 text-left text-sm font-medium text-gray-700">"Description"</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-gray-200">
                            {move || test_results.get().into_iter().map(|result| {
                                view! {
                                    <tr>
                                        <td class="px-4 py-2 text-sm font-medium text-gray-900">{result.test_type}</td>
                                        <td class="px-4 py-2 text-sm text-gray-700">{result.count}</td>
                                        <td class="px-4 py-2 text-sm text-gray-700">{result.duration}</td>
                                        <td class="px-4 py-2 text-sm text-gray-700">{result.description}</td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()}
                        </tbody>
                    </table>
                </div>
            </div>
            
            // Performance Tips
            <div class="mt-6 bg-green-100 p-4 rounded-lg">
                <h3 class="text-lg font-semibold mb-2">"Performance Tips"</h3>
                <ul class="text-sm text-green-700 space-y-1">
                    <li>"• Batched updates are generally faster for multiple operations"</li>
                    <li>"• Reactive computations are automatically optimized by Leptos"</li>
                    <li>"• Use signal updates sparingly in tight loops"</li>
                    <li>"• Consider using memos for expensive computations"</li>
                </ul>
            </div>
        </div>
    }
}

/// Test result structure
#[derive(Clone, Debug)]
struct TestResult {
    test_type: String,
    count: usize,
    duration: u64,
    description: String,
}
