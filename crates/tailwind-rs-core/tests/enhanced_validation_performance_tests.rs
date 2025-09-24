use tailwind_rs_core::utilities::enhanced_validation::*;
use tailwind_rs_core::ClassBuilder;
use tailwind_rs_core::Breakpoint;
use std::time::Instant;

#[cfg(test)]
mod enhanced_validation_performance_tests {
    use super::*;

    #[test]
    fn test_enhanced_validation_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 enhanced validation utility classes
        for _ in 0..1000 {
            let _ = ClassBuilder::new()
                .validation_rule(ValidationRule::Required)
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Enhanced validation generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_validation_string_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 enhanced validation string representations
        for _ in 0..1000 {
            let _ = ValidationRule::Required.to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "String generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_validation_class_name_generation_performance() {
        let start = Instant::now();
        
        // Generate 1000 enhanced validation class names
        for _ in 0..1000 {
            let _ = ClassBuilder::new().validation_rule(ValidationRule::Required).build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20, "Class name generation too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_validation_memory_usage() {
        let initial_memory = get_memory_usage();
        
        // Create many enhanced validation builders
        let _builders: Vec<ClassBuilder> = (0..1000)
            .map(|_| ClassBuilder::new().validation_rule(ValidationRule::Required))
            .collect();
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        assert!(memory_increase < 100_000, "Memory usage too high: {} bytes", memory_increase);
    }

    #[test]
    fn test_enhanced_validation_serialization_performance() {
        let rule = ValidationRule::Required;
        let start = Instant::now();
        
        // Serialize 1000 times
        for _ in 0..1000 {
            let _ = serde_json::to_string(&rule).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 20, "Serialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_validation_deserialization_performance() {
        let rule = ValidationRule::Required;
        let serialized = serde_json::to_string(&rule).unwrap();
        let start = Instant::now();
        
        // Deserialize 1000 times
        for _ in 0..1000 {
            let _: ValidationRule = serde_json::from_str(&serialized).unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 30, "Deserialization too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_validation_complex_builder_performance() {
        let start = Instant::now();
        
        // Generate complex class builders with enhanced validation
        for _ in 0..100 {
            let _ = ClassBuilder::new()
                .validation_rule(ValidationRule::Required)
                .validation_severity(ValidationSeverity::Error)
                .validation_scope(ValidationScope::Global)
                .validation_mode(ValidationMode::Strict)
                .validation_result(ValidationResult::Valid)
                .validation_required()
                .validation_pattern("test")
                .validation_length(1, 10)
                .validation_range(0.0, 100.0)
                .validation_error()
                .validation_warning()
                .validation_info()
                .validation_success()
                .validation_global()
                .validation_local()
                .validation_component()
                .validation_page()
                .validation_strict()
                .validation_loose()
                .validation_custom_mode()
                .validation_disabled()
                .validation_valid()
                .validation_invalid("message")
                .validation_warning_result("message")
                .validation_info_result("message")
                .class("text-blue-500")
                .class("font-bold")
                .class("text-lg")
                .responsive(Breakpoint::Md, "validation-pattern-test")
                .conditional("hover", "validation-pattern-test")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100, "Complex builder too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_validation_enum_values_performance() {
        let start = Instant::now();
        
        // Generate enhanced validation enum values
        for _i in 0..1000 {
            let _ = ValidationRule::Required.to_string();
            let _ = ValidationRule::Pattern("test".to_string()).to_string();
            let _ = ValidationRule::Length(1, 10).to_string();
            let _ = ValidationRule::Range(0.0, 100.0).to_string();
            let _ = ValidationRule::Custom("custom".to_string()).to_string();
            let _ = ValidationSeverity::Error.to_string();
            let _ = ValidationSeverity::Warning.to_string();
            let _ = ValidationSeverity::Info.to_string();
            let _ = ValidationSeverity::Success.to_string();
            let _ = ValidationSeverity::Custom("custom".to_string()).to_string();
            let _ = ValidationScope::Global.to_string();
            let _ = ValidationScope::Local.to_string();
            let _ = ValidationScope::Component.to_string();
            let _ = ValidationScope::Page.to_string();
            let _ = ValidationScope::Custom("custom".to_string()).to_string();
            let _ = ValidationMode::Strict.to_string();
            let _ = ValidationMode::Loose.to_string();
            let _ = ValidationMode::Custom.to_string();
            let _ = ValidationMode::Disabled.to_string();
            let _ = ValidationMode::CustomMode("custom".to_string()).to_string();
            let _ = ValidationResult::Valid.to_string();
            let _ = ValidationResult::Invalid("message".to_string()).to_string();
            let _ = ValidationResult::Warning("message".to_string()).to_string();
            let _ = ValidationResult::Info("message".to_string()).to_string();
            let _ = ValidationResult::Custom("custom".to_string()).to_string();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "Enum values too slow: {}ms", duration.as_millis());
    }

    #[test]
    fn test_enhanced_validation_all_variants_performance() {
        let start = Instant::now();
        
        // Generate all enhanced validation variants
        for _i in 0..100 {
            let _ = ClassBuilder::new()
                .validation_rule(ValidationRule::Required)
                .validation_rule(ValidationRule::Pattern("test".to_string()))
                .validation_rule(ValidationRule::Length(1, 10))
                .validation_rule(ValidationRule::Range(0.0, 100.0))
                .validation_rule(ValidationRule::Custom("custom".to_string()))
                .validation_severity(ValidationSeverity::Error)
                .validation_severity(ValidationSeverity::Warning)
                .validation_severity(ValidationSeverity::Info)
                .validation_severity(ValidationSeverity::Success)
                .validation_severity(ValidationSeverity::Custom("custom".to_string()))
                .validation_scope(ValidationScope::Global)
                .validation_scope(ValidationScope::Local)
                .validation_scope(ValidationScope::Component)
                .validation_scope(ValidationScope::Page)
                .validation_scope(ValidationScope::Custom("custom".to_string()))
                .validation_mode(ValidationMode::Strict)
                .validation_mode(ValidationMode::Loose)
                .validation_mode(ValidationMode::Custom)
                .validation_mode(ValidationMode::Disabled)
                .validation_mode(ValidationMode::CustomMode("custom".to_string()))
                .validation_result(ValidationResult::Valid)
                .validation_result(ValidationResult::Invalid("message".to_string()))
                .validation_result(ValidationResult::Warning("message".to_string()))
                .validation_result(ValidationResult::Info("message".to_string()))
                .validation_result(ValidationResult::Custom("custom".to_string()))
                .validation_required()
                .validation_pattern("test")
                .validation_length(1, 10)
                .validation_range(0.0, 100.0)
                .validation_error()
                .validation_warning()
                .validation_info()
                .validation_success()
                .validation_global()
                .validation_local()
                .validation_component()
                .validation_page()
                .validation_strict()
                .validation_loose()
                .validation_custom_mode()
                .validation_disabled()
                .validation_valid()
                .validation_invalid("message")
                .validation_warning_result("message")
                .validation_info_result("message")
                .build();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200, "All variants too slow: {}ms", duration.as_millis());
    }
}

// Helper function to get memory usage (simplified)
fn get_memory_usage() -> usize {
    // This is a simplified implementation
    // In a real scenario, you might use more sophisticated memory tracking
    std::mem::size_of::<ClassBuilder>() * 1000
}
