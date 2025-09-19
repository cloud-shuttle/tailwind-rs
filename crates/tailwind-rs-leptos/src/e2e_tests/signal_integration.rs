//! # Signal Integration Tests
//!
//! This module provides tests for signal-based component styling and reactivity.

use leptos::prelude::*;
use tailwind_rs_core::ClassBuilder;
use super::E2ETestSuite;

/// Test component that demonstrates signal-based styling
#[component]
pub fn SignalBasedComponent() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (is_even, set_is_even) = signal(true);
    
    // Reactive effect to update even/odd state
    Effect::new(move |_| {
        set_is_even.set(count.get() % 2 == 0);
    });
    
    let button_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("px-4 py-2 rounded font-medium");
        
        if is_even.get() {
            builder = builder.class("bg-green-500 text-white hover:bg-green-600");
        } else {
            builder = builder.class("bg-red-500 text-white hover:bg-red-600");
        }
        
        builder.build().to_css_classes()
    };
    
    view! {
        <div class="space-y-4">
            <button 
                class=button_classes
                on:click=move |_| set_count.update(|c| *c += 1)
            >
                "Count: " {count}
            </button>
            <p class="text-sm text-gray-600">
                "Number is " {move || if is_even.get() { "even" } else { "odd" }}
            </p>
        </div>
    }
}

/// Run signal integration tests
pub fn run_signal_tests() -> E2ETestSuite {
    let mut suite = E2ETestSuite::new();
    
    // Test signal-based component logic
    suite.add_test(
        "test_signal_based_component_logic".to_string(),
        test_signal_based_component_logic(),
        None,
    );
    
    // Test even/odd logic
    suite.add_test(
        "test_even_odd_logic".to_string(),
        test_even_odd_logic(),
        None,
    );
    
    // Test class switching
    suite.add_test(
        "test_class_switching".to_string(),
        test_class_switching(),
        None,
    );
    
    // Test signal updates
    suite.add_test(
        "test_signal_updates".to_string(),
        test_signal_updates(),
        None,
    );
    
    suite
}

/// Test the signal-based styling logic
fn test_signal_based_component_logic() -> bool {
    let (count, set_count) = signal(0);
    let (is_even, set_is_even) = signal(true);
    
    // Test even/odd logic
    set_count.set(2);
    set_is_even.set(count.get() % 2 == 0);
    let even_result = is_even.get();
    
    set_count.set(3);
    set_is_even.set(count.get() % 2 == 0);
    let odd_result = is_even.get();
    
    even_result && !odd_result
}

/// Test even/odd logic
fn test_even_odd_logic() -> bool {
    let test_cases = vec![
        (0, true),
        (1, false),
        (2, true),
        (3, false),
        (4, true),
        (5, false),
    ];
    
    test_cases.into_iter().all(|(value, expected)| {
        let (count, _) = signal(value);
        let (is_even, set_is_even) = signal(true);
        
        set_is_even.set(count.get() % 2 == 0);
        is_even.get() == expected
    })
}

/// Test class switching based on signals
fn test_class_switching() -> bool {
    let (is_even, set_is_even) = signal(true);
    
    let button_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("px-4 py-2 rounded font-medium");
        
        if is_even.get() {
            builder = builder.class("bg-green-500 text-white hover:bg-green-600");
        } else {
            builder = builder.class("bg-red-500 text-white hover:bg-red-600");
        }
        
        builder.build().to_css_classes()
    };
    
    let even_classes = button_classes();
    set_is_even.set(false);
    let odd_classes = button_classes();
    
    even_classes.contains("bg-green-500") && odd_classes.contains("bg-red-500")
}

/// Test signal updates
fn test_signal_updates() -> bool {
    let (count, set_count) = signal(0);
    
    // Test initial value
    let initial = count.get() == 0;
    
    // Test increment
    set_count.update(|c| *c += 1);
    let after_increment = count.get() == 1;
    
    // Test multiple increments
    set_count.update(|c| *c += 5);
    let after_multiple = count.get() == 6;
    
    initial && after_increment && after_multiple
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_based_component_logic_test() {
        assert!(test_signal_based_component_logic());
    }

    #[test]
    fn test_even_odd_logic_test() {
        assert!(test_even_odd_logic());
    }

    #[test]
    fn test_class_switching_test() {
        assert!(test_class_switching());
    }

    #[test]
    fn test_signal_updates_test() {
        assert!(test_signal_updates());
    }

    #[test]
    fn test_run_signal_tests() {
        let results = run_signal_tests();
        
        assert_eq!(results.total_tests, 4);
        assert_eq!(results.passed_tests, 4);
        assert_eq!(results.failed_tests, 0);
    }
}
