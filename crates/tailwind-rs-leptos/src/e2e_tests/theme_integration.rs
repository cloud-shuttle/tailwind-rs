//! # Theme Integration Tests
//!
//! This module provides tests for theme-based component styling and dark mode handling.

use leptos::prelude::*;
use tailwind_rs_core::ClassBuilder;
use super::E2ETestSuite;

/// Test component that demonstrates theme-based styling
#[component]
pub fn ThemeComponent() -> impl IntoView {
    let (is_dark_mode, set_is_dark_mode) = signal(false);
    let (theme, set_theme) = signal("default".to_string());
    
    let theme_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("p-4 rounded-lg border");
        
        // Dark mode classes
        if is_dark_mode.get() {
            builder = builder.class("bg-gray-800 text-white border-gray-700");
        } else {
            builder = builder.class("bg-white text-gray-900 border-gray-300");
        }
        
        // Theme-specific classes
        match theme.get().as_str() {
            "blue" => {
                if is_dark_mode.get() {
                    builder = builder.class("border-blue-600");
                } else {
                    builder = builder.class("border-blue-500");
                }
            },
            "green" => {
                if is_dark_mode.get() {
                    builder = builder.class("border-green-600");
                } else {
                    builder = builder.class("border-green-500");
                }
            },
            "red" => {
                if is_dark_mode.get() {
                    builder = builder.class("border-red-600");
                } else {
                    builder = builder.class("border-red-500");
                }
            },
            _ => {
                if is_dark_mode.get() {
                    builder = builder.class("border-gray-600");
                } else {
                    builder = builder.class("border-gray-500");
                }
            }
        }
        
        builder.build().to_css_classes()
    };
    
    view! {
        <div class="space-y-4">
            <div class="flex space-x-2">
                <button 
                    class="px-3 py-1 bg-gray-500 text-white rounded text-sm"
                    on:click=move |_| set_is_dark_mode.update(|dark| *dark = !*dark)
                >
                    {move || if is_dark_mode.get() { "Light Mode" } else { "Dark Mode" }}
                </button>
                
                <button 
                    class="px-3 py-1 bg-blue-500 text-white rounded text-sm"
                    on:click=move |_| set_theme.set("blue".to_string())
                >
                    "Blue"
                </button>
                
                <button 
                    class="px-3 py-1 bg-green-500 text-white rounded text-sm"
                    on:click=move |_| set_theme.set("green".to_string())
                >
                    "Green"
                </button>
                
                <button 
                    class="px-3 py-1 bg-red-500 text-white rounded text-sm"
                    on:click=move |_| set_theme.set("red".to_string())
                >
                    "Red"
                </button>
            </div>
            
            <div class=theme_classes>
                <h3 class="text-lg font-semibold mb-2">
                    "Theme Component"
                </h3>
                <p class="text-sm">
                    "Current theme: " {theme}
                </p>
                <p class="text-sm">
                    "Mode: " {move || if is_dark_mode.get() { "Dark" } else { "Light" }}
                </p>
            </div>
        </div>
    }
}

/// Run theme integration tests
pub fn run_theme_tests() -> E2ETestSuite {
    let mut suite = E2ETestSuite::new();
    
    // Test theme component logic
    suite.add_test(
        "test_theme_component_logic".to_string(),
        test_theme_component_logic(),
        None,
    );
    
    // Test dark mode handling
    suite.add_test(
        "test_dark_mode_handling".to_string(),
        test_dark_mode_handling(),
        None,
    );
    
    // Test theme switching
    suite.add_test(
        "test_theme_switching".to_string(),
        test_theme_switching(),
        None,
    );
    
    // Test theme class generation
    suite.add_test(
        "test_theme_class_generation".to_string(),
        test_theme_class_generation(),
        None,
    );
    
    suite
}

/// Test the theme component logic
fn test_theme_component_logic() -> bool {
    let (is_dark_mode, set_is_dark_mode) = signal(false);
    let (theme, _) = signal("blue".to_string());
    
    let theme_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("p-4 rounded-lg border");
        
        // Dark mode classes
        if is_dark_mode.get() {
            builder = builder.class("bg-gray-800 text-white border-gray-700");
        } else {
            builder = builder.class("bg-white text-gray-900 border-gray-300");
        }
        
        // Theme-specific classes
        match theme.get().as_str() {
            "blue" => {
                if is_dark_mode.get() {
                    builder = builder.class("border-blue-600");
                } else {
                    builder = builder.class("border-blue-500");
                }
            },
            _ => {
                if is_dark_mode.get() {
                    builder = builder.class("border-gray-600");
                } else {
                    builder = builder.class("border-gray-500");
                }
            }
        }
        
        builder.build().to_css_classes()
    };
    
    let light_result = theme_classes();
    set_is_dark_mode.set(true);
    let dark_result = theme_classes();
    
    light_result.contains("bg-white") && dark_result.contains("bg-gray-800")
}

/// Test dark mode handling
fn test_dark_mode_handling() -> bool {
    let (is_dark_mode, set_is_dark_mode) = signal(false);
    
    let theme_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("p-4 rounded-lg border");
        
        if is_dark_mode.get() {
            builder = builder.class("bg-gray-800 text-white border-gray-700");
        } else {
            builder = builder.class("bg-white text-gray-900 border-gray-300");
        }
        
        builder.build().to_css_classes()
    };
    
    // Test light mode
    let light_result = theme_classes();
    let light_passed = light_result.contains("bg-white") && light_result.contains("text-gray-900");
    
    // Test dark mode
    set_is_dark_mode.set(true);
    let dark_result = theme_classes();
    let dark_passed = dark_result.contains("bg-gray-800") && dark_result.contains("text-white");
    
    light_passed && dark_passed
}

/// Test theme switching
fn test_theme_switching() -> bool {
    let (theme, set_theme) = signal("default".to_string());
    
    // Test initial theme
    let initial = theme.get() == "default";
    
    // Test switching to blue
    set_theme.set("blue".to_string());
    let after_blue = theme.get() == "blue";
    
    // Test switching to green
    set_theme.set("green".to_string());
    let after_green = theme.get() == "green";
    
    // Test switching to red
    set_theme.set("red".to_string());
    let after_red = theme.get() == "red";
    
    initial && after_blue && after_green && after_red
}

/// Test theme class generation
fn test_theme_class_generation() -> bool {
    let mut builder = ClassBuilder::new();
    builder = builder.class("p-4 rounded-lg border");
    builder = builder.class("bg-white text-gray-900 border-gray-300");
    builder = builder.class("border-blue-500");
    
    let result = builder.build().to_css_classes();
    result.contains("p-4") && 
    result.contains("rounded-lg") && 
    result.contains("border") && 
    result.contains("bg-white") && 
    result.contains("text-gray-900") && 
    result.contains("border-gray-300") && 
    result.contains("border-blue-500")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_component_logic_test() {
        assert!(test_theme_component_logic());
    }

    #[test]
    fn test_dark_mode_handling_test() {
        assert!(test_dark_mode_handling());
    }

    #[test]
    fn test_theme_switching_test() {
        assert!(test_theme_switching());
    }

    #[test]
    fn test_theme_class_generation_test() {
        assert!(test_theme_class_generation());
    }

    #[test]
    fn test_run_theme_tests() {
        let results = run_theme_tests();
        
        assert_eq!(results.total_tests, 4);
        assert_eq!(results.passed_tests, 4);
        assert_eq!(results.failed_tests, 0);
    }
}
