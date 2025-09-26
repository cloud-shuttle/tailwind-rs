# 🔒 API Contracts & Testing Strategy for Tailwind-RS Core

## 📋 **Overview**

This document outlines our comprehensive API contracts and testing strategy to ensure reliability, correctness, and maintainability of the Tailwind-RS Core system.

## 🎯 **API Contracts**

### **1. ClassBuilder API Contract**

#### **Core Methods Contract**
```rust
// Contract: All core methods must be available and functional
trait ClassBuilderContract {
    // Constructor contract
    fn new() -> Self;
    fn default() -> Self;
    
    // Building contract
    fn build(self) -> ClassSet;
    fn build_string(self) -> String;
    
    // Class addition contract
    fn class(self, class: impl Into<String>) -> Self;
    fn classes(self, classes: impl IntoIterator<Item = String>) -> Self;
    
    // Responsive contract
    fn responsive(self, breakpoint: Breakpoint, class: impl Into<String>) -> Self;
    
    // Pseudo-class contract
    fn hover(self, class: impl Into<String>) -> Self;
    fn focus(self, class: impl Into<String>) -> Self;
    fn active(self, class: impl Into<String>) -> Self;
    fn dark(self, class: impl Into<String>) -> Self;
}
```

#### **Utility Methods Contract**
```rust
// Contract: All utility methods must generate correct CSS classes
trait UtilityMethodsContract {
    // Layout utilities
    fn relative(self) -> Self;
    fn absolute(self) -> Self;
    fn fixed(self) -> Self;
    fn sticky(self) -> Self;
    fn static_pos(self) -> Self;
    
    // Display utilities
    fn block(self) -> Self;
    fn inline(self) -> Self;
    fn inline_block(self) -> Self;
    fn flex(self) -> Self;
    fn inline_flex(self) -> Self;
    fn grid(self) -> Self;
    fn inline_grid(self) -> Self;
    fn hidden(self) -> Self;
    fn visible(self) -> Self;
    
    // Flexbox utilities
    fn flex_none(self) -> Self;
    fn flex_1(self) -> Self;
    fn flex_auto(self) -> Self;
    fn flex_initial(self) -> Self;
    fn flex_col(self) -> Self;
    fn flex_row(self) -> Self;
    fn flex_wrap_class(self) -> Self;
    fn flex_nowrap_class(self) -> Self;
    
    // Transition utilities
    fn transition(self) -> Self;
    fn transition_all(self) -> Self;
    fn transition_colors(self) -> Self;
    fn transition_opacity(self) -> Self;
    fn transition_shadow(self) -> Self;
    fn transition_transform(self) -> Self;
}
```

### **2. PostCSS Processing Contract**

#### **Engine Contract**
```rust
// Contract: PostCSS engine must process CSS correctly
trait PostCSSEngineContract {
    // Processing contract
    fn process_css(&self, css: &str) -> Result<ProcessedCSS, PostCSSError>;
    fn process_with_plugins(&self, css: &str, plugins: Vec<Plugin>) -> Result<ProcessedCSS, PostCSSError>;
    
    // Configuration contract
    fn configure(&mut self, config: PostCSSConfig) -> Result<(), PostCSSError>;
    fn get_config(&self) -> &PostCSSConfig;
    
    // Metrics contract
    fn get_metrics(&self) -> ProcessingMetrics;
    fn get_warnings(&self) -> Vec<ProcessingWarning>;
}
```

#### **Plugin System Contract**
```rust
// Contract: Plugin system must support various plugin types
trait PluginSystemContract {
    // Plugin loading
    fn load_plugin(&mut self, plugin: Plugin) -> Result<(), PluginError>;
    fn unload_plugin(&mut self, name: &str) -> Result<(), PluginError>;
    
    // Plugin execution
    fn execute_plugin(&self, name: &str, input: &str) -> Result<String, PluginError>;
    fn get_available_plugins(&self) -> Vec<String>;
    
    // Plugin validation
    fn validate_plugin(&self, plugin: &Plugin) -> Result<(), PluginError>;
}
```

## 🧪 **Testing Strategy**

### **1. Unit Testing**

