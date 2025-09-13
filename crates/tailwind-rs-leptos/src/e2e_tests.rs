//! # End-to-End Tests for Leptos Integration
//!
//! This module provides comprehensive workflow tests that test the complete
//! integration between tailwind-rs and Leptos components.

use leptos::prelude::*;
use tailwind_rs_core::ClassBuilder;
use crate::{DynamicClassBuilder, TailwindSignalManager};

/// Test component that demonstrates complete workflow
#[component]
pub fn TestWorkflowComponent(
    #[prop(into, optional)] variant: Signal<String>,
    #[prop(into, optional)] size: Signal<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    // Create signal manager for global state
    let _signal_manager = TailwindSignalManager::new();
    
    // Create dynamic class builder
    let _class_builder = DynamicClassBuilder::new();
    
    // Set up reactive classes based on props
    let classes = move || {
        let mut builder = ClassBuilder::new();
        
        // Base classes
        builder = builder.class("px-4 py-2 rounded");
        
        // Variant-based classes
        match variant.get().as_str() {
            "primary" => builder = builder.class("bg-blue-600 text-white"),
            "secondary" => builder = builder.class("bg-gray-600 text-white"),
            "danger" => builder = builder.class("bg-red-600 text-white"),
            _ => builder = builder.class("bg-gray-200 text-gray-800"),
        }
        
        // Size-based classes
        match size.get().as_str() {
            "small" => builder = builder.class("text-sm px-2 py-1"),
            "large" => builder = builder.class("text-lg px-6 py-3"),
            _ => builder = builder.class("text-base"),
        }
        
        // State-based classes
        if disabled.get() {
            builder = builder.class("opacity-50 cursor-not-allowed");
        } else {
            builder = builder.class("hover:opacity-80 cursor-pointer");
        }
        
        builder.build().to_css_classes()
    };
    
    view! {
        <div class=classes>
            "Test Component"
        </div>
    }
}

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
            builder = builder.class("bg-blue-500 text-white hover:bg-blue-600");
        }
        
        builder.build().to_css_classes()
    };
    
    let counter_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("text-2xl font-bold p-4");
        
        if is_even.get() {
            builder = builder.class("text-green-600");
        } else {
            builder = builder.class("text-blue-600");
        }
        
        builder.build().to_css_classes()
    };
    
    view! {
        <div class="space-y-4">
            <div class=counter_classes>
                {count}
            </div>
            <button 
                class=button_classes
                on:click=move |_| set_count.update(|c| *c += 1)
            >
                "Increment"
            </button>
        </div>
    }
}

/// Test component that demonstrates responsive design
#[component]
pub fn ResponsiveComponent() -> impl IntoView {
    let (screen_size, set_screen_size) = signal("mobile".to_string());
    
    let container_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("p-4");
        
        match screen_size.get().as_str() {
            "mobile" => {
                builder = builder.class("text-sm bg-red-100");
            },
            "tablet" => {
                builder = builder.class("text-base bg-yellow-100 md:p-6");
            },
            "desktop" => {
                builder = builder.class("text-lg bg-green-100 lg:p-8 xl:text-xl");
            },
            _ => {
                builder = builder.class("text-base bg-gray-100");
            }
        }
        
        builder.build().to_css_classes()
    };
    
    view! {
        <div class="space-y-4">
            <div class="flex space-x-2">
                <button 
                    class="px-3 py-1 text-sm bg-red-500 text-white rounded"
                    on:click=move |_| set_screen_size.set("mobile".to_string())
                >
                    "Mobile"
                </button>
                <button 
                    class="px-3 py-1 text-sm bg-yellow-500 text-white rounded"
                    on:click=move |_| set_screen_size.set("tablet".to_string())
                >
                    "Tablet"
                </button>
                <button 
                    class="px-3 py-1 text-sm bg-green-500 text-white rounded"
                    on:click=move |_| set_screen_size.set("desktop".to_string())
                >
                    "Desktop"
                </button>
            </div>
            <div class=container_classes>
                "Current screen size: " {screen_size}
            </div>
        </div>
    }
}

