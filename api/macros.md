# Macros Reference

This document provides comprehensive reference for all procedural macros in `tailwind-rs`, following our Test-Driven Development approach and comprehensive testing strategy.

## 🎯 Core Macros

### `classes!` Macro
The primary macro for type-safe class generation with compile-time validation.

#### Syntax
```rust
classes! {
    base: <base_classes>,
    variant: <variant_classes>,
    responsive: <responsive_classes>,
    state: <state_classes>,
    // ... additional patterns
}
```

#### Basic Usage
```rust
use tailwind_rs::*;

// Simple base classes
let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium",
};

// With variants
let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium",
    variant: "bg-blue-600 text-white hover:bg-blue-700",
};
```

#### Responsive Design
```rust
use tailwind_rs::*;

// Responsive classes
let classes = classes! {
    base: "text-sm",
    responsive: Responsive {
        sm: "text-base",
        md: "text-lg",
        lg: "text-xl",
    },
};

// Generates: "text-sm sm:text-base md:text-lg lg:text-xl"
```

#### State Classes
```rust
use tailwind_rs::*;

// State-based classes
let classes = classes! {
    base: "px-4 py-2 rounded-md",
    state: "hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500",
};
```

#### Complex Example
```rust
use tailwind_rs::*;

let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium transition-colors duration-200",
    variant: "bg-blue-600 text-white",
    responsive: Responsive {
        sm: "text-sm",
        md: "text-base",
        lg: "text-lg",
    },
    state: "hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500",
};
```

#### Compile-Time Validation
```rust
use tailwind_rs::*;

// This will cause a compile-time error for invalid classes
let classes = classes! {
    base: "invalid-class-name", // ❌ Compile error
};

// This will compile successfully
let classes = classes! {
    base: "bg-blue-600 text-white", // ✅ Valid classes
};
```

### `responsive!` Macro
Specialized macro for responsive class generation.

#### Syntax
```rust
responsive! {
    base: <base_classes>,
    sm: <small_classes>,
    md: <medium_classes>,
    lg: <large_classes>,
    xl: <extra_large_classes>,
    xl2: <extra_extra_large_classes>,
}
```

#### Usage
```rust
use tailwind_rs::*;

let classes = responsive! {
    base: "text-sm",
    sm: "text-base",
    md: "text-lg",
    lg: "text-xl",
    xl: "text-2xl",
};

// Generates: "text-sm sm:text-base md:text-lg lg:text-xl xl:text-2xl"
```

#### With Different Properties
```rust
use tailwind_rs::*;

let classes = responsive! {
    base: "p-4",
    sm: "p-6",
    md: "p-8",
    lg: "p-10",
    xl: "p-12",
};

// Generates: "p-4 sm:p-6 md:p-8 lg:p-10 xl:p-12"
```

### `state!` Macro
Specialized macro for state-based class generation.

#### Syntax
```rust
state! {
    base: <base_classes>,
    hover: <hover_classes>,
    focus: <focus_classes>,
    active: <active_classes>,
    disabled: <disabled_classes>,
    // ... other states
}
```

#### Usage
```rust
use tailwind_rs::*;

let classes = state! {
    base: "px-4 py-2 rounded-md",
    hover: "bg-blue-700",
    focus: "outline-none ring-2 ring-blue-500",
    active: "bg-blue-800",
    disabled: "opacity-50 cursor-not-allowed",
};

// Generates: "px-4 py-2 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 active:bg-blue-800 disabled:opacity-50 disabled:cursor-not-allowed"
```

### `variant!` Macro
Specialized macro for variant-based class generation.

#### Syntax
```rust
variant! {
    base: <base_classes>,
    primary: <primary_classes>,
    secondary: <secondary_classes>,
    danger: <danger_classes>,
    // ... other variants
}
```