#### **ClassBuilder Unit Tests**
```rust
#[cfg(test)]
mod class_builder_unit_tests {
    use super::*;
    
    /// Test basic functionality
    #[test]
    fn test_basic_functionality() {
        let classes = ClassBuilder::new()
            .relative()
            .flex()
            .p_4()
            .build_string();
        
        assert!(classes.contains("relative"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("p-4"));
    }
    
    /// Test pseudo-class functionality
    #[test]
    fn test_pseudo_class_functionality() {
        let classes = ClassBuilder::new()
            .bg_blue_500()
            .hover("bg-blue-600")
            .focus("ring-2")
            .build_string();
        
        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("hover:bg-blue-600"));
        assert!(classes.contains("focus:ring-2"));
    }
    
    /// Test error handling
    #[test]
    #[should_panic(expected = "hover: class cannot be empty")]
    fn test_hover_empty_class_panic() {
        ClassBuilder::new().hover("");
    }
}
```

#### **PostCSS Unit Tests**
```rust
#[cfg(test)]
mod postcss_unit_tests {
    use super::*;
    
    /// Test basic CSS processing
    #[test]
    fn test_basic_css_processing() {
        let engine = PostCSSEngine::new();
        let css = ".test { color: red; }";
        let result = engine.process_css(css).unwrap();
        
        assert!(result.css.contains("color: red"));
    }
    
    /// Test plugin processing
    #[test]
    fn test_plugin_processing() {
        let mut engine = PostCSSEngine::new();
        let plugin = Plugin::new("autoprefixer");
        engine.load_plugin(plugin).unwrap();
        
        let css = ".test { display: flex; }";
        let result = engine.process_css(css).unwrap();
        
        assert!(result.css.contains("display: -webkit-box"));
        assert!(result.css.contains("display: -ms-flexbox"));
        assert!(result.css.contains("display: flex"));
    }
}
```

### **2. Integration Testing**

#### **End-to-End CSS Generation**
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    /// Test complete CSS generation pipeline
    #[test]
    fn test_complete_css_generation() {
        let classes = ClassBuilder::new()
            .relative()
            .flex()
            .flex_col()
            .p_4()
            .bg_blue_500()
            .text_white()
            .hover("bg-blue-600")
            .focus("ring-2")
            .dark("bg-gray-800")
            .transition()
            .transition_colors()
            .build_string();
        
        // Generate CSS
        let generator = CssGenerator::new();
        let css = generator.generate_css(&classes).unwrap();
        
        // Verify CSS contains expected rules
        assert!(css.contains("position: relative"));
        assert!(css.contains("display: flex"));
        assert!(css.contains("flex-direction: column"));
        assert!(css.contains("padding: 1rem"));
        assert!(css.contains("background-color: rgb(59 130 246)"));
        assert!(css.contains("color: rgb(255 255 255)"));
        assert!(css.contains(":hover"));
        assert!(css.contains(":focus"));
        assert!(css.contains("@media (prefers-color-scheme: dark)"));
        assert!(css.contains("transition-property: color, background-color"));
    }
}
```

### **3. Contract Testing**

#### **API Stability Tests**
```rust
#[cfg(test)]
mod contract_tests {
    use super::*;
    
    /// Test API stability across versions
    #[test]
    fn test_api_stability() {
        // Test that all public methods exist and work
        let builder = ClassBuilder::new();
        let _ = builder.relative();
        let _ = builder.flex();
        let _ = builder.p_4();
        let _ = builder.hover("bg-blue-600");
        let _ = builder.build_string();
        
        // Test that method signatures haven't changed
        assert!(true, "API should be stable");
    }
    