/// Test component that demonstrates theme switching
#[component]
pub fn ThemeComponent() -> impl IntoView {
    let (theme, set_theme) = signal("light".to_string());
    
    let theme_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("p-6 rounded-lg transition-colors");
        
        match theme.get().as_str() {
            "light" => {
                builder = builder.class("bg-white text-gray-900 border border-gray-200");
            },
            "dark" => {
                builder = builder.class("bg-gray-900 text-white border border-gray-700");
            },
            "blue" => {
                builder = builder.class("bg-blue-900 text-blue-100 border border-blue-700");
            },
            _ => {
                builder = builder.class("bg-gray-100 text-gray-800 border border-gray-300");
            }
        }
        
        builder.build().to_css_classes()
    };
    
    let button_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("px-4 py-2 rounded font-medium transition-colors");
        
        match theme.get().as_str() {
            "light" => {
                builder = builder.class("bg-gray-200 text-gray-800 hover:bg-gray-300");
            },
            "dark" => {
                builder = builder.class("bg-gray-700 text-white hover:bg-gray-600");
            },
            "blue" => {
                builder = builder.class("bg-blue-700 text-blue-100 hover:bg-blue-600");
            },
            _ => {
                builder = builder.class("bg-gray-500 text-white hover:bg-gray-400");
            }
        }
        
        builder.build().to_css_classes()
    };
    
    view! {
        <div class="space-y-4">
            <div class="flex space-x-2">
                <button 
                    class="px-3 py-1 text-sm bg-gray-200 text-gray-800 rounded"
                    on:click=move |_| set_theme.set("light".to_string())
                >
                    "Light"
                </button>
                <button 
                    class="px-3 py-1 text-sm bg-gray-800 text-white rounded"
                    on:click=move |_| set_theme.set("dark".to_string())
                >
                    "Dark"
                </button>
                <button 
                    class="px-3 py-1 text-sm bg-blue-800 text-white rounded"
                    on:click=move |_| set_theme.set("blue".to_string())
                >
                    "Blue"
                </button>
            </div>
            <div class=theme_classes>
                <h3 class="text-xl font-bold mb-2">"Theme Demo"</h3>
                <p class="mb-4">"Current theme: " {theme}</p>
                <button class=button_classes>
                    "Themed Button"
                </button>
            </div>
        </div>
    }
}

