//! Advanced Signal Management Module
//! 
//! This module demonstrates advanced signal management techniques using
//! the actual leptos-shadcn-signal-management crate for enhanced reactivity
//! and state management.

use leptos::prelude::*;
use leptos::attr::global::ClassAttribute;
use leptos_shadcn_signal_management::{
    TailwindSignalManager, BatchedSignalUpdater,
    SignalMemoryManager, MemoryLeakDetector,
    Theme, Variant, Size, ResponsiveConfig,
};

/// Advanced signal management demo component
#[component]
pub fn AdvancedSignalManagementDemo() -> impl IntoView {
    // Create signal managers
    let tailwind_manager = TailwindSignalManager::new();
    let batched_updater = BatchedSignalUpdater::new();
    let memory_manager = SignalMemoryManager::new();
    let leak_detector = MemoryLeakDetector::new();
    
    // Get signals from the tailwind manager
    let theme_signal = tailwind_manager.theme();
    let variant_signal = tailwind_manager.variant();
    let size_signal = tailwind_manager.size();
    let responsive_signal = tailwind_manager.responsive();
    
    // Clone signals for use in multiple closures
    let theme_signal_display = theme_signal.clone();
    let variant_signal_display = variant_signal.clone();
    let size_signal_display = size_signal.clone();
    let responsive_signal_display = responsive_signal.clone();
    
    // Create some test signals for demonstration
    let (counter, set_counter) = signal(0);
    let (name, set_name) = signal("Tailwind-RS".to_string());
    let (is_visible, set_visible) = signal(true);
    
    // Create a signal group for memory management
    let group_name = "demo_group".to_string();
    let _ = memory_manager.create_group(group_name.clone());
    
    // Add signals to the group
    let signal1 = ArcRwSignal::new(42);
    let signal2 = ArcRwSignal::new(84);
    let _ = memory_manager.add_signal_to_group(&group_name, signal1.clone());
    let _ = memory_manager.add_signal_to_group(&group_name, signal2.clone());
    
    // Function to get memory stats
    let get_memory_stats = move || {
        let stats = memory_manager.get_stats();
        format!(
            "Active Signals: {}, Active Memos: {}, Estimated Memory: {} bytes, Tracked Groups: {}",
            stats.active_signals,
            stats.active_memos,
            stats.estimated_memory_bytes,
            stats.tracked_groups
        )
    };
    
    // Function to test batched updates
    let test_batched_updates = move || {
        // Queue multiple updates
        let _ = batched_updater.queue_update(move || set_counter.update(|c| *c += 1));
        let _ = batched_updater.queue_update(move || set_name.set("Updated!".to_string()));
        let _ = batched_updater.queue_update(move || set_visible.update(|v| *v = !*v));
        
        // Flush all updates at once
        let _ = batched_updater.flush_updates();
        leptos::logging::log!("Batched updates executed");
    };
    
    // Function to test memory leak detection
    let test_memory_leak_detection = move || {
        let leak_detected = leak_detector.check_for_leaks().unwrap_or(false);
        leptos::logging::log!("Memory leak detected: {}", leak_detected);
    };
    
    view! {
        <div class="p-6 max-w-6xl mx-auto">
            <h2 class="text-3xl font-bold mb-6">"Advanced Signal Management Demo"</h2>
            
            // Tailwind Signal Manager
            <div class="bg-white p-4 rounded-lg shadow mb-6">
                <h3 class="text-lg font-semibold mb-4">"Tailwind Signal Manager"</h3>
                <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                    <div>
                        <label class="block text-sm font-medium mb-2">"Theme"</label>
                        <select 
                            class="w-full px-3 py-2 border rounded"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                let theme = match value.as_str() {
                                    "dark" => Theme::Dark,
                                    "light" => Theme::Light,
                                    "custom" => Theme::Custom(std::collections::HashMap::new()),
                                    _ => Theme::Default,
                                };
                                theme_signal.set(theme);
                            }
                        >
                            <option value="default">"Default"</option>
                            <option value="dark">"Dark"</option>
                            <option value="light">"Light"</option>
                            <option value="custom">"Custom"</option>
                        </select>
                        <p class="text-sm text-gray-600 mt-1">
                            "Current: " {move || format!("{:?}", theme_signal_display.get())}
                        </p>
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium mb-2">"Variant"</label>
                        <select 
                            class="w-full px-3 py-2 border rounded"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                let variant = match value.as_str() {
                                    "secondary" => Variant::Secondary,
                                    "destructive" => Variant::Destructive,
                                    "outline" => Variant::Outline,
                                    "ghost" => Variant::Ghost,
                                    "link" => Variant::Link,
                                    _ => Variant::Primary,
                                };
                                variant_signal.set(variant);
                            }
                        >
                            <option value="primary">"Primary"</option>
                            <option value="secondary">"Secondary"</option>
                            <option value="destructive">"Destructive"</option>
                            <option value="outline">"Outline"</option>
                            <option value="ghost">"Ghost"</option>
                            <option value="link">"Link"</option>
                        </select>
                        <p class="text-sm text-gray-600 mt-1">
                            "Current: " {move || format!("{:?}", variant_signal_display.get())}
                        </p>
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium mb-2">"Size"</label>
                        <select 
                            class="w-full px-3 py-2 border rounded"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                let size = match value.as_str() {
                                    "small" => Size::Small,
                                    "large" => Size::Large,
                                    _ => Size::Medium,
                                };
                                size_signal.set(size);
                            }
                        >
                            <option value="small">"Small"</option>
                            <option value="medium">"Medium"</option>
                            <option value="large">"Large"</option>
                        </select>
                        <p class="text-sm text-gray-600 mt-1">
                            "Current: " {move || format!("{:?}", size_signal_display.get())}
                        </p>
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium mb-2">"Responsive"</label>
                        <input 
                            class="w-full px-3 py-2 border rounded"
                            type="text"
                            placeholder="e.g., sm:block md:flex"
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                let config = ResponsiveConfig {
                                    sm: if value.contains("sm:") { Some(value.clone()) } else { None },
                                    md: if value.contains("md:") { Some(value.clone()) } else { None },
                                    lg: if value.contains("lg:") { Some(value.clone()) } else { None },
                                    xl: if value.contains("xl:") { Some(value.clone()) } else { None },
                                };
                                responsive_signal.set(config);
                            }
                        />
                        <p class="text-sm text-gray-600 mt-1">
                            "Current: " {move || format!("{:?}", responsive_signal_display.get())}
                        </p>
                    </div>
                </div>
            </div>
            
            // Test Signals
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
                <div class="bg-white p-4 rounded-lg shadow">
                    <h4 class="font-semibold mb-2">"Counter Signal"</h4>
                    <p class="text-2xl font-bold text-blue-600">{counter}</p>
                    <button 
                        class="mt-2 px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600"
                        on:click=move |_| set_counter.update(|c| *c += 1)
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
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                    />
                </div>
                
                <div class="bg-white p-4 rounded-lg shadow">
                    <h4 class="font-semibold mb-2">"Visibility Signal"</h4>
                    <p class="text-lg text-purple-600">
                        {move || if is_visible.get() { "Visible" } else { "Hidden" }}
                    </p>
                    <button 
                        class="mt-2 px-3 py-1 bg-purple-500 text-white rounded hover:bg-purple-600"
                        on:click=move |_| set_visible.update(|v| *v = !*v)
                    >
                        "Toggle"
                    </button>
                </div>
            </div>
            
            // Memory Management
            <div class="bg-white p-4 rounded-lg shadow mb-6">
                <h3 class="text-lg font-semibold mb-4">"Memory Management"</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div>
                        <h4 class="font-medium mb-2">"Memory Statistics"</h4>
                        <p class="text-sm text-gray-700">{get_memory_stats()}</p>
                    </div>
                    <div>
                        <h4 class="font-medium mb-2">"Signal Group"</h4>
                        <p class="text-sm text-gray-700">
                            "Group: " {group_name} " | Memory management active"
                        </p>
                    </div>
                </div>
            </div>
            
            // Performance Testing
            <div class="bg-white p-4 rounded-lg shadow mb-6">
                <h3 class="text-lg font-semibold mb-4">"Performance Testing"</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <button 
                        class="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
                        on:click=move |_| test_batched_updates()
                    >
                        "Test Batched Updates"
                    </button>
                    
                    <button 
                        class="px-4 py-2 bg-orange-500 text-white rounded hover:bg-orange-600"
                        on:click=move |_| test_memory_leak_detection()
                    >
                        "Test Memory Leak Detection"
                    </button>
                </div>
                <p class="text-sm text-gray-600 mt-2">
                    "Check the browser console for performance logs"
                </p>
            </div>
            
            // Performance Tips
            <div class="bg-yellow-100 p-4 rounded-lg">
                <h3 class="text-lg font-semibold mb-2">"Advanced Signal Management Tips"</h3>
                <ul class="text-sm text-yellow-700 space-y-1">
                    <li>"• Use TailwindSignalManager for consistent theme/variant management"</li>
                    <li>"• Use BatchedSignalUpdater for performance-critical updates"</li>
                    <li>"• Use SignalMemoryManager for memory tracking and leak detection"</li>
                    <li>"• Group related signals together for better memory management"</li>
                    <li>"• Monitor memory usage with MemoryStats"</li>
                    <li>"• Use responsive configurations for adaptive layouts"</li>
                </ul>
            </div>
        </div>
    }
}