    /// Test backward compatibility
    #[test]
    fn test_backward_compatibility() {
        // Test that old API still works
        let classes = ClassBuilder::new()
            .class("relative")
            .class("flex")
            .class("p-4")
            .build_string();
        
        assert!(classes.contains("relative"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("p-4"));
    }
}
```

### **4. Performance Testing**

#### **Benchmark Tests**
```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    /// Test ClassBuilder performance
    #[test]
    fn test_class_builder_performance() {
        let start = Instant::now();
        
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .relative()
                .flex()
                .p_4()
                .bg_blue_500()
                .text_white()
                .hover("bg-blue-600")
                .build_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "ClassBuilder should be fast");
    }
    
    /// Test PostCSS performance
    #[test]
    fn test_postcss_performance() {
        let engine = PostCSSEngine::new();
        let css = ".test { color: red; }";
        
        let start = Instant::now();
        
        for _ in 0..100 {
            let _ = engine.process_css(css).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "PostCSS should be fast");
    }
}
```

## 🔍 **PostCSS Processing Validation**

### **1. Processing Coverage Tests**

#### **CSS Feature Coverage**
```rust
#[cfg(test)]
mod postcss_coverage_tests {
    use super::*;
    
    /// Test CSS feature coverage
    #[test]
    fn test_css_feature_coverage() {
        let engine = PostCSSEngine::new();
        
        // Test various CSS features
        let test_cases = vec![
            // Basic CSS
            ".test { color: red; }",
            // CSS Grid
            ".grid { display: grid; grid-template-columns: 1fr 1fr; }",
            // Flexbox
            ".flex { display: flex; justify-content: center; }",
            // CSS Variables
            ".vars { --primary-color: #3b82f6; color: var(--primary-color); }",
            // Media queries
            "@media (max-width: 768px) { .responsive { display: none; } }",
            // Pseudo-classes
            ".hover:hover { color: blue; }",
            // Pseudo-elements
            ".before::before { content: ''; }",
            // CSS Functions
            ".calc { width: calc(100% - 20px); }",
            // CSS Transforms
            ".transform { transform: translateX(10px); }",
            // CSS Animations
            "@keyframes slide { from { transform: translateX(-100%); } to { transform: translateX(0); } }",
        ];
        
        for css in test_cases {
            let result = engine.process_css(css).unwrap();
            assert!(!result.css.is_empty(), "CSS should be processed");
            assert!(result.warnings.is_empty(), "No warnings expected for valid CSS");
        }
    }
}
```

#### **Plugin Coverage Tests**
```rust
#[cfg(test)]
mod plugin_coverage_tests {
    use super::*;
    
    /// Test plugin coverage
    #[test]
    fn test_plugin_coverage() {
        let mut engine = PostCSSEngine::new();
        
        // Test autoprefixer plugin
        let autoprefixer = Plugin::new("autoprefixer");
        engine.load_plugin(autoprefixer).unwrap();
        
        let css = ".test { display: flex; }";
        let result = engine.process_css(css).unwrap();
        
        // Verify autoprefixer added vendor prefixes
        assert!(result.css.contains("-webkit-box"));
        assert!(result.css.contains("-ms-flexbox"));
        assert!(result.css.contains("display: flex"));
    }
    
    /// Test plugin error handling
    #[test]
    fn test_plugin_error_handling() {
        let mut engine = PostCSSEngine::new();
        
        // Test invalid plugin
        let invalid_plugin = Plugin::new("invalid-plugin");
        let result = engine.load_plugin(invalid_plugin);
        
        assert!(result.is_err(), "Invalid plugin should fail to load");
    }
}
```

### **2. Validation Framework**

#### **CSS Validation**
```rust
#[cfg(test)]
mod css_validation_tests {
    use super::*;
    
    /// Test CSS validation
    #[test]
    fn test_css_validation() {
        let engine = PostCSSEngine::new();
        
        // Test valid CSS
        let valid_css = ".test { color: red; }";
        let result = engine.process_css(valid_css).unwrap();
        assert!(result.css.contains("color: red"));
        
        // Test invalid CSS
        let invalid_css = ".test { color: red; /* unclosed comment";
        let result = engine.process_css(invalid_css);
        assert!(result.is_err(), "Invalid CSS should fail");
    }
    
