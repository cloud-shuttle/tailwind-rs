//! Memory Analysis Demo
//! 
//! This module demonstrates basic signal usage and memory management concepts
//! in Leptos applications.

use leptos::prelude::*;
use leptos::attr::global::ClassAttribute;

/// Memory analysis component that demonstrates signal usage
#[component]
pub fn MemoryAnalysisDemo() -> impl IntoView {
    // Create some test signals
    let (count, set_count) = signal(0);
    let (name, set_name) = signal("Tailwind-RS".to_string());
    let (is_visible, set_visible) = signal(true);
    
    // Function to get basic signal information
    let get_signal_info = move || {
        format!(
            "Active Signals: 3 (count, name, is_visible) | Memory: Optimized by Leptos"
        )
    };
    
    view! {
        <div class="p-6 max-w-4xl mx-auto">
            <h2 class="text-2xl font-bold mb-4">"Memory Analysis Demo"</h2>
            
            // Memory Statistics Display
            <div class="bg-gray-100 p-4 rounded-lg mb-4">
                <h3 class="text-lg font-semibold mb-2">"Signal Information"</h3>
                <p class="text-sm text-gray-700">{get_signal_info()}</p>
            </div>
            
            // Signal Controls
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
                <div class="bg-white p-4 rounded-lg shadow">
                    <h4 class="font-semibold mb-2">"Counter Signal"</h4>
                    <p class="text-2xl font-bold text-blue-600">{count}</p>
                    <button 
                        class="mt-2 px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600"
                        on:click=move |_| {
                            set_count.update(|c| *c += 1);
                        }
                    >
                        "Increment"
                    </button>
                </div>
                
                <div class="bg-white p-4 rounded-lg shadow">
                    <h4 class="font-semibold mb-2">"Name Signal"</h4>
                    <p class="text-lg text-green-600">{name}</p>
                    <input 
                        class="mt-2 px-2 py-1 border rounded"
                        type="text"
                        placeholder="Enter name"
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_name.set(value);
                        }
                    />
                </div>
                
                <div class="bg-white p-4 rounded-lg shadow">
                    <h4 class="font-semibold mb-2">"Visibility Signal"</h4>
                    <p class="text-lg text-purple-600">
                        {move || if is_visible.get() { "Visible" } else { "Hidden" }}
                    </p>
                    <button 
                        class="mt-2 px-3 py-1 bg-purple-500 text-white rounded hover:bg-purple-600"
                        on:click=move |_| {
                            set_visible.update(|v| *v = !*v);
                        }
                    >
                        "Toggle"
                    </button>
                </div>
            </div>
            
            // Memory Management Information
            <div class="bg-yellow-100 p-4 rounded-lg mb-4">
                <h3 class="text-lg font-semibold mb-2">"Memory Management"</h3>
                <p class="text-sm text-yellow-700">
                    "Leptos automatically manages signal memory and cleanup. 
                    Signals are garbage collected when components are unmounted."
                </p>
            </div>
            
            // Performance Information
            <div class="bg-red-100 p-4 rounded-lg">
                <h3 class="text-lg font-semibold mb-2">"Performance Notes"</h3>
                <p class="text-sm text-red-700">
                    "This demo shows basic signal usage. For advanced memory management 
                    and signal lifecycle control, consider using the leptos-shadcn-signal-management crate."
                </p>
            </div>
        </div>
    }
}

/// Performance testing component that demonstrates signal updates
#[component]
pub fn PerformanceTestDemo() -> impl IntoView {
    let (updates, set_updates) = signal(0);
    let (batch_size, set_batch_size) = signal(10);
    
    // Test individual updates
    let test_individual_updates = move || {
        let start = std::time::Instant::now();
        for _ in 0..batch_size.get() {
            set_updates.update(|u| *u += 1);
        }
        let duration = start.elapsed();
        leptos::logging::log!("Individual updates took: {:?}", duration);
    };
    
    // Test batched updates (simulated)
    let test_batched_updates = move || {
        let start = std::time::Instant::now();
        // Simulate batched updates by updating once with the total
        let current = updates.get();
        set_updates.set(current + batch_size.get());
        let duration = start.elapsed();
        leptos::logging::log!("Batched updates took: {:?}", duration);
    };
    
    view! {
        <div class="p-6 max-w-2xl mx-auto">
            <h2 class="text-2xl font-bold mb-4">"Performance Test Demo"</h2>
            
            <div class="bg-gray-100 p-4 rounded-lg mb-4">
                <h3 class="text-lg font-semibold mb-2">"Update Count: " {updates}</h3>
                <p class="text-sm text-gray-700">
                    "Batch Size: " {batch_size}
                </p>
                <input 
                    class="mt-2 px-2 py-1 border rounded"
                    type="number"
                    value={batch_size}
                    on:input=move |ev| {
                        let value = event_target_value(&ev).parse::<usize>().unwrap_or(10);
                        set_batch_size.set(value);
                    }
                />
            </div>
            
            <div class="grid grid-cols-2 gap-4">
                <button 
                    class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                    on:click=move |_| test_individual_updates()
                >
                    "Test Individual Updates"
                </button>
                
                <button 
                    class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
                    on:click=move |_| test_batched_updates()
                >
                    "Test Batched Updates"
                </button>
            </div>
            
            <div class="mt-4 p-4 bg-yellow-100 rounded-lg">
                <p class="text-sm text-yellow-700">
                    "Check the browser console to see performance timing results. 
                    Batched updates should be significantly faster for multiple signal updates."
                </p>
            </div>
        </div>
    }
}