/// Batched updates demo component
#[component]
pub fn BatchedUpdatesDemo() -> impl IntoView {
    let (counter1, set_counter1) = signal(0);
    let (counter2, set_counter2) = signal(0);
    let (counter3, set_counter3) = signal(0);
    
    let batched_updater = BatchedSignalUpdater::new();
    
    // Test individual updates
    let test_individual_updates = move || {
        let start = std::time::Instant::now();
        set_counter1.update(|c| *c += 1);
        set_counter2.update(|c| *c += 1);
        set_counter3.update(|c| *c += 1);
        let duration = start.elapsed();
        leptos::logging::log!("Individual updates took: {:?}", duration);
    };
    
    // Test batched updates
    let test_batched_updates = move || {
        let start = std::time::Instant::now();
        
        // Queue updates
        let _ = batched_updater.queue_update(move || set_counter1.update(|c| *c += 1));
        let _ = batched_updater.queue_update(move || set_counter2.update(|c| *c += 1));
        let _ = batched_updater.queue_update(move || set_counter3.update(|c| *c += 1));
        
        // Flush all updates
        let _ = batched_updater.flush_updates();
        
        let duration = start.elapsed();
        leptos::logging::log!("Batched updates took: {:?}", duration);
    };
    
    view! {
        <div class="p-6 max-w-2xl mx-auto">
            <h2 class="text-2xl font-bold mb-4">"Batched Updates Demo"</h2>
            
            <div class="bg-white p-4 rounded-lg shadow mb-4">
                <h3 class="text-lg font-semibold mb-2">"Counters"</h3>
                <div class="grid grid-cols-3 gap-4 text-center">
                    <div>
                        <p class="text-2xl font-bold text-blue-600">{counter1}</p>
                        <p class="text-sm text-gray-600">"Counter 1"</p>
                    </div>
                    <div>
                        <p class="text-2xl font-bold text-green-600">{counter2}</p>
                        <p class="text-sm text-gray-600">"Counter 2"</p>
                    </div>
                    <div>
                        <p class="text-2xl font-bold text-purple-600">{counter3}</p>
                        <p class="text-sm text-gray-600">"Counter 3"</p>
                    </div>
                </div>
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
                    Batched updates should be faster for multiple signal updates."
                </p>
            </div>
        </div>
    }
}