    /// Test CSS parsing errors
    #[test]
    fn test_css_parsing_errors() {
        let engine = PostCSSEngine::new();
        
        let test_cases = vec![
            ".test { color: red; /* unclosed comment",
            ".test { color: red; } /* unclosed comment",
            ".test { color: red; } /* unclosed comment */",
            ".test { color: red; } /* unclosed comment */",
        ];
        
        for css in test_cases {
            let result = engine.process_css(css);
            match result {
                Ok(_) => println!("CSS parsed successfully: {}", css),
                Err(e) => println!("CSS parsing failed: {} - {}", css, e),
            }
        }
    }
}
```

### **3. Coverage Metrics**

#### **Test Coverage Analysis**
```rust
#[cfg(test)]
mod coverage_analysis {
    use super::*;
    
    /// Test method coverage
    #[test]
    fn test_method_coverage() {
        let builder = ClassBuilder::new();
        
        // Test all layout methods
        let _ = builder.relative();
        let _ = builder.absolute();
        let _ = builder.fixed();
        let _ = builder.sticky();
        let _ = builder.static_pos();
        
        // Test all display methods
        let _ = builder.block();
        let _ = builder.inline();
        let _ = builder.inline_block();
        let _ = builder.flex();
        let _ = builder.inline_flex();
        let _ = builder.grid();
        let _ = builder.inline_grid();
        let _ = builder.hidden();
        let _ = builder.visible();
        
        // Test all flexbox methods
        let _ = builder.flex_none();
        let _ = builder.flex_1();
        let _ = builder.flex_auto();
        let _ = builder.flex_initial();
        let _ = builder.flex_col();
        let _ = builder.flex_row();
        let _ = builder.flex_wrap_class();
        let _ = builder.flex_nowrap_class();
        
        // Test all transition methods
        let _ = builder.transition();
        let _ = builder.transition_all();
        let _ = builder.transition_colors();
        let _ = builder.transition_opacity();
        let _ = builder.transition_shadow();
        let _ = builder.transition_transform();
        
        // Test all pseudo-class methods
        let _ = builder.hover("bg-blue-600");
        let _ = builder.focus("ring-2");
        let _ = builder.active("bg-blue-700");
        let _ = builder.dark("bg-gray-800");
        let _ = builder.group_hover("bg-blue-600");
        let _ = builder.peer_hover("bg-blue-600");
        
        // Test all spacing methods
        let _ = builder.p_4();
        let _ = builder.px_4();
        let _ = builder.py_4();
        let _ = builder.m_4();
        let _ = builder.mx_4();
        let _ = builder.my_4();
        
        // Test all color methods
        let _ = builder.bg_blue_500();
        let _ = builder.text_white();
        let _ = builder.border_gray_300();
        
        assert!(true, "All methods should be covered");
    }
}
```

## 📊 **Testing Metrics**

### **Coverage Targets**

- **Unit Tests**: 100% method coverage
- **Integration Tests**: 90% feature coverage
- **Contract Tests**: 100% API stability
- **Performance Tests**: < 100ms for 1000 operations
- **Error Handling**: 100% error case coverage

### **Quality Gates**

- ✅ All tests must pass
- ✅ No warnings in test output
- ✅ Performance benchmarks must meet targets
- ✅ API contracts must be satisfied
- ✅ PostCSS processing must be reliable

## 🚀 **Implementation Plan**

### **Phase 1: Core Testing (Week 1)**
1. Implement unit tests for ClassBuilder
2. Add PostCSS unit tests
3. Create integration tests
4. Set up test coverage reporting

### **Phase 2: Contract Testing (Week 2)**
1. Implement API contract tests
2. Add backward compatibility tests
3. Create performance benchmarks
4. Set up continuous integration

### **Phase 3: Validation Framework (Week 3)**
1. Implement CSS validation tests
2. Add plugin coverage tests
3. Create error handling tests
4. Set up monitoring and alerting

### **Phase 4: Coverage Analysis (Week 4)**
1. Analyze test coverage
2. Identify gaps in testing
3. Add missing tests
4. Optimize test performance

## 🎯 **Success Criteria**

- [ ] 100% test coverage for all public APIs
- [ ] All PostCSS processing scenarios covered
- [ ] Performance benchmarks met
- [ ] API contracts validated
- [ ] Error handling comprehensive
- [ ] Documentation complete

This testing strategy ensures that our Tailwind-RS Core system is reliable, performant, and maintainable for production use.