#### Usage
```rust
use tailwind_rs::*;

let classes = variant! {
    base: "px-4 py-2 rounded-md font-medium",
    primary: "bg-blue-600 text-white hover:bg-blue-700",
    secondary: "bg-gray-200 text-gray-900 hover:bg-gray-300",
    danger: "bg-red-600 text-white hover:bg-red-700",
};

// Generates: "px-4 py-2 rounded-md font-medium bg-blue-600 text-white hover:bg-blue-700 bg-gray-200 text-gray-900 hover:bg-gray-300 bg-red-600 text-white hover:bg-red-700"
```

## 🎨 Color Macros

### `color!` Macro
Generates color-based classes with intensity.

#### Syntax
```rust
color! {
    <color_name>: <intensity> => {
        background: <background_classes>,
        text: <text_classes>,
        border: <border_classes>,
        ring: <ring_classes>,
    }
}
```

#### Usage
```rust
use tailwind_rs::*;

let classes = color! {
    blue: 600 => {
        background: "bg-blue-600",
        text: "text-white",
        border: "border-blue-600",
        ring: "ring-blue-600",
    }
};

// Generates: "bg-blue-600 text-white border-blue-600 ring-blue-600"
```

### `color_variant!` Macro
Generates color variants with different intensities.

#### Syntax
```rust
color_variant! {
    <color_name> => {
        light: <light_intensity>,
        medium: <medium_intensity>,
        dark: <dark_intensity>,
    }
}
```

#### Usage
```rust
use tailwind_rs::*;

let classes = color_variant! {
    blue => {
        light: 500,
        medium: 600,
        dark: 700,
    }
};

// Generates: "bg-blue-500 hover:bg-blue-600 active:bg-blue-700"
```

## 🧪 Testing Macros

### `test_classes!` Macro
Generates test classes for unit testing.

#### Syntax
```rust
test_classes! {
    <test_name> => {
        input: <input_classes>,
        expected: <expected_classes>,
    }
}
```

#### Usage
```rust
use tailwind_rs::*;

test_classes! {
    button_primary => {
        input: "px-4 py-2 bg-blue-600 text-white",
        expected: "px-4 py-2 bg-blue-600 text-white",
    }
}
```

### `benchmark_classes!` Macro
Generates benchmark classes for performance testing.

#### Syntax
```rust
benchmark_classes! {
    <benchmark_name> => {
        classes: <classes_to_benchmark>,
        iterations: <number_of_iterations>,
    }
}
```

#### Usage
```rust
use tailwind_rs::*;

benchmark_classes! {
    button_generation => {
        classes: "px-4 py-2 bg-blue-600 text-white hover:bg-blue-700",
        iterations: 1000,
    }
}
```

## 🎯 Component Macros

### `component_classes!` Macro
Generates classes for component variants.

#### Syntax
```rust
component_classes! {
    <component_name> => {
        base: <base_classes>,
        variants: {
            <variant_name>: <variant_classes>,
            // ... more variants
        },
        sizes: {
            <size_name>: <size_classes>,
            // ... more sizes
        },
    }
}
```

#### Usage
```rust
use tailwind_rs::*;

component_classes! {
    button => {
        base: "rounded-md font-medium transition-colors duration-200",
        variants: {
            primary: "bg-blue-600 text-white hover:bg-blue-700",
            secondary: "bg-gray-200 text-gray-900 hover:bg-gray-300",
            danger: "bg-red-600 text-white hover:bg-red-700",
        },
        sizes: {
            small: "px-2 py-1 text-sm",
            medium: "px-4 py-2 text-base",
            large: "px-6 py-3 text-lg",
        },
    }
}
```

### `form_classes!` Macro
Generates classes for form elements.

#### Syntax
```rust
form_classes! {
    <form_element> => {
        base: <base_classes>,
        states: {
            <state_name>: <state_classes>,
            // ... more states
        },
        validation: {
            valid: <valid_classes>,
            invalid: <invalid_classes>,
        },
    }
}
```

