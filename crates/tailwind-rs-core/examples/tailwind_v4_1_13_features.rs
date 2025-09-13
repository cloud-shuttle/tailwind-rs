//! # Tailwind CSS v4.1.13 Features Example
//! 
//! This example demonstrates the new Tailwind v4.1.13 features implemented in our Rust-native implementation:
//! - @custom-variant support (replacing aria, data, supports theme keys)
//! - Enhanced variant validation and suggestions
//! - Custom variant restrictions (no - or _ at start/end)
//! - Better error handling for unknown named values

use tailwind_rs_core::{
    ClassBuilder, CustomVariant, CustomVariantManager, CustomVariantType,
    ClassValidator
};

fn main() {
    println!("🎨 Tailwind CSS v4.1.13 Features Demo");
    println!("=====================================\n");

    // 1. Custom Variant System (@custom-variant support)
    demo_custom_variants();
    
    // 2. Enhanced ClassBuilder with Custom Variants
    demo_enhanced_class_builder();
    
    // 3. Custom Variant Validation
    demo_custom_variant_validation();
    
    // 4. Custom Variant Manager
    demo_custom_variant_manager();
    
    // 5. Error Handling and Suggestions
    demo_error_handling_and_suggestions();
}

fn demo_custom_variants() {
    println!("1. 🆕 Custom Variant System (@custom-variant support)");
    println!("---------------------------------------------------");
    
    // Create ARIA variants (migrated from old aria theme keys)
    let aria_checked = CustomVariant::aria("checked".to_string(), None).unwrap();
    let aria_disabled = CustomVariant::aria("disabled".to_string(), None).unwrap();
    
    // Create data variants (migrated from old data theme keys)
    let data_theme_dark = CustomVariant::data("theme".to_string(), Some("dark".to_string())).unwrap();
    let data_state_loading = CustomVariant::data("state".to_string(), Some("loading".to_string())).unwrap();
    
    // Create supports variants (migrated from old supports theme keys)
    let supports_grid = CustomVariant::supports("grid".to_string(), None).unwrap();
    let supports_backdrop_filter = CustomVariant::supports("backdrop-filter".to_string(), None).unwrap();
    
    // Create custom variants
    let custom_theme = CustomVariant::custom("theme-mode".to_string(), Some("dark".to_string())).unwrap();
    
    println!("✅ ARIA Variants:");
    println!("   - {} -> {}", aria_checked.to_variant_string(), aria_checked.to_css_selector());
    println!("   - {} -> {}", aria_disabled.to_variant_string(), aria_disabled.to_css_selector());
    
    println!("✅ Data Variants:");
    println!("   - {} -> {}", data_theme_dark.to_variant_string(), data_theme_dark.to_css_selector());
    println!("   - {} -> {}", data_state_loading.to_variant_string(), data_state_loading.to_css_selector());
    
    println!("✅ Supports Variants:");
    println!("   - {} -> {}", supports_grid.to_variant_string(), supports_grid.to_css_selector());
    println!("   - {} -> {}", supports_backdrop_filter.to_variant_string(), supports_backdrop_filter.to_css_selector());
    
    println!("✅ Custom Variants:");
    println!("   - {} -> {}", custom_theme.to_variant_string(), custom_theme.to_css_selector());
    
    println!();
}

fn demo_enhanced_class_builder() {
    println!("2. 🚀 Enhanced ClassBuilder with Custom Variants");
    println!("-----------------------------------------------");
    
    // Build classes with custom variants (Tailwind v4.1.13 style)
    let classes = ClassBuilder::new()
        // Base classes
        .class("px-4 py-2")
        .class("bg-blue-600 text-white")
        .class("rounded-lg")
        
        // ARIA variants (new @custom-variant support)
        .aria("checked", "bg-green-500")
        .aria("disabled", "opacity-50 cursor-not-allowed")
        
        // Data variants (new @custom-variant support)
        .data("theme", Some("dark".to_string()), "bg-gray-800 text-white")
        .data("state", Some("loading".to_string()), "animate-pulse")
        
        // Supports variants (new @custom-variant support)
        .supports("grid", "grid grid-cols-2")
        .supports("backdrop-filter", "backdrop-blur-sm")
        
        // Custom variants
        .custom_variant("theme-mode=dark", "bg-gray-900 text-gray-100")
        .custom_variant("theme-mode=light", "bg-white text-gray-900")
        
        .build()
        .to_css_classes();
    
    println!("✅ Generated CSS Classes:");
    println!("   {}", classes);
    println!();
}

fn demo_custom_variant_validation() {
    println!("3. 🔍 Custom Variant Validation");
    println!("------------------------------");
    
    let validator = ClassValidator::new();
    
    // Valid custom variants
    let valid_variants = vec![
        "aria-checked",
        "data-theme=dark",
        "supports-grid",
        "container-sm",
        "media-print",
    ];
    
    println!("✅ Valid Custom Variants:");
    for variant in valid_variants {
        match validator.validate_custom_variant(variant) {
            Ok(_) => println!("   ✓ {}", variant),
            Err(e) => println!("   ✗ {} - {}", variant, e),
        }
    }
    
    // Invalid custom variants (v4.1.13 restrictions)
    let invalid_variants = vec![
        "@-invalid",        // Cannot start with @-
        "-invalid",         // Cannot start with -
        "invalid-",         // Cannot end with -
        "_invalid",         // Cannot start with _
        "invalid_",         // Cannot end with _
    ];
    
    println!("\n❌ Invalid Custom Variants (v4.1.13 restrictions):");
    for variant in invalid_variants {
        match validator.validate_custom_variant(variant) {
            Ok(_) => println!("   ✓ {} (unexpectedly valid)", variant),
            Err(e) => println!("   ✗ {} - {}", variant, e),
        }
    }
    
    println!();
}

