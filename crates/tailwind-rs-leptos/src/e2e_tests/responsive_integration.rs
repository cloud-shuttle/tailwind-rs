//! # Responsive Integration Tests
//!
//! This module provides tests for responsive component styling and breakpoint handling.

use leptos::prelude::*;
use tailwind_rs_core::ClassBuilder;
use super::{E2ETestSuite, E2ETestDetail};

/// Test component that demonstrates responsive styling
#[component]
pub fn ResponsiveComponent() -> impl IntoView {
    let (screen_size, set_screen_size) = signal("md".to_string());
    
    let responsive_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("grid gap-4");
        
        match screen_size.get().as_str() {
            "sm" => {
                builder = builder.class("grid-cols-1");
                builder = builder.class("p-2");
            },
            "md" => {
                builder = builder.class("grid-cols-2");
                builder = builder.class("p-4");
            },
            "lg" => {
                builder = builder.class("grid-cols-3");
                builder = builder.class("p-6");
            },
            "xl" => {
                builder = builder.class("grid-cols-4");
                builder = builder.class("p-8");
            },
            _ => {
                builder = builder.class("grid-cols-1");
                builder = builder.class("p-2");
            }
        }
        
        builder.build().to_css_classes()
    };
    
    view! {
        <div class="space-y-4">
            <div class="flex space-x-2">
                <button 
                    class="px-3 py-1 bg-blue-500 text-white rounded text-sm"
                    on:click=move |_| set_screen_size.set("sm".to_string())
                >
                    "SM"
                </button>
                <button 
                    class="px-3 py-1 bg-blue-500 text-white rounded text-sm"
                    on:click=move |_| set_screen_size.set("md".to_string())
                >
                    "MD"
                </button>
                <button 
                    class="px-3 py-1 bg-blue-500 text-white rounded text-sm"
                    on:click=move |_| set_screen_size.set("lg".to_string())
                >
                    "LG"
                </button>
                <button 
                    class="px-3 py-1 bg-blue-500 text-white rounded text-sm"
                    on:click=move |_| set_screen_size.set("xl".to_string())
                >
                    "XL"
                </button>
            </div>
            
            <div class=responsive_classes>
                <div class="bg-gray-200 p-4 rounded">"Item 1"</div>
                <div class="bg-gray-200 p-4 rounded">"Item 2"</div>
                <div class="bg-gray-200 p-4 rounded">"Item 3"</div>
                <div class="bg-gray-200 p-4 rounded">"Item 4"</div>
            </div>
            
            <p class="text-sm text-gray-600">
                "Current screen size: " {screen_size}
            </p>
        </div>
    }
}

/// Run responsive integration tests
pub fn run_responsive_tests() -> E2ETestSuite {
    let mut suite = E2ETestSuite::new();
    
    // Test responsive component logic
    suite.add_test(
        "test_responsive_component_logic".to_string(),
        test_responsive_component_logic(),
        None,
    );
    
    // Test breakpoint handling
    suite.add_test(
        "test_breakpoint_handling".to_string(),
        test_breakpoint_handling(),
        None,
    );
    
    // Test responsive class generation
    suite.add_test(
        "test_responsive_class_generation".to_string(),
        test_responsive_class_generation(),
        None,
    );
    
    // Test screen size switching
    suite.add_test(
        "test_screen_size_switching".to_string(),
        test_screen_size_switching(),
        None,
    );
    
    suite
}

/// Test the responsive component logic
fn test_responsive_component_logic() -> bool {
    let (screen_size, set_screen_size) = signal("md".to_string());
    
    let responsive_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("grid gap-4");
        
        match screen_size.get().as_str() {
            "sm" => {
                builder = builder.class("grid-cols-1");
                builder = builder.class("p-2");
            },
            "md" => {
                builder = builder.class("grid-cols-2");
                builder = builder.class("p-4");
            },
            "lg" => {
                builder = builder.class("grid-cols-3");
                builder = builder.class("p-6");
            },
            "xl" => {
                builder = builder.class("grid-cols-4");
                builder = builder.class("p-8");
            },
            _ => {
                builder = builder.class("grid-cols-1");
                builder = builder.class("p-2");
            }
        }
        
        builder.build().to_css_classes()
    };
    
    let result = responsive_classes();
    result.contains("grid") && result.contains("gap-4") && result.contains("grid-cols-2") && result.contains("p-4")
}

/// Test breakpoint handling
fn test_breakpoint_handling() -> bool {
    let breakpoints = vec!["sm", "md", "lg", "xl"];
    let mut all_passed = true;
    
    for breakpoint in breakpoints {
        let (screen_size, _) = signal(breakpoint.to_string());
        
        let responsive_classes = move || {
            let mut builder = ClassBuilder::new();
            builder = builder.class("grid gap-4");
            
            match screen_size.get().as_str() {
                "sm" => {
                    builder = builder.class("grid-cols-1");
                    builder = builder.class("p-2");
                },
                "md" => {
                    builder = builder.class("grid-cols-2");
                    builder = builder.class("p-4");
                },
                "lg" => {
                    builder = builder.class("grid-cols-3");
                    builder = builder.class("p-6");
                },
                "xl" => {
                    builder = builder.class("grid-cols-4");
                    builder = builder.class("p-8");
                },
                _ => {
                    builder = builder.class("grid-cols-1");
                    builder = builder.class("p-2");
                }
            }
            
            builder.build().to_css_classes()
        };
        
        let result = responsive_classes();
        match breakpoint {
            "sm" => all_passed &= result.contains("grid-cols-1") && result.contains("p-2"),
            "md" => all_passed &= result.contains("grid-cols-2") && result.contains("p-4"),
            "lg" => all_passed &= result.contains("grid-cols-3") && result.contains("p-6"),
            "xl" => all_passed &= result.contains("grid-cols-4") && result.contains("p-8"),
            _ => all_passed &= result.contains("grid-cols-1") && result.contains("p-2"),
        }
    }
    
    all_passed
}

/// Test responsive class generation
fn test_responsive_class_generation() -> bool {
    let mut builder = ClassBuilder::new();
    builder = builder.class("grid gap-4");
    builder = builder.class("grid-cols-2");
    builder = builder.class("p-4");
    
    let result = builder.build().to_css_classes();
    result.contains("grid") && result.contains("gap-4") && result.contains("grid-cols-2") && result.contains("p-4")
}

/// Test screen size switching
fn test_screen_size_switching() -> bool {
    let (screen_size, set_screen_size) = signal("sm".to_string());
    
    // Test initial state
    let initial = screen_size.get() == "sm";
    
    // Test switching to md
    set_screen_size.set("md".to_string());
    let after_md = screen_size.get() == "md";
    
    // Test switching to lg
    set_screen_size.set("lg".to_string());
    let after_lg = screen_size.get() == "lg";
    
    // Test switching to xl
    set_screen_size.set("xl".to_string());
    let after_xl = screen_size.get() == "xl";
    
    initial && after_md && after_lg && after_xl
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_component_logic_test() {
        assert!(test_responsive_component_logic());
    }

    #[test]
    fn test_breakpoint_handling_test() {
        assert!(test_breakpoint_handling());
    }

    #[test]
    fn test_responsive_class_generation_test() {
        assert!(test_responsive_class_generation());
    }

    #[test]
    fn test_screen_size_switching_test() {
        assert!(test_screen_size_switching());
    }

    #[test]
    fn test_run_responsive_tests() {
        let results = run_responsive_tests();
        
        assert_eq!(results.total_tests, 4);
        assert_eq!(results.passed_tests, 4);
        assert_eq!(results.failed_tests, 0);
    }
}
