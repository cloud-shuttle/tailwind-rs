//! # State Management Tests
//!
//! This module provides tests for complex state management and component interactions.

use super::E2ETestSuite;
use leptos::prelude::*;
use tailwind_rs_core::ClassBuilder;

/// Test component that demonstrates complex state management
#[component]
pub fn ComplexStateComponent() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (is_loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (items, set_items) = signal(Vec::<String>::new());

    // Derived state
    let is_empty = move || items.get().is_empty();
    let has_error = move || error.get().is_some();
    let can_increment = move || !is_loading.get() && !has_error();

    // Complex class logic based on multiple state variables
    let container_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("p-4 rounded-lg border transition-all duration-200");

        if has_error() {
            builder = builder.class("border-red-500 bg-red-50");
        } else if is_loading.get() {
            builder = builder.class("border-yellow-500 bg-yellow-50");
        } else {
            builder = builder.class("border-gray-300 bg-white");
        }

        builder.build().to_css_classes()
    };

    let button_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("px-4 py-2 rounded font-medium transition-colors");

        if can_increment() {
            builder = builder.class("bg-blue-600 text-white hover:bg-blue-700");
        } else {
            builder = builder.class("bg-gray-400 text-gray-600 cursor-not-allowed");
        }

        builder.build().to_css_classes()
    };

    let list_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("space-y-2");

        if is_empty() {
            builder = builder.class("text-gray-500 italic");
        }

        builder.build().to_css_classes()
    };

    // Simulate async operation
    let handle_increment = move |_| {
        if can_increment() {
            set_loading.set(true);
            set_error.set(None);

            // Simulate async operation
            set_timeout(
                move || {
                    set_count.update(|c| *c += 1);
                    set_items.update(|items| {
                        items.push(format!("Item {}", items.len() + 1));
                    });
                    set_loading.set(false);
                },
                std::time::Duration::from_millis(100),
            );
        }
    };

    let handle_reset = move |_| {
        set_count.set(0);
        set_items.set(Vec::new());
        set_error.set(None);
        set_loading.set(false);
    };

    let handle_error = move |_| {
        set_error.set(Some("Simulated error".to_string()));
        set_loading.set(false);
    };

    view! {
        <div class=container_classes>
            <h3 class="text-lg font-semibold mb-4">
                "Complex State Component"
            </h3>

            <div class="space-y-4">
                <div class="flex items-center space-x-4">
                    <span class="text-sm font-medium">
                        "Count: " {count}
                    </span>
                    <span class="text-sm">
                        "Items: " {move || items.get().len()}
                    </span>
                </div>

                <div class="flex space-x-2">
                    <button
                        class=button_classes
                        disabled=move || !can_increment()
                        on:click=handle_increment
                    >
                        {move || if is_loading.get() { "Loading..." } else { "Increment" }}
                    </button>

                    <button
                        class="px-4 py-2 bg-green-600 text-white rounded font-medium hover:bg-green-700"
                        on:click=handle_reset
                    >
                        "Reset"
                    </button>

                    <button
                        class="px-4 py-2 bg-red-600 text-white rounded font-medium hover:bg-red-700"
                        on:click=handle_error
                    >
                        "Error"
                    </button>
                </div>

                {move || if let Some(err) = error.get() {
                    view! {
                        <div class="p-3 bg-red-100 border border-red-400 text-red-700 rounded">
                            "Error: " {err}
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }}

                <div class=list_classes>
                    {move || if is_empty() {
                        view! { <p>"No items yet"</p> }.into_any()
                    } else {
                        view! {
                            <ul>
                                {items.get().into_iter().map(|item| {
                                    view! {
                                        <li class="p-2 bg-gray-100 rounded">
                                            {item}
                                        </li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                        }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}

/// Run state management tests
pub fn run_state_tests() -> E2ETestSuite {
    let mut suite = E2ETestSuite::new();

    // Test complex state component logic
    suite.add_test(
        "test_complex_state_component_logic".to_string(),
        test_complex_state_component_logic(),
        None,
    );

    // Test derived state
    suite.add_test("test_derived_state".to_string(), test_derived_state(), None);

    // Test state interactions
    suite.add_test(
        "test_state_interactions".to_string(),
        test_state_interactions(),
        None,
    );

    // Test complex class logic
    suite.add_test(
        "test_complex_class_logic".to_string(),
        test_complex_class_logic(),
        None,
    );

    suite
}

/// Test the complex state component logic
fn test_complex_state_component_logic() -> bool {
    let (count, set_count) = signal(0);
    let (is_loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (items, set_items) = signal(Vec::<String>::new());

    // Test initial state
    let initial_count = count.get() == 0;
    let initial_loading = !is_loading.get();
    let initial_error = error.get().is_none();
    let initial_items = items.get().is_empty();

    // Test state updates
    set_count.set(5);
    set_loading.set(true);
    set_error.set(Some("test error".to_string()));
    set_items.set(vec!["item1".to_string(), "item2".to_string()]);

    let updated_count = count.get() == 5;
    let updated_loading = is_loading.get();
    let updated_error = error.get().is_some();
    let updated_items = items.get().len() == 2;

    initial_count
        && initial_loading
        && initial_error
        && initial_items
        && updated_count
        && updated_loading
        && updated_error
        && updated_items
}

/// Test derived state
fn test_derived_state() -> bool {
    let (items, set_items) = signal(Vec::<String>::new());
    let (error, set_error) = signal(None::<String>);

    let is_empty = move || items.get().is_empty();
    let has_error = move || error.get().is_some();

    // Test empty state
    let empty_test = is_empty();

    // Test non-empty state
    set_items.set(vec!["item1".to_string()]);
    let non_empty_test = !is_empty();

    // Test error state
    set_error.set(Some("error".to_string()));
    let error_test = has_error();

    // Test no error state
    set_error.set(None);
    let no_error_test = !has_error();

    empty_test && non_empty_test && error_test && no_error_test
}

/// Test state interactions
fn test_state_interactions() -> bool {
    let (_count, _set_count) = signal(0);
    let (is_loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    let can_increment = move || !is_loading.get() && error.get().is_none();

    // Test initial state
    let initial_can_increment = can_increment();

    // Test loading state
    set_loading.set(true);
    let loading_cannot_increment = !can_increment();

    // Test error state
    set_loading.set(false);
    set_error.set(Some("error".to_string()));
    let error_cannot_increment = !can_increment();

    // Test normal state
    set_error.set(None);
    let normal_can_increment = can_increment();

    initial_can_increment
        && loading_cannot_increment
        && error_cannot_increment
        && normal_can_increment
}

/// Test complex class logic
fn test_complex_class_logic() -> bool {
    let (is_loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    let container_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("p-4 rounded-lg border");

        if error.get().is_some() {
            builder = builder.class("border-red-500 bg-red-50");
        } else if is_loading.get() {
            builder = builder.class("border-yellow-500 bg-yellow-50");
        } else {
            builder = builder.class("border-gray-300 bg-white");
        }

        builder.build().to_css_classes()
    };

    // Test normal state
    let normal_classes = container_classes();
    let normal_passed =
        normal_classes.contains("border-gray-300") && normal_classes.contains("bg-white");

    // Test loading state
    set_loading.set(true);
    let loading_classes = container_classes();
    let loading_passed =
        loading_classes.contains("border-yellow-500") && loading_classes.contains("bg-yellow-50");

    // Test error state
    set_loading.set(false);
    set_error.set(Some("error".to_string()));
    let error_classes = container_classes();
    let error_passed =
        error_classes.contains("border-red-500") && error_classes.contains("bg-red-50");

    normal_passed && loading_passed && error_passed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_state_component_logic_test() {
        assert!(test_complex_state_component_logic());
    }

    #[test]
    fn test_derived_state_test() {
        assert!(test_derived_state());
    }

    #[test]
    fn test_state_interactions_test() {
        assert!(test_state_interactions());
    }

    #[test]
    fn test_complex_class_logic_test() {
        assert!(test_complex_class_logic());
    }

    #[test]
    fn test_run_state_tests() {
        let results = run_state_tests();

        assert_eq!(results.total_tests, 4);
        assert_eq!(results.passed_tests, 4);
        assert_eq!(results.failed_tests, 0);
    }
}