#### Usage
```rust
use tailwind_rs::*;

form_classes! {
    input => {
        base: "block w-full rounded-md border-gray-300 shadow-sm",
        states: {
            focus: "border-blue-500 ring-1 ring-blue-500",
            disabled: "bg-gray-50 text-gray-500 cursor-not-allowed",
        },
        validation: {
            valid: "border-green-500 ring-1 ring-green-500",
            invalid: "border-red-500 ring-1 ring-red-500",
        },
    }
}
```

## 🚀 Performance Macros

### `cached_classes!` Macro
Generates cached classes for performance optimization.

#### Syntax
```rust
cached_classes! {
    <cache_key> => {
        classes: <classes_to_cache>,
        ttl: <time_to_live_in_seconds>,
    }
}
```

#### Usage
```rust
use tailwind_rs::*;

cached_classes! {
    "button-primary" => {
        classes: "px-4 py-2 bg-blue-600 text-white hover:bg-blue-700",
        ttl: 3600, // 1 hour
    }
}
```

### `optimized_classes!` Macro
Generates optimized classes with performance hints.

#### Syntax
```rust
optimized_classes! {
    <optimization_level> => {
        classes: <classes_to_optimize>,
    }
}
```

#### Usage
```rust
use tailwind_rs::*;

optimized_classes! {
    high => {
        classes: "px-4 py-2 bg-blue-600 text-white hover:bg-blue-700 focus:ring-2 focus:ring-blue-500",
    }
}
```

## 🎨 Theme Macros

### `theme_classes!` Macro
Generates theme-based classes.

#### Syntax
```rust
theme_classes! {
    <theme_name> => {
        <component_name>: {
            <variant_name>: <variant_classes>,
            // ... more variants
        },
        // ... more components
    }
}
```

#### Usage
```rust
use tailwind_rs::*;

theme_classes! {
    default => {
        button: {
            primary: "bg-blue-600 text-white hover:bg-blue-700",
            secondary: "bg-gray-200 text-gray-900 hover:bg-gray-300",
        },
        input: {
            default: "border-gray-300 focus:border-blue-500",
            error: "border-red-500 focus:border-red-500",
        },
    }
}
```

### `dark_theme_classes!` Macro
Generates dark theme classes.

#### Syntax
```rust
dark_theme_classes! {
    <component_name> => {
        <variant_name>: <variant_classes>,
        // ... more variants
    }
}
```

#### Usage
```rust
use tailwind_rs::*;

dark_theme_classes! {
    button => {
        primary: "bg-blue-500 text-white hover:bg-blue-600 dark:bg-blue-700 dark:hover:bg-blue-800",
        secondary: "bg-gray-200 text-gray-900 hover:bg-gray-300 dark:bg-gray-700 dark:text-gray-100 dark:hover:bg-gray-600",
    }
}
```

## 🔒 Security Macros

### `sanitized_classes!` Macro
Generates sanitized classes with security validation.

#### Syntax
```rust
sanitized_classes! {
    <classes_to_sanitize>
}
```

#### Usage
```rust
use tailwind_rs::*;

let classes = sanitized_classes! {
    "px-4 py-2 bg-blue-600 text-white"
};

// Automatically sanitizes and validates classes
```

### `validated_classes!` Macro
Generates validated classes with strict validation.

#### Syntax
```rust
validated_classes! {
    <classes_to_validate>
}
```

#### Usage
```rust
use tailwind_rs::*;

let classes = validated_classes! {
    "px-4 py-2 bg-blue-600 text-white"
};

// Strictly validates all classes at compile time
```

## 📊 Metrics Macros

### `metrics_classes!` Macro
Generates classes with metrics collection.

#### Syntax
```rust
metrics_classes! {
    <metric_name> => {
        classes: <classes_to_measure>,
    }
}
```

#### Usage
```rust
use tailwind_rs::*;

metrics_classes! {
    "button-generation-time" => {
        classes: "px-4 py-2 bg-blue-600 text-white",
    }
}
```

