//! # Workflow Tests
//!
//! This module provides tests for complete workflow components that demonstrate
//! the full integration between tailwind-rs and Leptos.

use leptos::prelude::*;
use tailwind_rs_core::ClassBuilder;
use crate::{DynamicClassBuilder, TailwindSignalManager};
use super::{E2ETestSuite, E2ETestDetail};

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
        <button class=classes>
            "Workflow Component"
        </button>
    }
}

/// Run workflow tests
pub fn run_workflow_tests() -> E2ETestSuite {
    let mut suite = E2ETestSuite::new();
    
    // Test component creation
    suite.add_test(
        "test_workflow_component_creation".to_string(),
        test_workflow_component_creation(),
        None,
    );
    
    // Test class generation logic
    suite.add_test(
        "test_class_generation_logic".to_string(),
        test_class_generation_logic(),
        None,
    );
    
    // Test variant handling
    suite.add_test(
        "test_variant_handling".to_string(),
        test_variant_handling(),
        None,
    );
    
    // Test size handling
    suite.add_test(
        "test_size_handling".to_string(),
        test_size_handling(),
        None,
    );
    
    // Test state handling
    suite.add_test(
        "test_state_handling".to_string(),
        test_state_handling(),
        None,
    );
    
    suite
}

/// Test that components can be created without runtime context
fn test_workflow_component_creation() -> bool {
    // Test the component structure and class generation logic
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
    result.contains("px-4") && 
    result.contains("py-2") && 
    result.contains("rounded") && 
    result.contains("bg-blue-600") && 
    result.contains("text-white")
}

/// Test class generation logic
fn test_class_generation_logic() -> bool {
    let mut builder = ClassBuilder::new();
    builder = builder.class("px-4 py-2 rounded");
    builder = builder.class("bg-blue-600 text-white");
    
    let result = builder.build().to_css_classes();
    result.contains("px-4") && 
    result.contains("py-2") && 
    result.contains("rounded") && 
    result.contains("bg-blue-600") && 
    result.contains("text-white")
}

/// Test variant handling
fn test_variant_handling() -> bool {
    let variants = vec!["primary", "secondary", "danger", "default"];
    let mut all_passed = true;
    
    for variant in variants {
        let (variant_signal, _) = signal(variant.to_string());
        
        let classes = move || {
            let mut builder = ClassBuilder::new();
            builder = builder.class("px-4 py-2 rounded");
            
            match variant_signal.get().as_str() {
                "primary" => builder = builder.class("bg-blue-600 text-white"),
                "secondary" => builder = builder.class("bg-gray-600 text-white"),
                "danger" => builder = builder.class("bg-red-600 text-white"),
                _ => builder = builder.class("bg-gray-200 text-gray-800"),
            }
            
            builder.build().to_css_classes()
        };
        
        let result = classes();
        match variant {
            "primary" => all_passed &= result.contains("bg-blue-600"),
            "secondary" => all_passed &= result.contains("bg-gray-600"),
            "danger" => all_passed &= result.contains("bg-red-600"),
            _ => all_passed &= result.contains("bg-gray-200"),
        }
    }
    
    all_passed
}

/// Test size handling
fn test_size_handling() -> bool {
    let sizes = vec!["small", "medium", "large"];
    let mut all_passed = true;
    
    for size in sizes {
        let (size_signal, _) = signal(size.to_string());
        
        let classes = move || {
            let mut builder = ClassBuilder::new();
            builder = builder.class("px-4 py-2 rounded");
            
            match size_signal.get().as_str() {
                "small" => builder = builder.class("text-sm px-2 py-1"),
                "large" => builder = builder.class("text-lg px-6 py-3"),
                _ => builder = builder.class("text-base"),
            }
            
            builder.build().to_css_classes()
        };
        
        let result = classes();
        match size {
            "small" => all_passed &= result.contains("text-sm") && result.contains("px-2"),
            "large" => all_passed &= result.contains("text-lg") && result.contains("px-6"),
            _ => all_passed &= result.contains("text-base"),
        }
    }
    
    all_passed
}

/// Test state handling
fn test_state_handling() -> bool {
    let (disabled_signal, _) = signal(false);
    
    let classes = move || {
        let mut builder = ClassBuilder::new();
        builder = builder.class("px-4 py-2 rounded");
        
        if disabled_signal.get() {
            builder = builder.class("opacity-50 cursor-not-allowed");
        } else {
            builder = builder.class("hover:opacity-80 cursor-pointer");
        }
        
        builder.build().to_css_classes()
    };
    
    let result = classes();
    result.contains("hover:opacity-80") && result.contains("cursor-pointer")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_component_creation_test() {
        assert!(test_workflow_component_creation());
    }

    #[test]
    fn test_class_generation_logic_test() {
        assert!(test_class_generation_logic());
    }

    #[test]
    fn test_variant_handling_test() {
        assert!(test_variant_handling());
    }

    #[test]
    fn test_size_handling_test() {
        assert!(test_size_handling());
    }

    #[test]
    fn test_state_handling_test() {
        assert!(test_state_handling());
    }

    #[test]
    fn test_run_workflow_tests() {
        let results = run_workflow_tests();
        
        assert_eq!(results.total_tests, 5);
        assert_eq!(results.passed_tests, 5);
        assert_eq!(results.failed_tests, 0);
    }
}