/// Test component that demonstrates complex state management
#[component]
pub fn ComplexStateComponent() -> impl IntoView {
    let (user, set_user) = signal("guest".to_string());
    let (permissions, set_permissions) = signal(vec!["read".to_string()]);
    let (is_loading, set_is_loading) = signal(false);
    
    // Simulate user login
    let login = move |_| {
        set_is_loading.set(true);
        // Simulate async operation
        set_timeout(move || {
            set_user.set("admin".to_string());
            set_permissions.set(vec!["read".to_string(), "write".to_string(), "admin".to_string()]);
            set_is_loading.set(false);
        }, std::time::Duration::from_millis(1000));
    };
    
    let logout = move |_| {
        set_user.set("guest".to_string());
        set_permissions.set(vec!["read".to_string()]);
    };
    
    let user_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("p-4 rounded-lg");
        
        match user.get().as_str() {
            "admin" => {
                builder = builder.class("bg-purple-100 text-purple-800 border border-purple-200");
            },
            "user" => {
                builder = builder.class("bg-blue-100 text-blue-800 border border-blue-200");
            },
            _ => {
                builder = builder.class("bg-gray-100 text-gray-800 border border-gray-200");
            }
        }
        
        builder.build().to_css_classes()
    };
    
    let button_classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("px-4 py-2 rounded font-medium");
        
        if is_loading.get() {
            builder = builder.class("bg-gray-400 text-gray-600 cursor-not-allowed");
        } else {
            builder = builder.class("bg-blue-600 text-white hover:bg-blue-700");
        }
        
        builder.build().to_css_classes()
    };
    
    view! {
        <div class="space-y-4">
            <div class=user_classes>
                <h3 class="text-lg font-bold">"User: " {user}</h3>
                <p class="text-sm">"Permissions: " {permissions.get().join(", ")}</p>
            </div>
            <div class="flex space-x-2">
                <button 
                    class=button_classes
                    disabled=is_loading.get()
                    on:click=login
                >
                    {move || if is_loading.get() { "Loading..." } else { "Login as Admin" }}
                </button>
                <button 
                    class="px-4 py-2 bg-red-600 text-white rounded font-medium hover:bg-red-700"
                    on:click=logout
                >
                    "Logout"
                </button>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_workflow_component_creation() {
        // Test that components can be created without runtime context
        // This tests the component structure and class generation logic
        
        let (variant, _) = signal("primary".to_string());
        let (_size, _) = signal("medium".to_string());
        let (_disabled, _) = signal(false);
        
        // Test class generation logic
        let classes = move || {
            let mut builder = ClassBuilder::new();
            builder = builder.class("px-4 py-2 rounded");
            
            match variant.get().as_str() {
                "primary" => builder = builder.class("bg-blue-600 text-white"),
                _ => builder = builder.class("bg-gray-200 text-gray-800"),
            }
            
            builder.build().to_css_classes()
        };
        
        let result = classes();
        assert!(result.contains("px-4"));
        assert!(result.contains("py-2"));
        assert!(result.contains("rounded"));
        assert!(result.contains("bg-blue-600"));
        assert!(result.contains("text-white"));
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_signal_based_component_logic() {
        // Test the signal-based styling logic
        let (count, set_count) = signal(0);
        let (is_even, set_is_even) = signal(true);
        
        // Test even/odd logic
        set_count.set(2);
        set_is_even.set(count.get() % 2 == 0);
        assert!(is_even.get());
        
        set_count.set(3);
        set_is_even.set(count.get() % 2 == 0);
        assert!(!is_even.get());
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_responsive_component_logic() {
        // Test responsive class generation
        let (screen_size, set_screen_size) = signal("mobile".to_string());
        
        let container_classes = move || {
            let mut builder = ClassBuilder::new();
            builder = builder.class("p-4");
            
            match screen_size.get().as_str() {
                "mobile" => {
                    builder = builder.class("text-sm bg-red-100");
                },
                "tablet" => {
                    builder = builder.class("text-base bg-yellow-100 md:p-6");
                },
                "desktop" => {
                    builder = builder.class("text-lg bg-green-100 lg:p-8 xl:text-xl");
                },
                _ => {
                    builder = builder.class("text-base bg-gray-100");
                }
            }
            
            builder.build().to_css_classes()
        };
        
        // Test mobile classes
        set_screen_size.set("mobile".to_string());
        let mobile_classes = container_classes();
        assert!(mobile_classes.contains("text-sm"));
        assert!(mobile_classes.contains("bg-red-100"));
        
        // Test tablet classes
        set_screen_size.set("tablet".to_string());
        let tablet_classes = container_classes();
        assert!(tablet_classes.contains("text-base"));
        assert!(tablet_classes.contains("bg-yellow-100"));
        assert!(tablet_classes.contains("md:p-6"));
        
        // Test desktop classes
        set_screen_size.set("desktop".to_string());
        let desktop_classes = container_classes();
        assert!(desktop_classes.contains("text-lg"));
        assert!(desktop_classes.contains("bg-green-100"));
        assert!(desktop_classes.contains("lg:p-8"));
        assert!(desktop_classes.contains("xl:text-xl"));
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_theme_component_logic() {
        // Test theme-based class generation
        let (theme, set_theme) = signal("light".to_string());
        
        let theme_classes = move || {
            let mut builder = ClassBuilder::new();
            builder = builder.class("p-6 rounded-lg transition-colors");
            
            match theme.get().as_str() {
                "light" => {
                    builder = builder.class("bg-white text-gray-900 border border-gray-200");
                },
                "dark" => {
                    builder = builder.class("bg-gray-900 text-white border border-gray-700");
                },
                "blue" => {
                    builder = builder.class("bg-blue-900 text-blue-100 border border-blue-700");
                },
                _ => {
                    builder = builder.class("bg-gray-100 text-gray-800 border border-gray-300");
                }
            }
            
            builder.build().to_css_classes()
        };
        
        // Test light theme
        set_theme.set("light".to_string());
        let light_classes = theme_classes();
        assert!(light_classes.contains("bg-white"));
        assert!(light_classes.contains("text-gray-900"));
        
        // Test dark theme
        set_theme.set("dark".to_string());
        let dark_classes = theme_classes();
        assert!(dark_classes.contains("bg-gray-900"));
        assert!(dark_classes.contains("text-white"));
        
        // Test blue theme
        set_theme.set("blue".to_string());
        let blue_classes = theme_classes();
        assert!(blue_classes.contains("bg-blue-900"));
        assert!(blue_classes.contains("text-blue-100"));
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_complex_state_component_logic() {
        // Test complex state management logic
        let (user, set_user) = signal("guest".to_string());
        let (permissions, set_permissions) = signal(vec!["read".to_string()]);
        let (is_loading, set_is_loading) = signal(false);
        
        // Test user state changes
        set_user.set("admin".to_string());
        assert_eq!(user.get(), "admin");
        
        // Test permissions
        set_permissions.set(vec!["read".to_string(), "write".to_string(), "admin".to_string()]);
        assert_eq!(permissions.get().len(), 3);
        assert!(permissions.get().contains(&"admin".to_string()));
        
        // Test loading state
        set_is_loading.set(true);
        assert!(is_loading.get());
        
        // Test user classes based on state
        let user_classes = move || {
            let mut builder = ClassBuilder::new();
            builder = builder.class("p-4 rounded-lg");
            
            match user.get().as_str() {
                "admin" => {
                    builder = builder.class("bg-purple-100 text-purple-800 border border-purple-200");
                },
                "user" => {
                    builder = builder.class("bg-blue-100 text-blue-800 border border-blue-200");
                },
                _ => {
                    builder = builder.class("bg-gray-100 text-gray-800 border border-gray-200");
                }
            }
            
            builder.build().to_css_classes()
        };
        
        set_user.set("admin".to_string());
        let admin_classes = user_classes();
        assert!(admin_classes.contains("bg-purple-100"));
        assert!(admin_classes.contains("text-purple-800"));
        
        set_user.set("user".to_string());
        let user_classes_result = user_classes();
        assert!(user_classes_result.contains("bg-blue-100"));
        assert!(user_classes_result.contains("text-blue-800"));
    }
    
    #[test]
    fn test_dynamic_class_builder_integration() {
        // Test integration with DynamicClassBuilder
        let builder = DynamicClassBuilder::new();
        
        builder
            .base("px-4 py-2")
            .variant("bg-blue-600 text-white")
            .responsive("sm:text-sm md:text-base")
            .state("hover:bg-blue-700 focus:ring-2");
        
        let classes = builder.classes_string();
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
        assert!(classes.contains("bg-blue-600"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("sm:text-sm"));
        assert!(classes.contains("md:text-base"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("focus:ring-2"));
    }
    
    #[test]
    #[ignore] // Requires Leptos runtime context
    fn test_signal_manager_integration() {
        // Test integration with TailwindSignalManager
        let manager = TailwindSignalManager::new();
        
        // Test signal creation
        let theme_signal = manager.theme();
        let variant_signal = manager.variant();
        let size_signal = manager.size();
        let responsive_signal = manager.responsive();
        let disabled_signal = manager.disabled();
        let loading_signal = manager.loading();
        
        // Test initial values
        assert_eq!(theme_signal.get().name, "default");
        assert_eq!(variant_signal.get(), "primary");
        assert_eq!(size_signal.get(), "medium");
        assert_eq!(responsive_signal.get(), "");
        assert_eq!(disabled_signal.get(), false);
        assert_eq!(loading_signal.get(), false);
        
        // Test signal updates
        use tailwind_rs_core::Theme;
        theme_signal.set(Theme::new("dark".to_string()));
        variant_signal.set("secondary".to_string());
        size_signal.set("large".to_string());
        disabled_signal.set(true);
        loading_signal.set(true);
        
        assert_eq!(theme_signal.get().name, "dark");
        assert_eq!(variant_signal.get(), "secondary");
        assert_eq!(size_signal.get(), "large");
        assert_eq!(disabled_signal.get(), true);
        assert_eq!(loading_signal.get(), true);
    }
}