fn demo_custom_variant_manager() {
    println!("4. 🎛️  Custom Variant Manager");
    println!("----------------------------");
    
    let mut manager = CustomVariantManager::with_defaults();
    
    // Register additional custom variants
    let custom_variants = vec![
        CustomVariant::aria("expanded".to_string(), None).unwrap(),
        CustomVariant::data("size".to_string(), Some("large".to_string())).unwrap(),
        CustomVariant::supports("flexbox".to_string(), None).unwrap(),
        CustomVariant::custom("user-preference".to_string(), Some("reduced-motion".to_string())).unwrap(),
    ];
    
    for variant in custom_variants {
        if let Err(e) = manager.register(variant) {
            println!("   ⚠️  Registration failed: {}", e);
        }
    }
    
    // Get variants by type
    let aria_variants = manager.get_by_type(&CustomVariantType::Aria);
    let data_variants = manager.get_by_type(&CustomVariantType::Data);
    
    println!("✅ ARIA Variants ({}):", aria_variants.len());
    for variant in aria_variants {
        println!("   - {}", variant.to_variant_string());
    }
    
    println!("✅ Data Variants ({}):", data_variants.len());
    for variant in data_variants {
        println!("   - {}", variant.to_variant_string());
    }
    
    // Add known values for suggestions
    let mut known_values = std::collections::HashSet::new();
    known_values.insert("dark".to_string());
    known_values.insert("light".to_string());
    known_values.insert("auto".to_string());
    manager.add_known_values("data-theme".to_string(), known_values);
    
    println!();
}

fn demo_error_handling_and_suggestions() {
    println!("5. 🚨 Error Handling and Suggestions");
    println!("-----------------------------------");
    
    let validator = ClassValidator::new();
    
    // Get suggestions for partial variants
    let suggestions = validator.get_variant_suggestions("aria-");
    println!("✅ Suggestions for 'aria-':");
    for suggestion in suggestions.iter().take(5) {
        println!("   - {}", suggestion);
    }
    
    // Validate variant-class combinations
    let variant_class_pairs = vec![
        ("aria-checked", "bg-green-500"),
        ("data-theme=dark", "bg-gray-800"),
        ("supports-grid", "grid"),
        ("invalid-variant", "bg-red-500"),
    ];
    
    println!("\n✅ Variant-Class Validation:");
    for (variant, class) in variant_class_pairs {
        match validator.validate_variant_class(variant, class) {
            Ok(_) => println!("   ✓ {}:{}", variant, class),
            Err(e) => println!("   ✗ {}:{} - {}", variant, class, e),
        }
    }
    
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_custom_variant_creation() {
        // Test ARIA variant creation
        let aria_variant = CustomVariant::aria("checked".to_string(), None).unwrap();
        assert_eq!(aria_variant.to_variant_string(), "aria-checked");
        assert_eq!(aria_variant.to_css_selector(), "[aria-checked]");
        
        // Test data variant with value
        let data_variant = CustomVariant::data("theme".to_string(), Some("dark".to_string())).unwrap();
        assert_eq!(data_variant.to_variant_string(), "data-theme=dark");
        assert_eq!(data_variant.to_css_selector(), "[data-theme=dark]");
    }
    
    #[test]
    fn test_custom_variant_validation() {
        let validator = ClassValidator::new();
        
        // Valid variants should pass
        assert!(validator.validate_custom_variant("aria-checked").is_ok());
        assert!(validator.validate_custom_variant("data-theme=dark").is_ok());
        
        // Invalid variants should fail
        assert!(validator.validate_custom_variant("@-invalid").is_err());
        assert!(validator.validate_custom_variant("-invalid").is_err());
        assert!(validator.validate_custom_variant("invalid-").is_err());
    }
    
    #[test]
    fn test_enhanced_class_builder() {
        let classes = ClassBuilder::new()
            .class("px-4 py-2")
            .aria("checked", "bg-green-500")
            .data("theme", Some("dark".to_string()), "bg-gray-800")
            .supports("grid", "grid")
            .build()
            .to_css_classes();
        
        // Should contain all the classes
        assert!(classes.contains("px-4"));
        assert!(classes.contains("py-2"));
        assert!(classes.contains("aria-checked:bg-green-500"));
        assert!(classes.contains("data-theme=dark:bg-gray-800"));
        assert!(classes.contains("supports-grid:grid"));
    }
    
    #[test]
    fn test_custom_variant_manager() {
        let mut manager = CustomVariantManager::new();
        
        // Register a custom variant
        let variant = CustomVariant::aria("checked".to_string(), None).unwrap();
        manager.register(variant).unwrap();
        
        // Should be able to retrieve it
        assert!(manager.contains("aria-checked"));
        assert!(manager.get("aria-checked").is_some());
        
        // Should get suggestions
        let suggestions = manager.get_suggestions("aria-");
        assert!(suggestions.contains(&"aria-checked".to_string()));
    }
}