## 🧪 Testing Examples

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classes_macro() {
        let classes = classes! {
            base: "px-4 py-2",
            variant: "bg-blue-600 text-white",
        };
        
        assert!(classes.contains("px-4"));
        assert!(classes.contains("bg-blue-600"));
    }

    #[test]
    fn test_responsive_macro() {
        let classes = responsive! {
            base: "text-sm",
            sm: "text-base",
            md: "text-lg",
        };
        
        assert!(classes.contains("text-sm"));
        assert!(classes.contains("sm:text-base"));
        assert!(classes.contains("md:text-lg"));
    }

    #[test]
    fn test_state_macro() {
        let classes = state! {
            base: "px-4 py-2",
            hover: "bg-blue-700",
            focus: "ring-2 ring-blue-500",
        };
        
        assert!(classes.contains("px-4"));
        assert!(classes.contains("hover:bg-blue-700"));
        assert!(classes.contains("focus:ring-2"));
    }
}
```

### Integration Testing
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_macro_integration() {
        let classes = classes! {
            base: "px-4 py-2 rounded-md font-medium",
            responsive: responsive! {
                base: "text-sm",
                sm: "text-base",
                md: "text-lg",
            },
            state: state! {
                base: "transition-colors duration-200",
                hover: "bg-blue-700",
                focus: "outline-none ring-2 ring-blue-500",
            },
        };
        
        assert!(!classes.is_empty());
        assert!(classes.contains("px-4"));
        assert!(classes.contains("sm:text-base"));
        assert!(classes.contains("hover:bg-blue-700"));
    }
}
```

### Performance Testing
```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_macro_performance() {
        let start = Instant::now();
        
        for _ in 0..1000 {
            let _classes = classes! {
                base: "px-4 py-2 bg-blue-600 text-white",
                state: "hover:bg-blue-700 focus:ring-2",
            };
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should complete in under 100ms
    }
}
```

## 📚 Best Practices

### 1. Use Appropriate Macros
```rust
// Good: Use specific macros for specific purposes
let responsive_classes = responsive! {
    base: "text-sm",
    sm: "text-base",
    md: "text-lg",
};

let state_classes = state! {
    base: "px-4 py-2",
    hover: "bg-blue-700",
    focus: "ring-2",
};

// Avoid: Mixing everything in one macro
let mixed_classes = classes! {
    base: "text-sm sm:text-base md:text-lg px-4 py-2 hover:bg-blue-700 focus:ring-2",
};
```

### 2. Compose Macros Logically
```rust
// Good: Logical composition
let classes = classes! {
    base: "px-4 py-2 rounded-md font-medium",
    responsive: responsive! {
        base: "text-sm",
        sm: "text-base",
        md: "text-lg",
    },
    state: state! {
        base: "transition-colors duration-200",
        hover: "bg-blue-700",
        focus: "outline-none ring-2 ring-blue-500",
    },
};
```

### 3. Use Type-Safe Variants
```rust
// Good: Type-safe variants
let classes = variant! {
    base: "px-4 py-2 rounded-md",
    primary: "bg-blue-600 text-white",
    secondary: "bg-gray-200 text-gray-900",
};

// Avoid: String-based variants
let classes = classes! {
    base: "px-4 py-2 rounded-md",
    variant: "bg-blue-600 text-white", // This could be any string
};
```

### 4. Test All Macro Combinations
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_macro_combinations() {
        // Test classes! macro
        let classes = classes! {
            base: "px-4 py-2",
            variant: "bg-blue-600 text-white",
        };
        assert!(!classes.is_empty());
        
        // Test responsive! macro
        let responsive = responsive! {
            base: "text-sm",
            sm: "text-base",
        };
        assert!(responsive.contains("sm:text-base"));
        
        // Test state! macro
        let state = state! {
            base: "px-4 py-2",
            hover: "bg-blue-700",
        };
        assert!(state.contains("hover:bg-blue-700"));
    }
}
```

---

This macros reference provides comprehensive documentation for all procedural macros in `tailwind-rs`. All macros are designed with type safety, performance, and testability in mind, following our established ADRs and best practices.

